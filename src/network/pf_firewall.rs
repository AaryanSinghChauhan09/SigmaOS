// OpenBSD pf-Inspired Firewall with Stateful Packet Filtering
// Provides comprehensive packet filtering, NAT, traffic shaping, and state tracking

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// pf-inspired action for rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    Block,
    Reject,
    Scrub,
    NoTrack,
}

/// pf-inspired direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfDirection {
    In,
    Out,
    InOut,
}

/// pf-inspired protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfProtocol {
    Tcp,
    Udp,
    Icmp,
    IcmpV6,
    Any,
}

/// pf-inspired interface specification
#[derive(Debug, Clone)]
pub struct PfInterface {
    pub name: String,
    pub is_virtual: bool,
}

impl PfInterface {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_virtual: false,
        }
    }

    pub fn virtual_interface(name: String) -> Self {
        Self {
            name,
            is_virtual: true,
        }
    }
}

/// pf-inspired address specification
#[derive(Debug, Clone)]
pub enum PfAddress {
    Any,
    Single(String),
    Range(String, String), // start, end
    Network(String, u8),   // network, prefix
    Table(String),         // table name
}

impl PfAddress {
    pub fn any() -> Self {
        PfAddress::Any
    }

    pub fn single(addr: String) -> Self {
        PfAddress::Single(addr)
    }

    pub fn network(addr: String, prefix: u8) -> Self {
        PfAddress::Network(addr, prefix)
    }
}

/// pf-inspired port specification
#[derive(Debug, Clone)]
pub enum PfPort {
    Any,
    Single(u16),
    Range(u16, u16),
}

impl PfPort {
    pub fn any() -> Self {
        PfPort::Any
    }

    pub fn single(port: u16) -> Self {
        PfPort::Single(port)
    }

    pub fn range(start: u16, end: u16) -> Self {
        PfPort::Range(start, end)
    }
}

/// pf-inspired state tracking options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfStateOption {
    Track,
    NoTrack,
    Floating,
    Sloppy,
}

/// pf-inspired rule options
#[derive(Debug, Clone)]
pub struct PfRuleOptions {
    pub log: bool,
    pub quick: bool, // If true, stop rule evaluation on match
    pub state_option: PfStateOption,
    pub keep_state: bool,
    pub modulate_state: bool,
    pub synproxy: bool,
}

impl Default for PfRuleOptions {
    fn default() -> Self {
        Self {
            log: false,
            quick: false,
            state_option: PfStateOption::Track,
            keep_state: true,
            modulate_state: false,
            synproxy: false,
        }
    }
}

/// pf-inspired firewall rule
#[derive(Debug, Clone)]
pub struct PfRule {
    pub id: u64,
    pub action: PfAction,
    pub direction: PfDirection,
    pub interface: Option<PfInterface>,
    pub protocol: PfProtocol,
    pub source_address: PfAddress,
    pub source_port: PfPort,
    pub destination_address: PfAddress,
    pub destination_port: PfPort,
    pub options: PfRuleOptions,
    pub description: String,
}

impl PfRule {
    pub fn new(id: u64, action: PfAction) -> Self {
        Self {
            id,
            action,
            direction: PfDirection::InOut,
            interface: None,
            protocol: PfProtocol::Any,
            source_address: PfAddress::Any,
            source_port: PfPort::Any,
            destination_address: PfAddress::Any,
            destination_port: PfPort::Any,
            options: PfRuleOptions::default(),
            description: String::new(),
        }
    }

