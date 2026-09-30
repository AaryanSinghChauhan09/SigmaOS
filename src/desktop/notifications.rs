// SigmaOS Advanced Desktop Notification Engine
// Zero-dependency #![no_std] notification priority queue, Do-Not-Disturb rules, and notification history manager

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationCategory {
    System,
    Network,
    Security,
    Application,
    Reminder,
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationUrgency {
    Low,
    Normal,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationItem {
    pub id: u64,
    pub app_name: String,
    pub title: String,
    pub body: String,
    pub category: NotificationCategory,
    pub urgency: NotificationUrgency,
    pub timestamp: u64,
    pub is_read: bool,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DndConfig {
    pub enabled: bool,
    pub allow_critical_override: bool,
}

impl Default for DndConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_critical_override: true,
        }
    }
}

pub struct AdvancedNotificationEngine {
    history: Vec<NotificationItem>,
    dnd_config: DndConfig,
    next_notification_id: u64,
    max_history_capacity: usize,
}

impl AdvancedNotificationEngine {
    pub fn new(max_history_capacity: usize) -> Self {
        Self {
            history: Vec::new(),
            dnd_config: DndConfig::default(),
            next_notification_id: 1,
            max_history_capacity,
        }
    }

    pub fn send_notification(
        &mut self,
        app_name: &str,
        title: &str,
        body: &str,
        category: NotificationCategory,
        urgency: NotificationUrgency,
        timestamp: u64,
        actions: &[&str],
    ) -> Option<u64> {
        // DND filtering check
        if self.dnd_config.enabled {
            if !(self.dnd_config.allow_critical_override && urgency == NotificationUrgency::Critical) {
                return None; // Suppressed by Do-Not-Disturb
            }
        }

        let id = self.next_notification_id;
        self.next_notification_id += 1;

        let item = NotificationItem {
            id,
            app_name: String::from(app_name),
            title: String::from(title),
            body: String::from(body),
            category,
            urgency,
            timestamp,
            is_read: false,
            actions: actions.iter().map(|&a| String::from(a)).collect(),
        };

        if self.history.len() >= self.max_history_capacity {
            self.history.remove(0);
        }

        self.history.push(item);
        Some(id)
    }

    pub fn mark_as_read(&mut self, notification_id: u64) -> bool {
        if let Some(item) = self.history.iter_mut().find(|n| n.id == notification_id) {
            item.is_read = true;
            true
        } else {
            false
        }
    }

    pub fn set_dnd(&mut self, enabled: bool, allow_critical: bool) {
        self.dnd_config.enabled = enabled;
        self.dnd_config.allow_critical_override = allow_critical;
    }

    pub fn unread_notifications(&self) -> Vec<&NotificationItem> {
        self.history.iter().filter(|n| !n.is_read).collect()
    }

    pub fn history(&self) -> &[NotificationItem] {
        &self.history
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_notification_engine() {
        let mut engine = AdvancedNotificationEngine::new(50);
        let id1 = engine
            .send_notification(
                "Systemd",
                "Service Started",
                "sshd.service active",
                NotificationCategory::System,
                NotificationUrgency::Normal,
                1700000000,
                &["View Logs"],
            )
            .unwrap();

        assert_eq!(engine.unread_notifications().len(), 1);

        // Turn on DND
        engine.set_dnd(true, true);

        // Normal notification should be suppressed
        let supp = engine.send_notification(
            "Mail",
            "New Message",
            "Hello World",
            NotificationCategory::Application,
            NotificationUrgency::Normal,
            1700000100,
            &[],
        );
        assert!(supp.is_none());

        // Critical notification should pass through
        let crit = engine.send_notification(
            "Kernel",
            "Thermal Warning",
            "CPU temp 85C",
            NotificationCategory::Security,
            NotificationUrgency::Critical,
            1700000200,
            &[],
        );
        assert!(crit.is_some());

        assert!(engine.mark_as_read(id1));
    }
}
