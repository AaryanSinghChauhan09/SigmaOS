// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaACPI - ACPI Event Handler
//!
//! SigmaACPI provides ACPI event handling for SigmaOS, inspired by acpid and systemd-acpid from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// ACPI event type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AcpiEventType {
    /// Power button pressed
    PowerButton,
    /// Sleep button pressed
    SleepButton,
    /// Lid opened
    LidOpen,
    /// Lid closed
    LidClose,
    /// AC adapter plugged in
    AcAdapterPlugged,
    /// AC adapter unplugged
    AcAdapterUnplugged,
    /// Battery critical
    BatteryCritical,
    /// Battery low
    BatteryLow,
    /// Thermal zone critical
    ThermalCritical,
    /// Thermal zone hot
    ThermalHot,
    /// Dock event
    Dock,
    /// Undock event
    Undock,
    /// Unknown event
    Unknown,
}

impl fmt::Display for AcpiEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcpiEventType::PowerButton => write!(f, "power_button"),
            AcpiEventType::SleepButton => write!(f, "sleep_button"),
            AcpiEventType::LidOpen => write!(f, "lid_open"),
            AcpiEventType::LidClose => write!(f, "lid_close"),
            AcpiEventType::AcAdapterPlugged => write!(f, "ac_adapter_plugged"),
            AcpiEventType::AcAdapterUnplugged => write!(f, "ac_adapter_unplugged"),
            AcpiEventType::BatteryCritical => write!(f, "battery_critical"),
            AcpiEventType::BatteryLow => write!(f, "battery_low"),
            AcpiEventType::ThermalCritical => write!(f, "thermal_critical"),
            AcpiEventType::ThermalHot => write!(f, "thermal_hot"),
            AcpiEventType::Dock => write!(f, "dock"),
            AcpiEventType::Undock => write!(f, "undock"),
            AcpiEventType::Unknown => write!(f, "unknown"),
        }
    }
}

/// ACPI event
#[derive(Debug, Clone)]
pub struct AcpiEvent {
    /// Event ID
    pub event_id: String,
    /// Event type
    pub event_type: AcpiEventType,
    /// Event data
    pub data: String,
    /// Timestamp
    pub timestamp: u64,
}

impl AcpiEvent {
    /// Create a new ACPI event
    pub fn new(event_id: String, event_type: AcpiEventType, data: String, timestamp: u64) -> Self {
        Self {
            event_id,
            event_type,
            data,
            timestamp,
        }
    }
}

/// ACPI action type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiActionType {
    /// Shutdown action
    Shutdown,
    /// Suspend action
    Suspend,
    /// Hibernate action
    Hibernate,
    /// Screen brightness
    ScreenBrightness,
    /// Execute command
    ExecuteCommand,
    /// Ignore action
    Ignore,
}

impl fmt::Display for AcpiActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcpiActionType::Shutdown => write!(f, "shutdown"),
            AcpiActionType::Suspend => write!(f, "suspend"),
            AcpiActionType::Hibernate => write!(f, "hibernate"),
            AcpiActionType::ScreenBrightness => write!(f, "screen_brightness"),
            AcpiActionType::ExecuteCommand => write!(f, "execute_command"),
            AcpiActionType::Ignore => write!(f, "ignore"),
        }
    }
}

/// ACPI action
#[derive(Debug, Clone)]
pub struct AcpiAction {
    /// Action ID
    pub action_id: String,
    /// Event type
    pub event_type: AcpiEventType,
    /// Action type
    pub action_type: AcpiActionType,
    /// Action parameter (e.g., command, brightness value)
    pub parameter: String,
}

impl AcpiAction {
    /// Create a new ACPI action
    pub fn new(action_id: String, event_type: AcpiEventType, action_type: AcpiActionType) -> Self {
        Self {
            action_id,
            event_type,
            action_type,
            parameter: String::new(),
        }
    }

    /// Set parameter
    pub fn with_parameter(mut self, param: String) -> Self {
        self.parameter = param;
        self
    }
}

