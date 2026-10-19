//! SigmaNotificationManager - Notification Management System
//!
//! Provides comprehensive notification management inspired by Linux Mint and Omarchy.
//! Supports notifications, do-not-disturb, notification rules, and notification history.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

/// Notification category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    System,
    Application,
    Network,
    Device,
    Security,
    Email,
    Chat,
    Calendar,
    Media,
    Other,
}

/// Notification
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub category: NotificationCategory,
    pub urgency: NotificationUrgency,
    pub icon: String,
    pub timestamp: String,
    pub action_id: Option<String>,
    pub action_label: Option<String>,
    pub expires: bool,
    pub timeout: u32, // seconds
}

impl Notification {
    pub fn new(id: String, app_name: String, title: String, body: String, category: NotificationCategory, urgency: NotificationUrgency) -> Self {
        Self {
            id,
            app_name,
            title,
            body,
            category,
            urgency,
            icon: String::new(),
            timestamp: String::new(),
            action_id: None,
            action_label: None,
            expires: false,
            timeout: 5,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = icon;
    }

    pub fn set_timestamp(&mut self, time: String) {
        self.timestamp = time;
    }

    pub fn set_action(&mut self, action_id: String, action_label: String) {
        self.action_id = Some(action_id);
        self.action_label = Some(action_label);
    }

    pub fn set_expires(&mut self, expires: bool, timeout: u32) {
        self.expires = expires;
        self.timeout = timeout;
    }

    pub fn set_urgency(&mut self, urgency: NotificationUrgency) {
        self.urgency = urgency;
    }

    pub fn is_critical(&self) -> bool {
        self.urgency == NotificationUrgency::Critical
    }
}

/// Notification rule
#[derive(Debug, Clone)]
pub struct NotificationRule {
    pub id: String,
    pub app_name: String,
    pub enabled: bool,
    pub show_popups: bool,
    pub play_sound: bool,
    pub urgency_threshold: NotificationUrgency,
}

impl NotificationRule {
    pub fn new(id: String, app_name: String) -> Self {
        Self {
            id,
            app_name,
            enabled: true,
            show_popups: true,
            play_sound: true,
            urgency_threshold: NotificationUrgency::Normal,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_show_popups(&mut self, show: bool) {
        self.show_popups = show;
    }

    pub fn set_play_sound(&mut self, play: bool) {
        self.play_sound = play;
    }

    pub fn set_urgency_threshold(&mut self, threshold: NotificationUrgency) {
        self.urgency_threshold = threshold;
    }
}

/// SigmaNotificationManager - Notification management
#[derive(Debug)]
pub struct SigmaNotificationManager {
    notifications: BTreeMap<String, Notification>,
    rules: BTreeMap<String, NotificationRule>,
    do_not_disturb: bool,
    history_limit: usize,
}

impl SigmaNotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: BTreeMap::new(),
            rules: BTreeMap::new(),
            do_not_disturb: false,
            history_limit: 50,
        }
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.insert(notification.id.clone(), notification);
        // Enforce history limit
        while self.notifications.len() > self.history_limit {
            if let Some(first_key) = self.notifications.keys().next().cloned() {
                self.notifications.remove(&first_key);
            } else {
                break;
            }
        }
    }

    pub fn remove_notification(&mut self, id: &str) -> bool {
        self.notifications.remove(id).is_some()
    }

    pub fn get_notification(&self, id: &str) -> Option<&Notification> {
        self.notifications.get(id)
    }

