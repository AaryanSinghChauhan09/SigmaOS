#![no_std]
#![cfg_attr(not(test), no_main)]

/// Sovereign Stateful Firewall & Netfilter-Style Connection Tracker for SigmaOS
/// Inspired by Linux Netfilter (iptables/nftables) and conntrack architectures

extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use core::sync::atomic::{AtomicU32, Ordering};

pub type RuleID = usize;

/// Netfilter-style packet hook points
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallHook {
    Prerouting,
    Input,
    Forward,
    Output,
    Postrouting,
}

/// Action taken on matching packet
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Accept = 0,
    Drop = 1,
    Reject = 2,
    Log = 3,
}

/// Supported packet protocols
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Any = 255,
}

/// Stateful Connection Tracking States
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    New,
    Established,
    Related,
    Invalid,
}

/// Represents an active network connection inside the state table
#[derive(Debug, Clone)]
pub struct ConntrackEntry {
    pub protocol: Protocol,
    pub source_ip: [u8; 4],
    pub destination_ip: [u8; 4],
    pub source_port: u16,
    pub destination_port: u16,
    pub last_seen_timestamp: u64,
}

/// Stateful Connection Tracker (Linux conntrack equivalent)
pub struct SovereignConntrack {
    pub active_connections: Vec<ConntrackEntry>,
}

impl SovereignConntrack {
    pub fn new() -> Self {
        Self {
            active_connections: Vec::new(),
        }
    }

    /// Evaluates packet state, automatically registering new flows and marking active ones as ESTABLISHED
    pub fn track_packet(
        &mut self,
        protocol: Protocol,
        src_ip: [u8; 4],
        dst_ip: [u8; 4],
        src_port: u16,
        dst_port: u16,
        timestamp: u64,
    ) -> ConnectionState {
        // Search for matching forward or reverse flow
        let found = self.active_connections.iter().any(|c| {
            c.protocol == protocol &&
            ((c.source_ip == src_ip && c.destination_ip == dst_ip && c.source_port == src_port && c.destination_port == dst_port) ||
             (c.source_ip == dst_ip && c.destination_ip == src_ip && c.source_port == dst_port && c.destination_port == src_port))
        });

        if found {
            ConnectionState::Established
        } else {
            // Register new connection flow
            self.active_connections.push(ConntrackEntry {
                protocol,
                source_ip: src_ip,
                destination_ip: dst_ip,
                source_port: src_port,
                destination_port: dst_port,
                last_seen_timestamp: timestamp,
            });
            ConnectionState::New
        }
    }
}

/// Rule trait for abstract capability matching
pub trait FirewallRule {
    fn id(&self) -> RuleID;
    fn action(&self) -> RuleAction;
    fn protocol(&self) -> Protocol;
    fn source_ip(&self) -> &[u8];
    fn destination_ip(&self) -> &[u8];
    fn source_port(&self) -> u16;
    fn destination_port(&self) -> u16;
    fn hook(&self) -> FirewallHook;
}

/// Simple netfilter firewall rule
pub struct SimpleFirewallRule {
    pub id: RuleID,
    pub action: AtomicU32,
    pub protocol: AtomicU32,
    pub source_ip: [u8; 4],
    pub destination_ip: [u8; 4],
    pub source_port: AtomicU32,
    pub destination_port: AtomicU32,
    pub hook: FirewallHook,
}

impl SimpleFirewallRule {
    pub fn new(
        id: RuleID,
        action: RuleAction,
        protocol: Protocol,
        source_ip: &[u8],
        destination_ip: &[u8],
        source_port: u16,
        destination_port: u16,
        hook: FirewallHook,
    ) -> Self {
        let mut src_ip = [0u8; 4];
        let mut dst_ip = [0u8; 4];
        let src_len = source_ip.len().min(4);
        let dst_len = destination_ip.len().min(4);
        src_ip[..src_len].copy_from_slice(&source_ip[..src_len]);
        dst_ip[..dst_len].copy_from_slice(&destination_ip[..dst_len]);

        SimpleFirewallRule {
            id,
            action: AtomicU32::new(action as u32),
            protocol: AtomicU32::new(protocol as u32),
            source_ip: src_ip,
            destination_ip: dst_ip,
            source_port: AtomicU32::new(source_port as u32),
            destination_port: AtomicU32::new(destination_port as u32),
            hook,
        }
    }
}

impl FirewallRule for SimpleFirewallRule {
    fn id(&self) -> RuleID {
        self.id
    }

    fn action(&self) -> RuleAction {
        unsafe { core::mem::transmute(self.action.load(Ordering::SeqCst)) }
    }

    fn protocol(&self) -> Protocol {
        unsafe { core::mem::transmute(self.protocol.load(Ordering::SeqCst)) }
    }

    fn source_ip(&self) -> &[u8] {
        &self.source_ip
    }

    fn destination_ip(&self) -> &[u8] {
        &self.destination_ip
    }

    fn source_port(&self) -> u16 {
        self.source_port.load(Ordering::SeqCst) as u16
    }