/// SigmaACPI - ACPI Event Handler
pub struct SigmaACPI {
    /// Event queue
    event_queue: Vec<AcpiEvent>,
    /// Actions (event_type -> action)
    actions: BTreeMap<AcpiEventType, AcpiAction>,
    /// Running state
    running: bool,
    /// Event counter
    event_counter: u32,
    /// Action execution counter
    action_counter: u32,
}

impl SigmaACPI {
    /// Create a new SigmaACPI instance
    pub fn new() -> Self {
        Self {
            event_queue: Vec::new(),
            actions: BTreeMap::new(),
            running: false,
            event_counter: 0,
            action_counter: 0,
        }
    }

    /// Start the ACPI daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the ACPI daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Register an action for an event type
    pub fn register_action(&mut self, action: AcpiAction) -> Result<(), String> {
        self.actions.insert(action.event_type, action);
        Ok(())
    }

    /// Unregister an action for an event type
    pub fn unregister_action(&mut self, event_type: AcpiEventType) -> Result<(), String> {
        if self.actions.remove(&event_type).is_none() {
            return Err(format!("No action registered for event type: {}", event_type));
        }
        Ok(())
    }

    /// Get action for an event type
    pub fn get_action(&self, event_type: AcpiEventType) -> Option<&AcpiAction> {
        self.actions.get(&event_type)
    }

    /// List all registered actions
    pub fn list_actions(&self) -> Vec<&AcpiAction> {
        self.actions.values().collect()
    }

    /// Add an event to the queue
    pub fn add_event(&mut self, event: AcpiEvent) -> Result<(), String> {
        if !self.running {
            return Err("SigmaACPI not running".to_string());
        }
        
        self.event_queue.push(event);
        self.event_counter += 1;
        Ok(())
    }

    /// Process the event queue
    pub fn process_events(&mut self) -> Vec<String> {
        let mut results = Vec::new();
        
        if !self.running {
            return results;
        }

        for event in self.event_queue.drain(..) {
            if let Some(action) = self.actions.get(&event.event_type) {
                self.action_counter += 1;
                
                match action.action_type {
                    AcpiActionType::Shutdown => {
                        results.push(format!("Shutdown triggered by {}", event.event_type));
                    }
                    AcpiActionType::Suspend => {
                        results.push(format!("Suspend triggered by {}", event.event_type));
                    }
                    AcpiActionType::Hibernate => {
                        results.push(format!("Hibernate triggered by {}", event.event_type));
                    }
                    AcpiActionType::ScreenBrightness => {
                        results.push(format!("Screen brightness set to {} by {}", action.parameter, event.event_type));
                    }
                    AcpiActionType::ExecuteCommand => {
                        results.push(format!("Execute command: {} by {}", action.parameter, event.event_type));
                    }
                    AcpiActionType::Ignore => {
                        results.push(format!("Ignored event: {}", event.event_type));
                    }
                }
            } else {
                results.push(format!("No action registered for event: {}", event.event_type));
            }
        }

        results
    }

    /// Get event queue size
    pub fn event_queue_size(&self) -> usize {
        self.event_queue.len()
    }

