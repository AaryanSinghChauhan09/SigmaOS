#![cfg_attr(not(test), no_std)]
// SPDX-License-Identifier: MIT
// SigmaOS Security Rules Subsystem (`src/security/rules.rs`)
// Linux & BSD Distribution-Inspired System Security, Audit, Sandboxing,
// Kernel Immutability, and Network Packet Filtering Rules.

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// =========================================================================
// 1. LINUX AUDITD-STYLE SYSCALL & PATH WATCH RULES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAccessType {
    Read,
    Write,
    Execute,
    AttributeChange,
}

#[derive(Debug, Clone)]
pub struct AuditWatchRule {
    pub rule_key: String,
    pub path: String,
    pub monitored_access: Vec<AuditAccessType>,
}

#[derive(Debug, Clone)]
pub struct AuditSyscallRule {
    pub rule_key: String,
    pub syscall_name: String,
    pub action_audit: bool,
}

pub struct SovereignAuditRuleEngine {
    pub watch_rules: Vec<AuditWatchRule>,
    pub syscall_rules: Vec<AuditSyscallRule>,
    pub audit_events_count: u64,
}

impl SovereignAuditRuleEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            watch_rules: Vec::new(),
            syscall_rules: Vec::new(),
            audit_events_count: 0,
        };
        engine.register_default_audit_rules();
        engine
    }

    fn register_default_audit_rules(&mut self) {
        self.add_watch_rule(
            "identity-protection",
            "/etc/shadow",
            vec![
                AuditAccessType::Read,
                AuditAccessType::Write,
                AuditAccessType::AttributeChange,
            ],
        );
        self.add_watch_rule(
            "sudoers-protection",
            "/etc/sudoers",
            vec![AuditAccessType::Write, AuditAccessType::AttributeChange],
        );

        self.add_syscall_rule("privilege-escalation", "sys_ptrace", true);
        self.add_syscall_rule("kernel-relinking", "sys_init_module", true);
    }

    pub fn add_watch_rule(&mut self, key: &str, path: &str, access: Vec<AuditAccessType>) {
        self.watch_rules.push(AuditWatchRule {
            rule_key: key.to_string(),
            path: path.to_string(),
            monitored_access: access,
        });
    }

    pub fn add_syscall_rule(&mut self, key: &str, syscall: &str, action: bool) {
        self.syscall_rules.push(AuditSyscallRule {
            rule_key: key.to_string(),
            syscall_name: syscall.to_string(),
            action_audit: action,
        });
    }

    pub fn inspect_file_access(&mut self, path: &str, access: AuditAccessType) -> bool {
        for rule in &self.watch_rules {
            if path == rule.path && rule.monitored_access.contains(&access) {
                self.audit_events_count += 1;
                return true;
            }
        }
        false
    }

    pub fn inspect_syscall(&mut self, syscall: &str) -> bool {
        for rule in &self.syscall_rules {
            if rule.syscall_name == syscall && rule.action_audit {
                self.audit_events_count += 1;
                return true;
            }
        }
        false
    }
}

