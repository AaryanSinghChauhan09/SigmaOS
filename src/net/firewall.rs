#![no_std]
#![no_main]

/// OOP-based Firewall for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 761
/// Implements packet filtering and network security

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RuleID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RuleAction { Accept = 0, Drop = 1, Reject = 2, Log = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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
