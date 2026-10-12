// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaTimer - Timer Units
//!
//! SigmaTimer provides timer units for SigmaOS, inspired by systemd-timers.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Timer trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerType {
    /// Calendar-based timer (cron-like)
    Calendar,
    /// Monotonic timer (elapsed time)
    Monotonic,
    /// Boot-based timer (time since boot)
    Boot,
    /// Active-based timer (system active time)
    Active,
}

impl fmt::Display for TriggerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerType::Calendar => write!(f, "calendar"),
            TriggerType::Monotonic => write!(f, "monotonic"),
            TriggerType::Boot => write!(f, "boot"),
            TriggerType::Active => write!(f, "active"),
        }
    }
}

/// Timer unit status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStatus {
    /// Timer is enabled
    Enabled,
    /// Timer is disabled
    Disabled,
    /// Timer is running
    Running,
    /// Timer failed
    Failed,
    /// Timer is waiting
    Waiting,
}

impl fmt::Display for TimerStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerStatus::Enabled => write!(f, "enabled"),
            TimerStatus::Disabled => write!(f, "disabled"),
            TimerStatus::Running => write!(f, "running"),
            TimerStatus::Failed => write!(f, "failed"),
            TimerStatus::Waiting => write!(f, "waiting"),
        }
    }
}

/// Timer accuracy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerAccuracy {
    /// High accuracy (microsecond)
    High,
    /// Normal accuracy (millisecond)
    Normal,
    /// Low accuracy (second)
    Low,
}

impl fmt::Display for TimerAccuracy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerAccuracy::High => write!(f, "high"),
            TimerAccuracy::Normal => write!(f, "normal"),
            TimerAccuracy::Low => write!(f, "low"),
        }
    }
}

/// Timer unit
#[derive(Debug, Clone)]
pub struct TimerUnit {
    /// Timer name
    pub name: String,
    /// Service to trigger
    pub service: String,
    /// Trigger type
    pub trigger_type: TriggerType,
    /// Calendar expression (for calendar timers)
    pub calendar_expr: String,
    /// Monotonic delay in seconds (for monotonic timers)
    pub monotonic_delay: u64,
    /// Boot delay in seconds (for boot timers)
    pub boot_delay: u64,
    /// Active delay in seconds (for active timers)
    pub active_delay: u64,
    /// Timer accuracy
    pub accuracy: TimerAccuracy,
    /// Persistent across reboots
    pub persistent: bool,
    /// Wake system if suspended
    pub wake_system: bool,
    /// Timer status
    pub status: TimerStatus,
    /// Enabled flag
    pub enabled: bool,
    /// Last trigger timestamp
    pub last_trigger: Option<u64>,
    /// Next trigger timestamp
    pub next_trigger: Option<u64>,
    /// Trigger count
    pub trigger_count: u32,
}

impl TimerUnit {
    /// Create a new timer unit
    pub fn new(name: String, service: String) -> Self {
        Self {
            name,
            service,
            trigger_type: TriggerType::Calendar,
            calendar_expr: String::new(),
            monotonic_delay: 0,
            boot_delay: 0,
            active_delay: 0,
            accuracy: TimerAccuracy::Normal,
            persistent: false,
            wake_system: false,
            status: TimerStatus::Enabled,
            enabled: true,
            last_trigger: None,
            next_trigger: None,
            trigger_count: 0,
        }
    }

    /// Set trigger type
    pub fn with_trigger_type(mut self, trigger_type: TriggerType) -> Self {
        self.trigger_type = trigger_type;
        self
    }

    /// Set calendar expression
    pub fn with_calendar_expr(mut self, expr: String) -> Self {
        self.calendar_expr = expr;
        self.trigger_type = TriggerType::Calendar;
        self
    }

    /// Set monotonic delay
    pub fn with_monotonic_delay(mut self, delay: u64) -> Self {
        self.monotonic_delay = delay;
        self.trigger_type = TriggerType::Monotonic;
        self
    }

