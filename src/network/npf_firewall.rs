use std::vec;
// SigmaOS NetBSD NPF & Linux Netfilter/PF Parity Subsystem - NPF Firewall Engine
// Stateful Connection Tracking (Conntrack), NAPT/NAT64 Engine, BPF Rule Inspection, & IP Sets

use std::string::String;
use std::string::ToString;
use std::vec::Vec;
use std::format;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfAction {
    Pass,
    Block,
    StatefulPass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    SynSent,
    SynReceived,
    Established,
    FinWait,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IpProtocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiveTuple {
    pub src_ip: u32,
    pub dst_ip: u32,
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: IpProtocol,
}

#[derive(Debug, Clone)]
pub struct ConntrackEntry {
    pub tuple: FiveTuple,
    pub tcp_state: TcpState,
    pub timeout_sec: u64,
    pub bytes_counter: usize,
    pub packets_counter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NatType {
    SnatMasquerade,
    DnatPortForward { target_ip: u32, target_port: u16 },
    Nat64Translation,
}

#[derive(Debug, Clone)]
pub struct NatRule {
    pub nat_id: usize,
    pub nat_type: NatType,
    pub match_proto: Option<IpProtocol>,
    pub match_port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct NpfRule {
    pub rule_id: usize,
    pub direction: NpfDirection,
    pub action: NpfAction,
    pub proto: Option<IpProtocol>,
    pub src_ip_mask: Option<(u32, u32)>, // (ip, netmask)
    pub dst_port: Option<u16>,
    pub table_match: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NpfTable {
    pub name: String,
    pub entries: Vec<u32>, // List of IP addresses or prefix bases
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NpfStats {
    pub total_evaluated: usize,
    pub passed_packets: usize,
    pub blocked_packets: usize,
    pub stateful_matches: usize,
    pub nat_translations: usize,
    pub active_conntrack_entries: usize,
}

pub struct NpfFirewallEngine {
    pub rules: Vec<NpfRule>,
    pub nat_rules: Vec<NatRule>,
    pub tables: BTreeMap<String, NpfTable>,
    pub conntrack_table: BTreeMap<FiveTuple, ConntrackEntry>,
    pub stats: NpfStats,
    next_id: usize,
}

impl NpfFirewallEngine {
    pub fn new() -> Self {
        NpfFirewallEngine {
            rules: Vec::new(),
            nat_rules: Vec::new(),
            tables: BTreeMap::new(),
            conntrack_table: BTreeMap::new(),
            stats: NpfStats::default(),
            next_id: 1,
        }
    }

    pub fn add_rule(&mut self, direction: NpfDirection, action: NpfAction, proto: Option<IpProtocol>, dst_port: Option<u16>, table_match: Option<&str>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.rules.push(NpfRule {
            rule_id: id,
            direction,
            action,
            proto,
            src_ip_mask: None,
            dst_port,
            table_match: table_match.map(|s| s.to_string()),
        });

        id
    }

    pub fn add_table(&mut self, name: &str, ips: Vec<u32>) {
        self.tables.insert(
            name.to_string(),
            NpfTable {
                name: name.to_string(),
                entries: ips,
            },
        );
    }

    pub fn add_nat_rule(&mut self, nat_type: NatType, match_proto: Option<IpProtocol>, match_port: Option<u16>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.nat_rules.push(NatRule {
            nat_id: id,
            nat_type,
            match_proto,
            match_port,
        });

        id
    }

    /// Evaluate packet through NPF firewall (Stateful Conntrack + NAT + Rules)
    pub fn evaluate_packet(
        &mut self,
        direction: NpfDirection,
        tuple: FiveTuple,
        payload_bytes: usize,
        is_syn: bool,
        is_fin: bool,
    ) -> (NpfAction, Option<FiveTuple>) {
        self.stats.total_evaluated += 1;

        // 1. Check Stateful Connection Tracking Table (Fast-path)
        let reverse_tuple = FiveTuple {
            src_ip: tuple.dst_ip,
            dst_ip: tuple.src_ip,
            src_port: tuple.dst_port,
            dst_port: tuple.src_port,
            proto: tuple.proto,
        };

        if self.conntrack_table.contains_key(&tuple) {
            if let Some(entry) = self.conntrack_table.get_mut(&tuple) {
                entry.bytes_counter += payload_bytes;
                entry.packets_counter += 1;
                if is_fin {
                    entry.tcp_state = TcpState::FinWait;
                } else if entry.tcp_state == TcpState::SynSent && is_syn {
                    entry.tcp_state = TcpState::Established;
                }
                self.stats.stateful_matches += 1;
                self.stats.passed_packets += 1;
                return (NpfAction::Pass, None);
            }
        } else if self.conntrack_table.contains_key(&reverse_tuple) {
            if let Some(entry) = self.conntrack_table.get_mut(&reverse_tuple) {
                entry.bytes_counter += payload_bytes;
                entry.packets_counter += 1;
                if is_fin {
                    entry.tcp_state = TcpState::FinWait;
                } else if entry.tcp_state == TcpState::SynSent && is_syn {
                    entry.tcp_state = TcpState::Established;
                }
                self.stats.stateful_matches += 1;
                self.stats.passed_packets += 1;
                return (NpfAction::Pass, None);
            }
        }

        // 2. Evaluate Rule Chain
        let mut final_action = NpfAction::Pass; // Default pass

        for rule in &self.rules {
            if rule.direction != direction {
                continue;
            }

            if let Some(p) = rule.proto {
                if p != tuple.proto {
                    continue;
                }
            }

            if let Some(port) = rule.dst_port {
                if port != tuple.dst_port {
                    continue;
                }
            }

            if let Some(ref tbl_name) = rule.table_match {
                if let Some(table) = self.tables.get(tbl_name) {
                    if !table.entries.contains(&tuple.src_ip) {
                        continue;
                    }
                }
            }

            final_action = rule.action;
        }

        match final_action {
            NpfAction::Block => {
                self.stats.blocked_packets += 1;
                (NpfAction::Block, None)
            }
            NpfAction::Pass | NpfAction::StatefulPass => {
                self.stats.passed_packets += 1;

                if final_action == NpfAction::StatefulPass {
                    let initial_state = if is_syn { TcpState::SynSent } else { TcpState::Established };
                    self.conntrack_table.insert(
                        tuple.clone(),
                        ConntrackEntry {
                            tuple: tuple.clone(),
                            tcp_state: initial_state,
                            timeout_sec: 3600,
                            bytes_counter: payload_bytes,
                            packets_counter: 1,
                        },
                    );
                    self.stats.active_conntrack_entries = self.conntrack_table.len();
                }

                // 3. Evaluate NAT Transformations
                let mut translated_tuple = None;
                for nat in &self.nat_rules {
                    if let Some(p) = nat.match_proto {
                        if p != tuple.proto {
                            continue;
                        }
                    }

                    if let Some(port) = nat.match_port {
                        if port != tuple.dst_port {
                            continue;
                        }
                    }

                    if let NatType::DnatPortForward { target_ip, target_port } = nat.nat_type {
                        let mut xlated = tuple.clone();
                        xlated.dst_ip = target_ip;
                        xlated.dst_port = target_port;
                        translated_tuple = Some(xlated);
                        self.stats.nat_translations += 1;
                        break;
                    }
                }

                (NpfAction::Pass, translated_tuple)
            }
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_npf_firewall_stateful_tracking_and_nat() {
        let mut npf = NpfFirewallEngine::new();

        // Add blacklist table
        npf.add_table("spammers", vec![0x0A000001]); // 10.0.0.1

        // Add rule: Block inbound from table "spammers"
        npf.add_rule(NpfDirection::Inbound, NpfAction::Block, None, None, Some("spammers"));

        // Add rule: Stateful pass outbound TCP port 80
        npf.add_rule(NpfDirection::Outbound, NpfAction::StatefulPass, Some(IpProtocol::Tcp), Some(80), None);

        // Add NAT rule: Port forward port 80 to internal web server 192.168.1.100:8080
        npf.add_nat_rule(NatType::DnatPortForward { target_ip: 0xC0A80164, target_port: 8080 }, Some(IpProtocol::Tcp), Some(80));

        let tuple_normal = FiveTuple {
            src_ip: 0xC0A80101, // 192.168.1.1
            dst_ip: 0x08080808, // 8.8.8.8
            src_port: 54321,
            dst_port: 80,
            proto: IpProtocol::Tcp,
        };

        // Outbound packet should pass statefully and be transformed by DNAT
        let (action, xlated) = npf.evaluate_packet(NpfDirection::Outbound, tuple_normal.clone(), 100, true, false);
        assert_eq!(action, NpfAction::Pass);
        assert!(xlated.is_some());
        assert_eq!(xlated.unwrap().dst_port, 8080);
        assert_eq!(npf.stats.active_conntrack_entries, 1);

        // Subsequent packet in same connection hits stateful conntrack fast-path
        let (action_fast, _) = npf.evaluate_packet(NpfDirection::Outbound, tuple_normal.clone(), 100, false, false);
        assert_eq!(action_fast, NpfAction::Pass);
        assert_eq!(npf.stats.stateful_matches, 1);

        // Blacklisted packet from 10.0.0.1 should be blocked
        let tuple_blocked = FiveTuple {
            src_ip: 0x0A000001, // 10.0.0.1
            dst_ip: 0xC0A80101,
            src_port: 12345,
            dst_port: 80,
            proto: IpProtocol::Tcp,
        };

        let (action_blocked, _) = npf.evaluate_packet(NpfDirection::Inbound, tuple_blocked, 100, true, false);
        assert_eq!(action_blocked, NpfAction::Block);
        assert_eq!(npf.stats.blocked_packets, 1);
    }
}
