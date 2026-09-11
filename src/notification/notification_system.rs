// src/notification/notification_system.rs
// Priority-based notification system for SigmaOS
// Superior to Omarchy's basic notification daemon
//
// Features:
// - Priority queuing (Critical, High, Normal, Low)
// - Do Not Disturb mode
// - Action buttons
// - History with search
// - Desktop integration
// - Grouped notifications

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::format;
use core::fmt;

/// Notification priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Priority {
    Critical,  // System errors, security alerts
    High,      // Important updates, warnings
    Normal,    // Regular notifications
    Low,       // Informational, tips
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Critical => write!(f, "Critical"),
            Self::High => write!(f, "High"),
            Self::Normal => write!(f, "Normal"),
            Self::Low => write!(f, "Low"),
        }
    }
}

/// Notification urgency
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

/// Notification action button
#[derive(Debug, Clone)]
pub struct Action {
    pub id: String,
    pub label: String,
    pub callback: ActionType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionType {
    Dismiss,
    Open,
    Reply,
    Snooze,
    Custom,
}

/// Notification entry
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: u64,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub priority: Priority,
    pub urgency: Urgency,
    pub timestamp: u64,
    pub timeout: Option<u32>,  // ms, None = no timeout
    pub actions: Vec<Action>,
    pub category: String,
    pub group_key: Option<String>,
    pub progress: Option<u8>,  // 0-100
    pub dismissed: bool,
    pub read: bool,
}

impl Notification {
    pub fn new(app_name: &str, summary: &str, body: &str) -> Self {
        static mut NEXT_ID: u64 = 1;
        let id = unsafe {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        };
        
        Self {
            id,
            app_name: app_name.into(),
            app_icon: "app-notification".into(),
            summary: summary.into(),
            body: body.into(),
            priority: Priority::Normal,
            urgency: Urgency::Normal,
            timestamp: Self::get_time(),
            timeout: Some(5000),  // 5 seconds default
            actions: Vec::new(),
            category: String::new(),
            group_key: None,
            progress: None,
            dismissed: false,
            read: false,
        }
    }
    
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    
    pub fn with_urgency(mut self, urgency: Urgency) -> Self {
        self.urgency = urgency;
        self
    }
    
    pub fn with_timeout(mut self, timeout_ms: u32) -> Self {
        self.timeout = Some(timeout_ms);
        self
    }
    
    pub fn persistent(mut self) -> Self {
        self.timeout = None;
        self
    }
    
    pub fn with_icon(mut self, icon: &str) -> Self {
        self.app_icon = icon.into();
        self
    }
    
    pub fn with_category(mut self, category: &str) -> Self {
        self.category = category.into();
        self
    }
    
    pub fn with_group_key(mut self, group_key: &str) -> Self {
        self.group_key = Some(group_key.into());
        self
    }
    
    pub fn with_progress(mut self, progress: u8) -> Self {
        self.progress = Some(progress.min(100));
        self
    }
    
    pub fn add_action(mut self, id: &str, label: &str, callback: ActionType) -> Self {
        self.actions.push(Action {
            id: id.into(),
            label: label.into(),
            callback,
        });
        self
    }
    
    fn get_time() -> u64 {
        // In real implementation, would get actual timestamp
        0
    }
}

/// Do Not Disturb configuration
#[derive(Debug, Clone)]
pub struct DndConfig {
    pub enabled: bool,
    pub allow_critical: bool,
    pub schedule_start: Option<(u8, u8)>,  // (hour, minute)
    pub schedule_end: Option<(u8, u8)>,
    pub allowed_apps: Vec<String>,
}

impl Default for DndConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            allow_critical: true,
            schedule_start: None,
            schedule_end: None,
            allowed_apps: Vec::new(),
        }
    }
}

/// Notification system
pub struct NotificationSystem {
    notifications: BTreeMap<u64, Notification>,
    queue: Vec<u64>,  // Sorted by priority
    history: Vec<u64>,
    dnd_config: DndConfig,
    max_history: usize,
}

impl NotificationSystem {
    pub fn new() -> Self {
        Self {
            notifications: BTreeMap::new(),
            queue: Vec::new(),
            history: Vec::new(),
            dnd_config: DndConfig::default(),
            max_history: 100,
        }
    }
    