    /// Set boot delay
    pub fn with_boot_delay(mut self, delay: u64) -> Self {
        self.boot_delay = delay;
        self.trigger_type = TriggerType::Boot;
        self
    }

    /// Set active delay
    pub fn with_active_delay(mut self, delay: u64) -> Self {
        self.active_delay = delay;
        self.trigger_type = TriggerType::Active;
        self
    }

    /// Set accuracy
    pub fn with_accuracy(mut self, accuracy: TimerAccuracy) -> Self {
        self.accuracy = accuracy;
        self
    }

    /// Set persistent
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    /// Set wake system
    pub fn with_wake_system(mut self, wake: bool) -> Self {
        self.wake_system = wake;
        self
    }

    /// Enable timer
    pub fn enable(&mut self) {
        self.enabled = true;
        self.status = TimerStatus::Enabled;
    }

    /// Disable timer
    pub fn disable(&mut self) {
        self.enabled = false;
        self.status = TimerStatus::Disabled;
    }

    /// Record trigger
    pub fn record_trigger(&mut self, timestamp: u64) {
        self.last_trigger = Some(timestamp);
        self.trigger_count += 1;
    }

    /// Set next trigger
    pub fn set_next_trigger(&mut self, timestamp: u64) {
        self.next_trigger = Some(timestamp);
    }
}

/// SigmaTimer - Timer Units Manager
pub struct SigmaTimer {
    /// Timer units (name -> timer)
    timers: BTreeMap<String, TimerUnit>,
    /// Running state
    running: bool,
    /// Current timestamp
    current_time: u64,
    /// Boot timestamp
    boot_time: u64,
}

impl SigmaTimer {
    /// Create a new SigmaTimer instance
    pub fn new() -> Self {
        Self {
            timers: BTreeMap::new(),
            running: false,
            current_time: 0,
            boot_time: 0,
        }
    }

    /// Start the timer manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the timer manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a timer unit
    pub fn add_timer(&mut self, timer: TimerUnit) -> Result<(), String> {
        if self.timers.contains_key(&timer.name) {
            return Err(format!("Timer '{}' already exists", timer.name));
        }
        self.timers.insert(timer.name.clone(), timer);
        Ok(())
    }

    /// Remove a timer unit
    pub fn remove_timer(&mut self, name: &str) -> Result<(), String> {
        if self.timers.remove(name).is_none() {
            return Err(format!("Timer '{}' not found", name));
        }
        Ok(())
    }

    /// Get a timer unit
    pub fn get_timer(&self, name: &str) -> Option<&TimerUnit> {
        self.timers.get(name)
    }

    /// Get a timer unit (mutable)
    pub fn get_timer_mut(&mut self, name: &str) -> Option<&mut TimerUnit> {
        self.timers.get_mut(name)
    }

    /// List all timers
    pub fn list_timers(&self) -> Vec<&TimerUnit> {
        self.timers.values().collect()
    }

    /// List enabled timers
    pub fn list_enabled_timers(&self) -> Vec<&TimerUnit> {
        self.timers.values().filter(|t| t.enabled).collect()
    }

    /// Enable a timer
    pub fn enable_timer(&mut self, name: &str) -> Result<(), String> {
        match self.timers.get_mut(name) {
            Some(timer) => {
                timer.enable();
                Ok(())
            }
            None => Err(format!("Timer '{}' not found", name)),
        }
    }

    /// Disable a timer
    pub fn disable_timer(&mut self, name: &str) -> Result<(), String> {
        match self.timers.get_mut(name) {
            Some(timer) => {
                timer.disable();
                Ok(())
            }
            None => Err(format!("Timer '{}' not found", name)),
        }
    }

    /// Set current time
    pub fn set_time(&mut self, time: u64) {
        self.current_time = time;
    }

    /// Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    /// Set boot time
    pub fn set_boot_time(&mut self, time: u64) {
        self.boot_time = time;
    }

    /// Get boot time
    pub fn boot_time(&self) -> u64 {
        self.boot_time
    }

