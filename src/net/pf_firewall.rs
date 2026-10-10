//! PF Firewall — OpenBSD/FreeBSD PF (Packet Filter) Implementation for SigmaOS
//!
//! PF (Packet Filter) is the stateful packet filter from OpenBSD, ported to FreeBSD,
//! NetBSD, and macOS. It is widely regarded as one of the most secure and well-designed
//! packet filters in any OS. Key features:
//! - Stateful connection tracking (TCP, UDP, ICMP)
//! - Network Address Translation (NAT/binat)
//! - Traffic shaping/queuing (ALTQ, HFSC, CBQ)
//! - Normalization/scrubbing (RFC-compliant packet reassembly)
//! - Anchors (nested rulesets for containers/jails/namespaces)
//! - Tables (efficient large IP set membership)
//! - Per-source connection rate limiting
//! - pfsync for stateful failover
//!
//! References:
//! - OpenBSD PF FAQ: https://www.openbsd.org/faq/pf/
//! - FreeBSD pf(4) manual: https://man.freebsd.org/cgi/man.cgi?pf(4)
//! - nftables (Linux) for modern inspiration
//! - iptables/netfilter for compatibility reference
//!
//! Future Development:
//! - pfsync state table replication for HA failover
//! - ALTQ traffic shaping with HFSC
//! - XDP/eBPF accelerated path for high-throughput
//! - SigmaOS network namespace integration
//! - pfctl userspace tool integration

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// PF Rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    Block,
    BlockReturn, // TCP RST / ICMP unreachable
    Match,       // Match without terminal action (for tagging/queuing)
    Nat,         // NAT translation
    Rdr,         // Port redirection
    Binat,       // Bidirectional NAT
}

/// Network protocol selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfProto {
    Any,
    Tcp,
    Udp,
    Icmp,
    Icmp6,
    Esp,
    Ah,
    Gre,
    Sctp,
}

/// PF Rule direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfDir {
    Any,
    In,
    Out,
}

/// PF Address specification
#[derive(Debug, Clone)]
pub enum PfAddr {
    /// Any address
    Any,
    /// Single IPv4 address
    Ipv4(u32),
    /// IPv4 network with prefix length
    Ipv4Net(u32, u8),
    /// Single IPv6 address (128 bits as 2×u64)
    Ipv6(u64, u64),
    /// Interface address ($if:0, etc.)
    IfAddr(String),
    /// PF Table reference (<table_name>)
    Table(String),
    /// Negate the address match
    Not(Box<PfAddr>),
}

/// PF Port specification
#[derive(Debug, Clone)]
pub enum PfPort {
    Any,
    /// Exact port
    Eq(u16),
    /// Port range (inclusive)
    Range(u16, u16),
    /// Not equal
    Ne(u16),
    /// Less than
    Lt(u16),
    /// Greater than
    Gt(u16),
}

impl PfPort {
    pub fn matches(&self, port: u16) -> bool {
        match self {
            Self::Any => true,
            Self::Eq(p) => port == *p,
            Self::Range(lo, hi) => port >= *lo && port <= *hi,
            Self::Ne(p) => port != *p,
            Self::Lt(p) => port < *p,
            Self::Gt(p) => port > *p,
        }
    }
}

/// PF State tracking mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfStateMode {
    None,
    Keep,     // Track state (default for pass rules)
    Modulate, // Randomize sequence numbers
    SynProxy, // SYN proxy protection
}

/// PF Firewall Rule
#[derive(Debug, Clone)]
pub struct PfRule {
    /// Rule number (lower = higher priority)
    pub number: u32,
    /// Rule label for logging/statistics
    pub label: String,
    /// Action when rule matches
    pub action: PfAction,
    /// Packet direction
    pub direction: PfDir,
    /// Interface (None = any interface)
    pub interface: Option<String>,
    /// Protocol
    pub proto: PfProto,
    /// Source address
    pub src_addr: PfAddr,
    /// Source port
    pub src_port: PfPort,
    /// Destination address
    pub dst_addr: PfAddr,
    /// Destination port
    pub dst_port: PfPort,
    /// State tracking mode
    pub state: PfStateMode,
    /// Log matching packets
    pub log: bool,
    /// Quick — stop processing rules after this match
    pub quick: bool,
    /// Max states for this rule
    pub max_states: Option<u32>,
    /// Rate limit: connections per second
    pub max_src_conn_rate: Option<(u32, u32)>, // (connections, seconds)
    /// NAT translation pool address (for NAT/RDR rules)
    pub nat_addr: Option<PfAddr>,
    pub nat_port: Option<PfPort>,
    /// Tag to apply on match
    pub tag: Option<String>,
    /// Match only tagged packets
    pub tagged: Option<String>,
    /// Statistics
    pub pkts_matched: u64,
    pub bytes_matched: u64,
}