    pub fn with_direction(mut self, direction: PfDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_interface(mut self, interface: PfInterface) -> Self {
        self.interface = Some(interface);
        self
    }

    pub fn with_protocol(mut self, protocol: PfProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_source(mut self, address: PfAddress, port: PfPort) -> Self {
        self.source_address = address;
        self.source_port = port;
        self
    }

    pub fn with_destination(mut self, address: PfAddress, port: PfPort) -> Self {
        self.destination_address = address;
        self.destination_port = port;
        self
    }

    pub fn with_options(mut self, options: PfRuleOptions) -> Self {
        self.options = options;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

/// pf-inspired connection state for stateful filtering
#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub id: u64,
    pub source_addr: String,
    pub source_port: u16,
    pub dest_addr: String,
    pub dest_port: u16,
    pub protocol: PfProtocol,
    pub state: TcpState,
    pub created_time: u64,
    pub last_activity: u64,
    pub bytes_in: u64,
    pub bytes_out: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    Closing,
    TimeWait,
    CloseWait,
    LastAck,
    Closed,
}

impl ConnectionState {
    pub fn new(
        id: u64,
        source_addr: String,
        source_port: u16,
        dest_addr: String,
        dest_port: u16,
        protocol: PfProtocol,
        timestamp: u64,
    ) -> Self {
        Self {
            id,
            source_addr,
            source_port,
            dest_addr,
            dest_port,
            protocol,
            state: TcpState::SynSent,
            created_time: timestamp,
            last_activity: timestamp,
            bytes_in: 0,
            bytes_out: 0,
        }
    }

    pub fn update_activity(&mut self, timestamp: u64, bytes: u64, is_inbound: bool) {
        self.last_activity = timestamp;
        if is_inbound {
            self.bytes_in += bytes;
        } else {
            self.bytes_out += bytes;
        }
    }

    pub fn is_expired(&self, current_time: u64, timeout: u64) -> bool {
        current_time - self.last_activity > timeout
    }
}

/// pf-inspired address table for dynamic address groups
#[derive(Debug, Clone)]
pub struct PfTable {
    pub name: String,
    pub addresses: Vec<String>,
    pub is_persistent: bool,
}

impl PfTable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            addresses: Vec::new(),
            is_persistent: false,
        }
    }

    pub fn persistent(name: String) -> Self {
        Self {
            name,
            addresses: Vec::new(),
            is_persistent: true,
        }
    }

    pub fn add_address(&mut self, address: String) {
        if !self.addresses.contains(&address) {
            self.addresses.push(address);
        }
    }

    pub fn remove_address(&mut self, address: &str) {
        self.addresses.retain(|a| a != address);
    }

    pub fn contains(&self, address: &str) -> bool {
        self.addresses.contains(&address.to_string())
    }
}

/// pf-inspired NAT rule
#[derive(Debug, Clone)]
pub struct PfNatRule {
    pub id: u64,
    pub interface: PfInterface,
    pub source_address: PfAddress,
    pub external_address: PfAddress,
    pub description: String,
}

impl PfNatRule {
    pub fn new(id: u64, interface: PfInterface) -> Self {
        Self {
            id,
            interface,
            source_address: PfAddress::Any,
            external_address: PfAddress::Any,
            description: String::new(),
        }
    }

    pub fn with_source(mut self, address: PfAddress) -> Self {
        self.source_address = address;
        self
    }

    pub fn with_external(mut self, address: PfAddress) -> Self {
        self.external_address = address;
        self
    }
}

/// pf-inspired traffic queue for QoS
#[derive(Debug, Clone)]
pub struct PfQueue {
    pub name: String,
    pub bandwidth: u64, // bytes per second
    pub priority: u8,
    pub borrow: bool,
}

impl PfQueue {
    pub fn new(name: String, bandwidth: u64, priority: u8) -> Self {
        Self {
            name,
            bandwidth,
            priority,
            borrow: true,
        }
    }
}

/// pf-inspired firewall manager
pub struct PfFirewall {
    rules: Vec<PfRule>,
    tables: BTreeMap<String, PfTable>,
    states: BTreeMap<u64, ConnectionState>,
    nat_rules: Vec<PfNatRule>,
    queues: BTreeMap<String, PfQueue>,
    next_rule_id: u64,
    next_state_id: u64,
    next_nat_id: u64,
    default_action: PfAction,
    state_timeout: u64, // seconds
    log_enabled: bool,
}