impl Default for SovereignAuditRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. OPENBSD PLEDGE & UNVEIL APPLICATION SANDBOXING RULES
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PledgeRule {
    pub app_name: String,
    pub promised_categories: Vec<String>, // e.g. "stdio", "rpath", "wpath", "inet"
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnveilRule {
    pub path: String,
    pub permissions: String, // e.g. "r", "rw", "rx", "rwc"
}

pub struct SovereignSandboxingRulesEngine {
    pub pledge_rules: BTreeMap<String, PledgeRule>,
    pub unveil_rules: BTreeMap<String, Vec<UnveilRule>>,
}

impl SovereignSandboxingRulesEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            pledge_rules: BTreeMap::new(),
            unveil_rules: BTreeMap::new(),
        };
        engine.register_default_sandbox_profiles();
        engine
    }

    fn register_default_sandbox_profiles(&mut self) {
        self.add_pledge_profile(
            "web_browser",
            vec![
                "stdio".to_string(),
                "rpath".to_string(),
                "wpath".to_string(),
                "inet".to_string(),
            ],
        );

        self.add_unveil_profile(
            "web_browser",
            vec![
                UnveilRule {
                    path: "/home/user/.config/browser".to_string(),
                    permissions: "rwc".to_string(),
                },
                UnveilRule {
                    path: "/usr/share/fonts".to_string(),
                    permissions: "r".to_string(),
                },
            ],
        );
    }

    pub fn add_pledge_profile(&mut self, app_name: &str, promised: Vec<String>) {
        self.pledge_rules.insert(
            app_name.to_string(),
            PledgeRule {
                app_name: app_name.to_string(),
                promised_categories: promised,
            },
        );
    }

    pub fn add_unveil_profile(&mut self, app_name: &str, rules: Vec<UnveilRule>) {
        self.unveil_rules.insert(app_name.to_string(), rules);
    }

    pub fn check_pledge(&self, app_name: &str, category: &str) -> bool {
        if let Some(rule) = self.pledge_rules.get(app_name) {
            rule.promised_categories.iter().any(|c| c == category)
        } else {
            true // Unrestricted fallback if no pledge rule set
        }
    }

    pub fn check_unveil(&self, app_name: &str, target_path: &str, req_perm: char) -> bool {
        if let Some(rules) = self.unveil_rules.get(app_name) {
            for rule in rules {
                if target_path.starts_with(&rule.path) {
                    return rule.permissions.contains(req_perm);
                }
            }
            false // Deny access to unrevealed paths
        } else {
            true // Unrestricted fallback
        }
    }
}

impl Default for SovereignSandboxingRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. FREEBSD SECURELEVEL KERNEL IMMUTABILITY RULES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurelevelState {
    PermanentlyUnsecure = -1,
    Insecure = 0,
    Secure = 1,
    HighlySecure = 2,
    NetworkHardened = 3,
}

pub struct SovereignSecurelevelRuleEngine {
    pub current_level: SecurelevelState,
}

impl SovereignSecurelevelRuleEngine {
    pub fn new() -> Self {
        Self {
            current_level: SecurelevelState::Insecure,
        }
    }

    pub fn raise_securelevel(&mut self, new_level: SecurelevelState) -> Result<(), &'static str> {
        if new_level < self.current_level {
            return Err("Securelevel: Cannot lower securelevel once raised (FreeBSD immutable rule)");
        }
        self.current_level = new_level;
        Ok(())
    }

    pub fn allow_raw_disk_write(&self) -> bool {
        self.current_level < SecurelevelState::Secure
    }

    pub fn allow_kernel_module_load(&self) -> bool {
        self.current_level < SecurelevelState::Secure
    }

    pub fn allow_clock_rewind(&self) -> bool {
        self.current_level < SecurelevelState::HighlySecure
    }
}

impl Default for SovereignSecurelevelRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. LINUX SYSCTL SECURITY HARDENING RULES
// =========================================================================

pub struct SysctlParameterRule {
    pub parameter_name: String,
    pub desired_value: String,
    pub active_value: String,
}

pub struct SovereignSysctlHardeningRules {
    pub parameters: Vec<SysctlParameterRule>,
}

impl SovereignSysctlHardeningRules {
    pub fn new() -> Self {
        let mut engine = Self {
            parameters: Vec::new(),
        };
        engine.register_default_sysctl_rules();
        engine
    }

    fn register_default_sysctl_rules(&mut self) {
        self.add_parameter("kernel.kptr_restrict", "2");
        self.add_parameter("kernel.dmesg_restrict", "1");
        self.add_parameter("fs.protected_symlinks", "1");
        self.add_parameter("fs.protected_hardlinks", "1");
        self.add_parameter("net.ipv4.conf.all.rp_filter", "1");
    }

    pub fn add_parameter(&mut self, name: &str, desired: &str) {
        self.parameters.push(SysctlParameterRule {
            parameter_name: name.to_string(),
            desired_value: desired.to_string(),
            active_value: desired.to_string(),
        });
    }

    pub fn enforce_hardening(&mut self) -> usize {
        let mut enforced = 0;
        for p in &mut self.parameters {
            if p.active_value != p.desired_value {
                p.active_value = p.desired_value.clone();
                enforced += 1;
            }
        }
        enforced
    }

    pub fn is_fully_hardened(&self) -> bool {
        self.parameters.iter().all(|p| p.active_value == p.desired_value)
    }
}

