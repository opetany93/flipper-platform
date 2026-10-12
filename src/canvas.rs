//! Drawing on the canvas a view port hands to its draw callback.

use core::ffi::CStr;
use core::marker::PhantomData;

use flipperzero_sys as sys;

#[derive(Clone, Copy)]
pub enum Font {
    Bold,
    Regular,
    Small,
    BigNumbers,
}

impl Font {
    const fn to_sys(self) -> sys::Font {
        match self {
            Self::Bold => sys::FontPrimary,
            Self::Regular => sys::FontSecondary,
            Self::Small => sys::FontKeyboard,
            Self::BigNumbers => sys::FontBigNumbers,
        }
    }
}

#[derive(Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Right,
}

impl HorizontalAlign {
    const fn to_sys(self) -> sys::Align {
        match self {
            Self::Left => sys::AlignLeft,
            Self::Right => sys::AlignRight,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Bottom,
}

impl VerticalAlign {
    const fn to_sys(self) -> sys::Align {
        match self {
            Self::Top => sys::AlignTop,
            Self::Bottom => sys::AlignBottom,
        }
    }
}

/// A borrowed drawing surface, valid for the duration of one draw callback.
///
/// The lifetime is what stops a canvas from being stashed somewhere and used
/// after the GUI service has moved on.
pub struct Canvas<'a> {
    raw: *mut sys::Canvas,
    _frame: PhantomData<&'a mut sys::Canvas>,
}

impl Canvas<'_> {
    /// Wraps the canvas handed to a draw callback.
    ///
    /// # Safety
    ///
    /// `raw` must be a valid canvas, and the returned lifetime must not outlive
    /// the callback it was handed to.
    pub(crate) unsafe fn from_raw(raw: *mut sys::Canvas) -> Self {
        Self {
            raw,
            _frame: PhantomData,
        }
    }

    // SAFETY, for every `unsafe` below: `self.raw` is valid for the lifetime of
    // `self`, per the contract of `from_raw`.

    pub fn clear(&mut self) {
        unsafe { sys::canvas_clear(self.raw) };
    }

    /// Selects the font for subsequent [`draw_str`](Self::draw_str) calls.
    pub fn set_font(&mut self, font: Font) {
        unsafe { sys::canvas_set_font(self.raw, font.to_sys()) };
    }

    /// `y` is the baseline, not the top edge
    pub fn draw_str(&mut self, x: i32, y: i32, text: &CStr) {
        unsafe { sys::canvas_draw_str(self.raw, x, y, text.as_ptr()) };
    }

    pub fn draw_str_aligned(
        &mut self,
        x: i32,
        y: i32,
        horizontal: HorizontalAlign,
        vertical: VerticalAlign,
        text: &CStr,
    ) {
        unsafe {
            sys::canvas_draw_str_aligned(
                self.raw,
                x,
                y,
                horizontal.to_sys(),
                vertical.to_sys(),
                text.as_ptr(),
            )
        };
    }

    pub fn draw_xbm(&mut self, x: i32, y: i32, width: usize, height: usize, bitmap: &[u8]) {
        assert!(
            bitmap.len() >= width.div_ceil(8) * height,
            "bitmap too short for {width}x{height}"
        );

        unsafe { sys::canvas_draw_xbm(self.raw, x, y, width, height, bitmap.as_ptr()) };
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        unsafe {
            sys::canvas_draw_line(self.raw, x1, y1, x2, y2);
        };
    }
}
