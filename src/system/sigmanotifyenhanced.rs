//! SigmaNotifyEnhanced - Enhanced Notification System
//!
//! Inspired by Linux Mint and modern desktop notification systems, this module provides
//! comprehensive notification management including notification categories, priorities,
//! persistence, and user interaction.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Notification category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    System,
    Application,
    Network,
    Security,
    Media,
    IM,
    Email,
    Calendar,
    Battery,
    Unknown,
}

/// Notification urgency enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
    Unknown,
}

/// Notification state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationState {
    Pending,
    Delivered,
    Dismissed,
    Expired,
    ActionTaken,
    Unknown,
}

/// Notification action data structure
#[derive(Debug, Clone)]
pub struct NotificationAction {
    pub action_id: String,
    pub label: String,
    pub action_type: ActionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    Default,
    Reply,
    Custom,
    Unknown,
}

/// Notification data structure
#[derive(Debug, Clone)]
pub struct Notification {
    pub notification_id: String,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub category: NotificationCategory,
    pub urgency: NotificationUrgency,
    pub state: NotificationState,
    pub timeout_ms: u32,
    pub resident: bool,
    pub actions: Vec<NotificationAction>,
    pub creation_time: u64,
    pub expire_time: u64,
}

impl Notification {
    /// Create a new notification
    pub fn new(
        notification_id: String,
        app_name: String,
        summary: String,
        body: String,
        category: NotificationCategory,
    ) -> Self {
        Self {
            notification_id,
            app_name,
            app_icon: String::new(),
            summary,
            body,
            category,
            urgency: NotificationUrgency::Normal,
            state: NotificationState::Pending,
            timeout_ms: 5000,
            resident: false,
            actions: Vec::new(),
            creation_time: 0,
            expire_time: 0,
        }
    }

    /// Set urgency
    pub fn set_urgency(&mut self, urgency: NotificationUrgency) {
        self.urgency = urgency;
    }

    /// Set timeout
    pub fn set_timeout(&mut self, timeout_ms: u32) {
        self.timeout_ms = timeout_ms;
    }

    /// Set resident (persistent)
    pub fn set_resident(&mut self, resident: bool) {
        self.resident = resident;
    }

    /// Add action
    pub fn add_action(&mut self, action: NotificationAction) {
        self.actions.push(action);
    }

    /// Set state
    pub fn set_state(&mut self, state: NotificationState) {
        self.state = state;
    }

    /// Set creation time
    pub fn set_creation_time(&mut self, creation_time: u64) {
        self.creation_time = creation_time;
    }

    /// Set expire time
    pub fn set_expire_time(&mut self, expire_time: u64) {
        self.expire_time = expire_time;
    }

    /// Check if notification is resident
    pub fn is_resident(&self) -> bool {
        self.resident
    }

    /// Check if notification is expired
    pub fn is_expired(&self, current_time: u64) -> bool {
        if self.resident {
            false
        } else if self.expire_time > 0 {
            current_time >= self.expire_time
        } else {
            false
        }
    }
}

/// Enhanced notification manager
pub struct SigmaNotifyEnhanced {
    notifications: BTreeMap<String, Notification>,
    running: AtomicBool,
}