impl Default for SovereignSysctlHardeningRules {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. SNORT / SURICATA & OPENBSD PF NETWORK FILTERING RULES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PfAction {
    Pass,
    Block,
    Scrub,
}

#[derive(Debug, Clone)]
pub struct PfFilterRule {
    pub rule_id: u32,
    pub action: PfAction,
    pub interface: String,
    pub protocol: String,
    pub src_cidr: String,
    pub dst_port: u16,
    pub keep_state: bool,
}

pub struct SovereignNetworkFilterRulesEngine {
    pub rules: Vec<PfFilterRule>,
    pub blocked_packet_count: u64,
}

impl SovereignNetworkFilterRulesEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            blocked_packet_count: 0,
        };
        engine.register_default_pf_rules();
        engine
    }

    fn register_default_pf_rules(&mut self) {
        self.add_rule(1, PfAction::Block, "em0", "TCP", "0.0.0.0/0", 23, true); // Block Telnet
        self.add_rule(2, PfAction::Pass, "em0", "TCP", "0.0.0.0/0", 443, true); // Allow HTTPS
    }

    pub fn add_rule(
        &mut self,
        id: u32,
        action: PfAction,
        iface: &str,
        proto: &str,
        src: &str,
        dst_port: u16,
        keep_state: bool,
    ) {
        self.rules.push(PfFilterRule {
            rule_id: id,
            action,
            interface: iface.to_string(),
            protocol: proto.to_string(),
            src_cidr: src.to_string(),
            dst_port,
            keep_state,
        });
    }

    pub fn evaluate_packet(&mut self, iface: &str, proto: &str, src_ip: &str, dst_port: u16) -> PfAction {
        for rule in &self.rules {
            if rule.interface == iface
                && rule.protocol == proto
                && (rule.src_cidr == "0.0.0.0/0" || rule.src_cidr == src_ip)
                && rule.dst_port == dst_port
            {
                if rule.action == PfAction::Block {
                    self.blocked_packet_count += 1;
                }
                return rule.action;
            }
        }
        PfAction::Pass
    }
}

impl Default for SovereignNetworkFilterRulesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_rule_engine() {
        let mut audit = SovereignAuditRuleEngine::new();
        assert!(audit.inspect_file_access("/etc/shadow", AuditAccessType::Read));
        assert!(!audit.inspect_file_access("/etc/hosts", AuditAccessType::Read));

        assert!(audit.inspect_syscall("sys_ptrace"));
        assert!(!audit.inspect_syscall("sys_getpid"));
        assert_eq!(audit.audit_events_count, 2);
    }

    #[test]
    fn test_sandboxing_rules_engine() {
        let sandbox = SovereignSandboxingRulesEngine::new();
        assert!(sandbox.check_pledge("web_browser", "inet"));
        assert!(!sandbox.check_pledge("web_browser", "proc"));

        assert!(sandbox.check_unveil("web_browser", "/home/user/.config/browser/cookies.db", 'r'));
        assert!(!sandbox.check_unveil("web_browser", "/etc/shadow", 'r'));
    }

    #[test]
    fn test_securelevel_rule_engine() {
        let mut sec = SovereignSecurelevelRuleEngine::new();
        assert!(sec.allow_raw_disk_write());

        assert!(sec.raise_securelevel(SecurelevelState::Secure).is_ok());
        assert!(!sec.allow_raw_disk_write());

        // Attempting to lower securelevel should fail
        assert!(sec.raise_securelevel(SecurelevelState::Insecure).is_err());
    }

    #[test]
    fn test_sysctl_hardening_rules() {
        let mut sysctl = SovereignSysctlHardeningRules::new();
        assert!(sysctl.is_fully_hardened());

        sysctl.parameters[0].active_value = "0".to_string(); // Simulate tampered state
        assert!(!sysctl.is_fully_hardened());

        let enforced = sysctl.enforce_hardening();
        assert_eq!(enforced, 1);
        assert!(sysctl.is_fully_hardened());
    }

    #[test]
    fn test_network_filter_rules_engine() {
        let mut pf = SovereignNetworkFilterRulesEngine::new();
        assert_eq!(
            pf.evaluate_packet("em0", "TCP", "1.2.3.4", 23),
            PfAction::Block
        );
        assert_eq!(pf.blocked_packet_count, 1);

        assert_eq!(
            pf.evaluate_packet("em0", "TCP", "1.2.3.4", 443),
            PfAction::Pass
        );
    }
}
