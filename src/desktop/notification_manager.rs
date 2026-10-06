//! Notification Manager
//!
//! Notification system inspired by Linux Mint's notifications and Omarchy's
//! notification utilities, supporting system and application notifications.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopNotificationUrgency {
    Low,
    Normal,
    Critical,
}

impl DesktopNotificationUrgency {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" => Some(DesktopNotificationUrgency::Low),
            "normal" => Some(DesktopNotificationUrgency::Normal),
            "critical" => Some(DesktopNotificationUrgency::Critical),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DesktopNotificationUrgency::Low => "Low",
            DesktopNotificationUrgency::Normal => "Normal",
            DesktopNotificationUrgency::Critical => "Critical",
        }
    }
}

/// Notification
#[derive(Debug, Clone)]
pub struct DesktopNotification {
    pub id: String,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub urgency: DesktopNotificationUrgency,
    pub icon: Option<String>,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub is_dismissed: bool,
}

impl DesktopNotification {
    pub fn new(
        id: String,
        app_name: String,
        title: String,
        body: String,
        urgency: DesktopNotificationUrgency,
    ) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id,
            app_name,
            title,
            body,
            urgency,
            icon: None,
            created_at,
            expires_at: None,
            is_dismissed: false,
        }
    }

    pub fn set_icon(&mut self, icon: String) {
        self.icon = Some(icon);
    }

    pub fn set_expires(&mut self, expires_seconds: u32) {
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + expires_seconds as u64;
        self.expires_at = Some(expires_at);
    }

    pub fn dismiss(&mut self) {
        self.is_dismissed = true;
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expires
        } else {
            false
        }
    }
}

/// Notification manager
#[derive(Debug)]
pub struct DesktopNotificationManager {
    notifications: HashMap<String, DesktopNotification>,
    next_id: u64,
    do_not_disturb: bool,
}

impl DesktopNotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: HashMap::new(),
            next_id: 1,
            do_not_disturb: false,
        }
    }

    /// Get DND status
    pub fn is_dnd_enabled(&self) -> bool {
        self.do_not_disturb
    }

    /// Set DND status
    pub fn set_dnd(&mut self, enabled: bool) {
        self.do_not_disturb = enabled;
    }

    /// Send a notification
    pub fn send(&mut self, app_name: String, title: String, body: String, urgency: DesktopNotificationUrgency) -> String {
        if self.do_not_disturb && urgency != DesktopNotificationUrgency::Critical {
            // Return ID but don't store if DND is enabled and not critical
            let id = format!("notif-{}", self.next_id);
            self.next_id += 1;
            return id;
        }

        let id = format!("notif-{}", self.next_id);
        self.next_id += 1;

        let notification = DesktopNotification::new(
            id.clone(),
            app_name,
            title,
            body,
            urgency,
        );

        self.notifications.insert(id.clone(), notification);
        id
    }

    /// Get a notification
    pub fn get(&self, id: &str) -> Option<&DesktopNotification> {
        self.notifications.get(id)
    }

    /// List all notifications
    pub fn list_all(&self) -> Vec<&DesktopNotification> {
        self.notifications.values().collect()
    }

    /// List active notifications
    pub fn list_active(&self) -> Vec<&DesktopNotification> {
        self.notifications.values()
            .filter(|n| !n.is_dismissed && !n.is_expired())
            .collect()
    }

    /// List by urgency
    pub fn list_by_urgency(&self, urgency: DesktopNotificationUrgency) -> Vec<&DesktopNotification> {
        self.notifications.values()
            .filter(|n| n.urgency == urgency)
            .collect()
    }

    /// List by app
    pub fn list_by_app(&self, app_name: &str) -> Vec<&DesktopNotification> {
        self.notifications.values()
            .filter(|n| n.app_name == app_name)
            .collect()
    }

    /// Dismiss a notification
    pub fn dismiss(&mut self, id: &str) -> Result<(), String> {
        let notification = self.notifications.get_mut(id)
            .ok_or_else(|| format!("Notification {} not found", id))?;

        notification.dismiss();
        Ok(())
    }

    /// Dismiss all notifications
    pub fn dismiss_all(&mut self) {
        for notification in self.notifications.values_mut() {
            notification.dismiss();
        }
    }

    /// Remove a notification
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        self.notifications.remove(id)
            .ok_or_else(|| format!("Notification {} not found", id))?;
        Ok(())
    }

    /// Clear expired notifications
    pub fn clear_expired(&mut self) -> usize {
        let expired_ids: Vec<String> = self.notifications.values()
            .filter(|n| n.is_expired())
            .map(|n| n.id.clone())
            .collect();

        let count = expired_ids.len();
        for id in expired_ids {
            let _ = self.remove(&id);
        }

        count
    }

    /// Clear dismissed notifications
    pub fn clear_dismissed(&mut self) -> usize {
        let dismissed_ids: Vec<String> = self.notifications.values()
            .filter(|n| n.is_dismissed)
            .map(|n| n.id.clone())
            .collect();

        let count = dismissed_ids.len();
        for id in dismissed_ids {
            let _ = self.remove(&id);
        }

        count
    }

    /// Get statistics
    pub fn get_statistics(&self) -> DesktopNotificationStatistics {
        let total_notifications = self.notifications.len();
        let active_count = self.notifications.values()
            .filter(|n| !n.is_dismissed && !n.is_expired())
            .count();
        let dismissed_count = self.notifications.values()
            .filter(|n| n.is_dismissed)
            .count();
        let expired_count = self.notifications.values()
            .filter(|n| n.is_expired())
            .count();
        let critical_count = self.notifications.values()
            .filter(|n| n.urgency == DesktopNotificationUrgency::Critical)
            .count();

        DesktopNotificationStatistics {
            total_notifications,
            active_count,
            dismissed_count,
            expired_count,
            critical_count,
            dnd_enabled: self.do_not_disturb,
        }
    }
}