    pub fn add_rule(&mut self, rule: NotificationRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    pub fn remove_rule(&mut self, id: &str) -> bool {
        self.rules.remove(id).is_some()
    }

    pub fn get_rule(&self, id: &str) -> Option<&NotificationRule> {
        self.rules.get(id)
    }

    pub fn get_rule_for_app(&self, app_name: &str) -> Option<&NotificationRule> {
        self.rules.values().find(|rule| rule.app_name == app_name)
    }

    pub fn set_do_not_disturb(&mut self, enabled: bool) {
        self.do_not_disturb = enabled;
    }

    pub fn set_history_limit(&mut self, limit: usize) {
        self.history_limit = limit;
    }

    pub fn list_critical(&self) -> Vec<&Notification> {
        self.notifications.values()
            .filter(|notif| notif.is_critical())
            .collect()
    }

    pub fn list_by_category(&self, category: NotificationCategory) -> Vec<&Notification> {
        self.notifications.values()
            .filter(|notif| notif.category == category)
            .collect()
    }

    pub fn list_by_app(&self, app_name: &str) -> Vec<&Notification> {
        self.notifications.values()
            .filter(|notif| notif.app_name == app_name)
            .collect()
    }

    pub fn should_show(&self, notification: &Notification) -> bool {
        if self.do_not_disturb && !notification.is_critical() {
            return false;
        }
        if let Some(rule) = self.get_rule_for_app(&notification.app_name) {
            if !rule.enabled {
                return false;
            }
            if notification.urgency < rule.urgency_threshold {
                return false;
            }
        }
        true
    }

    pub fn notification_count(&self) -> usize {
        self.notifications.len()
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for SigmaNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaNotificationManager instance
static mut SIGMANOTIFICATIONMANAGER_INSTANCE: Option<SigmaNotificationManager> = None;

/// Initialize the global SigmaNotificationManager instance
pub fn init_sigmanotificationmanager() {
    unsafe {
        SIGMANOTIFICATIONMANAGER_INSTANCE = Some(SigmaNotificationManager::new());
    }
}

/// Get the global SigmaNotificationManager instance
pub fn get_sigmanotificationmanager() -> Option<&'static SigmaNotificationManager> {
    unsafe {
        SIGMANOTIFICATIONMANAGER_INSTANCE.as_ref()
    }
}

/// Get the global SigmaNotificationManager instance (mutable)
pub fn get_sigmanotificationmanager_mut() -> Option<&'static mut SigmaNotificationManager> {
    unsafe {
        SIGMANOTIFICATIONMANAGER_INSTANCE.as_mut()
    }
}

/// Convenience function to add a notification
pub fn add_notification(notification: Notification) -> bool {
    if let Some(manager) = get_sigmanotificationmanager_mut() {
        manager.add_notification(notification);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_creation() {
        let notif = Notification::new(
            "notif1".to_string(),
            "System".to_string(),
            "Update Available".to_string(),
            "A new update is available".to_string(),
            NotificationCategory::System,
            NotificationUrgency::Normal,
        );
        assert_eq!(notif.id, "notif1");
        assert_eq!(notif.category, NotificationCategory::System);
    }

    #[test]
    fn test_notification_modification() {
        let mut notif = Notification::new(
            "notif1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            NotificationCategory::Application,
            NotificationUrgency::Low,
        );
        notif.set_urgency(NotificationUrgency::Critical);
        notif.set_action("action1".to_string(), "Click Me".to_string());
        assert!(notif.is_critical());
        assert_eq!(notif.action_label, Some("Click Me".to_string()));
    }

    #[test]
    fn test_notification_rule_creation() {
        let rule = NotificationRule::new("rule1".to_string(), "App".to_string());
        assert_eq!(rule.id, "rule1");
        assert!(rule.show_popups);
    }

    #[test]
    fn test_notification_rule_modification() {
        let mut rule = NotificationRule::new("rule1".to_string(), "Test".to_string());
        rule.set_enabled(false);
        rule.set_urgency_threshold(NotificationUrgency::Critical);
        assert!(!rule.enabled);
        assert_eq!(rule.urgency_threshold, NotificationUrgency::Critical);
    }

    #[test]
    fn test_sigmanotificationmanager_creation() {
        let manager = SigmaNotificationManager::new();
        assert_eq!(manager.notification_count(), 0);
        assert!(!manager.do_not_disturb);
    }

    #[test]
    fn test_sigmanotificationmanager_management() {
        let mut manager = SigmaNotificationManager::new();
        let notif = Notification::new(
            "notif1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            NotificationCategory::System,
            NotificationUrgency::Normal,
        );
        manager.add_notification(notif);
        assert_eq!(manager.notification_count(), 1);
    }

    #[test]
    fn test_do_not_disturb() {
        let mut manager = SigmaNotificationManager::new();
        manager.set_do_not_disturb(true);
        let notif = Notification::new(
            "notif1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            NotificationCategory::Application,
            NotificationUrgency::Normal,
        );
        assert!(!manager.should_show(&notif));
    }

    #[test]
    fn test_critical_bypasses_dnd() {
        let mut manager = SigmaNotificationManager::new();
        manager.set_do_not_disturb(true);
        let notif = Notification::new(
            "notif1".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            "Test".to_string(),
            NotificationCategory::Security,
            NotificationUrgency::Critical,
        );
        assert!(manager.should_show(&notif));
    }
}
