// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaNotify - System Notification Daemon
//!
//! SigmaNotify provides system notification handling for SigmaOS, inspired by libnotify and notify-send from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    /// Low urgency
    Low,
    /// Normal urgency
    Normal,
    /// Critical urgency
    Critical,
}

impl fmt::Display for NotificationUrgency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationUrgency::Low => write!(f, "low"),
            NotificationUrgency::Normal => write!(f, "normal"),
            NotificationUrgency::Critical => write!(f, "critical"),
        }
    }
}

/// Notification category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    /// Device notification
    Device,
    /// Network notification
    Network,
    /// System notification
    System,
    /// Email notification
    Email,
    /// Instant message
    InstantMessage,
    /// Call notification
    Call,
    /// Reminder notification
    Reminder,
    /// Calendar notification
    Calendar,
    /// Transfer notification
    Transfer,
    /// Other notification
    Other,
}

impl fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationCategory::Device => write!(f, "device"),
            NotificationCategory::Network => write!(f, "network"),
            NotificationCategory::System => write!(f, "system"),
            NotificationCategory::Email => write!(f, "email"),
            NotificationCategory::InstantMessage => write!(f, "im"),
            NotificationCategory::Call => write!(f, "call"),
            NotificationCategory::Reminder => write!(f, "reminder"),
            NotificationCategory::Calendar => write!(f, "calendar"),
            NotificationCategory::Transfer => write!(f, "transfer"),
            NotificationCategory::Other => write!(f, "other"),
        }
    }
}

/// Notification
#[derive(Debug, Clone)]
pub struct Notification {
    /// Notification ID
    pub notification_id: String,
    /// Application name
    pub app_name: String,
    /// Summary
    pub summary: String,
    /// Body
    pub body: String,
    /// Icon
    pub icon: Option<String>,
    /// Urgency
    pub urgency: NotificationUrgency,
    /// Category
    pub category: NotificationCategory,
    /// Timeout (milliseconds)
    pub timeout: u32,
    /// Timestamp
    pub timestamp: u64,
    /// Action ID
    pub action_id: Option<String>,
}

impl Notification {
    /// Create a new notification
    pub fn new(notification_id: String, app_name: String, summary: String, body: String) -> Self {
        Self {
            notification_id,
            app_name,
            summary,
            body,
            icon: None,
            urgency: NotificationUrgency::Normal,
            category: NotificationCategory::Other,
            timeout: 5000,
            timestamp: 0,
            action_id: None,
        }
    }

    /// Set icon
    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set urgency
    pub fn with_urgency(mut self, urgency: NotificationUrgency) -> Self {
        self.urgency = urgency;
        self
    }

    /// Set category
    pub fn with_category(mut self, category: NotificationCategory) -> Self {
        self.category = category;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set action ID
    pub fn with_action(mut self, action_id: String) -> Self {
        self.action_id = Some(action_id);
        self
    }
}

/// SigmaNotify - System Notification Daemon
pub struct SigmaNotify {
    /// Active notifications (notification_id -> notification)
    notifications: BTreeMap<String, Notification>,
    /// Notification history
    history: Vec<Notification>,
    /// Running state
    running: bool,
    /// Notification counter
    notification_counter: u32,
    /// Max history size
    max_history: usize,
}

impl SigmaNotify {
    /// Create a new SigmaNotify instance
    pub fn new() -> Self {
        Self {
            notifications: BTreeMap::new(),
            history: Vec::new(),
            running: false,
            notification_counter: 0,
            max_history: 100,
        }
    }

    /// Start the notification daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the notification daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Show a notification
    pub fn show_notification(&mut self, notification: Notification) -> Result<(), String> {
        if !self.running {
            return Err("SigmaNotify not running".to_string());
        }

        let id = notification.notification_id.clone();
        self.notifications.insert(id.clone(), notification.clone());
        self.notification_counter += 1;

        // Add to history
        self.history.push(notification);
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }

        Ok(())
    }

    /// Close a notification
    pub fn close_notification(&mut self, notification_id: &str) -> Result<(), String> {
        if self.notifications.remove(notification_id).is_none() {
            return Err(format!("Notification '{}' not found", notification_id));
        }
        Ok(())
    }

    /// Get a notification
    pub fn get_notification(&self, notification_id: &str) -> Option<&Notification> {
        self.notifications.get(notification_id)
    }

    /// List all active notifications
    pub fn list_notifications(&self) -> Vec<&Notification> {
        self.notifications.values().collect()
    }

