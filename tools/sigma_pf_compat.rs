// SPDX-License-Identifier: Apache-2.0
//! SigmaOS OpenBSD-style Packet Filter (PF) Compatibility Tool
//! Safe, zero-dependency, `#![no_std]` compliant utility

#![no_std]

pub enum PfAction {
    Pass,
    Block,
}

pub struct PfRule {
    pub id: u32,
    pub action: PfAction,
    pub protocol: &'static str,
    pub src_port: u16,
    pub dst_port: u16,
}

pub struct Packet {
    pub protocol: &'static str,
    pub src_port: u16,
    pub dst_port: u16,
}

pub struct PfEngine {
    pub rules: [Option<PfRule>; 8],
    pub state_table_size: usize,
}

impl PfEngine {
    pub fn new() -> Self {
        Self {
            rules: [None, None, None, None, None, None, None, None],
            state_table_size: 0,
        }
    }

    pub fn add_rule(&mut self, id: u32, action: PfAction, protocol: &'static str, src_port: u16, dst_port: u16) -> bool {
        for slot in self.rules.iter_mut() {
            if slot.is_none() {
                *slot = Some(PfRule {
                    id,
                    action,
                    protocol,
                    src_port,
                    dst_port,
                });
                return true;
            }
        }
        false
    }

    pub fn evaluate_packet(&mut self, pkt: &Packet) -> bool {
        let mut allow = true; // default pass (pf typically has default pass or default block)
        for rule in self.rules.iter().flatten() {
            if rule.protocol == pkt.protocol && rule.dst_port == pkt.dst_port {
                match rule.action {
                    PfAction::Pass => allow = true,
                    PfAction::Block => allow = false,
                }
            }
        }
        if allow {
            self.state_table_size += 1; // track active state table sizing
        }
        allow
    }
}

impl Default for PfEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pf_basic_rule() {
        let mut engine = PfEngine::new();
        engine.add_rule(1, PfAction::Block, "tcp", 0, 80);

        let pkt1 = Packet {
            protocol: "tcp",
            src_port: 1024,
            dst_port: 80,
        };
        assert!(!engine.evaluate_packet(&pkt1)); // Blocked

        let pkt2 = Packet {
            protocol: "tcp",
            src_port: 1025,
            dst_port: 443,
        };
        assert!(engine.evaluate_packet(&pkt2)); // Passed
    }
}
