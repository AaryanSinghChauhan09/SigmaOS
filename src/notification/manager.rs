//! SigmaOS Notification System
//!
//! Inspired by: Omarchy's notification daemon (popups, DND, deduping, replayable history)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! FAIL_FAST_SYSTEM, SECURE_DEFAULTS, RACE_FREE DESIGN, SAFE_CONCURRENCY,
//! DETERMINISTIC TESTING, ZERO_COPY, MEMORY_SAFETY, MEMORY_DISCIPLINE.

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Notification urgency level
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Urgency {
    Low,
    Normal,
    High,
    Critical,
}

/// Notification
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub urgency: Urgency,
    pub app_name: String,
    pub timestamp: u64,
    pub dismissed: bool,
    pub icon: Option<String>,
    pub actions: Vec<String>,
}

/// Notification history (replayable like Omarchy)
#[derive(Debug, Clone)]
pub struct NotificationHistory {
    pub notifications: VecDeque<Notification>,
    pub max_history: usize,
    pub do_not_disturb: bool,
    pub next_id: u64,
}

/// Notification manager
pub struct NotificationManager {
    pub history: NotificationHistory,
    pub popup_duration: u64, // seconds
    pub max_popups: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            history: NotificationHistory {
                notifications: VecDeque::new(),
                max_history: 100,
                do_not_disturb: false,
                next_id: 1,
            },
            popup_duration: 5,
            max_popups: 5,
        }
    }

    /// Send a notification
    pub fn send(
        &mut self,
        app_name: String,
        title: String,
        body: String,
        urgency: Urgency,
    ) -> Result<u64, String> {
        if self.history.do_not_disturb && urgency != Urgency::Critical {
            return Err("Do Not Disturb is active".to_string());
        }

        let id = self.history.next_id;
        self.history.next_id += 1;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let notification = Notification {
            id,
            title,
            body,
            urgency,
            app_name,
            timestamp,
            dismissed: false,
            icon: None,
            actions: Vec::new(),
        };

        // Deduplication: don't add duplicate notifications within 5 seconds
        if let Some(last) = self.history.notifications.back() {
            if last.app_name == notification.app_name
                && last.title == notification.title
                && last.body == notification.body
                && timestamp.saturating_sub(last.timestamp) < 5
            {
                return Ok(last.id);
            }
        }

        self.history.notifications.push_back(notification);

        // Trim history
        while self.history.notifications.len() > self.history.max_history {
            self.history.notifications.pop_front();
        }

        Ok(id)
    }

    /// Dismiss a notification by ID
    pub fn dismiss(&mut self, id: u64) -> Result<(), String> {
        for notif in self.history.notifications.iter_mut() {
            if notif.id == id {
                notif.dismissed = true;
                return Ok(());
            }
        }
        Err(format!("Notification {} not found", id))
    }

    /// Toggle Do Not Disturb
    pub fn toggle_dnd(&mut self) -> bool {
        self.history.do_not_disturb = !self.history.do_not_disturb;
        self.history.do_not_disturb
    }

    /// Set DND state
    pub fn set_dnd(&mut self, enabled: bool) {
        self.history.do_not_disturb = enabled;
    }

    /// Replay last N notifications
    pub fn replay(&self, count: usize) -> Vec<&Notification> {
        self.history
            .notifications
            .iter()
            .rev()
            .take(count)
            .collect()
    }

    /// List active (non-dismissed) notifications
    pub fn list_active(&self) -> Vec<&Notification> {
        self.history
            .notifications
            .iter()
            .filter(|n| !n.dismissed)
            .collect()
    }

    /// Clear all notifications
    pub fn clear_all(&mut self) {
        self.history.notifications.clear();
    }

    /// Get DND status
    pub fn is_dnd(&self) -> bool {
        self.history.do_not_disturb
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_notification() {
        let mut mgr = NotificationManager::new();
        let id = mgr.send(
            "system".to_string(),
            "Update ready".to_string(),
            "New packages available".to_string(),
            Urgency::Normal,
        )
        .unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.list_active().len(), 1);
    }

    #[test]
    fn test_dnd_blocks_notifications() {
        let mut mgr = NotificationManager::new();
        mgr.set_dnd(true);
        let result = mgr.send(
            "system".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            Urgency::Normal,
        );
        assert!(result.is_err());
        // Critical should still work
        let result = mgr.send(
            "system".to_string(),
            "Critical".to_string(),
            "Body".to_string(),
            Urgency::Critical,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_dismiss_notification() {
        let mut mgr = NotificationManager::new();
        mgr.send("app".to_string(), "Title".to_string(), "Body".to_string(), Urgency::Low)
            .unwrap();
        assert!(mgr.dismiss(1).is_ok());
        assert_eq!(mgr.list_active().len(), 0);
    }

    #[test]
    fn test_replay() {
        let mut mgr = NotificationManager::new();
        mgr.send("app1".to_string(), "T1".to_string(), "B1".to_string(), Urgency::Low)
            .unwrap();
        mgr.send("app2".to_string(), "T2".to_string(), "B2".to_string(), Urgency::Low)
            .unwrap();
        let replayed = mgr.replay(1);
        assert_eq!(replayed.len(), 1);
        assert_eq!(replayed[0].title, "T2");
    }

    #[test]
    fn test_dedup() {
        let mut mgr = NotificationManager::new();
        mgr.send("app".to_string(), "Title".to_string(), "Body".to_string(), Urgency::Low)
            .unwrap();
        // Send duplicate immediately - should return same ID
        let id2 = mgr.send("app".to_string(), "Title".to_string(), "Body".to_string(), Urgency::Low)
            .unwrap();
        assert_eq!(id2, 1);
        assert_eq!(mgr.list_active().len(), 1);
    }

    #[test]
    fn test_toggle_dnd() {
        let mut mgr = NotificationManager::new();
        assert!(!mgr.is_dnd());
        assert!(mgr.toggle_dnd());
        assert!(mgr.is_dnd());
        assert!(!mgr.toggle_dnd());
        assert!(!mgr.is_dnd());
    }
}