impl SigmaNotifyEnhanced {
    /// Create a new enhanced notification manager
    pub fn new() -> Self {
        Self {
            notifications: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the notification manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the notification manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Post a notification
    pub fn post_notification(&mut self, notification: Notification) -> bool {
        let notif_id = notification.notification_id.clone();
        self.notifications.insert(notif_id, notification);
        true
    }

    /// Dismiss a notification
    pub fn dismiss_notification(&mut self, notification_id: &str) -> bool {
        if let Some(notif) = self.notifications.get_mut(notification_id) {
            notif.set_state(NotificationState::Dismissed);
            true
        } else {
            false
        }
    }

    /// Invoke notification action
    pub fn invoke_action(&mut self, notification_id: &str, action_id: &str) -> bool {
        if let Some(notif) = self.notifications.get_mut(notification_id) {
            notif.set_state(NotificationState::ActionTaken);
            true
        } else {
            false
        }
    }

    /// Get a notification by ID
    pub fn get_notification(&self, notification_id: &str) -> Option<&Notification> {
        self.notifications.get(notification_id)
    }

    /// List all notifications
    pub fn list_notifications(&self) -> Vec<&Notification> {
        self.notifications.values().collect()
    }

    /// List notifications by category
    pub fn list_notifications_by_category(
        &self,
        category: NotificationCategory,
    ) -> Vec<&Notification> {
        self.notifications
            .values()
            .filter(|n| n.category == category)
            .collect()
    }

    /// List notifications by urgency
    pub fn list_notifications_by_urgency(
        &self,
        urgency: NotificationUrgency,
    ) -> Vec<&Notification> {
        self.notifications
            .values()
            .filter(|n| n.urgency == urgency)
            .collect()
    }

    /// List notifications by state
    pub fn list_notifications_by_state(&self, state: NotificationState) -> Vec<&Notification> {
        self.notifications
            .values()
            .filter(|n| n.state == state)
            .collect()
    }

    /// List pending notifications
    pub fn list_pending(&self) -> Vec<&Notification> {
        self.list_notifications_by_state(NotificationState::Pending)
    }

    /// List resident notifications
    pub fn list_resident(&self) -> Vec<&Notification> {
        self.notifications.values().filter(|n| n.resident).collect()
    }

    /// List critical notifications
    pub fn list_critical(&self) -> Vec<&Notification> {
        self.list_notifications_by_urgency(NotificationUrgency::Critical)
    }

    /// Remove a notification
    pub fn remove_notification(&mut self, notification_id: &str) -> Option<Notification> {
        self.notifications.remove(notification_id)
    }

    /// Clean up expired notifications
    pub fn cleanup_expired(&mut self, current_time: u64) -> usize {
        let mut expired_ids = Vec::new();

        for (id, notif) in &self.notifications {
            if notif.is_expired(current_time) {
                expired_ids.push(id.clone());
            }
        }

        for id in expired_ids {
            self.notifications.remove(&id);
        }

        expired_ids.len()
    }

    /// Get notification count
    pub fn notification_count(&self) -> usize {
        self.notifications.len()
    }
}

impl Default for SigmaNotifyEnhanced {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNotifyEnhanced instance (placeholder for actual global state)
static mut SIGMANOTIFYENHANCED_INSTANCE: Option<SigmaNotifyEnhanced> = None;

/// Initialize the global SigmaNotifyEnhanced instance
pub fn init_sigmanotifyenhanced() {
    unsafe {
        SIGMANOTIFYENHANCED_INSTANCE = Some(SigmaNotifyEnhanced::new());
    }
}

/// Get the global SigmaNotifyEnhanced instance
pub fn get_sigmanotifyenhanced() -> Option<&'static SigmaNotifyEnhanced> {
    unsafe { SIGMANOTIFYENHANCED_INSTANCE.as_ref() }
}

/// Get mutable global SigmaNotifyEnhanced instance
pub fn get_sigmanotifyenhanced_mut() -> Option<&'static mut SigmaNotifyEnhanced> {
    unsafe { SIGMANOTIFYENHANCED_INSTANCE.as_mut() }
}

/// Convenience function to post a notification
pub fn post_notification(notification: Notification) -> bool {
    if let Some(notify_mgr) = get_sigmanotifyenhanced_mut() {
        notify_mgr.post_notification(notification)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        assert_eq!(notification.notification_id, "notif1");
        assert_eq!(notification.app_name, "TestApp");
        assert_eq!(notification.category, NotificationCategory::Application);
        assert_eq!(notification.state, NotificationState::Pending);
    }

    #[test]
    fn test_notification_urgency() {
        let mut notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::System,
        );

        notification.set_urgency(NotificationUrgency::Critical);
        assert_eq!(notification.urgency, NotificationUrgency::Critical);
    }