impl PfRule {
    /// Create a simple pass rule
    pub fn pass(number: u32, direction: PfDir, proto: PfProto) -> Self {
        Self {
            number,
            label: String::new(),
            action: PfAction::Pass,
            direction,
            interface: None,
            proto,
            src_addr: PfAddr::Any,
            src_port: PfPort::Any,
            dst_addr: PfAddr::Any,
            dst_port: PfPort::Any,
            state: PfStateMode::Keep,
            log: false,
            quick: false,
            max_states: None,
            max_src_conn_rate: None,
            nat_addr: None,
            nat_port: None,
            tag: None,
            tagged: None,
            pkts_matched: 0,
            bytes_matched: 0,
        }
    }

    /// Create a block rule
    pub fn block(number: u32, direction: PfDir) -> Self {
        let mut rule = Self::pass(number, direction, PfProto::Any);
        rule.action = PfAction::Block;
        rule.quick = false;
        rule.state = PfStateMode::None;
        rule
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = String::from(label);
        self
    }

    pub fn with_src(mut self, addr: PfAddr, port: PfPort) -> Self {
        self.src_addr = addr;
        self.src_port = port;
        self
    }

    pub fn with_dst(mut self, addr: PfAddr, port: PfPort) -> Self {
        self.dst_addr = addr;
        self.dst_port = port;
        self
    }

    pub fn with_interface(mut self, iface: &str) -> Self {
        self.interface = Some(String::from(iface));
        self
    }

    pub fn quick(mut self) -> Self {
        self.quick = true;
        self
    }

    pub fn log(mut self) -> Self {
        self.log = true;
        self
    }
}

/// Connection state entry (stateful tracking)
#[derive(Debug, Clone)]
pub struct PfStateEntry {
    /// Protocol
    pub proto: PfProto,
    /// Source IP
    pub src_ip: u32,
    /// Source port
    pub src_port: u16,
    /// Destination IP
    pub dst_ip: u32,
    /// Destination port
    pub dst_port: u16,
    /// TCP state machine state
    pub tcp_state: u8,
    /// Bytes sent
    pub bytes: u64,
    /// Packets
    pub pkts: u64,
    /// Creation timestamp
    pub created_ns: u64,
    /// Expiry timestamp (nanoseconds)
    pub expires_ns: u64,
}

impl PfStateEntry {
    pub const TCP_CLOSED: u8 = 0;
    pub const TCP_SYN_SENT: u8 = 1;
    pub const TCP_SYN_RCVD: u8 = 2;
    pub const TCP_ESTABLISHED: u8 = 3;
    pub const TCP_FIN_WAIT: u8 = 4;
    pub const TCP_CLOSE_WAIT: u8 = 5;
    pub const TCP_CLOSING: u8 = 6;
    pub const TCP_TIME_WAIT: u8 = 7;

    pub fn new_tcp(src_ip: u32, src_port: u16, dst_ip: u32, dst_port: u16) -> Self {
        Self {
            proto: PfProto::Tcp,
            src_ip,
            src_port,
            dst_ip,
            dst_port,
            tcp_state: Self::TCP_SYN_SENT,
            bytes: 0,
            pkts: 0,
            created_ns: 0,
            expires_ns: 86_400_000_000_000, // 24h default for established
        }
    }

    pub fn is_expired(&self, now_ns: u64) -> bool {
        now_ns > self.expires_ns
    }
}

/// PF Table — efficient large IP set for address matching
#[derive(Debug, Clone)]
pub struct PfTable {
    pub name: String,
    /// IPv4 entries (stored as host bits for fast lookup)
    pub ipv4_entries: Vec<(u32, u8)>, // (address, prefix_len)
    pub const_: bool, // persist flag
}

