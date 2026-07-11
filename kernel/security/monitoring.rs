// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/monitoring.rs — Security Monitoring System
//
// Implements security monitoring for SigmaOS kernel
// Tracks security events, anomalies, and potential threats
// Inspired by Linux audit subsystem and SELinux AVC
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const MONITORING_OK: I32 = 0;
pub const MONITORING_ERR_FULL: I32 = -1;
pub const MONITORING_ERR_INVALID: I32 = -2;

const MAX_EVENTS: usize = 1000;
const EVENT_MESSAGE_LEN: usize = 256;
const EVENT_SOURCE_LEN: usize = 64;

// ─── Security Event Types ─────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SecurityEventType {
    // Access control events
    MacDeny,
    SeccompViolation,
    CapabilityCheck,
    
    // Memory events
    AslrViolation,
    StackProtectorViolation,
    HeapCorruption,
    
    // Network events
    SuspiciousConnection,
    PortScan,
    DdosAttack,
    
    // Filesystem events
    UnauthorizedAccess,
    IntegrityViolation,
    SuidModification,
    
    // Process events
    PrivilegeEscalation,
    RootkitDetected,
    AbnormalBehavior,
    
    // System events
    TpmFailure,
    SecureBootViolation,
    KernelModuleLoad,
    
    // Unknown
    Unknown,
}

// ─── Security Event Severity ───────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

// ─── Security Event ───────────────────────────────────────────────────────

#[repr(C)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub severity: EventSeverity,
    pub timestamp: U64,
    pub pid: U32,
    pub uid: U32,
    pub gid: U32,
    pub source: [U8; EVENT_SOURCE_LEN],
    pub message: [U8; EVENT_MESSAGE_LEN],
    pub processed: bool,
}

impl SecurityEvent {
    pub const fn empty() -> Self {
        Self {
            event_type: SecurityEventType::Unknown,
            severity: EventSeverity::Info,
            timestamp: 0,
            pid: 0,
            uid: 0,
            gid: 0,
            source: [0; EVENT_SOURCE_LEN],
            message: [0; EVENT_MESSAGE_LEN],
            processed: false,
        }
    }
}

// ─── Security Statistics ───────────────────────────────────────────────────

#[repr(C)]
pub struct SecurityStats {
    pub total_events: U64,
    pub mac_denies: U64,
    pub seccomp_violations: U64,
    pub privilege_escalations: U64,
    pub integrity_violations: U64,
    pub critical_events: U64,
    pub events_last_hour: U64,
}

impl SecurityStats {
    pub const fn new() -> Self {
        Self {
            total_events: 0,
            mac_denies: 0,
            seccomp_violations: 0,
            privilege_escalations: 0,
            integrity_violations: 0,
            critical_events: 0,
            events_last_hour: 0,
        }
    }
}

// ─── Security Monitor ───────────────────────────────────────────────────────

pub struct SecurityMonitor {
    pub events: [SecurityEvent; MAX_EVENTS],
    pub event_count: usize,
    pub event_index: usize,
    pub stats: SecurityStats,
    pub enabled: bool,
    pub alert_threshold: U32,
}

impl SecurityMonitor {
    pub const fn new() -> Self {
        Self {
            events: [SecurityEvent::empty(); MAX_EVENTS],
            event_count: 0,
            event_index: 0,
            stats: SecurityStats::new(),
            enabled: true,
            alert_threshold: 10,
        }
    }

    /// Initialize security monitor
    pub unsafe fn init(&mut self) -> I32 {
        self.enabled = true;
        MONITORING_OK
    }

    /// Log security event
    pub unsafe fn log_event(&mut self, event_type: SecurityEventType, severity: EventSeverity, 
                           pid: U32, uid: U32, source: &[U8], message: &[U8]) -> I32 {
        if !self.enabled {
            return MONITORING_OK;
        }

        // Circular buffer
        let index = self.event_index;
        let event = &mut self.events[index];

        event.event_type = event_type;
        event.severity = severity;
        event.timestamp = self.get_timestamp();
        event.pid = pid;
        event.uid = uid;
        event.gid = 0;
        event.processed = false;

        // Copy source
        let source_len = source.len().min(EVENT_SOURCE_LEN);
        for i in 0..source_len {
            event.source[i] = source[i];
        }

        // Copy message
        let msg_len = message.len().min(EVENT_MESSAGE_LEN);
        for i in 0..msg_len {
            event.message[i] = message[i];
        }

        // Update index
        self.event_index = (self.event_index + 1) % MAX_EVENTS;
        if self.event_count < MAX_EVENTS {
            self.event_count += 1;
        }

        // Update statistics
        self.update_stats(event_type, severity);

        // Check for alert threshold
        if severity == EventSeverity::Critical {
            self.trigger_alert(event);
        }

        MONITORING_OK
    }