    #[test]
    fn test_notification_resident() {
        let mut notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        assert!(!notification.is_resident());
        notification.set_resident(true);
        assert!(notification.is_resident());
    }

    #[test]
    fn test_notification_expiration() {
        let mut notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        notification.set_expire_time(1000);
        assert!(notification.is_expired(1000));
        assert!(!notification.is_expired(500));
    }

    #[test]
    fn test_sigmanotifyenhanced_creation() {
        let notify_mgr = SigmaNotifyEnhanced::new();
        assert!(!notify_mgr.is_running());
        assert_eq!(notify_mgr.notification_count(), 0);
    }

    #[test]
    fn test_sigmanotifyenhanced_start_stop() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        notify_mgr.start();
        assert!(notify_mgr.is_running());
        notify_mgr.stop();
        assert!(!notify_mgr.is_running());
    }

    #[test]
    fn test_sigmanotifyenhanced_post_notification() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        let posted = notify_mgr.post_notification(notification);
        assert!(posted);
        assert_eq!(notify_mgr.notification_count(), 1);
    }

    #[test]
    fn test_sigmanotifyenhanced_dismiss_notification() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        notify_mgr.post_notification(notification);
        let dismissed = notify_mgr.dismiss_notification("notif1");
        assert!(dismissed);

        let notif = notify_mgr.get_notification("notif1").unwrap();
        assert_eq!(notif.state, NotificationState::Dismissed);
    }

    #[test]
    fn test_sigmanotifyenhanced_list_by_category() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();

        notify_mgr.post_notification(Notification::new(
            "notif1".to_string(),
            "App1".to_string(),
            "Summary1".to_string(),
            "Body1".to_string(),
            NotificationCategory::Application,
        ));

        notify_mgr.post_notification(Notification::new(
            "notif2".to_string(),
            "App2".to_string(),
            "Summary2".to_string(),
            "Body2".to_string(),
            NotificationCategory::System,
        ));

        let app_notifs =
            notify_mgr.list_notifications_by_category(NotificationCategory::Application);
        assert_eq!(app_notifs.len(), 1);

        let sys_notifs = notify_mgr.list_notifications_by_category(NotificationCategory::System);
        assert_eq!(sys_notifs.len(), 1);
    }

    #[test]
    fn test_sigmanotifyenhanced_list_pending() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        notify_mgr.post_notification(Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        ));

        let pending = notify_mgr.list_pending();
        assert_eq!(pending.len(), 1);
    }

    #[test]
    fn test_sigmanotifyenhanced_list_resident() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        let mut notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        notification.set_resident(true);
        notify_mgr.post_notification(notification);

        let resident = notify_mgr.list_resident();
        assert_eq!(resident.len(), 1);
    }

    #[test]
    fn test_sigmanotifyenhanced_cleanup_expired() {
        let mut notify_mgr = SigmaNotifyEnhanced::new();
        let mut notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        notification.set_expire_time(1000);
        notify_mgr.post_notification(notification);

        let cleaned = notify_mgr.cleanup_expired(1000);
        assert_eq!(cleaned, 1);
        assert_eq!(notify_mgr.notification_count(), 0);
    }

    #[test]
    fn test_global_instance() {
        init_sigmanotifyenhanced();

        let notify_mgr = get_sigmanotifyenhanced();
        assert!(notify_mgr.is_some());

        let mut notify_mgr_mut = get_sigmanotifyenhanced_mut();
        assert!(notify_mgr_mut.is_some());

        if let Some(notify_mgr) = notify_mgr_mut {
            notify_mgr.start();
            assert!(notify_mgr.is_running());
        }
    }

    #[test]
    fn test_convenience_post_notification() {
        init_sigmanotifyenhanced();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationCategory::Application,
        );

        let posted = post_notification(notification);
        assert!(posted);

        let notify_mgr = get_sigmanotifyenhanced().unwrap();
        assert_eq!(notify_mgr.notification_count(), 1);
    }
}