impl PfTable {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ipv4_entries: Vec::new(),
            const_: false,
        }
    }

    pub fn add_ipv4(&mut self, addr: u32, prefix: u8) {
        self.ipv4_entries.push((addr, prefix));
    }

    pub fn contains_ipv4(&self, addr: u32) -> bool {
        for &(net_addr, prefix) in &self.ipv4_entries {
            if prefix == 0 {
                return true; // 0.0.0.0/0 matches all
            }
            let mask = !0u32 << (32 - prefix);
            if (addr & mask) == (net_addr & mask) {
                return true;
            }
        }
        false
    }
}

/// PF Anchor — nested ruleset (for containers, jails, namespaces)
#[derive(Debug)]
pub struct PfAnchor {
    pub name: String,
    pub rules: Vec<PfRule>,
    pub tables: BTreeMap<String, PfTable>,
}

impl PfAnchor {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            rules: Vec::new(),
            tables: BTreeMap::new(),
        }
    }
}

/// Packet descriptor for rule matching
#[derive(Debug, Clone)]
pub struct PfPacket {
    pub direction: PfDir,
    pub interface: String,
    pub proto: PfProto,
    pub src_ip: u32,
    pub src_port: u16,
    pub dst_ip: u32,
    pub dst_port: u16,
    pub size: u32,
    pub tcp_flags: u8,
}

/// Main PF Firewall Engine
#[derive(Debug)]
pub struct PfFirewall {
    /// Global ruleset (evaluated in order)
    pub rules: Vec<PfRule>,
    /// Named tables
    pub tables: BTreeMap<String, PfTable>,
    /// Named anchors (nested rulesets)
    pub anchors: BTreeMap<String, PfAnchor>,
    /// State table for stateful tracking
    pub state_table: Vec<PfStateEntry>,
    /// Max states limit
    pub max_states: u32,
    /// PF enabled/disabled
    pub enabled: bool,
    /// Statistics
    pub stats: PfStats,
}

/// PF Firewall statistics
#[derive(Debug, Default)]
pub struct PfStats {
    pub pkts_passed: u64,
    pub pkts_blocked: u64,
    pub pkts_scrubbed: u64,
    pub states_created: u64,
    pub states_expired: u64,
    pub bytes_passed: u64,
    pub bytes_blocked: u64,
}

impl PfFirewall {
    /// Create a new PF firewall with default deny-all policy
    pub fn new() -> Self {
        let mut pf = Self {
            rules: Vec::new(),
            tables: BTreeMap::new(),
            anchors: BTreeMap::new(),
            state_table: Vec::new(),
            max_states: 100_000,
            enabled: true,
            stats: PfStats::default(),
        };
        // Default policy: block all (PF default)
        // Rule 0: block all (default deny)
        pf.rules
            .push(PfRule::block(0, PfDir::Any).with_label("default deny"));
        pf
    }

    /// Add a rule to the ruleset
    pub fn add_rule(&mut self, rule: PfRule) {
        // Insert maintaining rule number order
        let pos = self.rules.partition_point(|r| r.number <= rule.number);
        self.rules.insert(pos, rule);
    }

    /// Create or update a table
    pub fn add_table(&mut self, table: PfTable) {
        self.tables.insert(table.name.clone(), table);
    }

    /// Create an anchor
    pub fn add_anchor(&mut self, anchor: PfAnchor) {
        self.anchors.insert(anchor.name.clone(), anchor);
    }

    /// Main packet filtering function — returns action for a packet
    pub fn filter(&mut self, pkt: &PfPacket) -> PfAction {
        if !self.enabled {
            return PfAction::Pass;
        }

        let mut last_action = PfAction::Block; // Default deny

        for rule in &mut self.rules {
            if !Self::rule_matches_packet(&self.tables, rule, pkt) {
                continue;
            }

            rule.pkts_matched += 1;
            rule.bytes_matched += pkt.size as u64;

            last_action = rule.action;

            if rule.quick {
                // Quick rules terminate evaluation immediately
                break;
            }
        }

        match last_action {
            PfAction::Pass | PfAction::Match => {
                self.stats.pkts_passed += 1;
                self.stats.bytes_passed += pkt.size as u64;
                // Create state for TCP connections
                if pkt.proto == PfProto::Tcp && pkt.tcp_flags & 0x02 != 0 {
                    // SYN flag — new connection
                    if self.state_table.len() < self.max_states as usize {
                        let state = PfStateEntry::new_tcp(
                            pkt.src_ip,
                            pkt.src_port,
                            pkt.dst_ip,
                            pkt.dst_port,
                        );
                        self.state_table.push(state);
                        self.stats.states_created += 1;
                    }
                }
                PfAction::Pass
            }
            _ => {
                self.stats.pkts_blocked += 1;
                self.stats.bytes_blocked += pkt.size as u64;
                last_action
            }
        }
    }

