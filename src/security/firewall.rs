// #![no_std]
extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protocol { Tcp, Udp, Icmp, Any }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action { Accept, Drop, Reject, Log, NatSnat(u32), NatDnat(u32) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState { New, Established, Related, Invalid }

pub struct Ipv4Address(pub u32);

pub struct MatchCriteria {
    pub source_ip: Option<Ipv4Address>,
    pub dest_ip: Option<Ipv4Address>,
    pub source_port: Option<u16>,
    pub dest_port: Option<u16>,
    pub protocol: Protocol,
    pub state: Option<ConnectionState>,
}

impl MatchCriteria {
    pub fn matches(&self, packet: &PacketInfo, state: ConnectionState) -> bool {
        if let Some(sip) = &self.source_ip { if packet.source_ip.0 != sip.0 { return false; } }
        if let Some(dip) = &self.dest_ip { if packet.dest_ip.0 != dip.0 { return false; } }
        if let Some(dport) = self.dest_port { if packet.dest_port != dport { return false; } }
        if self.protocol != Protocol::Any && self.protocol != packet.protocol { return false; }
        if let Some(req_state) = self.state { if req_state != state { return false; } }
        true
    }
}

pub struct Rule {
    pub criteria: MatchCriteria,
    pub action: Action,
}

pub struct PacketInfo {
    pub source_ip: Ipv4Address,
    pub dest_ip: Ipv4Address,
    pub source_port: u16,
    pub dest_port: u16,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionTuple {
    pub sip: u32, pub dip: u32, pub sport: u16, pub dport: u16, pub proto: Protocol,
}

pub struct ConnectionEntry {
    pub state: ConnectionState,
    pub last_seen: u64,
}

pub struct RateLimiter {
    pub tokens: usize,
    pub capacity: usize,
    pub fill_rate: usize,
    pub last_update: u64,
}

impl RateLimiter {
    pub fn allow(&mut self, now: u64) -> bool {
        let elapsed = now - self.last_update;
        self.tokens = core::cmp::min(self.capacity, self.tokens + (elapsed as usize * self.fill_rate));
        self.last_update = now;
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }
}

pub struct Firewall {
    pub input_rules: Vec<Rule>,
    pub output_rules: Vec<Rule>,
    pub forward_rules: Vec<Rule>,
    pub default_action: Action,
    pub conntrack: BTreeMap<ConnectionTuple, ConnectionEntry>,
    pub rate_limiter: RateLimiter,
}

impl Firewall {
    pub fn new() -> Self {
        Self {
            input_rules: Vec::new(),
            output_rules: Vec::new(),
            forward_rules: Vec::new(),
            default_action: Action::Drop,
            conntrack: BTreeMap::new(),
            rate_limiter: RateLimiter { tokens: 100, capacity: 100, fill_rate: 10, last_update: 0 },
        }
    }

    pub fn track_connection(&mut self, packet: &PacketInfo, now: u64) -> ConnectionState {
        let tuple = ConnectionTuple {
            sip: packet.source_ip.0, dip: packet.dest_ip.0,
            sport: packet.source_port, dport: packet.dest_port,
            proto: packet.protocol,
        };
        if let Some(entry) = self.conntrack.get_mut(&tuple) {
            entry.last_seen = now;
            entry.state = ConnectionState::Established;
            ConnectionState::Established
        } else {
            self.conntrack.insert(tuple, ConnectionEntry { state: ConnectionState::New, last_seen: now });
            ConnectionState::New
        }
    }

    pub fn evaluate_forward(&mut self, packet: &PacketInfo, now: u64) -> Action {
        let state = self.track_connection(packet, now);
        for rule in &self.forward_rules {
            if rule.criteria.matches(packet, state) {
                if rule.action == Action::Log {
                    if self.rate_limiter.allow(now) {
                        // Log packet logically
                    }
                    continue;
                }
                return rule.action;
            }
        }
        self.default_action
    }
}