    /// Get action count
    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    /// Get ACPI statistics
    pub fn acpi_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("actions_registered".to_string(), self.action_count().to_string());
        stats.insert("events_queued".to_string(), self.event_queue_size().to_string());
        stats.insert("events_processed".to_string(), self.event_counter.to_string());
        stats.insert("actions_executed".to_string(), self.action_counter.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaACPI {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaACPI instance (placeholder for actual global state)
static mut SIGMAACPI_INSTANCE: Option<SigmaACPI> = None;

/// Initialize the global SigmaACPI instance
pub fn init_sigmaacpi() {
    unsafe {
        SIGMAACPI_INSTANCE = Some(SigmaACPI::new());
    }
}

/// Get the global SigmaACPI instance
pub fn get_sigmaacpi() -> Option<&'static mut SigmaACPI> {
    unsafe { SIGMAACPI_INSTANCE.as_mut() }
}

/// Convenience function to add an event
pub fn add_acpi_event(event: AcpiEvent) -> Result<(), String> {
    if let Some(acpi) = get_sigmaacpi() {
        acpi.add_event(event)
    } else {
        Err("SigmaACPI not initialized".to_string())
    }
}

/// Convenience function to register an action
pub fn register_acpi_action(action: AcpiAction) -> Result<(), String> {
    if let Some(acpi) = get_sigmaacpi() {
        acpi.register_action(action)
    } else {
        Err("SigmaACPI not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_event_type_display() {
        assert_eq!(AcpiEventType::PowerButton.to_string(), "power_button");
        assert_eq!(AcpiEventType::LidClose.to_string(), "lid_close");
    }

    #[test]
    fn test_acpi_action_type_display() {
        assert_eq!(AcpiActionType::Shutdown.to_string(), "shutdown");
        assert_eq!(AcpiActionType::Suspend.to_string(), "suspend");
    }

    #[test]
    fn test_acpi_event() {
        let event = AcpiEvent::new(
            "event1".to_string(),
            AcpiEventType::PowerButton,
            "pressed".to_string(),
            1000
        );

        assert_eq!(event.event_id, "event1");
        assert_eq!(event.event_type, AcpiEventType::PowerButton);
        assert_eq!(event.timestamp, 1000);
    }

    #[test]
    fn test_acpi_action() {
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        )
        .with_parameter("now".to_string());

        assert_eq!(action.action_id, "action1");
        assert_eq!(action.action_type, AcpiActionType::Shutdown);
        assert_eq!(action.parameter, "now");
    }

    #[test]
    fn test_sigmaacpi() {
        let mut acpi = SigmaACPI::new();
        assert!(!acpi.is_running());
        acpi.start();
        assert!(acpi.is_running());
        acpi.stop();
        assert!(!acpi.is_running());
    }

    #[test]
    fn test_sigmaacpi_register_action() {
        let mut acpi = SigmaACPI::new();
        
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        );
        
        assert!(acpi.register_action(action).is_ok());
        assert_eq!(acpi.action_count(), 1);
    }

    #[test]
    fn test_sigmaacpi_unregister_action() {
        let mut acpi = SigmaACPI::new();
        
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        );
        
        acpi.register_action(action).unwrap();
        assert!(acpi.unregister_action(AcpiEventType::PowerButton).is_ok());
        assert_eq!(acpi.action_count(), 0);
    }

    #[test]
    fn test_sigmaacpi_add_event() {
        let mut acpi = SigmaACPI::new();
        acpi.start();
        
        let event = AcpiEvent::new(
            "event1".to_string(),
            AcpiEventType::PowerButton,
            "pressed".to_string(),
            1000
        );
        
        assert!(acpi.add_event(event).is_ok());
        assert_eq!(acpi.event_queue_size(), 1);
    }

    #[test]
    fn test_sigmaacpi_process_events() {
        let mut acpi = SigmaACPI::new();
        acpi.start();
        
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        );
        
        acpi.register_action(action).unwrap();
        
        let event = AcpiEvent::new(
            "event1".to_string(),
            AcpiEventType::PowerButton,
            "pressed".to_string(),
            1000
        );
        
        acpi.add_event(event).unwrap();
        
        let results = acpi.process_events();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_sigmaacpi_stats() {
        let mut acpi = SigmaACPI::new();
        acpi.start();
        
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        );
        
        acpi.register_action(action).unwrap();
        
        let stats = acpi.acpi_stats();
        assert_eq!(stats.get("actions_registered"), Some(&"1".to_string()));
        assert_eq!(stats.get("running"), Some(&"true".to_string()));
    }

    #[test]
    fn test_add_acpi_event_convenience() {
        init_sigmaacpi();
        
        let mut acpi = get_sigmaacpi().unwrap();
        acpi.start();
        
        let event = AcpiEvent::new(
            "event1".to_string(),
            AcpiEventType::PowerButton,
            "pressed".to_string(),
            1000
        );
        
        let result = add_acpi_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_acpi_action_convenience() {
        init_sigmaacpi();
        
        let action = AcpiAction::new(
            "action1".to_string(),
            AcpiEventType::PowerButton,
            AcpiActionType::Shutdown
        );
        
        let result = register_acpi_action(action);
        assert!(result.is_ok());
    }
}
