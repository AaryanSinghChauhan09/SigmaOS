// OOP-based Desktop Notification for SigmaOS with elementaryOS Human Interface Guidelines (HIG)
// Implements focus-first notifications (Do Not Disturb) and silent notification centers

// #![no_std]

use core::sync::atomic::AtomicUsize;

pub type NotificationID = usize;
pub const MAX_NOTIFICATIONS: usize = 16;
pub const MAX_EXCEPTIONS: usize = 8;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low = 0,
    Normal = 1,
    Critical = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationError {
    Success = 0,
    NotFound = 1,
    QueueFull = 2,
    SilentlyQueued = 3,
}

pub trait Notification {
    fn id(&self) -> NotificationID;
    fn title(&self) -> &[u8];
    fn body(&self) -> &[u8];
    fn urgency(&self) -> NotificationUrgency;
    fn app_name(&self) -> &[u8];
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleNotification {
    pub id: NotificationID,
    pub title: [u8; 128],
    pub title_len: usize,
    pub body: [u8; 512],
    pub body_len: usize,
    pub urgency: NotificationUrgency,
    pub app_name: [u8; 64],
    pub app_name_len: usize,
}

impl SimpleNotification {
    pub fn new(
        id: NotificationID,
        title: &[u8],
        body: &[u8],
        urgency: NotificationUrgency,
        app_name: &[u8],
    ) -> Self {
        let mut title_array = [0u8; 128];
        let mut body_array = [0u8; 512];
        let mut app_array = [0u8; 64];

        let title_len = title.len().min(128);
        let body_len = body.len().min(512);
        let app_len = app_name.len().min(64);

        title_array[..title_len].copy_from_slice(&title[..title_len]);
        body_array[..body_len].copy_from_slice(&body[..body_len]);
        app_array[..app_len].copy_from_slice(&app_name[..app_len]);

        SimpleNotification {
            id,
            title: title_array,
            title_len,
            body: body_array,
            body_len,
            urgency,
            app_name: app_array,
            app_name_len: app_len,
        }
    }
}

impl Notification for SimpleNotification {
    fn id(&self) -> NotificationID {
        self.id
    }

    fn title(&self) -> &[u8] {
        &self.title[..self.title_len]
    }

    fn body(&self) -> &[u8] {
        &self.body[..self.body_len]
    }

    fn urgency(&self) -> NotificationUrgency {
        self.urgency
    }

    fn app_name(&self) -> &[u8] {
        &self.app_name[..self.app_name_len]
    }
}

pub trait NotificationManager {
    fn show_notification(&mut self, notification: SimpleNotification) -> Result<NotificationID, NotificationError>;
    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError>;
    fn get_notification(&self, id: NotificationID) -> Option<&SimpleNotification>;
}

pub struct SimpleNotificationManager {
    pub notifications: [Option<SimpleNotification>; MAX_NOTIFICATIONS],
    pub notification_center: [Option<SimpleNotification>; MAX_NOTIFICATIONS],
    pub next_id: AtomicUsize,
    pub focus_mode_enabled: bool,
    pub exceptions: [[u8; 64]; MAX_EXCEPTIONS],
    pub exception_lens: [usize; MAX_EXCEPTIONS],
    pub exception_count: usize,
}

impl SimpleNotificationManager {
    pub fn new() -> Self {
        SimpleNotificationManager {
            notifications: [None; MAX_NOTIFICATIONS],
            notification_center: [None; MAX_NOTIFICATIONS],
            next_id: AtomicUsize::new(1),
            focus_mode_enabled: false,
            exceptions: [[0u8; 64]; MAX_EXCEPTIONS],
            exception_lens: [0; MAX_EXCEPTIONS],
            exception_count: 0,
        }
    }

    pub fn enable_focus_mode(&mut self, enabled: bool) {
        self.focus_mode_enabled = enabled;
    }

    pub fn is_focus_mode_enabled(&self) -> bool {
        self.focus_mode_enabled
    }

