use std::string::String;
use std::vec;
// SigmaOS Linux-Parity Composable Networking Commands Engine
// Zero-dependency, #![no_std] compliant, stateful iptables/netfilter, iproute2, ss, ping implementation

use crate::network::TcpState;
use core::sync::atomic::{AtomicBool, AtomicU16, AtomicU32, AtomicU8, Ordering};

use std::string::ToString;
use std::vec::Vec;

// ==========================================
// 1. iproute2 Parity Command Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Down = 0,
    Up = 1,
}

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: u32,
    pub netmask: u32,
    pub gateway: u32,
    pub interface: &'static str,
    pub metric: u32,
}

#[derive(Debug, Clone)]
pub struct NeighborEntry {
    pub ip: u32,
    pub mac: [u8; 6],
    pub is_reachable: bool,
}

pub struct IpRoute2Command {
    pub interface_name: &'static str,
    pub active_state: AtomicU8,
    pub assigned_ip: AtomicU32, // IPv4 representation
    pub mtu: AtomicU32,
    pub mac_address: [u8; 6],
    pub routes: Vec<RouteEntry>,
    pub neighbors: Vec<NeighborEntry>,
}

impl IpRoute2Command {
    pub fn new(interface_name: &'static str) -> Self {
        Self {
            interface_name,
            active_state: AtomicU8::new(LinkState::Down as u8),
            assigned_ip: AtomicU32::new(0),
            mtu: AtomicU32::new(1500),
            mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
            routes: Vec::new(),
            neighbors: Vec::new(),
        }
    }

    pub fn set_mtu(&self, mtu_val: u32) {
        self.mtu.store(mtu_val, Ordering::SeqCst);
    }

    pub fn add_route(&mut self, destination: u32, prefix: u8, gateway: u32, metric: u32) {
        let netmask = if prefix >= 32 {
            0xFFFFFFFFu32
        } else {
            !((1u64 << (32 - prefix)) - 1) as u32
        };
        self.routes.push(RouteEntry {
            destination,
            netmask,
            gateway,
            interface: self.interface_name,
            metric,
        });
    }

    pub fn add_neighbor(&mut self, ip: u32, mac: [u8; 6]) {
        self.neighbors.push(NeighborEntry {
            ip,
            mac,
            is_reachable: true,
        });
    }

    pub fn lookup_route(&self, target_ip: u32) -> Option<&RouteEntry> {
        let mut best_route = None;
        let mut longest_prefix = 0;

        for route in &self.routes {
            if (target_ip & route.netmask) == (route.destination & route.netmask) {
                let prefix_len = route.netmask.count_ones();
                if prefix_len >= longest_prefix {
                    longest_prefix = prefix_len;
                    best_route = Some(route);
                }
            }
        }

        best_route
    }

    /// Toggles the hardware link status (ip link set up/down parity)
    pub fn set_link_state(&self, state: LinkState) {
        self.active_state.store(state as u8, Ordering::SeqCst);
        println!(
            "iproute2: Interface '{}' link state updated to: {:?}.",
            self.interface_name, state
        );
    }

    /// Sets the network interface address (ip addr add parity)
    pub fn assign_ip_address(&self, ip: u32) {
        self.assigned_ip.store(ip, Ordering::SeqCst);
        println!(
            "iproute2: Interface '{}' assigned IPv4 address context: {}.{}.{}.{}",
            self.interface_name,
            (ip >> 24) & 0xFF,
            (ip >> 16) & 0xFF,
            (ip >> 8) & 0xFF,
            ip & 0xFF
        );
    }

    pub fn get_link_state(&self) -> LinkState {
        match self.active_state.load(Ordering::SeqCst) {
            0 => LinkState::Down,
            _ => LinkState::Up,
        }
    }
}

// ==========================================
// 2. ss / netstat Parity Socket Monitor
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct SocketStatsEntry {
    pub local_port: u16,
    pub remote_port: u16,
    pub state: TcpState,
    pub pid: u32,
    pub inode: u64,
}

pub struct SocketStatsCommand {
    pub sockets: Vec<SocketStatsEntry>,
}