    pub fn notify(&mut self, notification: Notification) -> u64 {
        let id = notification.id;
        
        // Check DND mode
        if self.should_block_notification(&notification) {
            // Store in history but don't show
            self.history.insert(0, id);
            self.notifications.insert(id, notification);
            return id;
        }
        
        // Add to queue based on priority
        self.queue.push(id);
        self.sort_queue();
        
        // Store notification
        self.notifications.insert(id, notification);
        
        // Trim history if needed
        if self.history.len() >= self.max_history {
            self.history.truncate(self.max_history);
        }
        
        id
    }
    
    fn should_block_notification(&self, notification: &Notification) -> bool {
        if !self.dnd_config.enabled {
            return false;
        }
        
        // Always allow critical if configured
        if self.dnd_config.allow_critical && 
           notification.priority == Priority::Critical {
            return false;
        }
        
        // Check allowed apps
        if self.dnd_config.allowed_apps.contains(&notification.app_name) {
            return false;
        }
        
        true
    }
    
    fn sort_queue(&mut self) {
        self.queue.sort_by(|a, b| {
            let notif_a = self.notifications.get(a);
            let notif_b = self.notifications.get(b);
            
            match (notif_a, notif_b) {
                (Some(a), Some(b)) => b.priority.cmp(&a.priority),
                _ => core::cmp::Ordering::Equal,
            }
        });
    }
    
    pub fn get_next_notification(&mut self) -> Option<Notification> {
        if let Some(id) = self.queue.first() {
            let id = *id;
            self.queue.remove(0);
            self.notifications.get(&id).cloned()
        } else {
            None
        }
    }
    
    pub fn get_all_notifications(&self) -> Vec<Notification> {
        self.queue.iter()
            .filter_map(|id| self.notifications.get(id))
            .cloned()
            .collect()
    }
    
