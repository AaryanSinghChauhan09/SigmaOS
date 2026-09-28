// Packet Filter Engine
// Inspired by OpenBSD PF and Linux iptables for firewall rules

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicU64, Ordering};

/// Action for packet filter rule
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    Block,
    Reject,
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfProtocol {
    Tcp,
    Udp,
    Icmp,
    All,
}

/// Packet filter rule
#[derive(Debug, Clone)]
pub struct PfRule {
    pub id: u64,
    pub action: PfAction,
    pub protocol: PfProtocol,
    pub source_addr: Option<Ipv4Addr>,
    pub source_port: Option<u16>,
    pub dest_addr: Option<Ipv4Addr>,
    pub dest_port: Option<u16>,
    pub enabled: bool,
}

/// Packet
#[derive(Debug, Clone)]
pub struct Packet {
    pub source_addr: Ipv4Addr,
    pub source_port: u16,
    pub dest_addr: Ipv4Addr,
    pub dest_port: u16,
    pub protocol: PfProtocol,
}

/// Packet filter engine
pub struct PacketFilter {
    next_rule_id: AtomicU64,
    rules: Vec<PfRule>,
    packet_count: AtomicU64,
    block_count: AtomicU64,
    pass_count: AtomicU64,
}

impl PacketFilter {
    pub fn new() -> Self {
        Self {
            next_rule_id: AtomicU64::new(1),
            rules: Vec::new(),
            packet_count: AtomicU64::new(0),
            block_count: AtomicU64::new(0),
            pass_count: AtomicU64::new(0),
        }
    }

    /// Add a filter rule
    pub fn add_rule(&mut self, rule: PfRule) -> u64 {
        let id = self.next_rule_id.fetch_add(1, Ordering::SeqCst);
        let mut new_rule = rule;
        new_rule.id = id;
        
        self.rules.push(new_rule);
        id
    }

    /// Remove a rule by ID
    pub fn remove_rule(&mut self, id: u64) -> Result<(), &'static str> {
        if let Some(pos) = self.rules.iter().position(|r| r.id == id) {
            self.rules.remove(pos);
            Ok(())
        } else {
            Err("Rule not found")
        }
    }

    /// Enable/disable rule
    pub fn set_rule_enabled(&mut self, id: u64, enabled: bool) -> Result<(), &'static str> {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == id) {
            rule.enabled = enabled;
            Ok(())
        } else {
            Err("Rule not found")
        }
    }

    /// Filter a packet
    pub fn filter_packet(&mut self, packet: &Packet) -> PfAction {
        self.packet_count.fetch_add(1, Ordering::SeqCst);
        
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            
            if self.rule_matches(rule, packet) {
                let action = rule.action;
                match action {
                    PfAction::Block => {
                        self.block_count.fetch_add(1, Ordering::SeqCst);
                    }
                    PfAction::Pass => {
                        self.pass_count.fetch_add(1, Ordering::SeqCst);
                    }
                    PfAction::Reject => {
                        self.block_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
                return action;
            }
        }
        
        // Default action: pass
        self.pass_count.fetch_add(1, Ordering::SeqCst);
        PfAction::Pass
    }

    /// Check if rule matches packet
    fn rule_matches(&self, rule: &PfRule, packet: &Packet) -> bool {
        // Check protocol
        if rule.protocol != PfProtocol::All && rule.protocol != packet.protocol {
            return false;
        }
        
        // Check source address
        if let Some(addr) = rule.source_addr {
            if addr != packet.source_addr {
                return false;
            }
        }
        
        // Check source port
        if let Some(port) = rule.source_port {
            if port != packet.source_port {
                return false;
            }
        }
        
        // Check destination address
        if let Some(addr) = rule.dest_addr {
            if addr != packet.dest_addr {
                return false;
            }
        }
        
        // Check destination port
        if let Some(port) = rule.dest_port {
            if port != packet.dest_port {
                return false;
            }
        }
        
        true
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[PfRule] {
        &self.rules
    }

    /// Get rule by ID
    pub fn get_rule(&self, id: u64) -> Option<&PfRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// Get packet count
    pub fn packet_count(&self) -> u64 {
        self.packet_count.load(Ordering::SeqCst)
    }

    /// Get block count
    pub fn block_count(&self) -> u64 {
        self.block_count.load(Ordering::SeqCst)
    }

    /// Get pass count
    pub fn pass_count(&self) -> u64 {
        self.pass_count.load(Ordering::SeqCst)
    }

    /// Clear all rules
    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rule() {
        let mut filter = PacketFilter::new();
        
        let rule = PfRule {
            id: 0,
            action: PfAction::Block,
            protocol: PfProtocol::Tcp,
            source_addr: None,
            source_port: None,
            dest_addr: None,
            dest_port: Some(22),
            enabled: true,
        };
        
        let id = filter.add_rule(rule);
        assert_eq!(id, 1);
        assert_eq!(filter.rule_count(), 1);
    }

    #[test]
    fn test_filter_packet() {
        let mut filter = PacketFilter::new();
        
        let rule = PfRule {
            id: 0,
            action: PfAction::Block,
            protocol: PfProtocol::Tcp,
            source_addr: None,
            source_port: None,
            dest_addr: None,
            dest_port: Some(22),
            enabled: true,
        };
        
        filter.add_rule(rule);
        
        let packet = Packet {
            source_addr: Ipv4Addr::new(192, 168, 1, 1),
            source_port: 50000,
            dest_addr: Ipv4Addr::new(192, 168, 1, 2),
            dest_port: 22,
            protocol: PfProtocol::Tcp,
        };
        
        let action = filter.filter_packet(&packet);
        assert_eq!(action, PfAction::Block);
    }

    #[test]
    fn test_filter_pass() {
        let mut filter = PacketFilter::new();
        
        let packet = Packet {
            source_addr: Ipv4Addr::new(192, 168, 1, 1),
            source_port: 50000,
            dest_addr: Ipv4Addr::new(192, 168, 1, 2),
            dest_port: 80,
            protocol: PfProtocol::Tcp,
        };
        
        let action = filter.filter_packet(&packet);
        assert_eq!(action, PfAction::Pass);
    }

    #[test]
    fn test_remove_rule() {
        let mut filter = PacketFilter::new();
        
        let rule = PfRule {
            id: 0,
            action: PfAction::Block,
            protocol: PfProtocol::Tcp,
            source_addr: None,
            source_port: None,
            dest_addr: None,
            dest_port: Some(22),
            enabled: true,
        };
        
        let id = filter.add_rule(rule);
        assert!(filter.remove_rule(id).is_ok());
        assert_eq!(filter.rule_count(), 0);
    }

    #[test]
    fn test_rule_enabled() {
        let mut filter = PacketFilter::new();
        
        let rule = PfRule {
            id: 0,
            action: PfAction::Block,
            protocol: PfProtocol::Tcp,
            source_addr: None,
            source_port: None,
            dest_addr: None,
            dest_port: Some(22),
            enabled: true,
        };
        
        let id = filter.add_rule(rule);
        assert!(filter.set_rule_enabled(id, false).is_ok());
        
        let packet = Packet {
            source_addr: Ipv4Addr::new(192, 168, 1, 1),
            source_port: 50000,
            dest_addr: Ipv4Addr::new(192, 168, 1, 2),
            dest_port: 22,
            protocol: PfProtocol::Tcp,
        };
        
        let action = filter.filter_packet(&packet);
        assert_eq!(action, PfAction::Pass); // Disabled rule
    }
}
