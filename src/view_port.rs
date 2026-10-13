use core::ffi::c_void;
use core::marker::PhantomData;

use flipperzero::gui::Gui;
use flipperzero_sys as sys;

use super::canvas::Canvas;
use super::input::InputEvent;

pub struct ViewPort<'a> {
    gui: Gui,
    raw: *mut sys::ViewPort,
    _callbacks: PhantomData<&'a ()>,
}

impl<'a> ViewPort<'a> {
    fn fullscreen<D, I>(on_draw: &'a D, on_input: &'a I) -> Self
    where
        D: Fn(&mut Canvas<'_>) + Sync,
        I: Fn(InputEvent) + Sync,
    {
        unsafe extern "C" fn draw_trampoline<D: Fn(&mut Canvas<'_>)>(
            canvas: *mut sys::Canvas,
            context: *mut c_void,
        ) {
            // SAFETY: `context` is the `&D` registered below, alive while attached.
            let on_draw: &D = unsafe { &*context.cast() };
            let mut canvas = unsafe { Canvas::from_raw(canvas) };

            on_draw(&mut canvas);
        }

        unsafe extern "C" fn input_trampoline<I: Fn(InputEvent)>(
            event: *mut sys::InputEvent,
            context: *mut c_void,
        ) {
            let on_input: &I = unsafe { &*context.cast() };
            let event = unsafe { &*event };

            if let Some(event) = InputEvent::from_sys(event) {
                on_input(event);
            }
        }

        let gui = Gui::open();

        let raw = unsafe {
            let raw = sys::view_port_alloc();
            sys::view_port_draw_callback_set(
                raw,
                Some(draw_trampoline::<D>),
                (on_draw as *const D).cast_mut().cast(),
            );
            sys::view_port_input_callback_set(
                raw,
                Some(input_trampoline::<I>),
                (on_input as *const I).cast_mut().cast(),
            );

            // Last: callbacks can fire from here on.
            sys::gui_add_view_port(gui.as_ptr(), raw, sys::GuiLayerFullscreen);

            raw
        };

        Self {
            gui,
            raw,
            _callbacks: PhantomData,
        }
    }

    /// `on_draw` and `on_input` run on the GUI thread and must not block.
    pub fn scoped<D, I, B>(on_draw: &'a D, on_input: &'a I, body: B)
    where
        D: Fn(&mut Canvas<'_>) + Sync,
        I: Fn(InputEvent) + Sync,
        B: FnOnce(&ViewPort),
    {
        let view_port = Self::fullscreen(on_draw, on_input);
        body(&view_port);
    }

    /// Returns before the redraw happens.
    pub fn request_redraw(&self) {
        unsafe { sys::view_port_update(self.raw) };
    }
}

impl Drop for ViewPort<'_> {
    fn drop(&mut self) {
        // Disable first: that stops further callbacks.
        unsafe {
            sys::view_port_enabled_set(self.raw, false);
            sys::gui_remove_view_port(self.gui.as_ptr(), self.raw);
            sys::view_port_free(self.raw);
        }
    }
}
