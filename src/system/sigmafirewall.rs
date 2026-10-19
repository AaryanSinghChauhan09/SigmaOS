//! SigmaFirewall - Firewall Management System
//!
//! Provides comprehensive firewall management inspired by Linux iptables, nftables, and UFW.
//! Supports rule management, chain management, policy enforcement, and network filtering.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
    Log,
    Unknown,
;

/// Firewall protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FirewallProtocol {
    Tcp,
    Udp,
    Icmp,
    All,
    Unknown,
;

/// Firewall chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FirewallChain {
    Input,
    Output,
    Forward,
    Unknown,
;

/// Firewall rule state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleState {
    Active,
    Inactive,
    Expired,
    Unknown,
;

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id: u64,
    pub chain: FirewallChain,
    pub action: FirewallAction,
    pub protocol: FirewallProtocol,
    pub source_ip: Option<String>,
    pub source_port: Option<u16>,
    pub dest_ip: Option<String>,
    pub dest_port: Option<u16>,
    pub state: RuleState,
    pub priority: i32,
    pub description: String,
;

impl FirewallRule {
    pub fn new(id: u64, chain: FirewallChain, action: FirewallAction) -> Self {
        Self {
            id,
            chain,
            action,
            protocol: FirewallProtocol::All,
            source_ip: None,
            source_port: None,
            dest_ip: None,
            dest_port: None,
            state: RuleState::Active,
            priority: 0,
            description: String::new(),
        }
    }

