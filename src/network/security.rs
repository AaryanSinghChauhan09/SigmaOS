// SigmaOS Network Security Module
// Implements firewall, TLS, and network security features
// Inspired by pf (OpenBSD), iptables (Linux), and Windows Firewall

use std::string::String;
use std::vec::Vec;

/// Network protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    TCP,
    UDP,
    ICMP,
    Any,
}

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id: u32,
    pub source_ip: String,
    pub source_port: Option<u16>,
    pub dest_ip: String,
    pub dest_port: Option<u16>,
    pub protocol: NetworkProtocol,
    pub action: FirewallAction,
    pub is_enabled: bool,
}

impl FirewallRule {
    pub fn new(
        id: u32,
        source_ip: String,
        dest_ip: String,
        protocol: NetworkProtocol,
        action: FirewallAction,
    ) -> Self {
        Self {
            id,
            source_ip,
            source_port: None,
            dest_ip,
            dest_port: None,
            protocol,
            action,
            is_enabled: true,
        }
    }

    /// Set source port
    pub fn with_source_port(mut self, port: u16) -> Self {
        self.source_port = Some(port);
        self
    }

    /// Set destination port
    pub fn with_dest_port(mut self, port: u16) -> Self {
        self.dest_port = Some(port);
        self
    }

    /// Enable rule
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    /// Disable rule
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
}

/// Firewall state
pub struct Firewall {
    pub rules: Vec<FirewallRule>,
    pub default_policy: FirewallAction,
    pub next_rule_id: u32,
}

impl Firewall {
    pub fn new(default_policy: FirewallAction) -> Self {
        Self {
            rules: Vec::new(),
            default_policy,
            next_rule_id: 1,
        }
    }

    /// Add a firewall rule
    pub fn add_rule(&mut self, rule: FirewallRule) -> u32 {
        let id = self.next_rule_id;
        self.next_rule_id += 1;

        let mut rule = rule;
        rule.id = id;
        self.rules.push(rule);

        id
    }

    /// Remove a firewall rule
    pub fn remove_rule(&mut self, rule_id: u32) -> Result<(), String> {
        if let Some(pos) = self.rules.iter().position(|r| r.id == rule_id) {
            self.rules.remove(pos);
            Ok(())
        } else {
            Err(format!("Rule {} not found", rule_id))
        }
    }

    /// Get rule by ID
    pub fn get_rule(&self, rule_id: u32) -> Option<&FirewallRule> {
        self.rules.iter().find(|r| r.id == rule_id)
    }

    /// Get mutable rule by ID
    pub fn get_rule_mut(&mut self, rule_id: u32) -> Option<&mut FirewallRule> {
        self.rules.iter_mut().find(|r| r.id == rule_id)
    }

    /// Evaluate packet against firewall rules
    pub fn evaluate_packet(
        &self,
        source_ip: &str,
        source_port: Option<u16>,
        dest_ip: &str,
        dest_port: Option<u16>,
        protocol: NetworkProtocol,
    ) -> FirewallAction {
        for rule in &self.rules {
            if !rule.is_enabled {
                continue;
            }

            // Check protocol match
            if rule.protocol != NetworkProtocol::Any && rule.protocol != protocol {
                continue;
            }

            // Check source IP match (simplified - should support CIDR)
            if rule.source_ip != "0.0.0.0" && rule.source_ip != source_ip {
                continue;
            }

            // Check source port match
            if let Some(rule_port) = rule.source_port {
                if let Some(pkt_port) = source_port {
                    if rule_port != pkt_port {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Check destination IP match
            if rule.dest_ip != "0.0.0.0" && rule.dest_ip != dest_ip {
                continue;
            }

            // Check destination port match
            if let Some(rule_port) = rule.dest_port {
                if let Some(pkt_port) = dest_port {
                    if rule_port != pkt_port {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            // Rule matched
            return rule.action;
        }

        // No rule matched, apply default policy
        self.default_policy
    }

    /// Set default policy
    pub fn set_default_policy(&mut self, policy: FirewallAction) {
        self.default_policy = policy;
    }
}

impl Default for Firewall {
    fn default() -> Self {
        Self::new(FirewallAction::Deny)
    }
}

/// TLS version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
    TLS1_2,
    TLS1_3,
}

/// TLS cipher suite
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsCipherSuite {
    pub name: String,
    pub id: u16,
}

impl TlsCipherSuite {
    pub fn new(name: String, id: u16) -> Self {
        Self { name, id }
    }
}

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub version: TlsVersion,
    pub cipher_suites: Vec<TlsCipherSuite>,
    pub verify_certificates: bool,
    pub verify_hostname: bool,
}

impl TlsConfig {
    pub fn new(version: TlsVersion) -> Self {
        Self {
            version,
            cipher_suites: Vec::new(),
            verify_certificates: true,
            verify_hostname: true,
        }
    }

    /// Add cipher suite
    pub fn add_cipher_suite(&mut self, suite: TlsCipherSuite) {
        self.cipher_suites.push(suite);
    }

    /// Set certificate verification
    pub fn set_verify_certificates(&mut self, verify: bool) {
        self.verify_certificates = verify;
    }

    /// Set hostname verification
    pub fn set_verify_hostname(&mut self, verify: bool) {
        self.verify_hostname = verify;
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self::new(TlsVersion::TLS1_3)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule::new(
            1,
            "192.168.1.1".to_string(),
            "0.0.0.0".to_string(),
            NetworkProtocol::TCP,
            FirewallAction::Allow,
        );

        assert_eq!(rule.id, 1);
        assert_eq!(rule.action, FirewallAction::Allow);
    }

    #[test]
    fn test_firewall_packet_evaluation() {
        let mut firewall = Firewall::new(FirewallAction::Deny);

        let rule = FirewallRule::new(
            0,
            "192.168.1.1".to_string(),
            "0.0.0.0".to_string(),
            NetworkProtocol::TCP,
            FirewallAction::Allow,
        );
        firewall.add_rule(rule);

        let action = firewall.evaluate_packet(
            "192.168.1.1",
            Some(443),
            "10.0.0.1",
            Some(12345),
            NetworkProtocol::TCP,
        );

        assert_eq!(action, FirewallAction::Allow);
    }

    #[test]
    fn test_firewall_default_policy() {
        let firewall = Firewall::new(FirewallAction::Deny);

        let action = firewall.evaluate_packet(
            "10.0.0.1",
            Some(12345),
            "10.0.0.2",
            Some(80),
            NetworkProtocol::TCP,
        );

        assert_eq!(action, FirewallAction::Deny);
    }

    #[test]
    fn test_tls_config() {
        let mut config = TlsConfig::new(TlsVersion::TLS1_3);

        config.add_cipher_suite(TlsCipherSuite::new(
            "TLS_AES_256_GCM_SHA384".to_string(),
            0x1302,
        ));
        config.set_verify_certificates(true);

        assert_eq!(config.version, TlsVersion::TLS1_3);
        assert!(config.verify_certificates);
        assert_eq!(config.cipher_suites.len(), 1);
    }
}