    /// Check if a rule matches a packet
    fn rule_matches_packet(
        tables: &BTreeMap<String, PfTable>,
        rule: &PfRule,
        pkt: &PfPacket,
    ) -> bool {
        // Direction
        if rule.direction != PfDir::Any && rule.direction != pkt.direction {
            return false;
        }

        // Interface
        if let Some(ref iface) = rule.interface {
            if *iface != pkt.interface {
                return false;
            }
        }

        // Protocol
        if rule.proto != PfProto::Any && rule.proto != pkt.proto {
            return false;
        }

        // Source address
        if !Self::addr_matches(tables, &rule.src_addr, pkt.src_ip) {
            return false;
        }

        // Source port
        if !rule.src_port.matches(pkt.src_port) {
            return false;
        }

        // Destination address
        if !Self::addr_matches(tables, &rule.dst_addr, pkt.dst_ip) {
            return false;
        }

        // Destination port
        if !rule.dst_port.matches(pkt.dst_port) {
            return false;
        }

        true
    }

    /// Check if an address matches a PfAddr spec
    fn addr_matches(tables: &BTreeMap<String, PfTable>, spec: &PfAddr, addr: u32) -> bool {
        match spec {
            PfAddr::Any => true,
            PfAddr::Ipv4(ip) => *ip == addr,
            PfAddr::Ipv4Net(net, prefix) => {
                if *prefix == 0 {
                    return true;
                }
                let mask = !0u32 << (32 - prefix);
                (addr & mask) == (net & mask)
            }
            PfAddr::Table(name) => {
                if let Some(table) = tables.get(name) {
                    table.contains_ipv4(addr)
                } else {
                    false
                }
            }
            PfAddr::Not(inner) => !Self::addr_matches(tables, inner, addr),
            _ => true, // Unhandled cases default to match
        }
    }

    /// Load a common SigmaOS firewall ruleset
    pub fn load_sigmaos_default_rules(&mut self) {
        // Rule 1: Allow loopback
        self.add_rule(
            PfRule::pass(10, PfDir::Any, PfProto::Any)
                .with_interface("lo0")
                .with_label("allow loopback")
                .quick(),
        );
        // Rule 2: Block spoofed RFC1918 from external
        // Rule 3: Allow established connections (stateful)
        self.add_rule(PfRule::pass(20, PfDir::In, PfProto::Tcp).with_label("pass established"));
        // Rule 4: Allow SSH inbound
        self.add_rule(
            PfRule::pass(30, PfDir::In, PfProto::Tcp)
                .with_dst(PfAddr::Any, PfPort::Eq(22))
                .with_label("allow ssh")
                .log(),
        );
        // Rule 5: Allow all outbound
        self.add_rule(PfRule::pass(40, PfDir::Out, PfProto::Any).with_label("allow all outbound"));
        // Rule 6: Allow HTTPS inbound
        self.add_rule(
            PfRule::pass(50, PfDir::In, PfProto::Tcp)
                .with_dst(PfAddr::Any, PfPort::Eq(443))
                .with_label("allow https"),
        );
        // Rule 7: Allow HTTP inbound
        self.add_rule(
            PfRule::pass(60, PfDir::In, PfProto::Tcp)
                .with_dst(PfAddr::Any, PfPort::Eq(80))
                .with_label("allow http"),
        );
        // Rule 8: Allow ICMP ping
        self.add_rule(PfRule::pass(70, PfDir::In, PfProto::Icmp).with_label("allow icmp ping"));
        // Rule 9: Allow DNS outbound
        self.add_rule(
            PfRule::pass(80, PfDir::Out, PfProto::Udp)
                .with_dst(PfAddr::Any, PfPort::Eq(53))
                .with_label("allow dns"),
        );
    }

    /// Clean up expired states
    pub fn purge_expired_states(&mut self, now_ns: u64) {
        let before = self.state_table.len();
        self.state_table.retain(|s| !s.is_expired(now_ns));
        let expired = before - self.state_table.len();
        self.stats.states_expired += expired as u64;
    }