    pub fn with_protocol(mut self, protocol: FirewallProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_source(mut self, ip: String, port: Option<u16>) -> Self {
        self.source_ip = Some(ip);
        self.source_port = port;
        self
    }

    pub fn with_destination(mut self, ip: String, port: Option<u16>) -> Self {
        self.dest_ip = Some(ip);
        self.dest_port = port;
        self
    }

    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn activate(&mut self) {
        self.state = RuleState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = RuleState::Inactive;
    }

    pub fn expire(&mut self) {
        self.state = RuleState::Expired;
    }
;

/// Firewall manager
pub struct SigmaFirewall {
    rules: BTreeMap<u64, FirewallRule>,
    next_rule_id: u64,
    default_input_policy: FirewallAction,
    default_output_policy: FirewallAction,
    default_forward_policy: FirewallAction,
    enabled: bool,
;

impl SigmaFirewall {
    pub fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
            next_rule_id: 1,
            default_input_policy: FirewallAction::Accept,
            default_output_policy: FirewallAction::Accept,
            default_forward_policy: FirewallAction::Drop,
            enabled: false,
        }
    }

    pub fn add_rule(&mut self, rule: FirewallRule) -> u64 {
        let id = rule.id;
        self.rules.insert(id, rule);
        id
    }

    pub fn remove_rule(&mut self, id: u64) -> bool {
        self.rules.remove(&id).is_some()
    }

    pub fn get_rule(&self, id: u64) -> Option<&FirewallRule> {
        self.rules.get(&id)
    }

    pub fn get_rule_mut(&mut self, id: u64) -> Option<&mut FirewallRule> {
        self.rules.get_mut(&id)
    }

    pub fn list_rules(&self) -> Vec<&FirewallRule> {
        self.rules.values().collect()
    }

    pub fn list_rules_by_chain(&self, chain: FirewallChain) -> Vec<&FirewallRule> {
        self.rules.values().filter(|r| r.chain == chain).collect()
    }

    pub fn list_rules_by_state(&self, state: RuleState) -> Vec<&FirewallRule> {
        self.rules.values().filter(|r| r.state == state).collect()
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_default_policy(&mut self, chain: FirewallChain, action: FirewallAction) {
        match chain {
            FirewallChain::Input => self.default_input_policy = action,
            FirewallChain::Output => self.default_output_policy = action,
            FirewallChain::Forward => self.default_forward_policy = action,
            _ => {}
        }
    }

    pub fn get_default_policy(&self, chain: FirewallChain) -> FirewallAction {
        match chain {
            FirewallChain::Input => self.default_input_policy,
            FirewallChain::Output => self.default_output_policy,
            FirewallChain::Forward => self.default_forward_policy,
            _ => FirewallAction::Unknown,
        }
    }

    pub fn activate_rule(&mut self, id: u64) -> bool {
        if let Some(rule) = self.rules.get_mut(&id) {
            rule.activate();
            true
        } else {
            false
        }
    }

    pub fn deactivate_rule(&mut self, id: u64) -> bool {
        if let Some(rule) = self.rules.get_mut(&id) {
            rule.deactivate();
            true
        } else {
            false
        }
    }

    pub fn flush_chain(&mut self, chain: FirewallChain) {
        self.rules.retain(|_, r| r.chain != chain);
    }

    pub fn flush_all(&mut self) {
        self.rules.clear();
    }

    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    pub fn active_rule_count(&self) -> usize {
        self.rules
            .values()
            .filter(|r| r.state == RuleState::Active)
            .count()
    }
;

impl Default for SigmaFirewall {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAFIREWALL_INSTANCE: Option<SigmaFirewall> = None;

pub fn init_sigmafirewall() {
    unsafe {
        SIGMAFIREWALL_INSTANCE = Some(SigmaFirewall::new());
    }
;

pub fn get_sigmafirewall() -> &'static SigmaFirewall {
    unsafe {
        SIGMAFIREWALL_INSTANCE
            .as_ref()
            .expect("SigmaFirewall not initialized")
    }
;

pub fn get_sigmafirewall_mut() -> &'static mut SigmaFirewall {
    unsafe {
        SIGMAFIREWALL_INSTANCE
            .as_mut()
            .expect("SigmaFirewall not initialized")
    }
;

pub fn create_rule(chain: FirewallChain, action: FirewallAction) -> u64 {
    let firewall = get_sigmafirewall_mut();
    let id = firewall.next_rule_id;
    firewall.next_rule_id += 1;
    let rule = FirewallRule::new(id, chain, action);
    firewall.add_rule(rule);
    id
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_rule_creation() {
        let rule = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Accept);
        assert_eq!(rule.id, 1);
        assert_eq!(rule.chain, FirewallChain::Input);
        assert_eq!(rule.action, FirewallAction::Accept);
        assert_eq!(rule.state, RuleState::Active);
    }

    #[test]
    fn test_firewall_rule_builder() {
        let rule = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Drop)
            .with_protocol(FirewallProtocol::Tcp)
            .with_source("192.168.1.1".to_string(), Some(80))
            .with_destination("10.0.0.1".to_string(), Some(443))
            .with_priority(10)
            .with_description("Block HTTP".to_string());

        assert_eq!(rule.protocol, FirewallProtocol::Tcp);
        assert_eq!(rule.source_ip, Some("192.168.1.1".to_string()));
        assert_eq!(rule.source_port, Some(80));
        assert_eq!(rule.dest_ip, Some("10.0.0.1".to_string()));
        assert_eq!(rule.dest_port, Some(443));
        assert_eq!(rule.priority, 10);
        assert_eq!(rule.description, "Block HTTP");
    }

    #[test]
    fn test_firewall_rule_state_transitions() {
        let mut rule = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Accept);
        assert_eq!(rule.state, RuleState::Active);

        rule.deactivate();
        assert_eq!(rule.state, RuleState::Inactive);

        rule.activate();
        assert_eq!(rule.state, RuleState::Active);

        rule.expire();
        assert_eq!(rule.state, RuleState::Expired);
    }

    #[test]
    fn test_firewall_manager() {
        let mut firewall = SigmaFirewall::new();
        assert_eq!(firewall.rule_count(), 0);
        assert!(!firewall.is_enabled());
    }

    #[test]
    fn test_add_remove_rule() {
        let mut firewall = SigmaFirewall::new();
        let rule = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Accept);
        firewall.add_rule(rule);
        assert_eq!(firewall.rule_count(), 1);
        assert!(firewall.get_rule(1).is_some());

        assert!(firewall.remove_rule(1));
        assert_eq!(firewall.rule_count(), 0);
        assert!(firewall.get_rule(1).is_none());
    }

    #[test]
    fn test_list_rules_by_chain() {
        let mut firewall = SigmaFirewall::new();
        firewall.add_rule(FirewallRule::new(
            1,
            FirewallChain::Input,
            FirewallAction::Accept,
        ));
        firewall.add_rule(FirewallRule::new(
            2,
            FirewallChain::Output,
            FirewallAction::Drop,
        ));
        firewall.add_rule(FirewallRule::new(
            3,
            FirewallChain::Input,
            FirewallAction::Reject,
        ));

        let input_rules = firewall.list_rules_by_chain(FirewallChain::Input);
        assert_eq!(input_rules.len(), 2);

        let output_rules = firewall.list_rules_by_chain(FirewallChain::Output);
        assert_eq!(output_rules.len(), 1);
    }

    #[test]
    fn test_list_rules_by_state() {
        let mut firewall = SigmaFirewall::new();
        let mut rule1 = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Accept);
        let mut rule2 = FirewallRule::new(2, FirewallChain::Output, FirewallAction::Drop);
        rule2.deactivate();
        firewall.add_rule(rule1);
        firewall.add_rule(rule2);

        let active_rules = firewall.list_rules_by_state(RuleState::Active);
        assert_eq!(active_rules.len(), 1);

        let inactive_rules = firewall.list_rules_by_state(RuleState::Inactive);
        assert_eq!(inactive_rules.len(), 1);
    }