impl PfFirewall {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            tables: BTreeMap::new(),
            states: BTreeMap::new(),
            nat_rules: Vec::new(),
            queues: BTreeMap::new(),
            next_rule_id: 1,
            next_state_id: 1,
            next_nat_id: 1,
            default_action: PfAction::Block,
            state_timeout: 86400, // 24 hours default
            log_enabled: false,
        }
    }

    pub fn with_default_action(action: PfAction) -> Self {
        let mut firewall = Self::new();
        firewall.default_action = action;
        firewall
    }

    pub fn add_rule(&mut self, rule: PfRule) -> u64 {
        let id = rule.id;
        self.rules.push(rule);
        id
    }

    pub fn remove_rule(&mut self, id: u64) -> Result<(), &'static str> {
        let pos = self.rules.iter().position(|r| r.id == id);
        if let Some(pos) = pos {
            self.rules.remove(pos);
            Ok(())
        } else {
            Err("Rule not found")
        }
    }

    pub fn get_rule(&self, id: u64) -> Option<&PfRule> {
        self.rules.iter().find(|r| r.id == id)
    }

    pub fn list_rules(&self) -> Vec<&PfRule> {
        self.rules.iter().collect()
    }

    pub fn add_table(&mut self, table: PfTable) -> Result<(), &'static str> {
        if self.tables.contains_key(&table.name) {
            return Err("Table already exists");
        }
        self.tables.insert(table.name.clone(), table);
        Ok(())
    }

    pub fn get_table(&self, name: &str) -> Option<&PfTable> {
        self.tables.get(name)
    }

    pub fn get_table_mut(&mut self, name: &str) -> Option<&mut PfTable> {
        self.tables.get_mut(name)
    }

    pub fn add_nat_rule(&mut self, rule: PfNatRule) -> u64 {
        let id = rule.id;
        self.nat_rules.push(rule);
        id
    }

    pub fn add_queue(&mut self, queue: PfQueue) -> Result<(), &'static str> {
        if self.queues.contains_key(&queue.name) {
            return Err("Queue already exists");
        }
        self.queues.insert(queue.name.clone(), queue);
        Ok(())
    }

    pub fn set_default_action(&mut self, action: PfAction) {
        self.default_action = action;
    }

    pub fn set_state_timeout(&mut self, timeout: u64) {
        self.state_timeout = timeout;
    }

    pub fn enable_logging(&mut self) {
        self.log_enabled = true;
    }

    pub fn disable_logging(&mut self) {
        self.log_enabled = false;
    }

    /// Process a packet through the firewall rules
    pub fn process_packet(
        &mut self,
        source_addr: String,
        source_port: u16,
        dest_addr: String,
        dest_port: u16,
        protocol: PfProtocol,
        direction: PfDirection,
        interface: Option<String>,
        timestamp: u64,
    ) -> PfAction {
        // Check if this packet belongs to an existing state
        let state_key =
            self.calculate_state_key(&source_addr, source_port, &dest_addr, dest_port, protocol);

        if let Some(state) = self.states.get(&state_key) {
            if !state.is_expired(timestamp, self.state_timeout) {
                // Allow packets for established connections
                return PfAction::Pass;
            } else {
                // Remove expired state
                self.states.remove(&state_key);
            }
        }

        // Evaluate rules in order
        for rule in &self.rules {
            if self.rule_matches(
                rule,
                &source_addr,
                source_port,
                &dest_addr,
                dest_port,
                protocol,
                direction,
                interface.as_deref(),
            ) {
                if rule.options.log && self.log_enabled {
                    // Log the packet match
                }

                if rule.options.keep_state && rule.action == PfAction::Pass {
                    self.create_state(
                        source_addr,
                        source_port,
                        dest_addr,
                        dest_port,
                        protocol,
                        timestamp,
                    );
                }

                if rule.options.quick {
                    return rule.action;
                }
            }
        }

        self.default_action
    }

    fn rule_matches(
        &self,
        rule: &PfRule,
        source_addr: &str,
        source_port: u16,
        dest_addr: &str,
        dest_port: u16,
        protocol: PfProtocol,
        direction: PfDirection,
        interface: Option<&str>,
    ) -> bool {
        // Check direction
        if rule.direction != PfDirection::InOut && rule.direction != direction {
            return false;
        }

        // Check interface
        if let Some(ref rule_iface) = rule.interface {
            if let Some(iface) = interface {
                if rule_iface.name != iface {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check protocol
        if rule.protocol != PfProtocol::Any && rule.protocol != protocol {
            return false;
        }

        // Check addresses and ports
        if !self.address_matches(&rule.source_address, source_addr) {
            return false;
        }

        if !self.port_matches(&rule.source_port, source_port) {
            return false;
        }

        if !self.address_matches(&rule.destination_address, dest_addr) {
            return false;
        }

        if !self.port_matches(&rule.destination_port, dest_port) {
            return false;
        }

        true
    }

    fn address_matches(&self, rule_addr: &PfAddress, packet_addr: &str) -> bool {
        match rule_addr {
            PfAddress::Any => true,
            PfAddress::Single(addr) => addr == packet_addr,
            PfAddress::Network(addr, _prefix) => {
                // Simplified network matching
                packet_addr.starts_with(addr.trim_end_matches(['0', '.']))
            }
            PfAddress::Table(table_name) => {
                if let Some(table) = self.tables.get(table_name) {
                    table.contains(packet_addr)
                } else {
                    false
                }
            }
            PfAddress::Range(_, _) => false, // Simplified
        }
    }

    fn port_matches(&self, rule_port: &PfPort, packet_port: u16) -> bool {
        match rule_port {
            PfPort::Any => true,
            PfPort::Single(port) => *port == packet_port,
            PfPort::Range(start, end) => packet_port >= *start && packet_port <= *end,
        }
    }

    fn calculate_state_key(
        &self,
        src: &str,
        src_port: u16,
        dst: &str,
        dst_port: u16,
        proto: PfProtocol,
    ) -> u64 {
        // Simple hash function for state lookup
        let mut hash: u64 = 5381;
        for byte in src.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash = hash.wrapping_mul(33).wrapping_add(src_port as u64);
        for byte in dst.bytes() {
            hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
        }
        hash = hash.wrapping_mul(33).wrapping_add(dst_port as u64);
        hash = hash.wrapping_mul(33).wrapping_add(proto as u64);
        hash
    }

    fn create_state(
        &mut self,
        source_addr: String,
        source_port: u16,
        dest_addr: String,
        dest_port: u16,
        protocol: PfProtocol,
        timestamp: u64,
    ) {
        let id = self.next_state_id;
        self.next_state_id += 1;

        let state = ConnectionState::new(
            id,
            source_addr,
            source_port,
            dest_addr,
            dest_port,
            protocol,
            timestamp,
        );
        let key = self.calculate_state_key(
            &state.source_addr,
            state.source_port,
            &state.dest_addr,
            state.dest_port,
            protocol,
        );
        self.states.insert(key, state);
    }

    /// Clean up expired states
    pub fn cleanup_expired_states(&mut self, current_time: u64) -> usize {
        let mut expired = Vec::new();

        for (&key, state) in &self.states {
            if state.is_expired(current_time, self.state_timeout) {
                expired.push(key);
            }
        }

        for key in expired {
            self.states.remove(&key);
        }

        expired.len()
    }

    /// Get firewall statistics
    pub fn get_stats(&self) -> PfStats {
        PfStats {
            total_rules: self.rules.len(),
            total_tables: self.tables.len(),
            total_states: self.states.len(),
            total_nat_rules: self.nat_rules.len(),
            total_queues: self.queues.len(),
            default_action: self.default_action,
            log_enabled: self.log_enabled,
        }
    }

    /// Flush all rules
    pub fn flush_rules(&mut self) {
        self.rules.clear();
    }

    /// Flush all states
    pub fn flush_states(&mut self) {
        self.states.clear();
    }

    /// Load rules from configuration (simplified)
    pub fn load_rules(&mut self, rules: Vec<PfRule>) {
        self.rules = rules;
    }
}

/// pf-inspired firewall statistics
#[derive(Debug)]
pub struct PfStats {
    pub total_rules: usize,
    pub total_tables: usize,
    pub total_states: usize,
    pub total_nat_rules: usize,
    pub total_queues: usize,
    pub default_action: PfAction,
    pub log_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_creation() {
        let firewall = PfFirewall::new();
        assert_eq!(firewall.default_action, PfAction::Block);
    }

    #[test]
    fn test_rule_creation() {
        let rule = PfRule::new(1, PfAction::Pass)
            .with_protocol(PfProtocol::Tcp)
            .with_destination(
                PfAddress::single("192.168.1.1".to_string()),
                PfPort::single(80),
            );

        assert_eq!(rule.action, PfAction::Pass);
        assert_eq!(rule.protocol, PfProtocol::Tcp);
    }

    #[test]
    fn test_table_operations() {
        let mut firewall = PfFirewall::new();
        let table = PfTable::new("blocked_hosts".to_string());
        firewall.add_table(table).unwrap();

        let table = firewall.get_table_mut("blocked_hosts").unwrap();
        table.add_address("10.0.0.1".to_string());

        assert!(table.contains("10.0.0.1"));
    }

    #[test]
    fn test_packet_processing() {
        let mut firewall = PfFirewall::with_default_action(PfAction::Block);

        let rule = PfRule::new(1, PfAction::Pass)
            .with_protocol(PfProtocol::Tcp)
            .with_destination(
                PfAddress::single("192.168.1.1".to_string()),
                PfPort::single(80),
            )
            .with_options(PfRuleOptions {
                quick: true,
                ..Default::default()
            });

        firewall.add_rule(rule);

        let action = firewall.process_packet(
            "10.0.0.1".to_string(),
            12345,
            "192.168.1.1".to_string(),
            80,
            PfProtocol::Tcp,
            PfDirection::Out,
            None,
            0,
        );

        assert_eq!(action, PfAction::Pass);
    }
}