    pub fn get_grouped_notifications(&self, group_key: &str) -> Vec<Notification> {
        self.queue.iter()
            .filter_map(|id| {
                self.notifications.get(id).and_then(|n| {
                    if n.group_key.as_deref() == Some(group_key) {
                        Some(n.clone())
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    
    pub fn dismiss(&mut self, id: u64) {
        self.queue.retain(|&notif_id| notif_id != id);
        
        if let Some(notification) = self.notifications.get_mut(&id) {
            notification.dismissed = true;
            self.history.insert(0, id);
        }
    }
    
    pub fn dismiss_all(&mut self) {
        for id in &self.queue {
            if let Some(notification) = self.notifications.get_mut(id) {
                notification.dismissed = true;
                self.history.insert(0, *id);
            }
        }
        self.queue.clear();
    }
    
    pub fn dismiss_group(&mut self, group_key: &str) {
        let mut to_dismiss = Vec::new();
        
        for id in &self.queue {
            if let Some(notification) = self.notifications.get(id) {
                if notification.group_key.as_deref() == Some(group_key) {
                    to_dismiss.push(*id);
                }
            }
        }
        
        for id in to_dismiss {
            self.dismiss(id);
        }
    }
    
    pub fn mark_read(&mut self, id: u64) {
        if let Some(notification) = self.notifications.get_mut(&id) {
            notification.read = true;
        }
    }
    
    pub fn get_history(&self) -> Vec<Notification> {
        self.history.iter()
            .filter_map(|id| self.notifications.get(id))
            .cloned()
            .collect()
    }
    
    pub fn search_history(&self, query: &str) -> Vec<Notification> {
        let query = query.to_lowercase();
        
        self.history.iter()
            .filter_map(|id| self.notifications.get(id))
            .filter(|n| {
                n.summary.to_lowercase().contains(&query) ||
                n.body.to_lowercase().contains(&query) ||
                n.app_name.to_lowercase().contains(&query)
            })
            .cloned()
            .collect()
    }
    
    pub fn clear_history(&mut self) {
        for id in &self.history {
            self.notifications.remove(id);
        }
        self.history.clear();
    }
    
    pub fn enable_dnd(&mut self) {
        self.dnd_config.enabled = true;
    }
    
    pub fn disable_dnd(&mut self) {
        self.dnd_config.enabled = false;
    }
    
    pub fn set_dnd_schedule(&mut self, start: (u8, u8), end: (u8, u8)) {
        self.dnd_config.schedule_start = Some(start);
        self.dnd_config.schedule_end = Some(end);
    }
    
    pub fn allow_app_in_dnd(&mut self, app_name: &str) {
        if !self.dnd_config.allowed_apps.contains(&app_name.to_string()) {
            self.dnd_config.allowed_apps.push(app_name.into());
        }
    }
    
    pub fn get_queue_count(&self) -> usize {
        self.queue.len()
    }
    
    pub fn get_unread_count(&self) -> usize {
        self.queue.iter()
            .filter_map(|id| self.notifications.get(id))
            .filter(|n| !n.read)
            .count()
    }
}

impl Default for NotificationSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification builder for common notifications
pub mod presets {
    use super::*;
    
    pub fn system_update(version: &str) -> Notification {
        Notification::new(
            "System",
            "Update Available",
            &format!("SigmaOS {} is ready to install", version)
        )
        .with_priority(Priority::High)
        .with_icon("system-software-update")
        .with_category("system.update")
        .add_action("install", "Install Now", ActionType::Open)
        .add_action("dismiss", "Later", ActionType::Dismiss)
    }
    
    pub fn security_alert(message: &str) -> Notification {
        Notification::new(
            "Security",
            "Security Alert",
            message
        )
        .with_priority(Priority::Critical)
        .with_urgency(Urgency::Critical)
        .with_icon("security-high")
        .with_category("security.alert")
        .persistent()
    }
    
    pub fn download_complete(filename: &str) -> Notification {
        Notification::new(
            "Downloads",
            "Download Complete",
            &format!("{} finished downloading", filename)
        )
        .with_priority(Priority::Low)
        .with_icon("folder-download")
        .with_category("transfer.complete")
        .add_action("open", "Open", ActionType::Open)
        .add_action("show", "Show in Folder", ActionType::Open)
    }
    
    pub fn battery_low(percent: u8) -> Notification {
        Notification::new(
            "Power",
            "Battery Low",
            &format!("{}% battery remaining", percent)
        )
        .with_priority(Priority::High)
        .with_icon("battery-caution")
        .with_category("power.low")
    }
    
    pub fn network_connected(ssid: &str) -> Notification {
        Notification::new(
            "Network",
            "Connected",
            &format!("Connected to {}", ssid)
        )
        .with_priority(Priority::Low)
        .with_icon("network-wireless")
        .with_category("network.connected")
        .with_timeout(3000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notification_creation() {
        let notif = Notification::new("Test App", "Summary", "Body")
            .with_priority(Priority::High)
            .with_timeout(10000);
        
        assert_eq!(notif.app_name, "Test App");
        assert_eq!(notif.summary, "Summary");
        assert_eq!(notif.priority, Priority::High);
        assert_eq!(notif.timeout, Some(10000));
    }
    
    #[test]
    fn test_notification_system() {
        let mut system = NotificationSystem::new();
        
        let notif = Notification::new("App", "Test", "Body");
        let id = system.notify(notif);
        
        assert_eq!(system.get_queue_count(), 1);
        
        let next = system.get_next_notification();
        assert!(next.is_some());
        assert_eq!(next.unwrap().id, id);
        
        assert_eq!(system.get_queue_count(), 0);
    }
    
    #[test]
    fn test_priority_ordering() {
        let mut system = NotificationSystem::new();
        
        system.notify(Notification::new("App", "Low", "Body").with_priority(Priority::Low));
        system.notify(Notification::new("App", "Critical", "Body").with_priority(Priority::Critical));
        system.notify(Notification::new("App", "Normal", "Body").with_priority(Priority::Normal));
        
        let next = system.get_next_notification().unwrap();
        assert_eq!(next.summary, "Critical");
    }
    
    #[test]
    fn test_dnd_mode() {
        let mut system = NotificationSystem::new();
        system.enable_dnd();
        
        let notif = Notification::new("App", "Test", "Body");
        system.notify(notif);
        
        // Should be in history but not queue
        assert_eq!(system.get_queue_count(), 0);
        assert_eq!(system.get_history().len(), 1);
    }
    
    #[test]
    fn test_grouped_notifications() {
        let mut system = NotificationSystem::new();
        
        system.notify(Notification::new("App", "Msg 1", "Body").with_group_key("chat"));
        system.notify(Notification::new("App", "Msg 2", "Body").with_group_key("chat"));
        system.notify(Notification::new("App", "Other", "Body"));
        
        let grouped = system.get_grouped_notifications("chat");
        assert_eq!(grouped.len(), 2);
    }
}