    #[test]
    fn test_enable_disable_firewall() {
        let mut firewall = SigmaFirewall::new();
        assert!(!firewall.is_enabled());

        firewall.enable();
        assert!(firewall.is_enabled());

        firewall.disable();
        assert!(!firewall.is_enabled());
    }

    #[test]
    fn test_default_policy() {
        let mut firewall = SigmaFirewall::new();
        assert_eq!(
            firewall.get_default_policy(FirewallChain::Input),
            FirewallAction::Accept
        );
        assert_eq!(
            firewall.get_default_policy(FirewallChain::Forward),
            FirewallAction::Drop
        );

        firewall.set_default_policy(FirewallChain::Input, FirewallAction::Drop);
        assert_eq!(
            firewall.get_default_policy(FirewallChain::Input),
            FirewallAction::Drop
        );
    }

    #[test]
    fn test_activate_deactivate_rule() {
        let mut firewall = SigmaFirewall::new();
        firewall.add_rule(FirewallRule::new(
            1,
            FirewallChain::Input,
            FirewallAction::Accept,
        ));

        assert!(firewall.activate_rule(1));
        assert_eq!(firewall.get_rule(1).unwrap().state, RuleState::Active);

        assert!(firewall.deactivate_rule(1));
        assert_eq!(firewall.get_rule(1).unwrap().state, RuleState::Inactive);
    }

    #[test]
    fn test_flush_chain() {
        let mut firewall = SigmaFirewall::new();
        firewall.add_rule(FirewallRule::new(
            1,
            FirewallChain::Input,
            FirewallAction::Accept,
        ));
        firewall.add_rule(FirewallRule::new(
            2,
            FirewallChain::Output,
            FirewallAction::Drop,
        ));
        firewall.add_rule(FirewallRule::new(
            3,
            FirewallChain::Input,
            FirewallAction::Reject,
        ));

        firewall.flush_chain(FirewallChain::Input);
        assert_eq!(firewall.rule_count(), 1);
        assert!(firewall.get_rule(2).is_some());
    }

    #[test]
    fn test_flush_all() {
        let mut firewall = SigmaFirewall::new();
        firewall.add_rule(FirewallRule::new(
            1,
            FirewallChain::Input,
            FirewallAction::Accept,
        ));
        firewall.add_rule(FirewallRule::new(
            2,
            FirewallChain::Output,
            FirewallAction::Drop,
        ));

        firewall.flush_all();
        assert_eq!(firewall.rule_count(), 0);
    }

    #[test]
    fn test_active_rule_count() {
        let mut firewall = SigmaFirewall::new();
        let mut rule1 = FirewallRule::new(1, FirewallChain::Input, FirewallAction::Accept);
        let mut rule2 = FirewallRule::new(2, FirewallChain::Output, FirewallAction::Drop);
        rule2.deactivate();
        firewall.add_rule(rule1);
        firewall.add_rule(rule2);

        assert_eq!(firewall.active_rule_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmafirewall();
        let firewall = get_sigmafirewall();
        assert!(!firewall.is_enabled());

        let firewall_mut = get_sigmafirewall_mut();
        firewall_mut.enable();
        assert!(firewall.is_enabled());
    }

    #[test]
    fn test_create_rule_convenience() {
        init_sigmafirewall();
        let id = create_rule(FirewallChain::Input, FirewallAction::Accept);
        assert_eq!(id, 1);

        let firewall = get_sigmafirewall();
        assert!(firewall.get_rule(id).is_some());
    }
;
