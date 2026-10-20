// SPDX-License-Identifier: MIT
// SigmaOS - Notification Manager
// Notification management inspired by Omarchy notification system and Linux notification daemons

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

/// Notification
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub app_name: String,
    pub timestamp: u64,
}

/// Notification manager
pub struct SigmaNotificationManager {
    notifications: Vec<Notification>,
}

impl SigmaNotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    pub fn send_notification(&mut self, title: String, body: String, urgency: NotificationUrgency, app_name: String) -> Notification {
        let notification = Notification {
            id: alloc::format!("notif_{}", self.notifications.len()),
            title,
            body,
            urgency,
            app_name,
            timestamp: 0,
        };
        self.notifications.push(notification.clone());
        notification
    }

    pub fn get_notifications(&self) -> &[Notification] {
        &self.notifications
    }

    pub fn get_notification(&self, id: &str) -> Option<&Notification> {
        self.notifications.iter().find(|n| n.id == id)
    }

    pub fn get_notifications_by_app(&self, app_name: &str) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| n.app_name == app_name).collect()
    }

    pub fn get_critical_notifications(&self) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| n.urgency == NotificationUrgency::Critical).collect()
    }

    pub fn dismiss_notification(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.notifications.iter().position(|n| n.id == id) {
            self.notifications.remove(idx);
            Ok(())
        } else {
            Err(format!("Notification {} not found", id))
        }
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
}

impl Default for SigmaNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_manager_creation() {
        let manager = SigmaNotificationManager::new();
        assert_eq!(manager.get_notifications().len(), 0);
    }

    #[test]
    fn test_send_notification() {
        let mut manager = SigmaNotificationManager::new();
        manager.send_notification("Test".to_string(), "Test body".to_string(), NotificationUrgency::Normal, "TestApp".to_string());
        assert_eq!(manager.get_notifications().len(), 1);
    }

    #[test]
    fn test_notifications_by_app() {
        let mut manager = SigmaNotificationManager::new();
        manager.send_notification("Test1".to_string(), "Body1".to_string(), NotificationUrgency::Normal, "App1".to_string());
        manager.send_notification("Test2".to_string(), "Body2".to_string(), NotificationUrgency::Normal, "App2".to_string());
        assert_eq!(manager.get_notifications_by_app("App1").len(), 1);
    }

    #[test]
    fn test_critical_notifications() {
        let mut manager = SigmaNotificationManager::new();
        manager.send_notification("Normal".to_string(), "Body".to_string(), NotificationUrgency::Normal, "App".to_string());
        manager.send_notification("Critical".to_string(), "Body".to_string(), NotificationUrgency::Critical, "App".to_string());
        assert_eq!(manager.get_critical_notifications().len(), 1);
    }

    #[test]
    fn test_dismiss_notification() {
        let mut manager = SigmaNotificationManager::new();
        let notif = manager.send_notification("Test".to_string(), "Body".to_string(), NotificationUrgency::Normal, "App".to_string());
        manager.dismiss_notification(notif.id).unwrap();
        assert_eq!(manager.get_notifications().len(), 0);
    }

    #[test]
    fn test_clear_all() {
        let mut manager = SigmaNotificationManager::new();
        manager.send_notification("Test1".to_string(), "Body1".to_string(), NotificationUrgency::Normal, "App".to_string());
        manager.send_notification("Test2".to_string(), "Body2".to_string(), NotificationUrgency::Normal, "App".to_string());
        manager.clear_all();
        assert_eq!(manager.get_notifications().len(), 0);
    }
}
