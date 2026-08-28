extern crate alloc;
// SigmaOS NetBSD NPF-Inspired Stateful Packet Filter Engine
// Zero-dependency, safe, robust stateful packet filtering and NAT framework
// Inspired by NetBSD's NPF (N-Packet Filter) architecture


use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfFilterAction {
    Pass,
    Block,
    ReturnReset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpfDirection {
    Inbound,
    Outbound,
    Both,
}

#[derive(Debug, Clone)]
pub struct NpfPacket {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8, // 6 = TCP, 17 = UDP, 1 = ICMP
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct NpfStateRule {
    pub src_ip: Option<[u8; 4]>,
    pub dst_ip: Option<[u8; 4]>,
    pub port: Option<u16>,
    pub protocol: Option<u8>,
    pub action: NpfFilterAction,
    pub direction: NpfDirection,
    pub stateful: bool,
}

#[derive(Debug, Clone)]
pub struct NpfStateKey {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: u8,
}

pub struct NpfFirewallEngine {
    pub rules: Vec<NpfStateRule>,
    pub active_states: BTreeMap<[u8; 18], bool>, // Key serialized: src(4)+dst(4)+src_port(2)+dst_port(2)+proto(1) -> valid
    pub nat_translations: BTreeMap<[u8; 6], ([u8; 4], u16)>, // (src_ip, src_port) -> (public_ip, public_port)
    pub public_ip: [u8; 4],
}

impl NpfFirewallEngine {
    pub fn new(public_ip: [u8; 4]) -> Self {
        Self {
            rules: Vec::new(),
            active_states: BTreeMap::new(),
            nat_translations: BTreeMap::new(),
            public_ip,
        }
    }

    pub fn add_rule(&mut self, rule: NpfStateRule) {
        self.rules.push(rule);
    }

    fn serialize_state_key(packet: &NpfPacket) -> [u8; 18] {
        let mut key = [0u8; 18];
        // Normalize direction (canonical order) so both inbound and outbound packets produce the identical state key
        let ((ip1, port1), (ip2, port2)) = if (packet.src_ip, packet.src_port) <= (packet.dst_ip, packet.dst_port) {
            ((packet.src_ip, packet.src_port), (packet.dst_ip, packet.dst_port))
        } else {
            ((packet.dst_ip, packet.dst_port), (packet.src_ip, packet.src_port))
        };

        key[0..4].copy_from_slice(&ip1);
        key[4..8].copy_from_slice(&ip2);
        key[8..10].copy_from_slice(&port1.to_be_bytes());
        key[10..12].copy_from_slice(&port2.to_be_bytes());
        key[12] = packet.protocol;
        key[17] = 1;
        key
    }

    pub fn evaluate_packet(&mut self, packet: &NpfPacket, direction: NpfDirection) -> NpfFilterAction {
        let key = Self::serialize_state_key(packet);

        // Check stateful session table first
        if self.active_states.contains_key(&key) {
            return NpfFilterAction::Pass;
        }

        for rule in &self.rules {
            if rule.direction != NpfDirection::Both && rule.direction != direction {
                continue;
            }

            if let Some(proto) = rule.protocol {
                if proto != packet.protocol {
                    continue;
                }
            }

            if let Some(src) = rule.src_ip {
                if src != packet.src_ip {
                    continue;
                }
            }

            if let Some(dst) = rule.dst_ip {
                if dst != packet.dst_ip {
                    continue;
                }
            }

            if let Some(p) = rule.port {
                if p != packet.src_port && p != packet.dst_port {
                    continue;
                }
            }

            if rule.stateful && rule.action == NpfFilterAction::Pass {
                self.active_states.insert(key, true);
            }

            return rule.action;
        }

        NpfFilterAction::Block // Default deny
    }

    pub fn apply_outbound_nat(&mut self, packet: &mut NpfPacket) {
        let mut nat_key = [0u8; 6];
        nat_key[0..4].copy_from_slice(&packet.src_ip);
        nat_key[4..6].copy_from_slice(&packet.src_port.to_be_bytes());

        let public_port = packet.src_port.wrapping_add(10000);
        self.nat_translations.insert(nat_key, (self.public_ip, public_port));

        packet.src_ip = self.public_ip;
        packet.src_port = public_port;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npf_stateful_filter() {
        let mut engine = NpfFirewallEngine::new([192, 168, 1, 1]);
        engine.add_rule(NpfStateRule {
            src_ip: Some([10, 0, 0, 2]),
            dst_ip: None,
            port: Some(80),
            protocol: Some(6),
            action: NpfFilterAction::Pass,
            direction: NpfDirection::Outbound,
            stateful: true,
        });

        let mut pkt = NpfPacket {
            src_ip: [10, 0, 0, 2],
            dst_ip: [1, 1, 1, 1],
            src_port: 54321,
            dst_port: 80,
            protocol: 6,
            payload: Vec::new(),
        };

        let action = engine.evaluate_packet(&pkt, NpfDirection::Outbound);
        assert_eq!(action, NpfFilterAction::Pass);

        // State table should allow returning packet
        let pass_again = engine.evaluate_packet(&pkt, NpfDirection::Outbound);
        assert_eq!(pass_again, NpfFilterAction::Pass);

        // Verify reverse direction packet (inbound response) hits state table
        let return_pkt = NpfPacket {
            src_ip: [1, 1, 1, 1],
            dst_ip: [10, 0, 0, 2],
            src_port: 80,
            dst_port: 54321,
            protocol: 6,
            payload: Vec::new(),
        };
        let return_pass = engine.evaluate_packet(&return_pkt, NpfDirection::Inbound);
        assert_eq!(return_pass, NpfFilterAction::Pass);

        // Apply NAT
        engine.apply_outbound_nat(&mut pkt);
        assert_eq!(pkt.src_ip, [192, 168, 1, 1]);
    }
}