    /// List notifications by category
    pub fn list_by_category(&self, category: NotificationCategory) -> Vec<&Notification> {
        self.notifications
            .values()
            .filter(|n| n.category == category)
            .collect()
    }

    /// List notifications by urgency
    pub fn list_by_urgency(&self, urgency: NotificationUrgency) -> Vec<&Notification> {
        self.notifications
            .values()
            .filter(|n| n.urgency == urgency)
            .collect()
    }

    /// Get notification history
    pub fn get_history(&self) -> &[Notification] {
        &self.history
    }

    /// Clear all notifications
    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }

    /// Get notification count
    pub fn notification_count(&self) -> usize {
        self.notifications.len()
    }

    /// Get notification statistics
    pub fn notification_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "active_notifications".to_string(),
            self.notification_count().to_string(),
        );
        stats.insert("history_size".to_string(), self.history.len().to_string());
        stats.insert(
            "total_shown".to_string(),
            self.notification_counter.to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaNotify {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNotify instance (placeholder for actual global state)
static mut SIGMANOTIFY_INSTANCE: Option<SigmaNotify> = None;

/// Initialize the global SigmaNotify instance
pub fn init_sigmanotify() {
    unsafe {
        SIGMANOTIFY_INSTANCE = Some(SigmaNotify::new());
    }
}

/// Get the global SigmaNotify instance
pub fn get_sigmanotify() -> Option<&'static mut SigmaNotify> {
    unsafe { SIGMANOTIFY_INSTANCE.as_mut() }
}

/// Convenience function to show a notification
pub fn show_notification(notification: Notification) -> Result<(), String> {
    if let Some(notify) = get_sigmanotify() {
        notify.show_notification(notification)
    } else {
        Err("SigmaNotify not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_urgency_display() {
        assert_eq!(NotificationUrgency::Low.to_string(), "low");
        assert_eq!(NotificationUrgency::Critical.to_string(), "critical");
    }

    #[test]
    fn test_notification_category_display() {
        assert_eq!(NotificationCategory::System.to_string(), "system");
        assert_eq!(NotificationCategory::Email.to_string(), "email");
    }

    #[test]
    fn test_notification() {
        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        )
        .with_icon("icon.png".to_string())
        .with_urgency(NotificationUrgency::Critical)
        .with_category(NotificationCategory::System)
        .with_timeout(10000);

        assert_eq!(notification.notification_id, "notif1");
        assert_eq!(notification.urgency, NotificationUrgency::Critical);
        assert_eq!(notification.icon, Some("icon.png".to_string()));
    }

    #[test]
    fn test_sigmanotify() {
        let mut notify = SigmaNotify::new();
        assert!(!notify.is_running());
        notify.start();
        assert!(notify.is_running());
        notify.stop();
        assert!(!notify.is_running());
    }

    #[test]
    fn test_sigmanotify_show_notification() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        );

        assert!(notify.show_notification(notification).is_ok());
        assert_eq!(notify.notification_count(), 1);
    }

    #[test]
    fn test_sigmanotify_close_notification() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        );

        notify.show_notification(notification).unwrap();
        assert!(notify.close_notification("notif1").is_ok());
        assert_eq!(notify.notification_count(), 0);
    }

    #[test]
    fn test_sigmanotify_list_by_category() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        )
        .with_category(NotificationCategory::System);

        notify.show_notification(notification).unwrap();

        let system_notifs = notify.list_by_category(NotificationCategory::System);
        assert_eq!(system_notifs.len(), 1);
    }

    #[test]
    fn test_sigmanotify_list_by_urgency() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        )
        .with_urgency(NotificationUrgency::Critical);

        notify.show_notification(notification).unwrap();

        let critical_notifs = notify.list_by_urgency(NotificationUrgency::Critical);
        assert_eq!(critical_notifs.len(), 1);
    }

    #[test]
    fn test_sigmanotify_history() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        );

        notify.show_notification(notification).unwrap();

        let history = notify.get_history();
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_sigmanotify_stats() {
        let mut notify = SigmaNotify::new();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        );

        notify.show_notification(notification).unwrap();

        let stats = notify.notification_stats();
        assert_eq!(stats.get("active_notifications"), Some(&"1".to_string()));
        assert_eq!(stats.get("total_shown"), Some(&"1".to_string()));
    }

    #[test]
    fn test_show_notification_convenience() {
        init_sigmanotify();

        let mut notify = get_sigmanotify().unwrap();
        notify.start();

        let notification = Notification::new(
            "notif1".to_string(),
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
        );

        let result = show_notification(notification);
        assert!(result.is_ok());
    }
}