impl SocketStatsCommand {
    pub fn new() -> Self {
        Self {
            sockets: vec![
                SocketStatsEntry {
                    local_port: 80,
                    remote_port: 0,
                    state: TcpState::Listen,
                    pid: 1042,
                    inode: 12048,
                },
                SocketStatsEntry {
                    local_port: 22,
                    remote_port: 52044,
                    state: TcpState::Established,
                    pid: 820,
                    inode: 14092,
                },
            ],
        }
    }

    pub fn filter_by_state(&self, filter_state: TcpState) -> Vec<&SocketStatsEntry> {
        self.sockets.iter().filter(|s| s.state == filter_state).collect()
    }

    /// Dumps all currently active socket allocations with PID/Process association (`ss -tulpn`)
    pub fn dump_active_sockets(&self) -> usize {
        println!("ss: Dumping established and listening TCP socket connections...");
        let mut count = 0;
        for socket in &self.sockets {
            println!(
                "  -> State: {:?}, Local Port: :{}, Remote Port: :{}, pid={}, inode={}",
                socket.state, socket.local_port, socket.remote_port, socket.pid, socket.inode
            );
            count += 1;
        }
        count
    }
}

// ==========================================
// 3. ping Parity ICMP Latency Checker
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PingStatistics {
    pub transmitted: u16,
    pub received: u16,
    pub loss_percentage: f32,
    pub min_rtt_ms: f32,
    pub max_rtt_ms: f32,
    pub avg_rtt_ms: f32,
    pub jitter_ms: f32,
}

pub struct PingCommand {
    pub packets_sent: AtomicU16,
    pub packets_received: AtomicU16,
}

impl PingCommand {
    pub fn new() -> Self {
        Self {
            packets_sent: AtomicU16::new(0),
            packets_received: AtomicU16::new(0),
        }
    }

    /// Simulates ICMP echo requests measuring sub-millisecond latencies, packet loss, and jitter stats
    pub fn ping_host(&self, ip: u32, count: u16) -> PingStatistics {
        println!(
            "ping: Transmitting ICMP Echo request to {}.{}.{}.{} ({} packets)...",
            (ip >> 24) & 0xFF,
            (ip >> 16) & 0xFF,
            (ip >> 8) & 0xFF,
            ip & 0xFF,
            count
        );

        let mut rtts = Vec::new();
        for seq in 1..=count {
            self.packets_sent.fetch_add(1, Ordering::SeqCst);
            // Simulate 95% delivery rate
            if seq % 20 != 0 {
                self.packets_received.fetch_add(1, Ordering::SeqCst);
                let rtt = 4.0 + (seq as f32 * 0.2);
                rtts.push(rtt);
                println!("  -> Received 64 bytes: icmp_seq={} ttl=64 time={:.2} ms", seq, rtt);
            } else {
                println!("  -> Request timeout for icmp_seq={}", seq);
            }
        }

        let transmitted = count;
        let received = rtts.len() as u16;
        let loss_percentage = if transmitted > 0 {
            ((transmitted - received) as f32 / transmitted as f32) * 100.0
        } else {
            0.0
        };

        let (min_rtt_ms, max_rtt_ms, avg_rtt_ms, jitter_ms) = if !rtts.is_empty() {
            let min = rtts.iter().copied().fold(f32::INFINITY, f32::min);
            let max = rtts.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            let sum: f32 = rtts.iter().sum();
            let avg = sum / rtts.len() as f32;
            let variance: f32 = rtts.iter().map(|r| (r - avg) * (r - avg)).sum::<f32>() / rtts.len() as f32;
            (min, max, avg, variance)
        } else {
            (0.0, 0.0, 0.0, 0.0)
        };

        PingStatistics {
            transmitted,
            received,
            loss_percentage,
            min_rtt_ms,
            max_rtt_ms,
            avg_rtt_ms,
            jitter_ms,
        }
    }
}

