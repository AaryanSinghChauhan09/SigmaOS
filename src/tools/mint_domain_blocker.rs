//! Linux Mint mintnanny-inspired Domain Blocker
//! 
//! This module implements a domain blocker inspired by Linux Mint's mintnanny,
//! which blocks outgoing traffic towards chosen domain names using /etc/hosts.

#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Blocked domain entry
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockedDomain {
    /// Domain name
    pub domain: String,
    /// Block reason
    pub reason: String,
    /// Timestamp when block was added
    pub added_timestamp: u64,
    /// Whether block is active
    pub active: bool,
}

/// Block rule type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockRuleType {
    /// Exact domain match
    Exact,
    /// Wildcard subdomain match
    Wildcard,
    /// Regex pattern match
    Regex,
}

/// Domain blocker - manages blocked domains
#[derive(Debug)]
pub struct MintDomainBlocker {
    /// Blocked domains
    pub blocked_domains: BTreeSet<BlockedDomain>,
    /// Block rules by type
    pub block_rules: Vec<(BlockRuleType, String)>,
    /// Redirect IP for blocked domains (typically 127.0.0.1)
    pub redirect_ip: String,
    /// Whether blocking is enabled
    pub enabled: bool,
}

impl MintDomainBlocker {
    /// Create a new Domain Blocker
    pub fn new() -> Self {
        Self {
            blocked_domains: BTreeSet::new(),
            block_rules: Vec::new(),
            redirect_ip: "127.0.0.1".to_string(),
            enabled: true,
        }
    }

    /// Add a domain to block
    pub fn block_domain(&mut self, domain: String, reason: String) -> Result<(), String> {
        if domain.is_empty() {
            return Err("Domain cannot be empty".to_string());
        }

        let blocked = BlockedDomain {
            domain: domain.clone(),
            reason,
            added_timestamp: 0, // Would be current time in real implementation
            active: true,
        };

        self.blocked_domains.insert(blocked);
        Ok(())
    }

    /// Remove a domain from block list
    pub fn unblock_domain(&mut self, domain: &str) -> Result<(), String> {
        let domain_to_remove = self.blocked_domains.iter().find(|d| d.domain == domain).cloned();
        if let Some(blocked) = domain_to_remove {
            self.blocked_domains.remove(&blocked);
            Ok(())
        } else {
            Err("Domain not found in block list".to_string())
        }
    }

