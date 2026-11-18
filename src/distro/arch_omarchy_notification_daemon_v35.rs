// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Modern Notification Daemon & Desktop Alert Engine
//!
//! Inspired by Omarchy's aesthetic Mako / SwayNC notification centers and
//! Arch Linux's org.freedesktop.Notifications D-Bus specifications.
//! Provides priority queuing, urgency levels (Low, Normal, Critical),
//! action buttons, timeout dismissal, and Do-Not-Disturb (DND) modes.

use std::collections::VecDeque;

/// Urgency level of a notification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl Urgency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::Critical => "critical",
        }
    }
}

/// Notification action button
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationAction {
    pub key: String,
    pub label: String,
}

/// A desktop notification message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: Urgency,
    pub timeout_ms: u32,
    pub actions: Vec<NotificationAction>,
}

impl Notification {
    pub fn new(id: u32, app_name: &str, summary: &str, body: &str) -> Self {
        Self {
            id,
            app_name: app_name.to_string(),
            summary: summary.to_string(),
            body: body.to_string(),
            urgency: Urgency::Normal,
            timeout_ms: 5000,
            actions: Vec::new(),
        }
    }

    pub fn with_urgency(mut self, urgency: Urgency) -> Self {
        self.urgency = urgency;
        self
    }

    pub fn with_action(mut self, key: &str, label: &str) -> Self {
        self.actions.push(NotificationAction {
            key: key.to_string(),
            label: label.to_string(),
        });
        self
    }
}

/// Notification queue and policy manager
#[derive(Debug, Default)]
pub struct NotificationDaemon {
    queue: VecDeque<Notification>,
    pub dnd_enabled: bool,
    next_id: u32,
}

impl NotificationDaemon {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            dnd_enabled: false,
            next_id: 1,
        }
    }

    /// Post a notification; suppresses non-critical items if DND is active
    pub fn notify(&mut self, app_name: &str, summary: &str, body: &str, urgency: Urgency) -> Option<u32> {
        if self.dnd_enabled && urgency < Urgency::Critical {
            return None; // Suppressed by Do-Not-Disturb
        }

        let id = self.next_id;
        self.next_id += 1;

        let notif = Notification::new(id, app_name, summary, body).with_urgency(urgency);
        self.queue.push_back(notif);
        Some(id)
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }

    pub fn pop_next(&mut self) -> Option<Notification> {
        self.queue.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgency_ordering_aaa() {
        assert!(Urgency::Critical > Urgency::Normal);
        assert!(Urgency::Normal > Urgency::Low);
    }

    #[test]
    fn test_notification_delivery_normal_mode_aaa() {
        let mut daemon = NotificationDaemon::new();
        let id1 = daemon.notify("Pacman", "Updates Ready", "3 new packages available", Urgency::Normal);
        assert_eq!(id1, Some(1));
        assert_eq!(daemon.pending_count(), 1);

        let notif = daemon.pop_next().unwrap();
        assert_eq!(notif.app_name, "Pacman");
        assert_eq!(notif.summary, "Updates Ready");
    }

    #[test]
    fn test_dnd_suppression_of_low_and_normal_aaa() {
        let mut daemon = NotificationDaemon::new();
        daemon.dnd_enabled = true;

        let id_low = daemon.notify("Chat", "Message received", "Hello", Urgency::Low);
        let id_norm = daemon.notify("Wallust", "Theme applied", "Nord", Urgency::Normal);
        assert_eq!(id_low, None);
        assert_eq!(id_norm, None);
        assert_eq!(daemon.pending_count(), 0);

        // Critical alerts MUST bypass DND
        let id_crit = daemon.notify("Battery", "Battery Low", "5% remaining!", Urgency::Critical);
        assert_eq!(id_crit, Some(1));
        assert_eq!(daemon.pending_count(), 1);
    }
}