impl Default for DesktopNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification statistics
#[derive(Debug, Clone)]
pub struct DesktopNotificationStatistics {
    pub total_notifications: usize,
    pub active_count: usize,
    pub dismissed_count: usize,
    pub expired_count: usize,
    pub critical_count: usize,
    pub dnd_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_urgency_from_str() {
        assert_eq!(DesktopNotificationUrgency::from_str("low"), Some(DesktopNotificationUrgency::Low));
        assert_eq!(DesktopNotificationUrgency::from_str("critical"), Some(DesktopNotificationUrgency::Critical));
    }

    #[test]
    fn test_notification_creation() {
        let notif = DesktopNotification::new(
            "test".to_string(),
            "TestApp".to_string(),
            "Test Title".to_string(),
            "Test Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        assert_eq!(notif.title, "Test Title");
    }

    #[test]
    fn test_notification_manager_creation() {
        let manager = DesktopNotificationManager::new();
        assert_eq!(manager.list_all().len(), 0);
    }

    #[test]
    fn test_send_notification() {
        let mut manager = DesktopNotificationManager::new();
        let id = manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        assert!(manager.get(&id).is_some());
    }

    #[test]
    fn test_dnd_mode() {
        let mut manager = DesktopNotificationManager::new();
        manager.set_dnd(true);
        assert!(manager.is_dnd_enabled());
        
        // Normal notification should not be stored in DND mode
        let id = manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        assert!(manager.get(&id).is_none());
        
        // Critical notification should still work
        let id = manager.send(
            "TestApp".to_string(),
            "Critical".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Critical,
        );
        assert!(manager.get(&id).is_some());
    }

    #[test]
    fn test_dismiss() {
        let mut manager = DesktopNotificationManager::new();
        let id = manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        assert!(manager.dismiss(&id).is_ok());
        assert!(manager.get(&id).unwrap().is_dismissed);
    }

    #[test]
    fn test_list_by_urgency() {
        let mut manager = DesktopNotificationManager::new();
        manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Critical,
        );
        let critical = manager.list_by_urgency(DesktopNotificationUrgency::Critical);
        assert_eq!(critical.len(), 1);
    }

    #[test]
    fn test_clear_dismissed() {
        let mut manager = DesktopNotificationManager::new();
        let id = manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        manager.dismiss(&id).ok();
        let count = manager.clear_dismissed();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_statistics() {
        let mut manager = DesktopNotificationManager::new();
        manager.send(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            DesktopNotificationUrgency::Normal,
        );
        let stats = manager.get_statistics();
        assert_eq!(stats.total_notifications, 1);
    }
}