    /// Get timer count
    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }

    /// Get enabled timer count
    pub fn enabled_timer_count(&self) -> usize {
        self.timers.values().filter(|t| t.enabled).count()
    }

    /// Get timer statistics
    pub fn timer_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_timers".to_string(), self.timer_count().to_string());
        stats.insert("enabled_timers".to_string(), self.enabled_timer_count().to_string());
        stats.insert("disabled_timers".to_string(), (self.timer_count() - self.enabled_timer_count()).to_string());
        stats
    }
}

impl Default for SigmaTimer {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaTimer instance (placeholder for actual global state)
static unsafe mut SIGMATIMER_INSTANCE: Option<SigmaTimer> = None;

/// Initialize the global SigmaTimer instance
pub fn init_sigmatimer() {
    unsafe {
        SIGMATIMER_INSTANCE = Some(SigmaTimer::new());
    }
}

/// Get the global SigmaTimer instance
pub fn get_sigmatimer() -> Option<&'static mut SigmaTimer> {
    unsafe { SIGMATIMER_INSTANCE.as_mut() }
}

/// Convenience function to add a timer unit
pub fn add_timer_unit(timer: TimerUnit) -> Result<(), String> {
    if let Some(timer_mgr) = get_sigmatimer() {
        timer_mgr.add_timer(timer)
    } else {
        Err("SigmaTimer not initialized".to_string())
    }
}

/// Convenience function to enable a timer
pub fn enable_timer_unit(name: &str) -> Result<(), String> {
    if let Some(timer_mgr) = get_sigmatimer() {
        timer_mgr.enable_timer(name)
    } else {
        Err("SigmaTimer not initialized".to_string())
    }
}

