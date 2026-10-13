use flipperzero::notification::{NotificationApp, NotificationMessage, NotificationSequence};
use flipperzero::notification_sequence;

pub struct BacklightLock(());

impl BacklightLock {
    #[must_use]
    pub fn acquire() -> Self {
        const BACKLIGHT_ON: NotificationSequence =
            notification_sequence![NotificationMessage::display_backlight_enforce_on()];
        NotificationApp::open().notify(&BACKLIGHT_ON);

        Self(())
    }
}

impl Drop for BacklightLock {
    fn drop(&mut self) {
        const BACKLIGHT_AUTO: NotificationSequence =
            notification_sequence![NotificationMessage::display_backlight_enforce_auto()];
        NotificationApp::open().notify(&BACKLIGHT_AUTO);
    }
}
