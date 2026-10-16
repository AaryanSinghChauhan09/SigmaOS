// SPDX-License-Identifier: MIT
// SigmaOS - Mako Notification Daemon
// Inspired by Mako (https://github.com/emersion/mako)
// Provides notification daemon for Wayland

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    Low,
    Normal,
    High,
    Critical,
}

/// Notification
#[derive(Debug)]
pub struct Notification {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub urgency: NotificationUrgency,
    pub timeout: u32, // milliseconds
    pub visible: AtomicBool,
    pub dismissed: AtomicBool,
}

impl Notification {
    pub fn new(
        id: u32,
        app_name: String,
        summary: String,
        body: String,
        urgency: NotificationUrgency,
        timeout: u32,
    ) -> Self {
        Self {
            id,
            app_name,
            summary,
            body,
            urgency,
            timeout,
            visible: AtomicBool::new(false),
            dismissed: AtomicBool::new(false),
        }
    }

    /// Show notification
    pub fn show(&self) {
        self.visible.store(true, Ordering::Relaxed);
    }

    /// Hide notification
    pub fn hide(&self) {
        self.visible.store(false, Ordering::Relaxed);
    }

    /// Dismiss notification
    pub fn dismiss(&self) {
        self.dismissed.store(true, Ordering::Relaxed);
        self.hide();
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible.load(Ordering::Relaxed)
    }

    /// Is dismissed
    pub fn is_dismissed(&self) -> bool {
        self.dismissed.load(Ordering::Relaxed)
    }
}

/// Mako configuration
#[derive(Debug, Clone, Copy)]
pub struct MakoConfig {
    pub max_visible: u32,
    pub default_timeout: u32,
    pub sort: bool,
    pub layer: Layer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

impl Default for MakoConfig {
    fn default() -> Self {
        Self {
            max_visible: 5,
            default_timeout: 5000, // 5 seconds
            sort: true,
            layer: Layer::Overlay,
        }
    }
}

/// Mako daemon
pub struct MakoDaemon {
    pub notifications: BTreeMap<u32, Notification>,
    pub next_id: AtomicU32,
    pub config: MakoConfig,
    pub active: AtomicBool,
}

impl MakoDaemon {
    pub fn new() -> Self {
        Self {
            notifications: BTreeMap::new(),
            next_id: AtomicU32::new(1),
            config: MakoConfig::default(),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize mako daemon
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Create notification
    pub fn create_notification(
        &mut self,
        app_name: String,
        summary: String,
        body: String,
        urgency: NotificationUrgency,
    ) -> u32 {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let timeout = self.config.default_timeout;
        let notification = Notification::new(id, app_name, summary, body, urgency, timeout);
        self.notifications.insert(id, notification);
        id
    }

    /// Get notification
    pub fn get_notification(&self, id: u32) -> Option<&Notification> {
        self.notifications.get(&id)
    }

    /// Get mutable notification
    pub fn get_notification_mut(&mut self, id: u32) -> Option<&mut Notification> {
        self.notifications.get_mut(&id)
    }

    /// Show notification
    pub fn show_notification(&self, id: u32) -> Result<(), &'static str> {
        let notification = self
            .notifications
            .get(&id)
            .ok_or("Notification not found")?;
        notification.show();
        Ok(())
    }

    /// Dismiss notification
    pub fn dismiss_notification(&self, id: u32) -> Result<(), &'static str> {
        let notification = self
            .notifications
            .get(&id)
            .ok_or("Notification not found")?;
        notification.dismiss();
        Ok(())
    }

    /// Close notification
    pub fn close_notification(&mut self, id: u32) -> Result<(), &'static str> {
        self.notifications
            .remove(&id)
            .ok_or("Notification not found")?;
        Ok(())
    }

    /// Get visible notifications
    pub fn get_visible_notifications(&self) -> Vec<&Notification> {
        let mut visible: Vec<_> = self
            .notifications
            .values()
            .filter(|n| n.is_visible() && !n.is_dismissed())
            .collect();

        if self.config.sort {
            visible.sort_by(|a, b| (b.urgency as i32).cmp(&(a.urgency as i32)));
        }

        visible.truncate(self.config.max_visible as usize);
        visible
    }

    /// Dismiss all notifications
    pub fn dismiss_all(&self) {
        for notification in self.notifications.values() {
            notification.dismiss();
        }
    }

    /// Set configuration
    pub fn set_config(&mut self, config: MakoConfig) {
        self.config = config;
    }
}

impl Default for MakoDaemon {
    fn default() -> Self {
        Self::new()
    }
}

/// Global mako daemon
static mut MAKO_DAEMON: Option<MakoDaemon> = None;

/// Initialize mako daemon
pub fn init_mako_daemon() {
    unsafe {
        MAKO_DAEMON = Some(MakoDaemon::new());
    }
}

/// Get global mako daemon
pub fn get_mako_daemon() -> Option<&'static mut MakoDaemon> {
    unsafe { MAKO_DAEMON.as_mut() }
}

/// Send notification (convenience function)
pub fn send_notification(
    app_name: String,
    summary: String,
    body: String,
    urgency: NotificationUrgency,
) -> u32 {
    if let Some(daemon) = get_mako_daemon() {
        let id = daemon.create_notification(app_name, summary, body, urgency);
        daemon.show_notification(id).ok();
        id
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mako_daemon_init() {
        let mut daemon = MakoDaemon::new();
        daemon.init();
        assert!(daemon.active.load(Ordering::Relaxed));
    }

    #[test]
    fn test_create_notification() {
        let mut daemon = MakoDaemon::new();
        let id = daemon.create_notification(
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationUrgency::Normal,
        );
        assert!(daemon.get_notification(id).is_some());
    }

    #[test]
    fn test_show_notification() {
        let mut daemon = MakoDaemon::new();
        let id = daemon.create_notification(
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationUrgency::Normal,
        );
        daemon.show_notification(id).unwrap();
        assert!(daemon.get_notification(id).unwrap().is_visible());
    }

    #[test]
    fn test_dismiss_notification() {
        let mut daemon = MakoDaemon::new();
        let id = daemon.create_notification(
            "TestApp".to_string(),
            "Test Summary".to_string(),
            "Test Body".to_string(),
            NotificationUrgency::Normal,
        );
        daemon.show_notification(id).unwrap();
        daemon.dismiss_notification(id).unwrap();
        assert!(daemon.get_notification(id).unwrap().is_dismissed());
    }

    #[test]
    fn test_get_visible_notifications() {
        let mut daemon = MakoDaemon::new();
        let id1 = daemon.create_notification(
            "TestApp".to_string(),
            "Test1".to_string(),
            "Body1".to_string(),
            NotificationUrgency::Normal,
        );
        let id2 = daemon.create_notification(
            "TestApp".to_string(),
            "Test2".to_string(),
            "Body2".to_string(),
            NotificationUrgency::Critical,
        );
        daemon.show_notification(id1).unwrap();
        daemon.show_notification(id2).unwrap();
        let visible = daemon.get_visible_notifications();
        assert!(visible.len() > 0);
    }

    #[test]
    fn test_send_notification() {
        init_mako_daemon();
        let id = send_notification(
            "TestApp".to_string(),
            "Test".to_string(),
            "Body".to_string(),
            NotificationUrgency::Normal,
        );
        assert!(id > 0);
    }
}
