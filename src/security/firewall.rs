
use std::vec::Vec;
use std::collections::BTreeMap;
use std::string::String;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Protocol { Tcp, Udp, Icmp, Any }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action { Accept, Drop, Reject, Log, NatSnat(u32), NatDnat(u32) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState { New, Established, Related, Invalid }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ipv4Address(pub u32);

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Ubuntu/UFW inspired Application Profile
#[derive(Debug, Clone)]
pub struct UfwAppProfile {
    pub title: String,
    pub description: String,
    pub ports: Vec<(u16, Protocol)>,
}

impl UfwAppProfile {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: String::from(title),
            description: String::from(description),
            ports: Vec::new(),
        }
    }

    pub fn add_port(&mut self, port: u16, protocol: Protocol) {
        self.ports.push((port, protocol));
    }
}

/// Linux iptables / nftables inspired NAT type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NatType {
    Snat { new_src_ip: u32 },
    Dnat { new_dst_ip: u32, new_dst_port: u16 },
    Masquerade { outgoing_ip: u32 },
}

pub struct NatRule {
    pub match_criteria: MatchCriteria,
    pub nat_type: NatType,
}

/// Token-Bucket Quality of Service (QoS) Traffic Shaper
#[derive(Debug, Clone)]
pub struct QosTrafficShaper {
    pub rate_bytes_per_sec: u64,
    pub burst_capacity_bytes: u64,
    pub available_tokens: u64,
    pub last_refill_timestamp: u64,
}

impl QosTrafficShaper {
    pub fn new(rate_bytes_per_sec: u64, burst_capacity_bytes: u64) -> Self {
        Self {
            rate_bytes_per_sec,
            burst_capacity_bytes,
            available_tokens: burst_capacity_bytes,
            last_refill_timestamp: 0,
        }
    }

    pub fn shape_packet_bandwidth(&mut self, packet_size_bytes: u64, now_secs: u64) -> bool {
        let elapsed = now_secs.saturating_sub(self.last_refill_timestamp);
        let refill_amount = elapsed.saturating_mul(self.rate_bytes_per_sec);
        self.available_tokens = (self.available_tokens.saturating_add(refill_amount)).min(self.burst_capacity_bytes);
        self.last_refill_timestamp = now_secs;

        if self.available_tokens >= packet_size_bytes {
            self.available_tokens -= packet_size_bytes;
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
    pub nat_rules: Vec<NatRule>,
    pub default_action: Action,
    pub conntrack: BTreeMap<ConnectionTuple, ConnectionEntry>,
    pub rate_limiter: RateLimiter,
    pub qos_shaper: Option<QosTrafficShaper>,
}

impl Firewall {
    pub fn new() -> Self {
        Self {
            input_rules: Vec::new(),
            output_rules: Vec::new(),
            forward_rules: Vec::new(),
            nat_rules: Vec::new(),
            default_action: Action::Drop,
            conntrack: BTreeMap::new(),
            rate_limiter: RateLimiter { tokens: 100, capacity: 100, fill_rate: 10, last_update: 0 },
            qos_shaper: Some(QosTrafficShaper::new(10_000_000, 1_000_000)), // 10MB/s rate, 1MB burst
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

    pub fn enable_app_profile(&mut self, profile: &UfwAppProfile, action: Action) {
        for &(port, protocol) in &profile.ports {
            let rule = Rule {
                criteria: MatchCriteria {
                    source_ip: None,
                    dest_ip: None,
                    source_port: None,
                    dest_port: Some(port),
                    protocol,
                    state: None,
                },
                action,
            };
            self.input_rules.push(rule);
        }
    }

    pub fn add_nat_rule(&mut self, rule: NatRule) {
        self.nat_rules.push(rule);
    }

    pub fn translate_nat(&self, packet: &PacketInfo, state: ConnectionState) -> Option<(PacketInfo, NatType)> {
        for nat_rule in &self.nat_rules {
            if nat_rule.match_criteria.matches(packet, state) {
                let mut translated = packet.clone();
                match &nat_rule.nat_type {
                    NatType::Snat { new_src_ip } => {
                        translated.source_ip = Ipv4Address(*new_src_ip);
                    }
                    NatType::Dnat { new_dst_ip, new_dst_port } => {
                        translated.dest_ip = Ipv4Address(*new_dst_ip);
                        translated.dest_port = *new_dst_port;
                    }
                    NatType::Masquerade { outgoing_ip } => {
                        translated.source_ip = Ipv4Address(*outgoing_ip);
                    }
                }
                return Some((translated, nat_rule.nat_type.clone()));
            }
        }
        None
    }

    pub fn cleanup_stale_connections(&mut self, now: u64, timeout_secs: u64) -> usize {
        let mut stale_tuples = Vec::new();
        for (tuple, entry) in self.conntrack.iter() {
            if now.saturating_sub(entry.last_seen) > timeout_secs {
                stale_tuples.push(tuple.clone());
            }
        }
        let count = stale_tuples.len();
        for tuple in stale_tuples {
            self.conntrack.remove(&tuple);
        }
        count
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

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ufw_application_profile() {
        let mut fw = Firewall::new();
        let mut web_server = UfwAppProfile::new("Nginx Full", "Web Server (HTTP + HTTPS)");
        web_server.add_port(80, Protocol::Tcp);
        web_server.add_port(443, Protocol::Tcp);

        fw.enable_app_profile(&web_server, Action::Accept);
        assert_eq!(fw.input_rules.len(), 2);
    }

    #[test]
    fn test_nat_port_forwarding() {
        let mut fw = Firewall::new();
        fw.add_nat_rule(NatRule {
            match_criteria: MatchCriteria {
                source_ip: None,
                dest_ip: Some(Ipv4Address(0x0A000001)), // 10.0.0.1
                source_port: None,
                dest_port: Some(8080),
                protocol: Protocol::Tcp,
                state: None,
            },
            nat_type: NatType::Dnat {
                new_dst_ip: 0xC0A80164, // 192.168.1.100
                new_dst_port: 80,
            },
        });

        let pkt = PacketInfo {
            source_ip: Ipv4Address(0x01020304),
            dest_ip: Ipv4Address(0x0A000001),
            source_port: 12345,
            dest_port: 8080,
            protocol: Protocol::Tcp,
        };

        let translated = fw.translate_nat(&pkt, ConnectionState::New);
        assert!(translated.is_some());
        let (new_pkt, nat_type) = translated.unwrap();
        assert_eq!(new_pkt.dest_ip, Ipv4Address(0xC0A80164));
        assert_eq!(new_pkt.dest_port, 80);
        assert_eq!(nat_type, NatType::Dnat { new_dst_ip: 0xC0A80164, new_dst_port: 80 });
    }

    #[test]
    fn test_qos_traffic_shaper() {
        let mut shaper = QosTrafficShaper::new(1000, 5000);
        assert!(shaper.shape_packet_bandwidth(1500, 0));
        assert_eq!(shaper.available_tokens, 3500);

        // Consume remaining burst
        assert!(shaper.shape_packet_bandwidth(3500, 0));
        assert_eq!(shaper.available_tokens, 0);

        // Exceeds tokens without time passing
        assert!(!shaper.shape_packet_bandwidth(100, 0));

        // Refill 1 second later (1000 bytes)
        assert!(shaper.shape_packet_bandwidth(800, 1));
        assert_eq!(shaper.available_tokens, 200);
    }
}