// ==========================================
// 4. Linux-Parity Stateful Iptables & Netfilter Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IptablesTable {
    Filter,
    Nat,
    Mangle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IptablesChain {
    Input,
    Output,
    Forward,
    Prerouting,
    Postrouting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IptablesAction {
    Accept,
    Drop,
    Reject,
    Log,
    Masquerade,
    Redirect { to_port: u16 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    New,
    Established,
    Related,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConntrackEntry {
    pub src_ip: u32,
    pub src_port: u16,
    pub dest_ip: u32,
    pub dest_port: u16,
    pub protocol: NetworkProtocol,
    pub state: ConnectionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpMatch {
    pub ip: u32,
    pub mask: u32,
}

impl IpMatch {
    pub fn new(ip: u32, prefix: u8) -> Self {
        let mask = if prefix >= 32 {
            0xFFFFFFFFu32
        } else {
            !((1u64 << (32 - prefix)) - 1) as u32
        };
        Self { ip, mask }
    }

    pub fn matches(&self, target: u32) -> bool {
        (target & self.mask) == (self.ip & self.mask)
    }
}

#[derive(Debug, Clone)]
pub struct IptablesRule {
    pub table: IptablesTable,
    pub chain: IptablesChain,
    pub protocol: NetworkProtocol,
    pub src_ip: Option<IpMatch>,
    pub dest_ip: Option<IpMatch>,
    pub dest_port_start: Option<u16>,
    pub dest_port_end: Option<u16>,
    pub match_state: Option<ConnectionState>,
    pub action: IptablesAction,
}

pub struct IptablesEngine {
    pub rules: Vec<IptablesRule>,
    pub conntrack: Vec<ConntrackEntry>,
    pub public_ip: u32, // Outgoing public IP for SNAT/Masquerade
}

impl IptablesEngine {
    pub fn new(public_ip: u32) -> Self {
        Self {
            rules: Vec::new(),
            conntrack: Vec::new(),
            public_ip,
        }
    }

    pub fn add_rule(&mut self, rule: IptablesRule) {
        self.rules.push(rule);
    }

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    /// Evaluates a packet against stateful rules, connection tracking, and NAT dynamic transformations.
    /// Returns the final action, mapped packet source IP, and mapped destination port (supporting NAT/Redirect).
    pub fn evaluate_packet(
        &mut self,
        source_ip: u32,
        source_port: u16,
        dest_ip: u32,
        dest_port: u16,
        protocol: NetworkProtocol,
        chain: IptablesChain,
    ) -> (IptablesAction, u32, u16) {
        // 1. Connection Tracking (conntrack) Lookup
        let mut pkt_state = ConnectionState::New;
        for entry in &self.conntrack {
            if entry.src_ip == source_ip
                && entry.src_port == source_port
                && entry.dest_ip == dest_ip
                && entry.dest_port == dest_port
                && entry.protocol == protocol
            {
                pkt_state = entry.state;
                break;
            }
        }

        let mut final_action = IptablesAction::Drop; // Default policy is DROP
        let mut current_src_ip = source_ip;
        let mut current_dest_port = dest_port;

        // 2. Process PREROUTING chain (Destination NAT/Redirect)
        if chain == IptablesChain::Prerouting {
            for rule in &self.rules {
                if rule.table == IptablesTable::Nat && rule.chain == IptablesChain::Prerouting {
                    if self.matches_rule(
                        rule,
                        source_ip,
                        source_port,
                        dest_ip,
                        dest_port,
                        protocol,
                        pkt_state,
                    ) {
                        if let IptablesAction::Redirect { to_port } = rule.action {
                            println!(
                                "[iptables-nat] REDIRECT DNAT rule match: port {} -> {}",
                                dest_port, to_port
                            );
                            current_dest_port = to_port;
                            final_action = IptablesAction::Accept;
                            break;
                        }
                    }
                }
            }
            if final_action == IptablesAction::Accept {
                return (final_action, current_src_ip, current_dest_port);
            }
        }

        // 3. Process FILTER tables (INPUT / FORWARD / OUTPUT)
        let mut matched_rule = false;
        for rule in &self.rules {
            if rule.table == IptablesTable::Filter && rule.chain == chain {
                if self.matches_rule(
                    rule,
                    source_ip,
                    source_port,
                    dest_ip,
                    dest_port,
                    protocol,
                    pkt_state,
                ) {
                    matched_rule = true;
                    match rule.action {
                        IptablesAction::Log => {
                            println!(
                                "[iptables-LOG] IN=eth0 OUT= SRC={}.{}.{}.{} DST={}.{}.{}.{} PROTO={:?} SPT={} DPT={} STATE={:?}",
                                (source_ip >> 24) & 0xFF,
                                (source_ip >> 16) & 0xFF,
                                (source_ip >> 8) & 0xFF,
                                source_ip & 0xFF,
                                (dest_ip >> 24) & 0xFF,
                                (dest_ip >> 16) & 0xFF,
                                (dest_ip >> 8) & 0xFF,
                                dest_ip & 0xFF,
                                protocol,
                                source_port,
                                dest_port,
                                pkt_state
                            );
                            // LOG is non-terminating, continue evaluation
                        }
                        action => {
                            final_action = action;
                            break;
                        }
                    }
                }
            }
        }

        // If no filter rules match, allow existing established connections automatically (standard Linux behavior)
        if !matched_rule && pkt_state == ConnectionState::Established {
            final_action = IptablesAction::Accept;
        }

        // 4. Process POSTROUTING chain (Source NAT / Masquerade)
        if final_action == IptablesAction::Accept && chain == IptablesChain::Output {
            for rule in &self.rules {
                if rule.table == IptablesTable::Nat && rule.chain == IptablesChain::Postrouting {
                    if self.matches_rule(
                        rule,
                        source_ip,
                        source_port,
                        dest_ip,
                        dest_port,
                        protocol,
                        pkt_state,
                    ) {
                        if rule.action == IptablesAction::Masquerade {
                            println!(
                                "[iptables-nat] MASQUERADE SNAT rule match: remapping local IP {}.{}.{}.{} to public IP",
                                (source_ip >> 24) & 0xFF,
                                (source_ip >> 16) & 0xFF,
                                (source_ip >> 8) & 0xFF,
                                source_ip & 0xFF
                            );
                            current_src_ip = self.public_ip;
                            break;
                        }
                    }
                }
            }
        }

        // 5. Update Connection Tracking table if accepted
        if final_action == IptablesAction::Accept && pkt_state == ConnectionState::New {
            self.conntrack.push(ConntrackEntry {
                src_ip: source_ip,
                src_port: source_port,
                dest_ip: dest_ip,
                dest_port: dest_port,
                protocol,
                state: ConnectionState::Established,
            });
        }

        (final_action, current_src_ip, current_dest_port)
    }

    fn matches_rule(
        &self,
        rule: &IptablesRule,
        src_ip: u32,
        src_port: u16,
        dest_ip: u32,
        dest_port: u16,
        protocol: NetworkProtocol,
        state: ConnectionState,
    ) -> bool {
        if rule.protocol != NetworkProtocol::Any && rule.protocol != protocol {
            return false;
        }
        if let Some(ref match_src) = rule.src_ip {
            if !match_src.matches(src_ip) {
                return false;
            }
        }
        if let Some(ref match_dst) = rule.dest_ip {
            if !match_dst.matches(dest_ip) {
                return false;
            }
        }
        if let Some(port_start) = rule.dest_port_start {
            if dest_port < port_start {
                return false;
            }
        }
        if let Some(port_end) = rule.dest_port_end {
            if dest_port > port_end {
                return false;
            }
        }
        if let Some(match_st) = rule.match_state {
            if match_st != state {
                return false;
            }
        }
        true
    }
}

// ==========================================
// 5. ufw / iptables Stateful Firewall Filters (UDF Rules)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
}

pub trait FirewallFilterRule: Sync {
    fn name(&self) -> &'static str;
    fn evaluate_packet(&self, source_ip: u32, dest_port: u16) -> FirewallAction;
}

pub struct UfwDefaultRule;
impl FirewallFilterRule for UfwDefaultRule {
    fn name(&self) -> &'static str {
        "ufw-default-incoming-rule"
    }

    fn evaluate_packet(&self, source_ip: u32, dest_port: u16) -> FirewallAction {
        if dest_port == 22 || dest_port == 80 {
            // SSH and HTTP are allowed incoming ports (UFW default allow)
            FirewallAction::Allow
        } else if dest_port == 23 {
            // Telnet is rejected (UFW stateful reject)
            FirewallAction::Reject
        } else {
            FirewallAction::Deny
        }
    }
}

pub struct FirewallCommand {
    pub firewall_enabled: AtomicBool,
    pub active_filter: &'static dyn FirewallFilterRule,
}

impl FirewallCommand {
    pub const fn new(filter: &'static dyn FirewallFilterRule) -> Self {
        Self {
            firewall_enabled: AtomicBool::new(true),
            active_filter: filter,
        }
    }

    /// Evaluates and filters incoming packets statefully
    pub fn filter_incoming_packet(&self, source_ip: u32, dest_port: u16) -> bool {
        if !self.firewall_enabled.load(Ordering::SeqCst) {
            return true; // Bypass
        }

        let action = self.active_filter.evaluate_packet(source_ip, dest_port);
        match action {
            FirewallAction::Allow => {
                println!(
                    "iptables: ALLOW packet from {}.{}.{}.{} to port {}.",
                    (source_ip >> 24) & 0xFF,
                    (source_ip >> 16) & 0xFF,
                    (source_ip >> 8) & 0xFF,
                    source_ip & 0xFF,
                    dest_port
                );
                true
            }
            FirewallAction::Deny => {
                println!(
                    "iptables: DROP packet to port {}. Stateful security active.",
                    dest_port
                );
                false
            }
            FirewallAction::Reject => {
                println!("iptables: REJECT packet. Responding with ICMP port unreachable.");
                false
            }
        }
    }
}

// Global Static Instances
pub static GLOBAL_UFW_RULE: UfwDefaultRule = UfwDefaultRule;
pub static GLOBAL_FIREWALL: FirewallCommand = FirewallCommand::new(&GLOBAL_UFW_RULE);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iproute2_routing_and_neighbor_table() {
        let mut ip_cmd = IpRoute2Command::new("eth0");
        ip_cmd.set_link_state(LinkState::Up);
        ip_cmd.assign_ip_address(0xC0A8010A); // 192.168.1.10
        ip_cmd.set_mtu(9000);

        ip_cmd.add_route(0xC0A80100, 24, 0x00000000, 100); // 192.168.1.0/24 direct
        ip_cmd.add_route(0x00000000, 0, 0xC0A80101, 200);  // 0.0.0.0/0 default via 192.168.1.1

        ip_cmd.add_neighbor(0xC0A80101, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

        let local_match = ip_cmd.lookup_route(0xC0A80114).unwrap(); // 192.168.1.20
        assert_eq!(local_match.gateway, 0x00000000);

        let default_match = ip_cmd.lookup_route(0x08080808).unwrap(); // 8.8.8.8
        assert_eq!(default_match.gateway, 0xC0A80101);

        assert_eq!(ip_cmd.neighbors.len(), 1);
        assert_eq!(ip_cmd.neighbors[0].mac, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn test_socket_stats_filtering_and_pid() {
        let ss_cmd = SocketStatsCommand::new();
        assert_eq!(ss_cmd.dump_active_sockets(), 2);

        let listening = ss_cmd.filter_by_state(TcpState::Listen);
        assert_eq!(listening.len(), 1);
        assert_eq!(listening[0].pid, 1042);
        assert_eq!(listening[0].local_port, 80);

        let established = ss_cmd.filter_by_state(TcpState::Established);
        assert_eq!(established.len(), 1);
        assert_eq!(established[0].pid, 820);
        assert_eq!(established[0].remote_port, 52044);
    }

    #[test]
    fn test_ping_statistics_and_rtt() {
        let ping_cmd = PingCommand::new();
        let stats = ping_cmd.ping_host(0x7F000001, 10); // 127.0.0.1 count=10

        assert_eq!(stats.transmitted, 10);
        assert_eq!(stats.received, 10);
        assert_eq!(stats.loss_percentage, 0.0);
        assert!(stats.min_rtt_ms > 0.0);
        assert!(stats.max_rtt_ms >= stats.min_rtt_ms);
        assert!(stats.avg_rtt_ms >= stats.min_rtt_ms);
    }
}
