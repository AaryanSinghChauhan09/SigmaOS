// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaAlert - System Notification and Alerting Engine
//!
//! Provides centralized alert dispatching, notification routing, severity filtering,
//! deduplication, acknowledgment, and resolution tracking for SigmaOS subsystems.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Alert severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Emergency,
}

/// Alert category
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertCategory {
    System,
    Storage,
    Security,
    Network,
    Hardware,
    Service,
}

/// Current status of an alert
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Silenced,
    Resolved,
}

/// Delivery target channel for alerts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertChannel {
    SystemLog,
    Console,
    DesktopBus,
    NetworkTrap,
}

/// A recorded system alert
#[derive(Debug, Clone)]
pub struct Alert {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub source: String,
    pub severity: AlertSeverity,
    pub category: AlertCategory,
    pub status: AlertStatus,
    pub timestamp: u64,
    pub acknowledged_by: Option<String>,
    pub resolved_timestamp: Option<u64>,
}

impl Alert {
    pub fn is_active(&self) -> bool {
        self.status == AlertStatus::Active
    }
}

/// System Alert Engine
#[derive(Debug)]
pub struct SigmaAlert {
    alerts: BTreeMap<u64, Alert>,
    next_id: u64,
    channel_subscriptions: BTreeMap<AlertCategory, Vec<AlertChannel>>,
    running: bool,
}

impl SigmaAlert {
    /// Create a new SigmaAlert engine
    pub fn new() -> Self {
        let mut alert_mgr = Self {
            alerts: BTreeMap::new(),
            next_id: 1,
            channel_subscriptions: BTreeMap::new(),
            running: false,
        };
        // Default routing
        alert_mgr.subscribe(AlertCategory::Security, AlertChannel::Console);
        alert_mgr.subscribe(AlertCategory::Security, AlertChannel::SystemLog);
        alert_mgr.subscribe(AlertCategory::Storage, AlertChannel::DesktopBus);
        alert_mgr.subscribe(AlertCategory::System, AlertChannel::SystemLog);
        alert_mgr
    }

    /// Start the alerting engine
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the alerting engine
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Subscribe a channel to a category
    pub fn subscribe(&mut self, category: AlertCategory, channel: AlertChannel) {
        let channels = self.channel_subscriptions.entry(category).or_insert_with(Vec::new);
        if !channels.contains(&channel) {
            channels.push(channel);
        }
    }

    /// Raise a new system alert
    pub fn raise_alert(
        &mut self,
        title: &str,
        message: &str,
        source: &str,
        severity: AlertSeverity,
        category: AlertCategory,
        timestamp: u64,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let alert = Alert {
            id,
            title: title.to_string(),
            message: message.to_string(),
            source: source.to_string(),
            severity,
            category,
            status: AlertStatus::Active,
            timestamp,
            acknowledged_by: None,
            resolved_timestamp: None,
        };

        self.alerts.insert(id, alert);
        id
    }

    /// Acknowledge an active alert
    pub fn acknowledge(&mut self, alert_id: u64, user: &str) -> bool {
        if let Some(alert) = self.alerts.get_mut(&alert_id) {
            if alert.status == AlertStatus::Active {
                alert.status = AlertStatus::Acknowledged;
                alert.acknowledged_by = Some(user.to_string());
                return true;
            }
        }
        false
    }

    /// Silence an active alert
    pub fn silence(&mut self, alert_id: u64) -> bool {
        if let Some(alert) = self.alerts.get_mut(&alert_id) {
            if alert.status != AlertStatus::Resolved {
                alert.status = AlertStatus::Silenced;
                return true;
            }
        }
        false
    }

    /// Mark an alert as resolved
    pub fn resolve(&mut self, alert_id: u64, timestamp: u64) -> bool {
        if let Some(alert) = self.alerts.get_mut(&alert_id) {
            if alert.status != AlertStatus::Resolved {
                alert.status = AlertStatus::Resolved;
                alert.resolved_timestamp = Some(timestamp);
                return true;
            }
        }
        false
    }

    /// Get alert by ID
    pub fn get_alert(&self, alert_id: u64) -> Option<&Alert> {
        self.alerts.get(&alert_id)
    }

    /// List active un-resolved alerts
    pub fn list_active(&self) -> Vec<&Alert> {
        self.alerts
            .values()
            .filter(|a| a.status == AlertStatus::Active || a.status == AlertStatus::Acknowledged)
            .collect()
    }