    /// Check if a domain is blocked
    pub fn is_domain_blocked(&self, domain: &str) -> bool {
        // Check exact match
        if self.blocked_domains.iter().any(|d| d.domain == domain && d.active) {
            return true;
        }

        // Check wildcard rules
        for (rule_type, pattern) in &self.block_rules {
            match rule_type {
                BlockRuleType::Wildcard => {
                    if domain.ends_with(&pattern.trim_start_matches('*')) {
                        return true;
                    }
                }
                BlockRuleType::Exact => {
                    if domain == pattern {
                        return true;
                    }
                }
                BlockRuleType::Regex => {
                    // In real implementation, would use regex matching
                    if domain.contains(pattern) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Add a block rule
    pub fn add_block_rule(&mut self, rule_type: BlockRuleType, pattern: String) {
        self.block_rules.push((rule_type, pattern));
    }

    /// Remove a block rule
    pub fn remove_block_rule(&mut self, rule_type: BlockRuleType, pattern: &str) {
        self.block_rules.retain(|(t, p)| *t != rule_type || p != pattern);
    }

    /// Get all blocked domains
    pub fn get_blocked_domains(&self) -> Vec<&BlockedDomain> {
        self.blocked_domains.iter().collect()
    }

    /// Get active blocked domains only
    pub fn get_active_blocked_domains(&self) -> Vec<&BlockedDomain> {
        self.blocked_domains
            .iter()
            .filter(|d| d.active)
            .collect()
    }

    /// Enable or disable blocking
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Set redirect IP for blocked domains
    pub fn set_redirect_ip(&mut self, ip: String) {
        self.redirect_ip = ip;
    }

    /// Generate hosts file entries for blocked domains
    pub fn generate_hosts_entries(&self) -> Vec<String> {
        if !self.enabled {
            return Vec::new();
        }

        let mut entries = Vec::new();
        for domain in &self.blocked_domains {
            if domain.active {
                entries.push(format!("{} {}", self.redirect_ip, domain.domain));
            }
        }
        entries
    }

    /// Import blocked domains from a list
    pub fn import_domains(&mut self, domains: Vec<(String, String)>) -> usize {
        let mut count = 0;
        for (domain, reason) in domains {
            if self.block_domain(domain, reason).is_ok() {
                count += 1;
            }
        }
        count
    }

    /// Export blocked domains to a list
    pub fn export_domains(&self) -> Vec<(String, String)> {
        self.blocked_domains
            .iter()
            .map(|d| (d.domain.clone(), d.reason.clone()))
            .collect()
    }

    /// Clear all blocked domains
    pub fn clear_all(&mut self) {
        self.blocked_domains.clear();
        self.block_rules.clear();
    }

    /// Get block statistics
    pub fn get_statistics(&self) -> BlockStatistics {
        BlockStatistics {
            total_blocked: self.blocked_domains.len(),
            active_blocked: self.blocked_domains.iter().filter(|d| d.active).count(),
            total_rules: self.block_rules.len(),
            enabled: self.enabled,
        }
    }
}

/// Block statistics
#[derive(Debug, Clone)]
pub struct BlockStatistics {
    /// Total number of blocked domains
    pub total_blocked: usize,
    /// Number of active blocks
    pub active_blocked: usize,
    /// Total number of block rules
    pub total_rules: usize,
    /// Whether blocking is enabled
    pub enabled: bool,
}

impl Default for MintDomainBlocker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_blocker_creation() {
        let blocker = MintDomainBlocker::new();
        assert_eq!(blocker.blocked_domains.len(), 0);
        assert!(blocker.enabled);
    }

    #[test]
    fn test_block_domain() {
        let mut blocker = MintDomainBlocker::new();
        
        let result = blocker.block_domain("example.com".to_string(), "Test block".to_string());
        assert!(result.is_ok());
        assert_eq!(blocker.blocked_domains.len(), 1);
    }

    #[test]
    fn test_unblock_domain() {
        let mut blocker = MintDomainBlocker::new();
        
        blocker.block_domain("example.com".to_string(), "Test block".to_string()).unwrap();
        let result = blocker.unblock_domain("example.com");
        assert!(result.is_ok());
        assert_eq!(blocker.blocked_domains.len(), 0);
    }

    #[test]
    fn test_is_domain_blocked() {
        let mut blocker = MintDomainBlocker::new();
        
        blocker.block_domain("example.com".to_string(), "Test block".to_string()).unwrap();
        assert!(blocker.is_domain_blocked("example.com"));
        assert!(!blocker.is_domain_blocked("notblocked.com"));
    }

    #[test]
    fn test_generate_hosts_entries() {
        let mut blocker = MintDomainBlocker::new();
        
        blocker.block_domain("example.com".to_string(), "Test block".to_string()).unwrap();
        let entries = blocker.generate_hosts_entries();
        assert_eq!(entries.len(), 1);
        assert!(entries[0].contains("127.0.0.1"));
        assert!(entries[0].contains("example.com"));
    }

    #[test]
    fn test_import_export() {
        let mut blocker = MintDomainBlocker::new();
        
        let domains = vec![
            ("example.com".to_string(), "Test 1".to_string()),
            ("test.com".to_string(), "Test 2".to_string()),
        ];
        
        let count = blocker.import_domains(domains);
        assert_eq!(count, 2);
        
        let exported = blocker.export_domains();
        assert_eq!(exported.len(), 2);
    }

    #[test]
    fn test_block_statistics() {
        let mut blocker = MintDomainBlocker::new();
        
        blocker.block_domain("example.com".to_string(), "Test block".to_string()).unwrap();
        blocker.add_block_rule(BlockRuleType::Wildcard, "*.ads.com".to_string());
        
        let stats = blocker.get_statistics();
        assert_eq!(stats.total_blocked, 1);
        assert_eq!(stats.active_blocked, 1);
        assert_eq!(stats.total_rules, 1);
        assert!(stats.enabled);
    }

    #[test]
    fn test_wildcard_rule() {
        let mut blocker = MintDomainBlocker::new();
        
        blocker.add_block_rule(BlockRuleType::Wildcard, "*.ads.com".to_string());
        assert!(blocker.is_domain_blocked("tracker.ads.com"));
        assert!(blocker.is_domain_blocked("analytics.ads.com"));
        assert!(!blocker.is_domain_blocked("ads.com"));
    }
}
