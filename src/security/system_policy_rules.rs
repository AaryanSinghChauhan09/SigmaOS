// SigmaOS System Security Policy Rules Engine
// Inspired by Linux polkit (/usr/share/polkit-1/actions/), sysctl.d (/etc/sysctl.d/), and OpenBSD doas.conf
// Provides PolicyKit authorization evaluation, kernel sysctl parameter enforcement, and doas privilege escalation rules.

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

/// PolicyKit Action Authorization Result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolkitResult {
    Yes,
    No,
    AuthSelfKeep,
    AuthAdmin,
}

/// Linux PolicyKit Rule Action
#[derive(Debug, Clone)]
pub struct PolkitActionRule {
    pub action_id: String,
    pub required_group: String,
    pub result_any: PolkitResult,
    pub result_inactive: PolkitResult,
    pub result_active: PolkitResult,
}

/// Linux/BSD Kernel Sysctl Parameter Rule
#[derive(Debug, Clone)]
pub struct SysctlRule {
    pub key: String,
    pub value: String,
    pub source_conf: String, // e.g. "/etc/sysctl.d/99-sovereign-security.conf"
}

/// OpenBSD doas.conf Privilege Rule
#[derive(Debug, Clone)]
pub struct DoasRule {
    pub permit: bool,
    pub identity: String,  # e.g. "wheel" or "alice"
    pub target_user: String, // e.g. "root"
    pub command: Option<String>, // None matches all commands
    pub nopass: bool,
    pub setenv: Vec<String>,
}

/// System Security Policy Rule Engine
#[derive(Debug, Clone)]
pub struct SovereignSystemPolicyRuleEngine {
    pub polkit_rules: HashMap<String, PolkitActionRule>,
    pub sysctl_parameters: HashMap<String, SysctlRule>,
    pub doas_rules: Vec<DoasRule>,
}

impl SovereignSystemPolicyRuleEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            polkit_rules: HashMap::new(),
            sysctl_parameters: HashMap::new(),
            doas_rules: Vec::new(),
        };
        engine.seed_default_security_policies();
        engine
    }

    /// Seeds default Linux/OpenBSD security policy rules
    pub fn seed_default_security_policies(&mut self) {
        // Polkit action: org.sigmaos.system.reboot
        self.polkit_rules.insert(
            "org.sigmaos.system.reboot".to_string(),
            PolkitActionRule {
                action_id: "org.sigmaos.system.reboot".to_string(),
                required_group: "wheel".to_string(),
                result_any: PolkitResult::AuthAdmin,
                result_inactive: PolkitResult::AuthAdmin,
                result_active: PolkitResult::Yes,
            },
        );

        // Sysctl hardening: 99-sovereign-security.conf
        self.set_sysctl("kernel.randomize_va_space", "2", "/etc/sysctl.d/99-sovereign-security.conf");
        self.set_sysctl("net.ipv4.conf.all.rp_filter", "1", "/etc/sysctl.d/99-sovereign-security.conf");
        self.set_sysctl("fs.protected_symlinks", "1", "/etc/sysctl.d/99-sovereign-security.conf");

        // doas.conf: permit nopass :wheel as root
        self.doas_rules.push(DoasRule {
            permit: true,
            identity: "wheel".to_string(),
            target_user: "root".to_string(),
            command: None,
            nopass: true,
            setenv: vec!["PATH".to_string(), "LANG".to_string()],
        });
    }

    /// Sets a kernel sysctl parameter
    pub fn set_sysctl(&mut self, key: &str, value: &str, source: &str) {
        self.sysctl_parameters.insert(
            key.to_string(),
            SysctlRule {
                key: key.to_string(),
                value: value.to_string(),
                source_conf: source.to_string(),
            },
        );
    }

    /// Evaluates PolicyKit action permission for a given user & group
    pub fn evaluate_polkit_action(&self, action_id: &str, is_active_session: bool) -> PolkitResult {
        if let Some(rule) = self.polkit_rules.get(action_id) {
            if is_active_session {
                rule.result_active
            } else {
                rule.result_inactive
            }
        } else {
            PolkitResult::AuthAdmin
        }
    }

    /// Evaluates doas command privilege escalation permission
    pub fn evaluate_doas_permission(&self, identity: &str, target_user: &str, cmd: &str) -> bool {
        for rule in self.doas_rules.iter().rev() {
            if (rule.identity == identity || rule.identity.starts_with(':'))
                && rule.target_user == target_user
            {
                if let Some(ref rule_cmd) = rule.command {
                    if rule_cmd != cmd {
                        continue;
                    }
                }
                return rule.permit;
            }
        }
        false
    }
}

impl Default for SovereignSystemPolicyRuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polkit_action_evaluation() {
        let engine = SovereignSystemPolicyRuleEngine::new();
        let res_active = engine.evaluate_polkit_action("org.sigmaos.system.reboot", true);
        assert_eq!(res_active, PolkitResult::Yes);

        let res_inactive = engine.evaluate_polkit_action("org.sigmaos.system.reboot", false);
        assert_eq!(res_inactive, PolkitResult::AuthAdmin);
    }

    #[test]
    fn test_sysctl_policy_and_doas() {
        let mut engine = SovereignSystemPolicyRuleEngine::new();
        assert_eq!(
            engine.sysctl_parameters.get("kernel.randomize_va_space").unwrap().value,
            "2"
        );

        let permitted = engine.evaluate_doas_permission("wheel", "root", "/sbin/sigma-pkg");
        assert!(permitted);
    }
}
