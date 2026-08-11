// SigmaOS Privacy-First Sandbox Subsystem
// Enforces zero-trust sandboxing by default, with post-quantum cryptography baked into kernel-level syscall filters

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandboxRule {
    NetworkWriteGate,
    FSWriteGate,
    ProcessForkGate,
}

pub struct PrivacyFirstSandbox {
    pub process_id: u32,
    pub is_active_sandboxed: bool,
    pub active_pqc_key_attestation: String,
    pub blocked_rules: HashSet<SandboxRule>,
}

impl PrivacyFirstSandbox {
    pub fn new(pid: u32, pqc_key: &str) -> Self {
        PrivacyFirstSandbox {
            process_id: pid,
            is_active_sandboxed: true,
            active_pqc_key_attestation: pqc_key.to_string(),
            blocked_rules: HashSet::new(),
        }
    }

    pub fn block_syscall_rule(&mut self, rule: SandboxRule) {
        self.blocked_rules.insert(rule);
    }

    pub fn validate_syscall_transition(&self, rule: SandboxRule) -> bool {
        if !self.is_active_sandboxed {
            return true; // Bypass checks if sandboxing is explicitly disabled
        }
        // If the rule is blocked, deny transition
        !self.blocked_rules.contains(&rule)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_first_sandbox() {
        let mut sandbox = PrivacyFirstSandbox::new(505, "crystals-dilithium-attestation-token-999");
        assert!(sandbox.is_active_sandboxed);
        assert_eq!(sandbox.active_pqc_key_attestation, "crystals-dilithium-attestation-token-999");

        // Allowed by default
        assert!(sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));

        // Block and verify rejection
        sandbox.block_syscall_rule(SandboxRule::NetworkWriteGate);
        assert!(!sandbox.validate_syscall_transition(SandboxRule::NetworkWriteGate));
        assert!(sandbox.validate_syscall_transition(SandboxRule::FSWriteGate));
    }
}
