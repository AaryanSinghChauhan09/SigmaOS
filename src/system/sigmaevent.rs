// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaEvent - System Event Manager
//!
//! SigmaEvent provides system event management for SigmaOS, inspired by udev/dbus from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// Device event
    Device,
    /// Filesystem event
    Filesystem,
    /// Network event
    Network,
    /// System event
    System,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Device => write!(f, "device"),
            EventType::Filesystem => write!(f, "filesystem"),
            EventType::Network => write!(f, "network"),
            EventType::System => write!(f, "system"),
        }
    }
}

/// Event action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventAction {
    /// Add
    Add,
    /// Remove
    Remove,
    /// Change
    Change,
}

impl fmt::Display for EventAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventAction::Add => write!(f, "add"),
            EventAction::Remove => write!(f, "remove"),
            EventAction::Change => write!(f, "change"),
        }
    }
}

/// System event
#[derive(Debug, Clone)]
pub struct SystemEvent {
    /// Event ID
    pub event_id: String,
    /// Event type
    pub event_type: EventType,
    /// Event action
    pub action: EventAction,
    /// Source
    pub source: String,
    /// Data
    pub data: String,
    /// Timestamp
    pub timestamp: u64,
}

impl SystemEvent {
    /// Create a new system event
    pub fn new(
        event_id: String,
        event_type: EventType,
        action: EventAction,
        source: String,
        data: String,
    ) -> Self {
        Self {
            event_id,
            event_type,
            action,
            source,
            data,
            timestamp: 0,
        }
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
}

/// SigmaEvent - System Event Manager
pub struct SigmaEvent {
    /// System events (event_id -> event)
    events: BTreeMap<String, SystemEvent>,
    /// Running state
    running: bool,
    /// Event counter
    event_counter: u32,
}

impl SigmaEvent {
    /// Create a new SigmaEvent instance
    pub fn new() -> Self {
        Self {
            events: BTreeMap::new(),
            running: false,
            event_counter: 0,
        }
    }

    /// Start the event manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the event manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Emit a system event
    pub fn emit_event(&mut self, event: SystemEvent) -> Result<(), String> {
        if !self.running {
            return Err("SigmaEvent not running".to_string());
        }

        if self.events.contains_key(&event.event_id) {
            return Err(format!("Event '{}' already exists", event.event_id));
        }

        self.events.insert(event.event_id.clone(), event);
        self.event_counter += 1;
        Ok(())
    }

    /// Remove a system event
    pub fn remove_event(&mut self, event_id: &str) -> Result<(), String> {
        if self.events.remove(event_id).is_none() {
            return Err(format!("Event '{}' not found", event_id));
        }
        Ok(())
    }

    /// Get a system event
    pub fn get_event(&self, event_id: &str) -> Option<&SystemEvent> {
        self.events.get(event_id)
    }

    /// List all events
    pub fn list_events(&self) -> Vec<&SystemEvent> {
        self.events.values().collect()
    }

    /// List events by type
    pub fn list_by_type(&self, event_type: EventType) -> Vec<&SystemEvent> {
        self.events
            .values()
            .filter(|e| e.event_type == event_type)
            .collect()
    }

    /// List events by action
    pub fn list_by_action(&self, action: EventAction) -> Vec<&SystemEvent> {
        self.events
            .values()
            .filter(|e| e.action == action)
            .collect()
    }

    /// Get event count
    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    /// Get event manager statistics
    pub fn event_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_events".to_string(), self.event_count().to_string());
        stats.insert(
            "device_events".to_string(),
            self.list_by_type(EventType::Device).len().to_string(),
        );
        stats.insert(
            "network_events".to_string(),
            self.list_by_type(EventType::Network).len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaEvent {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaEvent instance (placeholder for actual global state)
static mut SIGMAEVENT_INSTANCE: Option<SigmaEvent> = None;

/// Initialize the global SigmaEvent instance
pub fn init_sigmaevent() {
    unsafe {
        SIGMAEVENT_INSTANCE = Some(SigmaEvent::new());
    }
}

/// Get the global SigmaEvent instance
pub fn get_sigmaevent() -> Option<&'static mut SigmaEvent> {
    unsafe { SIGMAEVENT_INSTANCE.as_mut() }
}

/// Convenience function to emit a system event
pub fn emit_system_event(event: SystemEvent) -> Result<(), String> {
    if let Some(event_mgr) = get_sigmaevent() {
        event_mgr.emit_event(event)
    } else {
        Err("SigmaEvent not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type_display() {
        assert_eq!(EventType::Device.to_string(), "device");
        assert_eq!(EventType::Network.to_string(), "network");
    }

    #[test]
    fn test_event_action_display() {
        assert_eq!(EventAction::Add.to_string(), "add");
        assert_eq!(EventAction::Remove.to_string(), "remove");
    }

    #[test]
    fn test_system_event() {
        let event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        )
        .with_timestamp(1234567890);

        assert_eq!(event.event_id, "evt1");
        assert_eq!(event.timestamp, 1234567890);
    }

    #[test]
    fn test_sigmaevent() {
        let mut event = SigmaEvent::new();
        assert!(!event.is_running());
        event.start();
        assert!(event.is_running());
        event.stop();
        assert!(!event.is_running());
    }

    #[test]
    fn test_sigmaevent_emit_event() {
        let mut event = SigmaEvent::new();
        event.start();

        let sys_event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        );

        assert!(event.emit_event(sys_event).is_ok());
        assert_eq!(event.event_count(), 1);
    }

    #[test]
    fn test_sigmaevent_list_by_type() {
        let mut event = SigmaEvent::new();
        event.start();

        let sys_event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        );

        event.emit_event(sys_event).unwrap();

        let events = event.list_by_type(EventType::Device);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_sigmaevent_list_by_action() {
        let mut event = SigmaEvent::new();
        event.start();

        let sys_event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        );

        event.emit_event(sys_event).unwrap();

        let events = event.list_by_action(EventAction::Add);
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_sigmaevent_stats() {
        let mut event = SigmaEvent::new();
        event.start();

        let sys_event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        );

        event.emit_event(sys_event).unwrap();

        let stats = event.event_stats();
        assert_eq!(stats.get("total_events"), Some(&"1".to_string()));
        assert_eq!(stats.get("device_events"), Some(&"1".to_string()));
    }

    #[test]
    fn test_emit_system_event_convenience() {
        init_sigmaevent();

        let mut event = get_sigmaevent().unwrap();
        event.start();

        let sys_event = SystemEvent::new(
            "evt1".to_string(),
            EventType::Device,
            EventAction::Add,
            "udev".to_string(),
            "Device added".to_string(),
        );

        let result = emit_system_event(sys_event);
        assert!(result.is_ok());
    }
}
