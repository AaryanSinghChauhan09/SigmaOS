#![no_std]

/// Intrusion Detection System for SigmaOS
/// Based on 100-Improvement-Ideas.md #39: Intrusion detection system
/// Implements real-time intrusion detection and alerting

use core::sync::atomic::{AtomicU64, Ordering};

/// Alert severity
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

/// Alert type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertType {
    UnauthorizedAccess = 0,
    SuspiciousActivity = 1,
    MalwareDetected = 2,
    DataExfiltration = 3,
    PrivilegeEscalation = 4,
    ConfigurationChange = 5,
}

/// Security alert
#[repr(C)]
pub struct SecurityAlert {
    pub id: u64,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub timestamp: u64,
    pub source_ip: [u8; 16],
    pub description: [u8; 256],
    pub resolved: bool,
}

impl SecurityAlert {
    pub fn new(id: u64, alert_type: AlertType, severity: AlertSeverity, description: &str) -> Self {
        let mut desc_array = [0u8; 256];
        let desc_bytes = description.as_bytes();
        let len = desc_bytes.len().min(255);
        
        unsafe {
            core::ptr::copy_nonoverlapping(desc_bytes.as_ptr(), desc_array.as_mut_ptr(), len);
        }
        
        SecurityAlert {
            id,
            alert_type,
            severity,
            timestamp: get_current_time(),
            source_ip: [0u8; 16],
            description: desc_array,
            resolved: false,
        }
    }
}

/// Detection rule
#[repr(C)]
pub struct DetectionRule {
    pub id: u64,
    pub name: [u8; 64],
    pub pattern: [u8; 128],
    pub enabled: bool,
}

impl DetectionRule {
    pub fn new(id: u64, name: &str, pattern: &str) -> Self {
        let mut name_array = [0u8; 64];
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len().min(63);
        
        unsafe {
            core::ptr::copy_nonoverlapping(name_bytes.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        
        let mut pattern_array = [0u8; 128];
        let pattern_bytes = pattern.as_bytes();
        let pattern_len = pattern_bytes.len().min(127);
        
        unsafe {
            core::ptr::copy_nonoverlapping(pattern_bytes.as_ptr(), pattern_array.as_mut_ptr(), pattern_len);
        }
        
        DetectionRule {
            id,
            name: name_array,
            pattern: pattern_array,
            enabled: true,
        }
    }
}

/// Intrusion detection system
pub struct IntrusionDetectionSystem {
    pub alerts: Vec<Option<SecurityAlert>>,
    pub rules: Vec<Option<DetectionRule>>,
    pub next_alert_id: AtomicU64,
    pub next_rule_id: AtomicU64,
    pub monitoring_enabled: bool,
    pub auto_block_enabled: bool,
}

impl IntrusionDetectionSystem {
    pub fn new() -> Self {
        IntrusionDetectionSystem {
            alerts: Vec::new(),
            rules: Vec::new(),
            next_alert_id: AtomicU64::new(1),
            next_rule_id: AtomicU64::new(1),
            monitoring_enabled: true,
            auto_block_enabled: false,
        }
    }
    
    /// Add detection rule
    pub fn add_rule(&mut self, name: &str, pattern: &str) -> u64 {
        let id = self.next_rule_id.fetch_add(1, Ordering::SeqCst);
        let rule = DetectionRule::new(id, name, pattern);
        self.rules.push(Some(rule));
        id
    }
    
    /// Remove rule
    pub fn remove_rule(&mut self, id: u64) -> bool {
        for rule_option in &mut self.rules {
            if let Some(ref rule) = *rule_option {
                if rule.id == id {
                    *rule_option = None;
                    return true;
                }
            }
        }
        false
    }
    
    /// Enable/disable rule
    pub fn set_rule_enabled(&mut self, id: u64, enabled: bool) -> bool {
        for rule_option in &mut self.rules {
            if let Some(ref mut rule) = *rule_option {
                if rule.id == id {
                    rule.enabled = enabled;
                    return true;
                }
            }
        }
        false
    }
    
    /// Analyze event
    pub fn analyze_event(&mut self, event_data: &[u8]) -> Option<SecurityAlert> {
        if !self.monitoring_enabled {
            return None;
        }
        
        for rule_option in &self.rules {
            if let Some(ref rule) = *rule_option {
                if !rule.enabled {
                    continue;
                }
                
                // Simple pattern matching
                if self.matches_pattern(event_data, &rule.pattern) {
                    let id = self.next_alert_id.fetch_add(1, Ordering::SeqCst);
                    let alert = SecurityAlert::new(
                        id,
                        AlertType::SuspiciousActivity,
                        AlertSeverity::Medium,
                        "Pattern match detected"
                    );
                    self.alerts.push(Some(alert.clone()));
                    return Some(alert);
                }
            }
        }
        
        None
    }
    
    fn matches_pattern(&self, data: &[u8], pattern: &[u8]) -> bool {
        if data.len() < pattern.len() {
            return false;
        }
        
        for i in 0..=data.len() - pattern.len() {
            let mut match_found = true;
            for j in 0..pattern.len() {
                if pattern[j] != 0 && data[i + j] != pattern[j] {
                    match_found = false;
                    break;
                }
            }
            if match_found {
                return true;
            }
        }
        
        false
    }
    
    /// Create manual alert
    pub fn create_alert(&mut self, alert_type: AlertType, severity: AlertSeverity, description: &str) -> u64 {
        let id = self.next_alert_id.fetch_add(1, Ordering::SeqCst);
        let alert = SecurityAlert::new(id, alert_type, severity, description);
        self.alerts.push(Some(alert));
        id
    }
    
    /// Resolve alert
    pub fn resolve_alert(&mut self, id: u64) -> bool {
        for alert_option in &mut self.alerts {
            if let Some(ref mut alert) = *alert_option {
                if alert.id == id {
                    alert.resolved = true;
                    return true;
                }
            }
        }
        false
    }
    
    /// Get unresolved alerts
    pub fn get_unresolved_alerts(&self) -> Vec<&SecurityAlert> {
        let mut alerts = Vec::new();
        for alert_option in &self.alerts {
            if let Some(ref alert) = *alert_option {
                if !alert.resolved {
                    alerts.push(alert);
                }
            }
        }
        alerts
    }
    
    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<&SecurityAlert> {
        let mut alerts = Vec::new();
        for alert_option in &self.alerts {
            if let Some(ref alert) = *alert_option {
                if alert.severity == severity {
                    alerts.push(alert);
                }
            }
        }
        alerts
    }
    
    /// Enable/disable monitoring
    pub fn set_monitoring_enabled(&mut self, enabled: bool) {
        self.monitoring_enabled = enabled;
    }
    
    /// Enable/disable auto-block
    pub fn set_auto_block_enabled(&mut self, enabled: bool) {
        self.auto_block_enabled = enabled;
    }
    
    /// Initialize default rules
    pub fn initialize_defaults(&mut self) {
        self.add_rule("SSH Brute Force", "ssh");
        self.add_rule("SQL Injection", "sql");
        self.add_rule("XSS Attack", "script");
        self.add_rule("Port Scan", "scan");
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Get current time (nanoseconds)
fn get_current_time() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1_000_000, Ordering::SeqCst)
}
