#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(not(target_os = "none"))]
use std_alloc::boxed::Box;

#![no_std]
#![no_main]

/// OOP-based Firewall & AI Intrusion Detection System (IDS) for SigmaOS
/// Implements standard packet filtering, Snort-style signature checking,
/// and CrowdStrike Falcon-inspired AI anomaly rate monitoring.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RuleID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction { Accept = 0, Drop = 1, Reject = 2, Log = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol { TCP = 6, UDP = 17, ICMP = 1, Any = 255 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FirewallError { Success = 0, InvalidRule = 1, NotFound = 2 }

pub trait FirewallRule {
    fn id(&self) -> RuleID;
    fn action(&self) -> RuleAction;
    fn protocol(&self) -> Protocol;
    fn source_ip(&self) -> &[u8];
    fn destination_ip(&self) -> &[u8];
    fn source_port(&self) -> u16;
    fn destination_port(&self) -> u16;
}

#[repr(C)]
pub struct SimpleFirewallRule {
    pub id: RuleID,
    pub action: AtomicUsize,
    pub protocol: AtomicUsize,
    pub source_ip: [u8; 4],
    pub destination_ip: [u8; 4],
    pub source_port: AtomicUsize,
    pub destination_port: AtomicUsize,
}

impl SimpleFirewallRule {
    pub fn new(id: RuleID, action: RuleAction, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> Self {
        let mut src_ip = [0u8; 4];
        let mut dst_ip = [0u8; 4];
        let src_len = source_ip.len().min(4);
        let dst_len = destination_ip.len().min(4);
        for i in 0..src_len { src_ip[i] = source_ip[i]; }
        for i in 0..dst_len { dst_ip[i] = destination_ip[i]; }
        SimpleFirewallRule {
            id,
            action: AtomicUsize::new(action as usize),
            protocol: AtomicUsize::new(protocol as usize),
            source_ip: src_ip,
            destination_ip: dst_ip,
            source_port: AtomicUsize::new(source_port as usize),
            destination_port: AtomicUsize::new(destination_port as usize),
        }
    }
}

impl FirewallRule for SimpleFirewallRule {
    fn id(&self) -> RuleID { self.id }
    fn action(&self) -> RuleAction { unsafe { core::mem::transmute(self.action.load(Ordering::SeqCst)) } }
    fn protocol(&self) -> Protocol { unsafe { core::mem::transmute(self.protocol.load(Ordering::SeqCst)) } }
    fn source_ip(&self) -> &[u8] { &self.source_ip }
    fn destination_ip(&self) -> &[u8] { &self.destination_ip }
    fn source_port(&self) -> u16 { self.source_port.load(Ordering::SeqCst) as u16 }
    fn destination_port(&self) -> u16 { self.destination_port.load(Ordering::SeqCst) as u16 }
}

pub trait Firewall {
    fn add_rule(&mut self, rule: Box<dyn FirewallRule>) -> Result<RuleID, FirewallError>;
    fn remove_rule(&mut self, id: RuleID) -> Result<(), FirewallError>;
    fn get_rule(&self, id: RuleID) -> Option<&dyn FirewallRule>;
    fn filter_packet(&self, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> RuleAction;
}

#[repr(C)]
pub struct SimpleFirewall {
    pub rules: Vec<Option<Box<dyn FirewallRule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFirewall {
    pub fn new() -> Self {
        SimpleFirewall {
            rules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Firewall for SimpleFirewall {
    fn add_rule(&mut self, rule: Box<dyn FirewallRule>) -> Result<RuleID, FirewallError> {
        let id = rule.id();
        self.rules.push(Some(rule));
        Ok(id)
    }

    fn remove_rule(&mut self, id: RuleID) -> Result<(), FirewallError> {
        for rule_option in &mut self.rules {
            if let Some(ref rule) = *rule_option {
                if rule.id() == id {
                    return Ok(());
                }
            }
        }
        Err(FirewallError::NotFound)
    }

    fn get_rule(&self, id: RuleID) -> Option<&dyn FirewallRule> {
        for rule_option in &self.rules {
            if let Some(ref rule) = *rule_option {
                if rule.id() == id { return Some(rule.as_ref()); }
            }
        }
        None
    }

    fn filter_packet(&self, protocol: Protocol, source_ip: &[u8], destination_ip: &[u8], source_port: u16, destination_port: u16) -> RuleAction {
        for rule_option in &self.rules {
            if let Some(ref rule) = *rule_option {
                if rule.protocol() == Protocol::Any || rule.protocol() == protocol {
                    if rule.source_ip() == source_ip || rule.source_ip() == &[0, 0, 0, 0] {
                        if rule.destination_ip() == destination_ip || rule.destination_ip() == &[0, 0, 0, 0] {
                            if rule.source_port() == source_port || rule.source_port() == 0 {
                                if rule.destination_port() == destination_port || rule.destination_port() == 0 {
                                    return rule.action();
                                }
                            }
                        }
                    }
                }
            }
        }
        RuleAction::Accept
    }
}

pub trait NAT {
    fn add_mapping(&mut self, internal_ip: &[u8], internal_port: u16, external_port: u16) -> Result<(), FirewallError>;
    fn remove_mapping(&mut self, internal_port: u16) -> Result<(), FirewallError>;
    fn translate(&self, internal_ip: &[u8], internal_port: u16) -> Option<(u16, [u8; 4])>;
}

#[repr(C)]
pub struct SimpleNAT {
    pub mappings: Vec<([u8; 4], u16, u16)>,
}

impl SimpleNAT {
    pub fn new() -> Self {
        SimpleNAT {
            mappings: Vec::new(),
        }
    }
}

impl NAT for SimpleNAT {
    fn add_mapping(&mut self, internal_ip: &[u8], internal_port: u16, external_port: u16) -> Result<(), FirewallError> {
        let mut ip_array = [0u8; 4];
        let ip_len = internal_ip.len().min(4);
        for i in 0..ip_len { ip_array[i] = internal_ip[i]; }
        self.mappings.push((ip_array, internal_port, external_port));
        Ok(())
    }

    fn remove_mapping(&mut self, internal_port: u16) -> Result<(), FirewallError> {
        for i in 0..self.mappings.len() {
            if self.mappings[i].1 == internal_port {
                self.mappings.remove(i);
                return Ok(());
            }
        }
        Err(FirewallError::NotFound)
    }

    fn translate(&self, internal_ip: &[u8], internal_port: u16) -> Option<(u16, [u8; 4])> {
        for &(ref ip, int_port, ext_port) in &self.mappings {
            if ip == internal_ip && int_port == internal_port {
                return Some((ext_port, *ip));
            }
        }
        None
    }
}

/// Snort-style security signature description (IDS payload checker)
#[derive(Debug, Clone)]
pub struct SnortSignature {
    pub pattern: Vec<u8>,
    pub protocol: Protocol,
    pub dst_port: u16,
    pub alert_message: &'static str,
}

/// Active traffic count per IP for Crowdstrike Falcon-inspired AI Flood monitoring
#[derive(Debug, Clone)]
pub struct CrowdstrikeIpTelemetry {
    pub source_ip: [u8; 4],
    pub packet_count: u32,
    pub anomaly_flagged: bool,
}

/// Sovereign AI Network Intrusion Detection System (combines Snort & Crowdstrike)
pub struct SovereignSecurityIds {
    pub signatures: Vec<SnortSignature>,
    pub telemetry: Vec<CrowdstrikeIpTelemetry>,
    pub max_packet_flood_threshold: u32, // Max allowed requests per IP frame
}

impl Default for SovereignSecurityIds {
    fn default() -> Self {
        Self::new()
    }
}

impl SovereignSecurityIds {
    pub fn new() -> Self {
        SovereignSecurityIds {
            signatures: Vec::new(),
            telemetry: Vec::new(),
            max_packet_flood_threshold: 50,
        }
    }

    pub fn add_signature(&mut self, sig: SnortSignature) {
        self.signatures.push(sig);
    }

    /// Evaluates active packets against signature lists and behavioral floods
    pub fn inspect_packet(
        &mut self,
        source_ip: [u8; 4],
        protocol: Protocol,
        dst_port: u16,
        payload: &[u8],
    ) -> Result<RuleAction, &'static str> {
        // 1. Crowdstrike-style Flood/DDOS Anomaly Check
        let mut telemetry_index = None;
        for i in 0..self.telemetry.len() {
            if self.telemetry[i].source_ip == source_ip {
                telemetry_index = Some(i);
                break;
            }
        }

        let idx = if let Some(i) = telemetry_index {
            i
        } else {
            let item = CrowdstrikeIpTelemetry {
                source_ip,
                packet_count: 0,
                anomaly_flagged: false,
            };
            self.telemetry.push(item);
            self.telemetry.len() - 1
        };

        self.telemetry[idx].packet_count += 1;

        if self.telemetry[idx].packet_count >= self.max_packet_flood_threshold {
            self.telemetry[idx].anomaly_flagged = true;
            return Err("CrowdStrike: DDOS flood anomaly detected. Dropping packet flow.");
        }

        // 2. Snort-style Signature Inspection
        for sig in self.signatures.iter() {
            if (sig.protocol == Protocol::Any || sig.protocol == protocol) && sig.dst_port == dst_port {
                // Perform simple substring search in standard #![no_std] slice
                if contains_substring(payload, &sig.pattern) {
                    return Err(sig.alert_message);
                }
            }
        }

        Ok(RuleAction::Accept)
    }
}

fn contains_substring(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() { return true; }
    if haystack.len() < needle.len() { return false; }
    for i in 0..=haystack.len() - needle.len() {
        if &haystack[i..i + needle.len()] == needle {
            return true;
        }
    }
    false
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snort_signature_matching() {
        let mut ids = SovereignSecurityIds::new();

        let sig = SnortSignature {
            pattern: b"exploit_command".to_vec(),
            protocol: Protocol::TCP,
            dst_port: 80,
            alert_message: "Snort: Blocked standard remote code execution exploit attempt.",
        };

        ids.add_signature(sig);

        // Safe packet -> should be accepted
        let action_clean = ids.inspect_packet([192, 168, 1, 10], Protocol::TCP, 80, b"GET /index.html HTTP/1.1");
        assert_eq!(action_clean, Ok(RuleAction::Accept));

        // Exploiting packet -> should be rejected with Snort alert
        let action_exploit = ids.inspect_packet([192, 168, 1, 10], Protocol::TCP, 80, b"POST /submit?cmd=exploit_command");
        assert_eq!(action_exploit, Err("Snort: Blocked standard remote code execution exploit attempt."));
    }

    #[test]
    fn test_crowdstrike_flood_ddos_detection() {
        let mut ids = SovereignSecurityIds::new();
        ids.max_packet_flood_threshold = 4; // limit to 4 packets

        let attacker_ip = [10, 0, 0, 9];

        // First 3 packets are accepted
        for _ in 0..3 {
            assert_eq!(ids.inspect_packet(attacker_ip, Protocol::UDP, 53, b"dns_query"), Ok(RuleAction::Accept));
        }

        // 4th packet triggers Crowdstrike flood anomaly threshold
        let block_res = ids.inspect_packet(attacker_ip, Protocol::UDP, 53, b"dns_query");
        assert!(block_res.is_err());
        assert!(ids.telemetry[0].anomaly_flagged);
    }
}
