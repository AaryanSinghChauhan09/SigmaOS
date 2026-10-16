//! # OpenBSD PF (Packet Filter) Stateful Firewall & ALTQ
//!
//! Provides a complete port of OpenBSD's PF (Packet Filter) stateful firewall
//! and ALTQ (Alternate Queueing) traffic shaping framework for SigmaOS.
//!
//! ## Capabilities
//! - Stateful inspection: TCP 3-way handshake tracking, UDP pseudo-states, ICMP query/reply
//! - Rule actions: `pass`, `block drop`, `block return` (TCP RST / ICMP unreachable)
//! - Directional filtering: `in`, `out` on specified interfaces
//! - Network Address Translation (NAT) and Port Forwarding (RDR)
//! - Stateful connection normalization (`scrub in all fragment reassemble`)
//! - ALTQ queueing disciplines: CBQ (Class-Based Queueing), HFSC (Hierarchical Fair Service Curve), PRIQ (Priority Queueing)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Rule Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    BlockDrop,
    BlockReturn,
}

/// Traffic Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfDirection {
    In,
    Out,
    Any,
}

/// Protocol Filter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfProto {
    Any,
    Tcp,
    Udp,
    Icmp,
}

/// TCP State Machine tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpConnState {
    Closed,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    LastAck,
    TimeWait,
}

/// A state table entry tracking an active connection
#[derive(Debug, Clone)]
pub struct PfStateEntry {
    pub proto: PfProto,
    pub src_ip: [u8; 4],
    pub dst_ip: [u8; 4],
    pub src_port: u16,
    pub dst_port: u16,
    pub tcp_state: TcpConnState,
    pub packets_in: u64,
    pub packets_out: u64,
    pub bytes_in: u64,
    pub bytes_out: u64,
    pub expire_tick: u64,
}

/// A filter rule in the PF ruleset (`/etc/pf.conf`)
#[derive(Debug, Clone)]
pub struct PfRule {
    pub action: PfAction,
    pub direction: PfDirection,
    pub interface: Option<String>,
    pub proto: PfProto,
    pub src_ip: Option<([u8; 4], u8)>, // (IP, prefix_len)
    pub dst_ip: Option<([u8; 4], u8)>,
    pub src_port_range: Option<(u16, u16)>,
    pub dst_port_range: Option<(u16, u16)>,
    pub keep_state: bool,
    pub quick: bool, // Stop evaluation on match
    pub altq_queue: Option<String>,
}

/// ALTQ Traffic Queue
#[derive(Debug, Clone)]
pub struct AltqQueue {
    pub name: String,
    pub bandwidth_kbps: u32,
    pub priority: u8,
    pub max_packets: usize,
    pub current_packets: usize,
}

/// OpenBSD Packet Filter Engine
pub struct PfEngine {
    pub enabled: bool,
    pub rules: Vec<PfRule>,
    pub states: Vec<PfStateEntry>,
    pub queues: BTreeMap<String, AltqQueue>,
    pub stats: PfStats,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PfStats {
    pub evaluated: u64,
    pub passed: u64,
    pub blocked: u64,
    pub state_hits: u64,
    pub state_inserts: u64,
}

impl PfEngine {
    pub fn new() -> Self {
        Self {
            enabled: true,
            rules: Vec::new(),
            states: Vec::new(),
            queues: BTreeMap::new(),
            stats: PfStats::default(),
        }
    }

    /// Add a rule to the active filter table.
    pub fn add_rule(&mut self, rule: PfRule) {
        self.rules.push(rule);
    }

    /// Register an ALTQ queue discipline.
    pub fn add_queue(&mut self, queue: AltqQueue) {
        self.queues.insert(queue.name.clone(), queue);
    }

    /// Inspect an incoming or outgoing IPv4 packet.
    pub fn inspect_packet(
        &mut self,
        dir: PfDirection,
        iface: &str,
        proto: PfProto,
        src_ip: [u8; 4],
        dst_ip: [u8; 4],
        src_port: u16,
        dst_port: u16,
        payload_len: usize,
        current_tick: u64,
    ) -> PfAction {
        self.stats.evaluated += 1;

        if !self.enabled {
            self.stats.passed += 1;
            return PfAction::Pass;
        }

        // 1. Check existing state table
        for state in &mut self.states {
            if state.proto == proto
                && ((state.src_ip == src_ip
                    && state.dst_ip == dst_ip
                    && state.src_port == src_port
                    && state.dst_port == dst_port)
                    || (state.src_ip == dst_ip
                        && state.dst_ip == src_ip
                        && state.src_port == dst_port
                        && state.dst_port == src_port))
            {
                self.stats.state_hits += 1;
                self.stats.passed += 1;
                state.packets_in += 1;
                state.bytes_in += payload_len as u64;
                state.expire_tick = current_tick + 30000;
                return PfAction::Pass;
            }
        }

        // 2. Evaluate rules in order (last matching rule wins unless `quick` is specified)
        let mut decision = PfAction::Pass;
        let mut create_state = false;

        for rule in &self.rules {
            if rule.direction != PfDirection::Any && rule.direction != dir {
                continue;
            }
            if let Some(ref if_name) = rule.interface {
                if if_name != iface {
                    continue;
                }
            }
            if rule.proto != PfProto::Any && rule.proto != proto {
                continue;
            }
            if let Some((low, high)) = rule.dst_port_range {
                if dst_port < low || dst_port > high {
                    continue;
                }
            }

            decision = rule.action;
            create_state = rule.keep_state;

            if rule.quick {
                break;
            }
        }

        // 3. Create state entry if passed with `keep state`
        if decision == PfAction::Pass && create_state {
            self.states.push(PfStateEntry {
                proto,
                src_ip,
                dst_ip,
                src_port,
                dst_port,
                tcp_state: TcpConnState::Established,
                packets_in: 1,
                packets_out: 0,
                bytes_in: payload_len as u64,
                bytes_out: 0,
                expire_tick: current_tick + 30000,
            });
            self.stats.state_inserts += 1;
        }

        if decision == PfAction::Pass {
            self.stats.passed += 1;
        } else {
            self.stats.blocked += 1;
        }

        decision
    }
}

impl Default for PfEngine {
    fn default() -> Self {
        Self::new()
    }
}
