#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Firewall manager for network firewall configuration
/// Inspired by Linux iptables, nftables, and UFW
#[derive(Debug, Clone)]
pub struct SigmaFirewallManager2 {
    rules: Vec<FirewallManager2Rule>,
    chains: Vec<FirewallManager2Chain>,
}

/// A firewall rule
#[derive(Debug, Clone)]
pub struct FirewallManager2Rule {
    id: String,
    chain: String,
    source: String,
    destination: String,
    protocol: Protocol,
    action: RuleAction,
    state: RuleState,
    priority: u32,
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    All,
}

/// Rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Accept,
    Drop,
    Reject,
    Log,
}

/// Rule state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleState {
    Active,
    Disabled,
}

/// A firewall chain
#[derive(Debug, Clone)]
pub struct FirewallManager2Chain {
    name: String,
    policy: ChainPolicy,
    state: ChainState,
}

/// Chain policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainPolicy {
    Accept,
    Drop,
}

/// Chain state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChainState {
    Active,
    Disabled,
}

impl SigmaFirewallManager2 {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            chains: Vec::new(),
        }
    }

    /// Add a firewall rule
    pub fn add_rule(&mut self, id: String, chain: String, source: String, destination: String, protocol: Protocol, action: RuleAction, priority: u32) -> Result<String, String> {
        if self.rules.iter().any(|r| r.id == id) {
            return Err(format!("Rule with id {} already exists", id));
        }
        
        let rule = FirewallManager2Rule {
            id: id.clone(),
            chain,
            source,
            destination,
            protocol,
            action,
            state: RuleState::Active,
            priority,
        };
        self.rules.push(rule);
        Ok(id)
    }

    /// Remove a rule by ID
    pub fn remove_rule(&mut self, id: &str) -> Result<(), String> {
        let pos = self.rules.iter().position(|r| r.id == id)
            .ok_or_else(|| format!("Rule with id {} not found", id))?;
        self.rules.remove(pos);
        Ok(())
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, id: &str) -> Result<(), String> {
        let rule = self.rules.iter_mut().find(|r| r.id == id)
            .ok_or_else(|| format!("Rule with id {} not found", id))?;
        rule.state = RuleState::Active;
        Ok(())
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, id: &str) -> Result<(), String> {
        let rule = self.rules.iter_mut().find(|r| r.id == id)
            .ok_or_else(|| format!("Rule with id {} not found", id))?;
        rule.state = RuleState::Disabled;
        Ok(())
    }

    /// Create a chain
    pub fn create_chain(&mut self, name: String, policy: ChainPolicy) -> Result<(), String> {
        if self.chains.iter().any(|c| c.name == name) {
            return Err(format!("Chain with name {} already exists", name));
        }
        
        let chain = FirewallManager2Chain {
            name,
            policy,
            state: ChainState::Active,
        };
        self.chains.push(chain);
        Ok(())
    }

    /// Remove a chain
    pub fn remove_chain(&mut self, name: &str) -> Result<(), String> {
        let pos = self.chains.iter().position(|c| c.name == name)
            .ok_or_else(|| format!("Chain with name {} not found", name))?;
        self.chains.remove(pos);
        Ok(())
    }

    /// Set chain policy
    pub fn set_chain_policy(&mut self, name: &str, policy: ChainPolicy) -> Result<(), String> {
        let chain = self.chains.iter_mut().find(|c| c.name == name)
            .ok_or_else(|| format!("Chain with name {} not found", name))?;
        chain.policy = policy;
        Ok(())
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[FirewallManager2Rule] {
        &self.rules
    }

    /// Get active rules
    pub fn get_active_rules(&self) -> Vec<&FirewallManager2Rule> {
        self.rules.iter().filter(|r| r.state == RuleState::Active).collect()
    }

    /// Get rules by chain
    pub fn get_rules_by_chain(&self, chain: &str) -> Vec<&FirewallManager2Rule> {
        self.rules.iter().filter(|r| r.chain == chain).collect()
    }

    /// Get rule by ID
    pub fn get_rule(&self, id: &str) -> Option<&FirewallManager2Rule> {
        self.rules.iter().find(|r| r.id == id)
    }

    /// Get all chains
    pub fn get_chains(&self) -> &[FirewallManager2Chain] {
        &self.chains
    }

    /// Get chain by name
    pub fn get_chain(&self, name: &str) -> Option<&FirewallManager2Chain> {
        self.chains.iter().find(|c| c.name == name)
    }
}

impl Default for SigmaFirewallManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_rule() {
        let mut manager = SigmaFirewallManager2::new();
        let result = manager.add_rule("rule1".to_string(), "INPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Tcp, RuleAction::Accept, 100);
        assert!(result.is_ok());
        assert_eq!(manager.get_rules().len(), 1);
    }

    #[test]
    fn test_duplicate_rule() {
        let mut manager = SigmaFirewallManager2::new();
        manager.add_rule("rule1".to_string(), "INPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Tcp, RuleAction::Accept, 100).unwrap();
        let result = manager.add_rule("rule1".to_string(), "OUTPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Udp, RuleAction::Drop, 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_enable_disable_rule() {
        let mut manager = SigmaFirewallManager2::new();
        manager.add_rule("rule1".to_string(), "INPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Tcp, RuleAction::Accept, 100).unwrap();
        
        manager.disable_rule("rule1").unwrap();
        assert_eq!(manager.get_rule("rule1").unwrap().state, RuleState::Disabled);
        
        manager.enable_rule("rule1").unwrap();
        assert_eq!(manager.get_rule("rule1").unwrap().state, RuleState::Active);
    }

    #[test]
    fn test_create_chain() {
        let mut manager = SigmaFirewallManager2::new();
        let result = manager.create_chain("INPUT".to_string(), ChainPolicy::Drop);
        assert!(result.is_ok());
        assert_eq!(manager.get_chains().len(), 1);
    }

    #[test]
    fn test_set_chain_policy() {
        let mut manager = SigmaFirewallManager2::new();
        manager.create_chain("INPUT".to_string(), ChainPolicy::Drop).unwrap();
        
        manager.set_chain_policy("INPUT", ChainPolicy::Accept).unwrap();
        assert_eq!(manager.get_chain("INPUT").unwrap().policy, ChainPolicy::Accept);
    }

    #[test]
    fn test_get_rules_by_chain() {
        let mut manager = SigmaFirewallManager2::new();
        manager.add_rule("rule1".to_string(), "INPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Tcp, RuleAction::Accept, 100).unwrap();
        manager.add_rule("rule2".to_string(), "OUTPUT".to_string(), "0.0.0.0/0".to_string(), "0.0.0.0/0".to_string(), Protocol::Udp, RuleAction::Drop, 200).unwrap();
        
        let input_rules = manager.get_rules_by_chain("INPUT");
        assert_eq!(input_rules.len(), 1);
        assert_eq!(input_rules[0].id, "rule1");
    }
}
