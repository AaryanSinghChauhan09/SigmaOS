// SigmaOS Zorin Exec Guard Security Policy Engine
// Implements Default-Deny Capability Enforcement with Developer Exception Rules

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExecCapability {
    FileRead,
    FileWrite,
    ProcessExec,
    NetworkTcp,
    NetworkUdp,
    DisplayAccess,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecDecision {
    Allow {
        capabilities: Vec<ExecCapability>,
        trust_source: String,
    },
    PromptDeveloper {
        binary_path: String,
        suggested_alternative: String,
        reason: String,
    },
    HardDeny {
        reason: String,
        suggested_alternative: String,
    },
}

#[derive(Debug, Clone)]
pub struct TrustedCertificate {
    pub issuer: String,
    pub thumbprint: String,
    pub actions: Vec<ExecCapability>,
}

#[derive(Debug, Clone)]
pub struct PathRule {
    pub path_prefix: String,
    pub allow_unsigned_execution: bool,
    pub enforce_strict_sandboxing: bool,
    pub allowed_capabilities: Vec<ExecCapability>,
    pub blocked_capabilities: Vec<ExecCapability>,
}

#[derive(Debug, Clone)]
pub struct BinaryOverride {
    pub binary_name: String,
    pub override_hash: String,
    pub allow_all_child_processes: bool,
}

#[derive(Debug, Clone)]
pub struct DeveloperModeConfig {
    pub enabled: bool,
    pub interactive_prompts: bool,
    pub log_violations: bool,
}

impl Default for DeveloperModeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interactive_prompts: true,
            log_violations: true,
        }
    }
}

pub struct ZorinExecGuardPolicyEngine {
    pub developer_mode: DeveloperModeConfig,
    pub trusted_certificates: Vec<TrustedCertificate>,
    pub path_rules: Vec<PathRule>,
    pub binary_overrides: Vec<BinaryOverride>,
    pub verified_system_binaries: Vec<String>,
    pub alternative_suggestions: BTreeMap<String, String>,
    pub violation_journal: Vec<String>,
}

impl ZorinExecGuardPolicyEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            developer_mode: DeveloperModeConfig::default(),
            trusted_certificates: Vec::new(),
            path_rules: Vec::new(),
            binary_overrides: Vec::new(),
            verified_system_binaries: Vec::new(),
            alternative_suggestions: BTreeMap::new(),
            violation_journal: Vec::new(),
        };

        // System defaults
        engine.verified_system_binaries.push("/bin/sh".to_string());
        engine.verified_system_binaries.push("/bin/ls".to_string());
        engine.verified_system_binaries.push("/usr/bin/sigma-browser".to_string());
        engine.verified_system_binaries.push("/usr/bin/zenith".to_string());

        // Default alternative suggestions
        engine.alternative_suggestions.insert(
            "setup.exe".to_string(),
            "sigpkg install apps.sigmaos.org/web-installer".to_string(),
        );
        engine.alternative_suggestions.insert(
            "photoshop.exe".to_string(),
            "apx install photopea-desktop".to_string(),
        );
        engine.alternative_suggestions.insert(
            "steam_setup.exe".to_string(),
            "sigpkg install com.valvesoftware.Steam".to_string(),
        );

        engine
    }

    /// Load exception rules from TOML configuration string
    pub fn load_config_toml(&mut self, toml_str: &str) -> Result<(), &'static str> {
        for line in toml_str.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            if line.contains("enabled = true") {
                self.developer_mode.enabled = true;
            } else if line.contains("enabled = false") {
                self.developer_mode.enabled = false;
            } else if line.contains("interactive_prompts = true") {
                self.developer_mode.interactive_prompts = true;
            } else if line.contains("interactive_prompts = false") {
                self.developer_mode.interactive_prompts = false;
            }
        }
        Ok(())
    }

    /// Register trusted certificate rule
    pub fn add_trusted_certificate(&mut self, cert: TrustedCertificate) {
        self.trusted_certificates.push(cert);
    }

    /// Register path-based developer exception rule
    pub fn add_path_rule(&mut self, rule: PathRule) {
        self.path_rules.push(rule);
    }

    /// Register binary hash override rule
    pub fn add_binary_override(&mut self, over: BinaryOverride) {
        self.binary_overrides.push(over);
    }

    /// Inspect execution request against default-deny capability model
    pub fn evaluate_execution(
        &mut self,
        binary_path: &str,
        cert_thumbprint: Option<&str>,
        sha256_hash: Option<&str>,
    ) -> ExecDecision {
        let filename = binary_path.split('/').last().unwrap_or(binary_path);

        // 1. Check system verified binaries
        if self.verified_system_binaries.iter().any(|b| b == binary_path) {
            return ExecDecision::Allow {
                capabilities: vec![
                    ExecCapability::FileRead,
                    ExecCapability::FileWrite,
                    ExecCapability::ProcessExec,
                    ExecCapability::NetworkTcp,
                    ExecCapability::NetworkUdp,
                    ExecCapability::DisplayAccess,
                ],
                trust_source: "System Verified Core Binary".to_string(),
            };
        }

        // 2. Check trusted certificates
        if let Some(thumbprint) = cert_thumbprint {
            if let Some(cert) = self.trusted_certificates.iter().find(|c| c.thumbprint == thumbprint) {
                return ExecDecision::Allow {
                    capabilities: cert.actions.clone(),
                    trust_source: format!("Certificate Issuer: {}", cert.issuer),
                };
            }
        }

        // 3. Check binary overrides
        if let Some(hash) = sha256_hash {
            if let Some(over) = self.binary_overrides.iter().find(|b| b.override_hash == hash || b.binary_name == filename) {
                return ExecDecision::Allow {
                    capabilities: vec![
                        ExecCapability::FileRead,
                        ExecCapability::FileWrite,
                        ExecCapability::ProcessExec,
                    ],
                    trust_source: format!("Binary Override Hash: {}", over.override_hash),
                };
            }
        }

        // 4. Check developer path rules
        if self.developer_mode.enabled {
            for rule in &self.path_rules {
                if binary_path.starts_with(&rule.path_prefix) {
                    if rule.allow_unsigned_execution {
                        return ExecDecision::Allow {
                            capabilities: rule.allowed_capabilities.clone(),
                            trust_source: format!("Developer Workspace Rule: {}", rule.path_prefix),
                        };
                    }
                }
            }
        }

        // 5. Unrecognized execution fallback logic
        let suggested_alt = self.alternative_suggestions.get(filename)
            .cloned()
            .unwrap_or_else(|| format!("sigpkg search {}", filename));

        if self.developer_mode.interactive_prompts {
            if self.developer_mode.log_violations {
                self.violation_journal.push(format!("VIOLATION_PROMPT: Binary '{}' requested execution", binary_path));
            }
            ExecDecision::PromptDeveloper {
                binary_path: binary_path.to_string(),
                suggested_alternative: suggested_alt,
                reason: "Unsigned execution attempt from unverified path".to_string(),
            }
        } else {
            if self.developer_mode.log_violations {
                self.violation_journal.push(format!("VIOLATION_DENY: Binary '{}' blocked by default-deny policy", binary_path));
            }
            ExecDecision::HardDeny {
                reason: "Default-Deny policy blocked untrusted binary execution".to_string(),
                suggested_alternative: suggested_alt,
            }
        }
    }
}

impl Default for ZorinExecGuardPolicyEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_guard_default_deny_and_system_binaries() {
        let mut engine = ZorinExecGuardPolicyEngine::new();

        // System binary should be allowed
        let decision = engine.evaluate_execution("/bin/ls", None, None);
        assert!(matches!(decision, ExecDecision::Allow { .. }));

        // Unknown binary should prompt developer when interactive_prompts = true
        let decision = engine.evaluate_execution("/home/user/unknown.sh", None, None);
        assert!(matches!(decision, ExecDecision::PromptDeveloper { .. }));
    }

    #[test]
    fn test_developer_path_rules_and_capability_attenuation() {
        let mut engine = ZorinExecGuardPolicyEngine::new();

        engine.add_path_rule(PathRule {
            path_prefix: "/home/developer/workspace/".to_string(),
            allow_unsigned_execution: true,
            enforce_strict_sandboxing: true,
            allowed_capabilities: vec![
                ExecCapability::FileRead,
                ExecCapability::FileWrite,
                ExecCapability::ProcessExec,
            ],
            blocked_capabilities: vec![
                ExecCapability::NetworkTcp,
                ExecCapability::DisplayAccess,
            ],
        });

        let decision = engine.evaluate_execution(
            "/home/developer/workspace/my_test_app",
            None,
            None,
        );

        if let ExecDecision::Allow { capabilities, trust_source } = decision {
            assert!(capabilities.contains(&ExecCapability::FileRead));
            assert!(!capabilities.contains(&ExecCapability::NetworkTcp));
            assert!(trust_source.contains("Developer Workspace Rule"));
        } else {
            panic!("Expected Allow decision for workspace binary");
        }
    }

    #[test]
    fn test_hard_deny_and_alternative_suggestions() {
        let mut engine = ZorinExecGuardPolicyEngine::new();
        engine.developer_mode.interactive_prompts = false;

        let decision = engine.evaluate_execution("/tmp/photoshop.exe", None, None);
        if let ExecDecision::HardDeny { suggested_alternative, .. } = decision {
            assert!(suggested_alternative.contains("photopea"));
        } else {
            panic!("Expected HardDeny decision");
        }
    }
}