    pub fn add_focus_exception(&mut self, app_name: &[u8]) -> Result<(), &'static str> {
        if self.exception_count >= MAX_EXCEPTIONS {
            return Err("Maximum focus exceptions reached");
        }
        let len = app_name.len().min(64);
        self.exceptions[self.exception_count][..len].copy_from_slice(&app_name[..len]);
        self.exception_lens[self.exception_count] = len;
        self.exception_count += 1;
        Ok(())
    }

    pub fn is_focus_exception(&self, app_name: &[u8]) -> bool {
        for i in 0..self.exception_count {
            let len = self.exception_lens[i];
            if len == app_name.len() && &self.exceptions[i][..len] == app_name {
                return true;
            }
        }
        false
    }
}

impl Default for SimpleNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationManager for SimpleNotificationManager {
    fn show_notification(&mut self, notification: SimpleNotification) -> Result<NotificationID, NotificationError> {
        // Under Focus Mode (Do Not Disturb), filter standard notifications
        if self.focus_mode_enabled {
            let is_critical = notification.urgency == NotificationUrgency::Critical;
            let is_exempt = self.is_focus_exception(notification.app_name());

            if !is_critical && !is_exempt {
                // Route silently to the Notification Center queue instead of the active display
                for slot in self.notification_center.iter_mut() {
                    if slot.is_none() {
                        *slot = Some(notification);
                        return Err(NotificationError::SilentlyQueued);
                    }
                }
                return Err(NotificationError::QueueFull);
            }
        }

        // Add to active notifications array
        for slot in self.notifications.iter_mut() {
            if slot.is_none() {
                *slot = Some(notification);
                return Ok(notification.id);
            }
        }
        Err(NotificationError::QueueFull)
    }

    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError> {
        for slot in self.notifications.iter_mut() {
            if let Some(ref notification) = *slot {
                if notification.id == id {
                    *slot = None;
                    return Ok(());
                }
            }
        }
        for slot in self.notification_center.iter_mut() {
            if let Some(ref notification) = *slot {
                if notification.id == id {
                    *slot = None;
                    return Ok(());
                }
            }
        }
        Err(NotificationError::NotFound)
    }

    fn get_notification(&self, id: NotificationID) -> Option<&SimpleNotification> {
        for slot in self.notifications.iter() {
            if let Some(ref notification) = *slot {
                if notification.id == id {
                    return Some(notification);
                }
            }
        }
        for slot in self.notification_center.iter() {
            if let Some(ref notification) = *slot {
                if notification.id == id {
                    return Some(notification);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_notification_delivery() {
        let mut manager = SimpleNotificationManager::new();
        let notification = SimpleNotification::new(
            1,
            b"Calendar Invite",
            b"Meeting at 2 PM",
            NotificationUrgency::Normal,
            b"io.elementary.calendar",
        );

        assert_eq!(manager.show_notification(notification), Ok(1));
        assert!(manager.get_notification(1).is_some());
    }

    #[test]
    fn test_elementary_focus_mode_filter() {
        let mut manager = SimpleNotificationManager::new();
        manager.enable_focus_mode(true);

        // 1. Normal notification from non-exempt app is filtered to the notification center
        let normal_notif = SimpleNotification::new(
            1,
            b"New Message",
            b"Hello there",
            NotificationUrgency::Normal,
            b"io.elementary.mail",
        );
        assert_eq!(
            manager.show_notification(normal_notif),
            Err(NotificationError::SilentlyQueued)
        );

        // 2. Critical notification bypasses the Focus Mode filter
        let critical_notif = SimpleNotification::new(
            2,
            b"Low Battery",
            b"10% remaining",
            NotificationUrgency::Critical,
            b"org.gnome.power",
        );
        assert_eq!(manager.show_notification(critical_notif), Ok(2));

        // 3. Exempt application notification bypasses Focus Mode
        manager.add_focus_exception(b"io.elementary.mail").unwrap();
        let normal_exempt_notif = SimpleNotification::new(
            3,
            b"Urgent Mail",
            b"High priority server alert",
            NotificationUrgency::Normal,
            b"io.elementary.mail",
        );
        assert_eq!(manager.show_notification(normal_exempt_notif), Ok(3));
    }
}
