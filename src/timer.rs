use core::ffi::c_void;
use core::marker::PhantomData;

use flipperzero::furi::time::FuriDuration;
use flipperzero_sys as sys;

/// `on_tick` runs on the timer thread shared by the whole system and must not block.
///
/// Leaking the timer (`mem::forget`, an `Rc` cycle) leaves the callback attached.
pub struct PeriodicTimer<'a> {
    timer: *mut sys::FuriTimer,
    _callback: PhantomData<&'a ()>,
}

impl<'a> PeriodicTimer<'a> {
    #[must_use]
    pub fn new<F: Fn() + Sync>(on_tick: &'a F) -> Self {
        unsafe extern "C" fn trampoline<F: Fn()>(context: *mut c_void) {
            // SAFETY: `context` is the `&F` registered below, alive while attached.
            let on_tick: &F = unsafe { &*context.cast() };

            on_tick();
        }

        let timer = unsafe {
            sys::furi_timer_alloc(
                Some(trampoline::<F>),
                sys::FuriTimerTypePeriodic,
                (on_tick as *const F).cast_mut().cast::<c_void>(),
            )
        };

        Self {
            timer,
            _callback: PhantomData,
        }
    }

    pub fn start(&mut self, period: FuriDuration) {
        assert!(FuriDuration::ZERO < period);
        assert!(FuriDuration::WAIT_FOREVER != period);
        unsafe { sys::furi_timer_start(self.timer, period.as_ticks()) };
    }

    pub fn restart(&mut self, period: FuriDuration) {
        assert!(FuriDuration::ZERO < period);
        assert!(FuriDuration::WAIT_FOREVER != period);
        unsafe { sys::furi_timer_restart(self.timer, period.as_ticks()) };
    }

    pub fn stop(&mut self) {
        unsafe {
            sys::furi_timer_stop(self.timer);
        }
    }
}

impl Drop for PeriodicTimer<'_> {
    fn drop(&mut self) {
        unsafe {
            sys::furi_timer_stop(self.timer);
            sys::furi_timer_free(self.timer);
        }
    }
}
