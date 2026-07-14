#![no_std]

/// AI Anomaly Detection Firewall for SigmaOS
/// Implements intelligent firewall with anomaly detection
/// Based on 100-Improvement-Ideas.md #33: AI anomaly detection firewall

use core::sync::atomic::{AtomicU64, Ordering};

/// Protocol types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP = 0,
    UDP = 1,
    ICMP = 2,
    Unknown = 99,
}

/// Action types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow = 0,
    Deny = 1,
    RateLimit = 2,
    LogOnly = 3,
}

/// Traffic direction
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Inbound = 0,
    Outbound = 1,
    Both = 2,
}

/// Network packet
#[repr(C)]
pub struct NetworkPacket {
    pub source_ip: [u8; 16],
    pub dest_ip: [u8; 16],
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: Protocol,
    pub size: u64,
    pub timestamp: u64,
}

impl NetworkPacket {
    pub fn new() -> Self {
        NetworkPacket {
            source_ip: [0u8; 16],
            dest_ip: [0u8; 16],
            source_port: 0,
            dest_port: 0,
            protocol: Protocol::Unknown,
            size: 0,
            timestamp: get_current_time(),
        }
    }
}

/// Firewall rule
#[repr(C)]
pub struct FirewallRule {
    pub id: u64,
    pub direction: Direction,
    pub protocol: Protocol,
    pub source_ip: [u8; 16],
    pub source_mask: u8,
    pub dest_port: u16,
    pub action: FirewallAction,
    pub enabled: bool,
}

impl FirewallRule {
    pub fn new(id: u64, action: FirewallAction) -> Self {
        FirewallRule {
            id,
            direction: Direction::Both,
            protocol: Protocol::Unknown,
            source_ip: [0u8; 16],
            source_mask: 0,
            dest_port: 0,
            action,
            enabled: true,
        }
    }
}

/// Anomaly detection result
#[repr(C)]
pub struct AnomalyResult {
    pub is_anomaly: bool,
    pub confidence: f32,
    pub anomaly_type: AnomalyType,
}

impl AnomalyResult {
    pub fn normal() -> Self {
        AnomalyResult {
            is_anomaly: false,
            confidence: 0.0,
            anomaly_type: AnomalyType::None,
        }
    }
}

/// Anomaly types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnomalyType {
    None = 0,
    PortScan = 1,
    DDoS = 2,
    DataExfiltration = 3,
    UnusualProtocol = 4,
    RateExceeded = 5,
}

/// Traffic statistics
#[repr(C)]
pub struct TrafficStats {
    pub total_packets: AtomicU64,
    pub allowed_packets: AtomicU64,
    pub denied_packets: AtomicU64,
    pub total_bytes: AtomicU64,
}

impl TrafficStats {
    pub fn new() -> Self {
        TrafficStats {
            total_packets: AtomicU64::new(0),
            allowed_packets: AtomicU64::new(0),
            denied_packets: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
        }
    }
    
    pub fn record_packet(&self, allowed: bool, size: u64) {
        self.total_packets.fetch_add(1, Ordering::SeqCst);
        self.total_bytes.fetch_add(size, Ordering::SeqCst);
        if allowed {
            self.allowed_packets.fetch_add(1, Ordering::SeqCst);
        } else {
            self.denied_packets.fetch_add(1, Ordering::SeqCst);
        }
    }
}

/// AI Anomaly Detection Firewall
pub struct AIFirewall {
    rules: Vec<Option<FirewallRule>>,
    next_rule_id: AtomicU64,
    stats: TrafficStats,
    anomaly_threshold: f32,
    learning_mode: bool,
}

impl AIFirewall {
    pub fn new() -> Self {
        AIFirewall {
            rules: Vec::new(),
            next_rule_id: AtomicU64::new(1),
            stats: TrafficStats::new(),
            anomaly_threshold: 0.7,
            learning_mode: false,
        }
    }
    
    pub fn with_threshold(threshold: f32) -> Self {
        AIFirewall {
            rules: Vec::new(),
            next_rule_id: AtomicU64::new(1),
            stats: TrafficStats::new(),
            anomaly_threshold: threshold,
            learning_mode: false,
        }
    }
    
    /// Add firewall rule
    pub fn add_rule(&mut self, rule: FirewallRule) {
        self.rules.push(Some(rule));
    }
    
    /// Remove firewall rule
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
    
    /// Process packet through firewall
    pub fn process_packet(&mut self, packet: &NetworkPacket) -> FirewallAction {
        self.stats.total_packets.fetch_add(1, Ordering::SeqCst);
        self.stats.total_bytes.fetch_add(packet.size, Ordering::SeqCst);
        
        // Check for anomalies
        let anomaly = self.detect_anomaly(packet);
        
        if anomaly.is_anomaly && anomaly.confidence > self.anomaly_threshold {
            self.stats.denied_packets.fetch_add(1, Ordering::SeqCst);
            return FirewallAction::Deny;
        }
        
        // Check rules
        for rule_option in &self.rules {
            if let Some(ref rule) = *rule_option {
                if rule.enabled && self.rule_matches(rule, packet) {
                    let action = rule.action;
                    if action == FirewallAction::Allow {
                        self.stats.allowed_packets.fetch_add(1, Ordering::SeqCst);
                    } else {
                        self.stats.denied_packets.fetch_add(1, Ordering::SeqCst);
                    }
                    return action;
                }
            }
        }
        
        // Default allow if no rules match
        self.stats.allowed_packets.fetch_add(1, Ordering::SeqCst);
        FirewallAction::Allow
    }
    
    fn rule_matches(&self, rule: &FirewallRule, packet: &NetworkPacket) -> bool {
        // Check protocol
        if rule.protocol != Protocol::Unknown && rule.protocol != packet.protocol {
            return false;
        }
        
        // Check port
        if rule.dest_port != 0 && rule.dest_port != packet.dest_port {
            return false;
        }
        
        true
    }
    
    /// AI-based anomaly detection
    fn detect_anomaly(&self, packet: &NetworkPacket) -> AnomalyResult {
        let mut result = AnomalyResult::normal();
        
        // Simple anomaly detection based on packet characteristics
        // In real implementation, this would use ML models
        
        // Check for unusual port (potential port scan)
        if packet.dest_port > 1024 && packet.dest_port < 65535 {
            let random_port_score = 0.3;
            if random_port_score > result.confidence {
                result.confidence = random_port_score;
                result.anomaly_type = AnomalyType::PortScan;
            }
        }
        
        // Check for large packet size (potential data exfiltration)
        if packet.size > 1000000 {
            let large_packet_score = 0.5;
            if large_packet_score > result.confidence {
                result.confidence = large_packet_score;
                result.anomaly_type = AnomalyType::DataExfiltration;
            }
        }
        
        result.is_anomaly = result.confidence > self.anomaly_threshold;
        result
    }
    
    /// Get firewall statistics
    pub fn stats(&self) -> &TrafficStats {
        &self.stats
    }
    
    /// Enable/disable learning mode
    pub fn set_learning_mode(&mut self, enabled: bool) {
        self.learning_mode = enabled;
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