    fn destination_port(&self) -> u16 {
        self.destination_port.load(Ordering::SeqCst) as u16
    }

    fn hook(&self) -> FirewallHook {
        self.hook
    }
}

/// Network Address Translation (NAT) Mapping entry
#[derive(Debug, Clone)]
pub struct NatMapping {
    pub internal_ip: [u8; 4],
    pub internal_port: u16,
    pub external_port: u16,
}

/// High-Performance Sovereign Firewall
pub struct SovereignFirewall {
    pub rules: Vec<Box<dyn FirewallRule>>,
    pub conntrack: SovereignConntrack,
    pub nat_mappings: Vec<NatMapping>,
}

impl SovereignFirewall {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            conntrack: SovereignConntrack::new(),
            nat_mappings: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: Box<dyn FirewallRule>) -> RuleID {
        let id = rule.id();
        self.rules.push(rule);
        id
    }

    pub fn remove_rule(&mut self, id: RuleID) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.id() == id) {
            self.rules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Evaluates a packet across hooks, stateful conntrack flows, and security rules
    pub fn filter_packet(
        &mut self,
        hook: FirewallHook,
        protocol: Protocol,
        src_ip: [u8; 4],
        dst_ip: [u8; 4],
        src_port: u16,
        dst_port: u16,
        timestamp: u64,
    ) -> RuleAction {
        // 1. Stateful Inspection: check if packet is part of an ESTABLISHED connection
        let state = self.conntrack.track_packet(protocol, src_ip, dst_ip, src_port, dst_port, timestamp);
        if state == ConnectionState::Established {
            return RuleAction::Accept; // Instant fast-path acceptance (iptables state ESTABLISHED rule equivalent)
        }

        // 2. Netfilter Rule Chain Match
        for rule in &self.rules {
            if rule.hook() == hook {
                if rule.protocol() == Protocol::Any || rule.protocol() == protocol {
                    if rule.source_ip() == &[0, 0, 0, 0] || rule.source_ip() == src_ip {
                        if rule.destination_ip() == &[0, 0, 0, 0] || rule.destination_ip() == dst_ip {
                            if rule.source_port() == 0 || rule.source_port() == src_port {
                                if rule.destination_port() == 0 || rule.destination_port() == dst_port {
                                    return rule.action();
                                }
                            }
                        }
                    }
                }
            }
        }

        RuleAction::Accept // Default policy
    }

    // NAT Mapping support
    pub fn add_nat_mapping(&mut self, internal_ip: [u8; 4], internal_port: u16, external_port: u16) {
        self.nat_mappings.push(NatMapping {
            internal_ip,
            internal_port,
            external_port,
        });
    }

    pub fn translate_nat(&self, internal_ip: [u8; 4], internal_port: u16) -> Option<u16> {
        self.nat_mappings
            .iter()
            .find(|m| m.internal_ip == internal_ip && m.internal_port == internal_port)
            .map(|m| m.external_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stateful_conntrack() {
        let mut conntrack = SovereignConntrack::new();
        let src = [192, 168, 1, 10];
        let dst = [8, 8, 8, 8];

        // First packet initiates a NEW connection flow
        let state1 = conntrack.track_packet(Protocol::Tcp, src, dst, 45210, 80, 1000);
        assert_eq!(state1, ConnectionState::New);

        // Reverse or subsequent response packet is marked as ESTABLISHED
        let state2 = conntrack.track_packet(Protocol::Tcp, dst, src, 80, 45210, 1001);
        assert_eq!(state2, ConnectionState::Established);
    }

    #[test]
    fn test_netfilter_rules_matching() {
        let mut firewall = SovereignFirewall::new();
        let src = [192, 168, 1, 50];
        let dst = [10, 0, 0, 1];

        // Register a drop rule on forwarding chain for UDP protocol
        let rule = SimpleFirewallRule::new(
            1,
            RuleAction::Drop,
            Protocol::Udp,
            &src,
            &[0, 0, 0, 0],
            0,
            53,
            FirewallHook::Forward,
        );
        firewall.add_rule(Box::new(rule));

        // Matching UDP on Forward hook should be dropped
        let action1 = firewall.filter_packet(
            FirewallHook::Forward,
            Protocol::Udp,
            src,
            dst,
            5520,
            53,
            2000,
        );
        assert_eq!(action1, RuleAction::Drop);

        // Different protocol (TCP) on same hook should be accepted
        let action2 = firewall.filter_packet(
            FirewallHook::Forward,
            Protocol::Tcp,
            src,
            dst,
            5520,
            53,
            2001,
        );
        assert_eq!(action2, RuleAction::Accept);
    }

    #[test]
    fn test_nat_address_translation() {
        let mut firewall = SovereignFirewall::new();
        let internal = [192, 168, 1, 15];

        firewall.add_nat_mapping(internal, 8080, 80);
        assert_eq!(firewall.translate_nat(internal, 8080), Some(80));
        assert_eq!(firewall.translate_nat(internal, 9000), None);
    }
}