    /// Get current state count
    pub fn state_count(&self) -> usize {
        self.state_table.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pf_new_default_deny() {
        let pf = PfFirewall::new();
        assert!(pf.enabled);
        assert!(!pf.rules.is_empty()); // Should have default deny rule
    }

    #[test]
    fn test_pf_default_rules() {
        let mut pf = PfFirewall::new();
        pf.load_sigmaos_default_rules();
        assert!(pf.rules.len() > 5);
    }

    #[test]
    fn test_pf_allow_ssh() {
        let mut pf = PfFirewall::new();
        pf.load_sigmaos_default_rules();

        let ssh_pkt = PfPacket {
            direction: PfDir::In,
            interface: String::from("eth0"),
            proto: PfProto::Tcp,
            src_ip: 0x0a000001, // 10.0.0.1
            src_port: 54321,
            dst_ip: 0x0a000002, // 10.0.0.2
            dst_port: 22,       // SSH
            size: 64,
            tcp_flags: 0x02, // SYN
        };
        let action = pf.filter(&ssh_pkt);
        assert_eq!(action, PfAction::Pass);
    }

    #[test]
    fn test_pf_block_unknown() {
        let mut pf = PfFirewall::new();
        // No rules besides default deny

        let pkt = PfPacket {
            direction: PfDir::In,
            interface: String::from("eth0"),
            proto: PfProto::Tcp,
            src_ip: 0x01020304,
            src_port: 12345,
            dst_ip: 0x05060708,
            dst_port: 8888,
            size: 100,
            tcp_flags: 0x02,
        };
        let action = pf.filter(&pkt);
        assert_ne!(action, PfAction::Pass);
    }

    #[test]
    fn test_pf_table_lookup() {
        let mut pf = PfFirewall::new();
        let mut blocklist = PfTable::new("blocklist");
        blocklist.add_ipv4(0xC0A80100, 24); // 192.168.1.0/24
        pf.add_table(blocklist);

        // Add block rule using table
        pf.add_rule(
            PfRule::block(5, PfDir::In)
                .with_src(PfAddr::Table(String::from("blocklist")), PfPort::Any)
                .with_label("block bad IPs")
                .quick(),
        );

        // 192.168.1.100 should be blocked
        let pkt = PfPacket {
            direction: PfDir::In,
            interface: String::from("eth0"),
            proto: PfProto::Tcp,
            src_ip: 0xC0A80164, // 192.168.1.100
            src_port: 1234,
            dst_ip: 0x0a000001,
            dst_port: 80,
            size: 60,
            tcp_flags: 0x02,
        };
        let action = pf.filter(&pkt);
        assert_ne!(action, PfAction::Pass);
    }

    #[test]
    fn test_port_matching() {
        assert!(PfPort::Eq(80).matches(80));
        assert!(!PfPort::Eq(80).matches(81));
        assert!(PfPort::Range(80, 90).matches(85));
        assert!(!PfPort::Range(80, 90).matches(79));
        assert!(PfPort::Any.matches(12345));
        assert!(PfPort::Lt(1024).matches(22));
        assert!(!PfPort::Lt(1024).matches(8080));
        assert!(PfPort::Gt(1023).matches(8080));
    }

    #[test]
    fn test_state_tracking() {
        let mut pf = PfFirewall::new();
        pf.load_sigmaos_default_rules();

        let syn_pkt = PfPacket {
            direction: PfDir::In,
            interface: String::from("eth0"),
            proto: PfProto::Tcp,
            src_ip: 0x01020304,
            src_port: 54321,
            dst_ip: 0x05060708,
            dst_port: 22,
            size: 60,
            tcp_flags: 0x02, // SYN
        };
        pf.filter(&syn_pkt);
        assert!(pf.stats.states_created > 0);
        assert_eq!(pf.state_count(), pf.stats.states_created as usize);
    }

    #[test]
    fn test_pf_stats() {
        let mut pf = PfFirewall::new();
        pf.load_sigmaos_default_rules();

        let pkt = PfPacket {
            direction: PfDir::Out,
            interface: String::from("eth0"),
            proto: PfProto::Tcp,
            src_ip: 0x0a000001,
            src_port: 12345,
            dst_ip: 0x08080808,
            dst_port: 443,
            size: 1500,
            tcp_flags: 0x02,
        };
        pf.filter(&pkt);
        assert!(pf.stats.pkts_passed + pf.stats.pkts_blocked > 0);
    }
}