    /// Get recent events
    pub fn get_recent_events(&self, count: usize) -> &[SecurityEvent] {
        let start = if self.event_count >= count {
            self.event_index - count
        } else {
            0
        };
        
        &self.events[start..self.event_index]
    }

    /// Get statistics
    pub fn get_stats(&self) -> &SecurityStats {
        &self.stats
    }

    /// Enable/disable monitoring
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set alert threshold
    pub fn set_alert_threshold(&mut self, threshold: U32) {
        self.alert_threshold = threshold;
    }

    /// Clear old events
    pub unsafe fn clear_old_events(&mut self, older_than: U64) {
        let now = self.get_timestamp();
        
        for i in 0..self.event_count {
            if now - self.events[i].timestamp > older_than {
                self.events[i] = SecurityEvent::empty();
            }
        }
    }

    /// Update statistics
    fn update_stats(&mut self, event_type: SecurityEventType, severity: EventSeverity) {
        self.stats.total_events += 1;

        match event_type {
            SecurityEventType::MacDeny => self.stats.mac_denies += 1,
            SecurityEventType::SeccompViolation => self.stats.seccomp_violations += 1,
            SecurityEventType::PrivilegeEscalation => self.stats.privilege_escalations += 1,
            SecurityEventType::IntegrityViolation => self.stats.integrity_violations += 1,
            _ => {}
        }

        if severity == EventSeverity::Critical {
            self.stats.critical_events += 1;
        }
    }

    /// Trigger security alert
    fn trigger_alert(&self, event: &SecurityEvent) {
        // In real implementation, would send alert to userspace
        // For now, stub
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> U64 {
        // In real implementation, get from RTC
        0
    }
}

// ─── Global Security Monitor ─────────────────────────────────────────────────

static mut SECURITY_MONITOR: SecurityMonitor = SecurityMonitor::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_security_monitor_init() -> I32 {
    SECURITY_MONITOR.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_log_event(event_type: U32, severity: U32, 
                                                 pid: U32, uid: U32, 
                                                 source: *const U8, source_len: U32,
                                                 message: *const U8, message_len: U32) -> I32 {
    let event_type = match event_type {
        0 => SecurityEventType::MacDeny,
        1 => SecurityEventType::SeccompViolation,
        2 => SecurityEventType::CapabilityCheck,
        3 => SecurityEventType::AslrViolation,
        4 => SecurityEventType::StackProtectorViolation,
        5 => SecurityEventType::HeapCorruption,
        6 => SecurityEventType::SuspiciousConnection,
        7 => SecurityEventType::PortScan,
        8 => SecurityEventType::DdosAttack,
        9 => SecurityEventType::UnauthorizedAccess,
        10 => SecurityEventType::IntegrityViolation,
        11 => SecurityEventType::SuidModification,
        12 => SecurityEventType::PrivilegeEscalation,
        13 => SecurityEventType::RootkitDetected,
        14 => SecurityEventType::AbnormalBehavior,
        15 => SecurityEventType::TpmFailure,
        16 => SecurityEventType::SecureBootViolation,
        17 => SecurityEventType::KernelModuleLoad,
        _ => SecurityEventType::Unknown,
    };

    let severity = match severity {
        0 => EventSeverity::Info,
        1 => EventSeverity::Warning,
        2 => EventSeverity::Error,
        3 => EventSeverity::Critical,
        _ => EventSeverity::Info,
    };

    let source_slice = core::slice::from_raw_parts(source, source_len as usize);
    let message_slice = core::slice::from_raw_parts(message, message_len as usize);

    SECURITY_MONITOR.log_event(event_type, severity, pid, uid, source_slice, message_slice)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_get_event_count() -> U32 {
    SECURITY_MONITOR.event_count as U32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_get_critical_count() -> U64 {
    SECURITY_MONITOR.stats.critical_events
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_set_enabled(enabled: bool) {
    SECURITY_MONITOR.set_enabled(enabled);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_security_set_alert_threshold(threshold: U32) {
    SECURITY_MONITOR.set_alert_threshold(threshold);
}
