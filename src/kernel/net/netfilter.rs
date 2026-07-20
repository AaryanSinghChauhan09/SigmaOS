use core::sync::atomic::{AtomicUsize, Ordering};
use std::string::{String, ToString};
/// SigmaOS Netfilter — stateless and stateful packet filtering
/// Absorbs Linux netfilter hooks: PREROUTING, INPUT, FORWARD, OUTPUT, POSTROUTING
/// Supports: ACCEPT, DROP, REJECT, LOG actions; conntrack state matching
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfHookpoint {
    PreRouting,
    Input,
    Forward,
    Output,
    PostRouting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfVerdict {
    Accept,
    Drop,
    Reject,
    Queue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NfProtocol {
    Tcp = 6,
    Udp = 17,
    Icmp = 1,
    Any = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConntrackState {
    New,
    Established,
    Related,
    Invalid,
}

/// Packet representation for filter matching
#[derive(Debug, Clone)]
pub struct NetPacket {
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub protocol: NfProtocol,
    pub conntrack_state: ConntrackState,
    pub payload_len: usize,
}

impl NetPacket {
    pub fn tcp(src: [u8; 4], dst: [u8; 4], src_port: u16, dst_port: u16) -> Self {
        NetPacket {
            src_ip: src,
            dst_ip: dst,
            src_port,
            dst_port,
            protocol: NfProtocol::Tcp,
            conntrack_state: ConntrackState::New,
            payload_len: 0,
        }
    }
}

/// A single firewall rule
#[derive(Debug, Clone)]
pub struct NfRule {
    pub name: String,
    pub hook: NfHookpoint,
    pub src_ip: Option<[u8; 4]>,
    pub dst_ip: Option<[u8; 4]>,
    pub dst_port: Option<u16>,
    pub protocol: NfProtocol,
    pub conntrack_state: Option<ConntrackState>,
    pub verdict: NfVerdict,
}

impl NfRule {
    pub fn matches(&self, pkt: &NetPacket) -> bool {
        if let Some(src) = self.src_ip {
            if pkt.src_ip != src {
                return false;
            }
        }
        if let Some(dst) = self.dst_ip {
            if pkt.dst_ip != dst {
                return false;
            }
        }
        if let Some(port) = self.dst_port {
            if pkt.dst_port != port {
                return false;
            }
        }
        if self.protocol != NfProtocol::Any && pkt.protocol != self.protocol {
            return false;
        }
        if let Some(ct) = self.conntrack_state {
            if pkt.conntrack_state != ct {
                return false;
            }
        }
        true
    }
}

/// Netfilter table (iptables / nftables equivalent)
pub struct NetfilterTable {
    pub rules: Vec<NfRule>,
    pub default_policy: NfVerdict,
    packets_accepted: AtomicUsize,
    packets_dropped: AtomicUsize,
}

impl NetfilterTable {
    pub fn new_permissive() -> Self {
        NetfilterTable {
            rules: Vec::new(),
            default_policy: NfVerdict::Accept,
            packets_accepted: AtomicUsize::new(0),
            packets_dropped: AtomicUsize::new(0),
        }
    }

    pub fn new_restrictive() -> Self {
        NetfilterTable {
            rules: Vec::new(),
            default_policy: NfVerdict::Drop,
            packets_accepted: AtomicUsize::new(0),
            packets_dropped: AtomicUsize::new(0),
        }
    }

    pub fn add_rule(&mut self, rule: NfRule) {
        self.rules.push(rule);
    }

    /// Process a packet through the rule chain
    pub fn process(&self, hook: NfHookpoint, pkt: &NetPacket) -> NfVerdict {
        for rule in self.rules.iter().filter(|r| r.hook == hook) {
            if rule.matches(pkt) {
                match rule.verdict {
                    NfVerdict::Accept => {
                        self.packets_accepted.fetch_add(1, Ordering::Relaxed);
                    }
                    NfVerdict::Drop | NfVerdict::Reject => {
                        self.packets_dropped.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => {}
                }
                return rule.verdict;
            }
        }
        // No rule matched — apply default policy
        match self.default_policy {
            NfVerdict::Accept => {
                self.packets_accepted.fetch_add(1, Ordering::Relaxed);
            }
            NfVerdict::Drop => {
                self.packets_dropped.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
        self.default_policy
    }

    pub fn packets_accepted(&self) -> usize {
        self.packets_accepted.load(Ordering::Relaxed)
    }
    pub fn packets_dropped(&self) -> usize {
        self.packets_dropped.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_netfilter_accept_all() {
        let table = NetfilterTable::new_permissive();
        let pkt = NetPacket::tcp([192, 168, 1, 1], [10, 0, 0, 1], 12345, 80);
        assert_eq!(table.process(NfHookpoint::Input, &pkt), NfVerdict::Accept);
        assert_eq!(table.packets_accepted(), 1);
    }

    #[test]
    fn test_netfilter_drop_ssh() {
        let mut table = NetfilterTable::new_permissive();
        table.add_rule(NfRule {
            name: "block_ssh".to_string(),
            hook: NfHookpoint::Input,
            src_ip: None,
            dst_ip: None,
            dst_port: Some(22),
            protocol: NfProtocol::Tcp,
            conntrack_state: None,
            verdict: NfVerdict::Drop,
        });
        let ssh_pkt = NetPacket::tcp([1, 2, 3, 4], [10, 0, 0, 1], 9000, 22);
        assert_eq!(table.process(NfHookpoint::Input, &ssh_pkt), NfVerdict::Drop);
        assert_eq!(table.packets_dropped(), 1);

        let http_pkt = NetPacket::tcp([1, 2, 3, 4], [10, 0, 0, 1], 9001, 80);
        assert_eq!(
            table.process(NfHookpoint::Input, &http_pkt),
            NfVerdict::Accept
        );
    }

    #[test]
    fn test_default_drop_policy() {
        let table = NetfilterTable::new_restrictive();
        let pkt = NetPacket::tcp([1, 2, 3, 4], [5, 6, 7, 8], 1234, 8080);
        assert_eq!(table.process(NfHookpoint::Forward, &pkt), NfVerdict::Drop);
    }
}