/// Convenience function to disable a timer
pub fn disable_timer_unit(name: &str) -> Result<(), String> {
    if let Some(timer_mgr) = get_sigmatimer() {
        timer_mgr.disable_timer(name)
    } else {
        Err("SigmaTimer not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_type_display() {
        assert_eq!(TriggerType::Calendar.to_string(), "calendar");
        assert_eq!(TriggerType::Monotonic.to_string(), "monotonic");
    }

    #[test]
    fn test_timer_status_display() {
        assert_eq!(TimerStatus::Enabled.to_string(), "enabled");
        assert_eq!(TimerStatus::Running.to_string(), "running");
    }

    #[test]
    fn test_timer_accuracy_display() {
        assert_eq!(TimerAccuracy::High.to_string(), "high");
        assert_eq!(TimerAccuracy::Normal.to_string(), "normal");
    }

    #[test]
    fn test_timer_unit() {
        let timer = TimerUnit::new("daily-backup".to_string(), "backup.service".to_string())
            .with_trigger_type(TriggerType::Calendar)
            .with_calendar_expr("0 2 * * *".to_string())
            .with_accuracy(TimerAccuracy::Normal)
            .with_persistent(true)
            .with_wake_system(false);

        assert_eq!(timer.name, "daily-backup");
        assert_eq!(timer.service, "backup.service");
        assert_eq!(timer.trigger_type, TriggerType::Calendar);
        assert_eq!(timer.calendar_expr, "0 2 * * *");
        assert!(timer.persistent);
    }

    #[test]
    fn test_timer_unit_monotonic() {
        let timer = TimerUnit::new("delayed-task".to_string(), "task.service".to_string())
            .with_monotonic_delay(3600);

        assert_eq!(timer.trigger_type, TriggerType::Monotonic);
        assert_eq!(timer.monotonic_delay, 3600);
    }

    #[test]
    fn test_timer_unit_boot() {
        let timer = TimerUnit::new("boot-task".to_string(), "task.service".to_string())
            .with_boot_delay(60);

        assert_eq!(timer.trigger_type, TriggerType::Boot);
        assert_eq!(timer.boot_delay, 60);
    }

    #[test]
    fn test_timer_unit_enable_disable() {
        let mut timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        
        assert!(timer.enabled);
        timer.disable();
        assert!(!timer.enabled);
        timer.enable();
        assert!(timer.enabled);
    }

    #[test]
    fn test_timer_unit_record_trigger() {
        let mut timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        
        timer.record_trigger(1000);
        assert_eq!(timer.trigger_count, 1);
        assert_eq!(timer.last_trigger, Some(1000));
        
        timer.record_trigger(2000);
        assert_eq!(timer.trigger_count, 2);
    }

    #[test]
    fn test_sigmatimer() {
        let mut timer_mgr = SigmaTimer::new();
        assert!(!timer_mgr.is_running());
        timer_mgr.start();
        assert!(timer_mgr.is_running());
        timer_mgr.stop();
        assert!(!timer_mgr.is_running());
    }

    #[test]
    fn test_sigmatimer_add_timer() {
        let mut timer_mgr = SigmaTimer::new();
        
        let timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        assert!(timer_mgr.add_timer(timer).is_ok());
        assert_eq!(timer_mgr.timer_count(), 1);
    }

    #[test]
    fn test_sigmatimer_duplicate_timer() {
        let mut timer_mgr = SigmaTimer::new();
        
        let timer1 = TimerUnit::new("test".to_string(), "test.service".to_string());
        let timer2 = TimerUnit::new("test".to_string(), "test2.service".to_string());
        
        assert!(timer_mgr.add_timer(timer1).is_ok());
        assert!(timer_mgr.add_timer(timer2).is_err());
    }

    #[test]
    fn test_sigmatimer_enable_disable() {
        let mut timer_mgr = SigmaTimer::new();
        
        let timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        timer_mgr.add_timer(timer).unwrap();
        
        assert!(timer_mgr.enable_timer("test").is_ok());
        assert!(timer_mgr.disable_timer("test").is_ok());
    }

    #[test]
    fn test_sigmatimer_list_timers() {
        let mut timer_mgr = SigmaTimer::new();
        
        let timer1 = TimerUnit::new("timer1".to_string(), "service1.service".to_string());
        let timer2 = TimerUnit::new("timer2".to_string(), "service2.service".to_string());
        
        timer_mgr.add_timer(timer1).unwrap();
        timer_mgr.add_timer(timer2).unwrap();
        
        let timers = timer_mgr.list_timers();
        assert_eq!(timers.len(), 2);
    }

    #[test]
    fn test_sigmatimer_enabled_timers() {
        let mut timer_mgr = SigmaTimer::new();
        
        let mut timer1 = TimerUnit::new("timer1".to_string(), "service1.service".to_string());
        timer1.disable();
        
        let timer2 = TimerUnit::new("timer2".to_string(), "service2.service".to_string());
        
        timer_mgr.add_timer(timer1).unwrap();
        timer_mgr.add_timer(timer2).unwrap();
        
        let enabled = timer_mgr.list_enabled_timers();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_sigmatimer_stats() {
        let mut timer_mgr = SigmaTimer::new();
        
        let mut timer1 = TimerUnit::new("timer1".to_string(), "service1.service".to_string());
        timer1.disable();
        
        let timer2 = TimerUnit::new("timer2".to_string(), "service2.service".to_string());
        
        timer_mgr.add_timer(timer1).unwrap();
        timer_mgr.add_timer(timer2).unwrap();
        
        let stats = timer_mgr.timer_stats();
        assert_eq!(stats.get("total_timers"), Some(&"2".to_string()));
        assert_eq!(stats.get("enabled_timers"), Some(&"1".to_string()));
        assert_eq!(stats.get("disabled_timers"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_timer_unit_convenience() {
        init_sigmatimer();
        
        let timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        let result = add_timer_unit(timer);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_enable_timer_unit_convenience() {
        init_sigmatimer();
        
        let timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        add_timer_unit(timer).unwrap();
        
        let result = enable_timer_unit("test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_disable_timer_unit_convenience() {
        init_sigmatimer();
        
        let timer = TimerUnit::new("test".to_string(), "test.service".to_string());
        add_timer_unit(timer).unwrap();
        
        let result = disable_timer_unit("test");
        assert!(result.is_ok());
    }
}
