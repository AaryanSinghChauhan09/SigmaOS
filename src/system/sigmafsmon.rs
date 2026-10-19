//! SigmaFSMon - Filesystem Monitoring
//!
//! Inspired by Linux inotify, fanotify, and iostat, this module provides
//! comprehensive filesystem monitoring, I/O performance tracking, and event
//! auditing for SigmaOS.
//!
//! Features include:
//! - File and directory watch registration
//! - Event notification tracking (Create, Modify, Delete, Access, Mount, Unmount)
//! - I/O statistics tracking (IOPS, throughput, latency, queue depth)
//! - Capacity and threshold alerts
//! - Global instance and convenience API

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Filesystem event type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FSEventType {
    Create,
    Modify,
    Delete,
    Access,
    AttributeChange,
    Mount,
    Unmount,
    Overflow,
    Unknown,
;

/// Filesystem monitor status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FSMonState {
    Active,
    Paused,
    Stopped,
    Warning,
    Critical,
    Unknown,
;

/// Alert severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
;

/// Tracked filesystem event
#[derive(Debug, Clone)]
pub struct FSEvent {
    pub event_id: u64,
    pub path: String,
    pub event_type: FSEventType,
    pub timestamp: u64,
    pub process_id: u32,
    pub is_directory: bool,
;

impl FSEvent {
    /// Create a new filesystem event
    pub fn new(
        event_id: u64,
        path: String,
        event_type: FSEventType,
        timestamp: u64,
        process_id: u32,
        is_directory: bool,
    ) -> Self {
        Self {
            event_id,
            path,
            event_type,
            timestamp,
            process_id,
            is_directory,
        }
    }
;

/// Watch descriptor for monitoring paths
#[derive(Debug, Clone)]
pub struct FSWatch {
    pub watch_id: String,
    pub path: String,
    pub recursive: bool,
    pub mask: u32,
    pub is_active: bool,
;

impl FSWatch {
    /// Create a new watch descriptor
    pub fn new(watch_id: String, path: String, recursive: bool, mask: u32) -> Self {
        Self {
            watch_id,
            path,
            recursive,
            mask,
            is_active: true,
        }
    }

    /// Set watch active status
    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }
;

/// Filesystem I/O performance and capacity metrics
#[derive(Debug, Clone)]
pub struct FSMetrics {
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub average_latency_us: u32,
;

impl FSMetrics {
    /// Create a new filesystem metrics record
    pub fn new(mount_point: String, total_bytes: u64) -> Self {
        Self {
            mount_point,
            total_bytes,
            used_bytes: 0,
            read_ops: 0,
            write_ops: 0,
            bytes_read: 0,
            bytes_written: 0,
            average_latency_us: 0,
        }
    }

    /// Calculate usage percentage (0 - 100)
    pub fn usage_percent(&self) -> u8 {
        if self.total_bytes == 0 {
            0
        } else {
            ((self.used_bytes as u128 * 100) / self.total_bytes as u128) as u8
        }
    }

    /// Record a read operation
    pub fn record_read(&mut self, bytes: u64, latency_us: u32) {
        self.read_ops += 1;
        self.bytes_read += bytes;
        self.update_latency(latency_us);
    }

    /// Record a write operation
    pub fn record_write(&mut self, bytes: u64, latency_us: u32) {
        self.write_ops += 1;
        self.bytes_written += bytes;
        self.update_latency(latency_us);
    }

    fn update_latency(&mut self, latency_us: u32) {
        let total_ops = self.read_ops + self.write_ops;
        if total_ops > 0 {
            self.average_latency_us = (((self.average_latency_us as u64 * (total_ops - 1))
                + latency_us as u64)
                / total_ops) as u32;
        }
    }
;

/// Filesystem capacity or performance alert
#[derive(Debug, Clone)]
pub struct FSAlert {
    pub alert_id: String,
    pub mount_point: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: u64,
;

/// Main SigmaFSMon manager struct
pub struct SigmaFSMon {
    watches: BTreeMap<String, FSWatch>,
    metrics: BTreeMap<String, FSMetrics>,
    events: Vec<FSEvent>,
    alerts: Vec<FSAlert>,
    running: AtomicBool,
    state: FSMonState,
    max_event_history: usize,
    warning_threshold_pct: u8,
    critical_threshold_pct: u8,
;

impl SigmaFSMon {
    /// Create a new SigmaFSMon instance
    pub fn new() -> Self {
        Self {
            watches: BTreeMap::new(),
            metrics: BTreeMap::new(),
            events: Vec::new(),
            alerts: Vec::new(),
            running: AtomicBool::new(false),
            state: FSMonState::Stopped,
            max_event_history: 1000,
            warning_threshold_pct: 85,
            critical_threshold_pct: 95,
        }
    }

    /// Start the filesystem monitoring daemon
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
        self.state = FSMonState::Active;
    }

    /// Stop the monitoring daemon
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.state = FSMonState::Stopped;
    }

    /// Pause monitoring
    pub fn pause(&mut self) {
        self.state = FSMonState::Paused;
    }

    /// Check if daemon is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Get current state
    pub fn state(&self) -> FSMonState {
        self.state
    }

    /// Register a path watch
    pub fn add_watch(&mut self, watch: FSWatch) {
        self.watches.insert(watch.watch_id.clone(), watch);
    }

    /// Remove a watch by ID
    pub fn remove_watch(&mut self, watch_id: &str) -> Option<FSWatch> {
        self.watches.remove(watch_id)
    }

    /// Get watch by ID
    pub fn get_watch(&self, watch_id: &str) -> Option<&FSWatch> {
        self.watches.get(watch_id)
    }

    /// List all watches
    pub fn list_watches(&self) -> Vec<&FSWatch> {
        self.watches.values().collect()
    }

    /// Register or update filesystem metrics
    pub fn register_mount(&mut self, metrics: FSMetrics) {
        self.metrics.insert(metrics.mount_point.clone(), metrics);
    }

    /// Get metrics for a mount point
    pub fn get_metrics(&self, mount_point: &str) -> Option<&FSMetrics> {
        self.metrics.get(mount_point)
    }

    /// Get mutable metrics for a mount point
    pub fn get_metrics_mut(&mut self, mount_point: &str) -> Option<&mut FSMetrics> {
        self.metrics.get_mut(mount_point)
    }

    /// Record a filesystem event
    pub fn record_event(&mut self, event: FSEvent) {
        if self.events.len() >= self.max_event_history {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    /// List recorded events
    pub fn list_events(&self) -> &[FSEvent] {
        &self.events
    }

    /// List events matching a path prefix
    pub fn list_events_by_path(&self, prefix: &str) -> Vec<&FSEvent> {
        self.events
            .iter()
            .filter(|e| e.path.starts_with(prefix))
            .collect()
    }

    /// List events by type
    pub fn list_events_by_type(&self, event_type: FSEventType) -> Vec<&FSEvent> {
        self.events
            .iter()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// Check capacity thresholds and emit alerts
    pub fn check_thresholds(&mut self, timestamp: u64) -> Vec<FSAlert> {
        let mut new_alerts = Vec::new();
        for (mount, m) in self.metrics.iter() {
            let pct = m.usage_percent();
            if pct >= self.critical_threshold_pct {
                let alert = FSAlert {
                    alert_id: alloc::format!("alert-crit-{}", mount),
                    mount_point: mount.clone(),
                    severity: AlertSeverity::Critical,
                    message: alloc::format!("Filesystem {} critically full at {}%", mount, pct),
                    timestamp,
                };
                new_alerts.push(alert);
            } else if pct >= self.warning_threshold_pct {
                let alert = FSAlert {
                    alert_id: alloc::format!("alert-warn-{}", mount),
                    mount_point: mount.clone(),
                    severity: AlertSeverity::Warning,
                    message: alloc::format!("Filesystem {} usage high at {}%", mount, pct),
                    timestamp,
                };
                new_alerts.push(alert);
            }
        }

        for alert in &new_alerts {
            self.alerts.push(alert.clone());
        }
        new_alerts
    }

    /// List active alerts
    pub fn list_alerts(&self) -> &[FSAlert] {
        &self.alerts
    }

    /// Total active watch count
    pub fn watch_count(&self) -> usize {
        self.watches.len()
    }

    /// Total event count in history
    pub fn event_count(&self) -> usize {
        self.events.len()
    }
;

impl Default for SigmaFSMon {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaFSMon instance (placeholder for global state)
static mut SIGMA_FSMON_INSTANCE: Option<SigmaFSMon> = None;

/// Initialize global SigmaFSMon instance
pub fn init_sigmafsmon() {
    unsafe {
        SIGMA_FSMON_INSTANCE = Some(SigmaFSMon::new());
    }
;

/// Get global SigmaFSMon reference
pub fn get_sigmafsmon() -> Option<&'static SigmaFSMon> {
    unsafe { SIGMA_FSMON_INSTANCE.as_ref() }
;

/// Get mutable global SigmaFSMon reference
pub fn get_sigmafsmon_mut() -> Option<&'static mut SigmaFSMon> {
    unsafe { SIGMA_FSMON_INSTANCE.as_mut() }
;

/// Convenience function to add a filesystem watch
pub fn add_fs_watch(watch: FSWatch) {
    if let Some(mon) = get_sigmafsmon_mut() {
        mon.add_watch(watch);
    }
;

/// Convenience function to record an event
pub fn record_fs_event(event: FSEvent) {
    if let Some(mon) = get_sigmafsmon_mut() {
        mon.record_event(event);
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_watch_creation() {
        let watch = FSWatch::new("w1".to_string(), "/etc".to_string(), true, 0xFF);
        assert_eq!(watch.watch_id, "w1");
        assert_eq!(watch.path, "/etc");
        assert!(watch.recursive);
        assert!(watch.is_active);
    }

    #[test]
    fn test_fs_metrics_and_usage() {
        let mut metrics = FSMetrics::new("/home".to_string(), 100_000_000);
        assert_eq!(metrics.usage_percent(), 0);

        metrics.used_bytes = 90_000_000;
        assert_eq!(metrics.usage_percent(), 90);

        metrics.record_read(4096, 120);
        metrics.record_write(8192, 240);
        assert_eq!(metrics.read_ops, 1);
        assert_eq!(metrics.write_ops, 1);
        assert_eq!(metrics.bytes_read, 4096);
        assert_eq!(metrics.bytes_written, 8192);
        assert!(metrics.average_latency_us > 0);
    }

    #[test]
    fn test_sigmafsmon_lifecycle_and_events() {
        let mut mon = SigmaFSMon::new();
        mon.start();
        assert!(mon.is_running());
        assert_eq!(mon.state(), FSMonState::Active);

        let watch = FSWatch::new("w-var".to_string(), "/var/log".to_string(), false, 1);
        mon.add_watch(watch);
        assert_eq!(mon.watch_count(), 1);

        let ev1 = FSEvent::new(
            1,
            "/var/log/syslog".to_string(),
            FSEventType::Modify,
            100,
            42,
            false,
        );
        let ev2 = FSEvent::new(
            2,
            "/var/log/auth.log".to_string(),
            FSEventType::Create,
            101,
            42,
            false,
        );
        mon.record_event(ev1);
        mon.record_event(ev2);
        assert_eq!(mon.event_count(), 2);

        let created_events = mon.list_events_by_type(FSEventType::Create);
        assert_eq!(created_events.len(), 1);

        mon.stop();
        assert!(!mon.is_running());
        assert_eq!(mon.state(), FSMonState::Stopped);
    }

    #[test]
    fn test_threshold_alerts() {
        let mut mon = SigmaFSMon::new();
        let mut m = FSMetrics::new("/data".to_string(), 1_000_000);
        m.used_bytes = 960_000; // 96% -> critical
        mon.register_mount(m);

        let alerts = mon.check_thresholds(1000);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].severity, AlertSeverity::Critical);
    }

    #[test]
    fn test_global_instance() {
        init_sigmafsmon();
        let watch = FSWatch::new("gw".to_string(), "/tmp".to_string(), false, 0);
        add_fs_watch(watch);

        let mon = get_sigmafsmon().unwrap();
        assert_eq!(mon.watch_count(), 1);
    }
;
