extern crate alloc;
// SigmaOS OpenBSD-grade Packet Filter (PF) Stateful Firewall Subsystem
// Zero-dependency, #![no_std] compliant, highly-optimized for low-overhead routing.


use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;
use core::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterAction {
    Pass,
    Block,
    Scrub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrafficDirection {
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct FilterRule {
    pub id: u32,
    pub action: FilterAction,
    pub direction: TrafficDirection,
    pub interface: String,
    pub proto: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct FirewallState {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub proto: String,
}

pub struct OpenBsdPacketFilter {
    pub rules: RefCell<Vec<FilterRule>>,
    pub states: RefCell<Vec<FirewallState>>,
    pub is_enabled: core::sync::atomic::AtomicBool,
}

impl OpenBsdPacketFilter {
    pub fn new() -> Self {
        Self {
            rules: RefCell::new(Vec::new()),
            states: RefCell::new(Vec::new()),
            is_enabled: core::sync::atomic::AtomicBool::new(true),
        }
    }

    pub fn load_ruleset(&self, rules: Vec<FilterRule>) {
        *self.rules.borrow_mut() = rules;
    }

    pub fn check_packet(&self, dir: TrafficDirection, interface: &str, proto: &str, src_ip: &str, dst_ip: &str, src_port: u16, dst_port: u16) -> FilterAction {
        if !self.is_enabled.load(core::sync::atomic::Ordering::Relaxed) {
            return FilterAction::Pass;
        }

        // 1. Check state table (Stateful Filtering - PF's core strength)
        {
            let states = self.states.borrow();
            for state in states.iter() {
                if state.proto == proto
                    && ((state.src_ip == src_ip && state.dst_ip == dst_ip && state.src_port == src_port && state.dst_port == dst_port)
                        || (state.src_ip == dst_ip && state.dst_ip == src_ip && state.src_port == dst_port && state.dst_port == src_port))
                {
                    return FilterAction::Pass; // Fast path for established connections
                }
            }
        }

        // 2. Evaluate ruleset (Last match wins - OpenBSD PF standard behavior)
        let mut final_action = FilterAction::Pass; // default pass if no rules match
        let rules = self.rules.borrow();

        for rule in rules.iter() {
            if rule.direction != dir {
                continue;
            }
            if !rule.interface.is_empty() && rule.interface != interface {
                continue;
            }
            if !rule.proto.is_empty() && rule.proto != proto {
                continue;
            }
            if !rule.src_ip.is_empty() && rule.src_ip != "*" && rule.src_ip != src_ip {
                continue;
            }
            if !rule.dst_ip.is_empty() && rule.dst_ip != "*" && rule.dst_ip != dst_ip {
                continue;
            }
            if let Some(p) = rule.src_port {
                if p != src_port {
                    continue;
                }
            }
            if let Some(p) = rule.dst_port {
                if p != dst_port {
                    continue;
                }
            }

            final_action = rule.action;
        }

        // 3. Create state if Action is Pass
        if final_action == FilterAction::Pass {
            let mut states = self.states.borrow_mut();
            states.push(FirewallState {
                src_ip: src_ip.to_string(),
                dst_ip: dst_ip.to_string(),
                src_port,
                dst_port,
                proto: proto.to_string(),
            });
        }

        final_action
    }
}

impl Default for OpenBsdPacketFilter {
    fn default() -> Self {
        Self::new()
    }
}