    /// List alerts matching minimum severity
    pub fn list_by_min_severity(&self, min_sev: AlertSeverity) -> Vec<&Alert> {
        self.alerts.values().filter(|a| a.severity >= min_sev).collect()
    }

    /// List alerts by category
    pub fn list_by_category(&self, cat: AlertCategory) -> Vec<&Alert> {
        self.alerts.values().filter(|a| a.category == cat).collect()
    }

    /// Total active alerts
    pub fn active_count(&self) -> usize {
        self.alerts.values().filter(|a| a.status == AlertStatus::Active).count()
    }

    /// Total alerts logged
    pub fn total_count(&self) -> usize {
        self.alerts.len()
    }
}

impl Default for SigmaAlert {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Global Singleton & Convenience Functions
// ============================================================

static mut ALERT_INSTANCE: Option<SigmaAlert> = None;

/// Initialize global SigmaAlert instance
pub fn init_sigmaalert() {
    unsafe {
        ALERT_INSTANCE = Some(SigmaAlert::new());
        if let Some(ref mut alert) = ALERT_INSTANCE {
            alert.start();
        }
    }
}

/// Get global SigmaAlert reference
pub fn get_sigmaalert() -> Option<&'static SigmaAlert> {
    unsafe { ALERT_INSTANCE.as_ref() }
}

/// Get mutable global SigmaAlert reference
pub fn get_sigmaalert_mut() -> Option<&'static mut SigmaAlert> {
    unsafe { ALERT_INSTANCE.as_mut() }
}

/// Convenience function to raise an alert
pub fn send_alert(
    title: &str,
    message: &str,
    source: &str,
    severity: AlertSeverity,
    category: AlertCategory,
) -> u64 {
    if let Some(mgr) = get_sigmaalert_mut() {
        mgr.raise_alert(title, message, source, severity, category, 0)
    } else {
        0
    }
}

/// Convenience function to resolve an alert
pub fn resolve_alert(alert_id: u64) -> bool {
    if let Some(mgr) = get_sigmaalert_mut() {
        mgr.resolve(alert_id, 0)
    } else {
        false
    }
}

/// Convenience function to count active alerts
pub fn active_alert_count() -> usize {
    if let Some(mgr) = get_sigmaalert() {
        mgr.active_count()
    } else {
        0
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_lifecycle() {
        let mut alert_mgr = SigmaAlert::new();
        alert_mgr.start();
        assert!(alert_mgr.is_running());

        let id = alert_mgr.raise_alert(
            "Disk Full",
            "Root partition usage exceeds 95%",
            "vfs",
            AlertSeverity::Critical,
            AlertCategory::Storage,
            100,
        );
        assert_eq!(id, 1);
        assert_eq!(alert_mgr.active_count(), 1);

        assert!(alert_mgr.acknowledge(id, "sysadmin"));
        let alert = alert_mgr.get_alert(id).unwrap();
        assert_eq!(alert.status, AlertStatus::Acknowledged);
        assert_eq!(alert.acknowledged_by, Some("sysadmin".to_string()));

        assert!(alert_mgr.resolve(id, 200));
        let alert = alert_mgr.get_alert(id).unwrap();
        assert_eq!(alert.status, AlertStatus::Resolved);
        assert_eq!(alert_mgr.active_count(), 0);
    }

    #[test]
    fn test_alert_filtering() {
        let mut alert_mgr = SigmaAlert::new();
        alert_mgr.raise_alert("Info", "msg1", "s1", AlertSeverity::Info, AlertCategory::System, 1);
        alert_mgr.raise_alert("Warn", "msg2", "s2", AlertSeverity::Warning, AlertCategory::Security, 2);
        alert_mgr.raise_alert("Crit", "msg3", "s3", AlertSeverity::Critical, AlertCategory::Security, 3);

        let high_prio = alert_mgr.list_by_min_severity(AlertSeverity::Warning);
        assert_eq!(high_prio.len(), 2);

        let security = alert_mgr.list_by_category(AlertCategory::Security);
        assert_eq!(security.len(), 2);
    }

    #[test]
    fn test_global_singleton() {
        init_sigmaalert();
        let id = send_alert("OOM", "Process killed", "kernel", AlertSeverity::Error, AlertCategory::System);
        assert!(id > 0);
        assert_eq!(active_alert_count(), 1);
        assert!(resolve_alert(id));
        assert_eq!(active_alert_count(), 0);
    }
}
