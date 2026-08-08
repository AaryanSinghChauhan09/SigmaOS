#![no_std]

/// OOP-based Desktop Notification for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 746
/// Implements notification system and alerts

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type NotificationID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency { Low = 0, Normal = 1, Critical = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationError { Success = 0, NotFound = 1 }

pub trait Notification {
    fn id(&self) -> NotificationID;
    fn title(&self) -> &[u8];
    fn body(&self) -> &[u8];
    fn urgency(&self) -> NotificationUrgency;
    fn app_name(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleNotification {
    pub id: NotificationID,
    pub title: [u8; 128],
    pub body: [u8; 512],
    pub urgency: AtomicUsize,
    pub app_name: [u8; 64],
}

impl SimpleNotification {
    pub fn new(id: NotificationID, title: &[u8], body: &[u8], urgency: NotificationUrgency, app_name: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut body_array = [0u8; 512];
        let mut app_array = [0u8; 64];
        let title_len = title.len().min(127);
        let body_len = body.len().min(511);
        let app_len = app_name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(body.as_ptr(), body_array.as_mut_ptr(), body_len);
            core::ptr::copy_nonoverlapping(app_name.as_ptr(), app_array.as_mut_ptr(), app_len);
        }
        SimpleNotification {
            id,
            title: title_array,
            body: body_array,
            urgency: AtomicUsize::new(urgency as usize),
            app_name: app_array,
        }
    }
}

impl Notification for SimpleNotification {
    fn id(&self) -> NotificationID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn body(&self) -> &[u8] {
        let len = self.body.iter().position(|&b| b == 0).unwrap_or(512);
        &self.body[..len]
    }
    fn urgency(&self) -> NotificationUrgency {
        let raw = self.urgency.load(Ordering::SeqCst);
        match raw {
            1 => NotificationUrgency::Normal,
            2 => NotificationUrgency::Critical,
            _ => NotificationUrgency::Low,
        }
    }
    fn app_name(&self) -> &[u8] {
        let len = self.app_name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.app_name[..len]
    }
}

pub trait NotificationManager {
    fn show_notification(&mut self, notification: Box<dyn Notification>) -> Result<NotificationID, NotificationError>;
    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError>;
    fn get_notification(&self, id: NotificationID) -> Option<&dyn Notification>;
    fn get_active_notifications(&self) -> Vec<NotificationID>;
}

#[repr(C)]
pub struct SimpleNotificationManager {
    pub notifications: Vec<Option<Box<dyn Notification>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNotificationManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleNotificationManager {
            notifications: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl NotificationManager for SimpleNotificationManager {
    fn show_notification(&mut self, notification: Box<dyn Notification>) -> Result<NotificationID, NotificationError> {
        let id = notification.id();
        self.notifications.push(Some(notification));
        Ok(id)
    }
    
    fn dismiss_notification(&mut self, id: NotificationID) -> Result<(), NotificationError> {
        for notification_option in &mut self.notifications {
            if let Some(ref notification) = *notification_option {
                if notification.id() == id {
                    *notification_option = None;
                    return Ok(());
                }
            }
        }
        Err(NotificationError::NotFound)
    }
    
    fn get_notification(&self, id: NotificationID) -> Option<&dyn Notification> {
        for notification_option in &self.notifications {
            if let Some(ref notification) = *notification_option {
                if notification.id() == id { return Some(notification.as_ref()); }
            }
        }
        None
    }
    
    fn get_active_notifications(&self) -> Vec<NotificationID> {
        let mut ids = Vec::new();
        for notification_option in &self.notifications {
            if let Some(ref notification) = *notification_option {
                ids.push(notification.id());
            }
        }
        ids
    }
}

pub trait DoNotDisturb {
    fn enable_dnd(&mut self, enabled: bool);
    fn is_dnd_enabled(&self) -> bool;
    fn set_dnd_exceptions(&mut self, app_name: &[u8]);
}

#[repr(C)]
pub struct SimpleDoNotDisturb {
    pub enabled: AtomicUsize,
    pub exceptions: Vec<[u8; 64]>,
}

impl SimpleDoNotDisturb {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleDoNotDisturb {
            enabled: AtomicUsize::new(0),
            exceptions: Vec::new(),
        }
    }
}

impl DoNotDisturb for SimpleDoNotDisturb {
    fn enable_dnd(&mut self, enabled: bool) {
        self.enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }
    
    fn is_dnd_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
    
    fn set_dnd_exceptions(&mut self, app_name: &[u8]) {
        let mut app_array = [0u8; 64];
        let app_len = app_name.len().min(63);
        for i in 0..app_len {
            app_array[i] = app_name[i];
        }
        self.exceptions.push(app_array);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn test_simple_notification_creation() {
        let notif = SimpleNotification::new(101, b"Hello", b"World", NotificationUrgency::Critical, b"TestApp");
        assert_eq!(notif.id(), 101);
        assert_eq!(notif.title(), b"Hello");
        assert_eq!(notif.body(), b"World");
        assert_eq!(notif.urgency(), NotificationUrgency::Critical);
        assert_eq!(notif.app_name(), b"TestApp");
    }

    #[test]
    fn test_simple_notification_manager() {
        let mut manager = SimpleNotificationManager::new();
        let notif1 = SimpleNotification::new(1, b"Title 1", b"Body 1", NotificationUrgency::Low, b"App 1");
        let notif2 = SimpleNotification::new(2, b"Title 2", b"Body 2", NotificationUrgency::Normal, b"App 2");

        assert_eq!(manager.show_notification(Box::new(notif1)).unwrap(), 1);
        assert_eq!(manager.show_notification(Box::new(notif2)).unwrap(), 2);

        let active = manager.get_active_notifications();
        assert_eq!(active.len(), 2);
        assert!(active.contains(&1));
        assert!(active.contains(&2));

        let retrieved = manager.get_notification(1).unwrap();
        assert_eq!(retrieved.title(), b"Title 1");

        assert!(manager.dismiss_notification(1).is_ok());
        assert!(manager.get_notification(1).is_none());
        assert_eq!(manager.get_active_notifications().len(), 1);
        assert_eq!(manager.dismiss_notification(99), Err(NotificationError::NotFound));
    }

    #[test]
    fn test_simple_do_not_disturb() {
        let mut dnd = SimpleDoNotDisturb::new();
        assert!(!dnd.is_dnd_enabled());

        dnd.enable_dnd(true);
        assert!(dnd.is_dnd_enabled());

        dnd.set_dnd_exceptions(b"ExceptionApp");
        assert_eq!(dnd.exceptions.len(), 1);

        let mut expected = [0u8; 64];
        expected[..12].copy_from_slice(b"ExceptionApp");
        assert_eq!(dnd.exceptions[0], expected);
    }
}
