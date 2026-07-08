// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// net/firewall/firewall.rs — SigmaOS Firewall Module
//
// Implements a packet filtering firewall with user-defined rules.
// Supports iptables/nftables compatibility.
//
// Language: Rust (no_std, no alloc)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type Bool = bool;

// ─── Firewall Error Codes ─────────────────────────────────────────────────────

pub const FW_OK: I32 = 0;
pub const FW_ERR_NULL_PTR: I32 = -1;
pub const FW_ERR_INVALID_RULE: I32 = -2;
pub const FW_ERR_TABLE_FULL: I32 = -3;
pub const FW_ERR_CHAIN_NOT_FOUND: I32 = -4;
pub const FW_ERR_PERMISSION_DENIED: I32 = -5;

// ─── Firewall Actions ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
    Log,
    Jump,
    Return,
}

// ─── Firewall Hooks ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirewallHook {
    Prerouting,
    Input,
    Forward,
    Output,
    Postrouting,
}

// ─── Firewall Protocols ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FirewallProtocol {
    All,
    TCP,
    UDP,
    ICMP,
    ICMPv6,
    GRE,
    ESP,
    AH,
}

// ─── Firewall Rule Match ─────────────────────────────────────────────────

#[repr(C)]
pub struct FirewallMatch {
    pub src_ip: [U8; 16],      // IPv4 or IPv6 address
    pub src_ip_len: U8,         // 4 for IPv4, 16 for IPv6
    pub src_mask: [U8; 16],     // Subnet mask
    pub src_mask_len: U8,
    pub dst_ip: [U8; 16],
    pub dst_ip_len: U8,
    pub dst_mask: [U8; 16],
    pub dst_mask_len: U8,
    pub src_port_start: U16,
    pub src_port_end: U16,
    pub dst_port_start: U16,
    pub dst_port_end: U16,
    pub protocol: FirewallProtocol,
    pub interface: [U8; 16],     // Interface name
    pub interface_len: U8,
    pub in_interface: [U8; 16],
    pub in_interface_len: U8,
    pub out_interface: [U8; 16],
    pub out_interface_len: U8,
}

impl FirewallMatch {
    pub const fn new() -> Self {
        FirewallMatch {
            src_ip: [0; 16],
            src_ip_len: 0,
            src_mask: [0; 16],
            src_mask_len: 0,
            dst_ip: [0; 16],
            dst_ip_len: 0,
            dst_mask: [0; 16],
            dst_mask_len: 0,
            src_port_start: 0,
            src_port_end: 0,
            dst_port_start: 0,
            dst_port_end: 0,
            protocol: FirewallProtocol::All,
            interface: [0; 16],
            interface_len: 0,
            in_interface: [0; 16],
            in_interface_len: 0,
            out_interface: [0; 16],
            out_interface_len: 0,
        }
    }
}

// ─── Firewall Rule ───────────────────────────────────────────────────────

#[repr(C)]
pub struct FirewallRule {
    pub match: FirewallMatch,
    pub action: FirewallAction,
    pub target_chain: [U8; 32],    // For Jump action
    pub target_chain_len: U8,
    pub log_prefix: [U8; 64],      // For Log action
    pub log_prefix_len: U8,
    pub enabled: Bool,
    pub rule_id: U32,
}

impl FirewallRule {
    pub const fn new() -> Self {
        FirewallRule {
            match: FirewallMatch::new(),
            action: FirewallAction::Accept,
            target_chain: [0; 32],
            target_chain_len: 0,
            log_prefix: [0; 64],
            log_prefix_len: 0,
            enabled: true,
            rule_id: 0,
        }
    }
}

// ─── Firewall Chain ─────────────────────────────────────────────────────

#[repr(C)]
pub struct FirewallChain {
    pub name: [U8; 32],
    pub name_len: U8,
    pub hook: FirewallHook,
    pub policy: FirewallAction,    // Default policy
    pub rules: [FirewallRule; 256],
    pub rule_count: U32,
    pub enabled: Bool,
}

impl FirewallChain {
    pub const fn new() -> Self {
        FirewallChain {
            name: [0; 32],
            name_len: 0,
            hook: FirewallHook::Input,
            policy: FirewallAction::Accept,
            rules: [FirewallRule::new(); 256],
            rule_count: 0,
            enabled: true,
        }
    }
}

// ─── Firewall Table ─────────────────────────────────────────────────────

pub struct FirewallTable {
    pub name: [U8; 32],
    pub name_len: U8,
    pub chains: [FirewallChain; 8],
    pub chain_count: U32,
    pub enabled: Bool,
}

impl FirewallTable {
    pub const fn new() -> Self {
        FirewallTable {
            name: [0; 32],
            name_len: 0,
            chains: [FirewallChain::new(); 8],
            chain_count: 0,
            enabled: true,
        }
    }
}

// ─── Firewall Trait ─────────────────────────────────────────────────────

/// Trait for firewall implementations
pub trait Firewall {
    /// Add a rule to a chain
    fn add_rule(&mut self, table_name: &[U8], chain_name: &[U8], rule: &FirewallRule) -> I32;
    
    /// Remove a rule from a chain
    fn remove_rule(&mut self, table_name: &[U8], chain_name: &[U8], rule_id: U32) -> I32;
    
    /// Get rules in a chain
    fn get_rules(&self, table_name: &[U8], chain_name: &[U8], rules: &mut [FirewallRule], count: &mut U32) -> I32;
    
    /// Enable/disable a rule
    fn set_rule_enabled(&mut self, table_name: &[U8], chain_name: &[U8], rule_id: U32, enabled: Bool) -> I32;
    
    /// Set chain policy
    fn set_chain_policy(&mut self, table_name: &[U8], chain_name: &[U8], policy: FirewallAction) -> I32;
    
    /// Evaluate packet against rules
    fn evaluate_packet(&self, hook: FirewallHook, packet: &[U8]) -> FirewallAction;
    
    /// Flush all rules in a chain
    fn flush_chain(&mut self, table_name: &[U8], chain_name: &[U8]) -> I32;
    
    /// Get firewall statistics
    fn get_stats(&self) -> FirewallStats;
}

// ─── Firewall Statistics ─────────────────────────────────────────────────

#[repr(C)]
pub struct FirewallStats {
    pub total_packets: U64,
    pub accepted_packets: U64,
    pub dropped_packets: U64,
    pub rejected_packets: U64,
    pub logged_packets: U64,
    pub total_bytes: U64,
}

impl FirewallStats {
    pub const fn new() -> Self {
        FirewallStats {
            total_packets: 0,
            accepted_packets: 0,
            dropped_packets: 0,
            rejected_packets: 0,
            logged_packets: 0,
            total_bytes: 0,
        }
    }
}

// ─── SigmaOS Firewall Implementation ─────────────────────────────────────

pub struct SigmaFirewall {
    tables: [FirewallTable; 4],
    table_count: U32,
    stats: FirewallStats,
}

impl SigmaFirewall {
    pub const fn new() -> Self {
        SigmaFirewall {
            tables: [FirewallTable::new(); 4],
            table_count: 0,
            stats: FirewallStats::new(),
        }
    }

    /// Find table by name
    fn find_table(&self, name: &[U8]) -> Option<usize> {
        for i in 0..self.table_count as usize {
            let table = &self.tables[i];
            if table.name_len as usize == name.len() {
                let mut matches = true;
                for j in 0..name.len() {
                    if table.name[j] != name[j] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Find chain by name in table
    fn find_chain(&self, table_idx: usize, name: &[U8]) -> Option<usize> {
        let table = &self.tables[table_idx];
        for i in 0..table.chain_count as usize {
            let chain = &table.chains[i];
            if chain.name_len as usize == name.len() {
                let mut matches = true;
                for j in 0..name.len() {
                    if chain.name[j] != name[j] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Check if packet matches rule
    fn matches_rule(&self, rule: &FirewallRule, packet: &[U8]) -> Bool {
        // In a real implementation, this would:
        // 1. Parse packet headers (IP, TCP, UDP, etc.)
        // 2. Compare source/destination IP addresses
        // 3. Compare source/destination ports
        // 4. Compare protocol
        // 5. Compare interface names
        
        // Stub: always match
        true
    }

    /// Initialize default tables and chains
    pub unsafe fn init(&mut self) -> I32 {
        // Create filter table
        let filter_name = b"filter";
        self.tables[0].name_len = filter_name.len() as U8;
        for i in 0..filter_name.len() {
            self.tables[0].name[i] = filter_name[i];
        }
        self.tables[0].enabled = true;

        // Create INPUT chain
        let input_name = b"INPUT";
        self.tables[0].chains[0].name_len = input_name.len() as U8;
        for i in 0..input_name.len() {
            self.tables[0].chains[0].name[i] = input_name[i];
        }
        self.tables[0].chains[0].hook = FirewallHook::Input;
        self.tables[0].chains[0].policy = FirewallAction::Accept;
        self.tables[0].chains[0].enabled = true;
        self.tables[0].chain_count = 1;

        // Create OUTPUT chain
        let output_name = b"OUTPUT";
        self.tables[0].chains[1].name_len = output_name.len() as U8;
        for i in 0..output_name.len() {
            self.tables[0].chains[1].name[i] = output_name[i];
        }
        self.tables[0].chains[1].hook = FirewallHook::Output;
        self.tables[0].chains[1].policy = FirewallAction::Accept;
        self.tables[0].chains[1].enabled = true;
        self.tables[0].chain_count = 2;

        // Create FORWARD chain
        let forward_name = b"FORWARD";
        self.tables[0].chains[2].name_len = forward_name.len() as U8;
        for i in 0..forward_name.len() {
            self.tables[0].chains[2].name[i] = forward_name[i];
        }
        self.tables[0].chains[2].hook = FirewallHook::Forward;
        self.tables[0].chains[2].policy = FirewallAction::Accept;
        self.tables[0].chains[2].enabled = true;
        self.tables[0].chain_count = 3;

        self.table_count = 1;
        FW_OK
    }
}

impl Firewall for SigmaFirewall {
    fn add_rule(&mut self, table_name: &[U8], chain_name: &[U8], rule: &FirewallRule) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain = &mut self.tables[table_idx].chains[chain_idx];
        if chain.rule_count >= 256 {
            return FW_ERR_TABLE_FULL;
        }

        let rule_idx = chain.rule_count as usize;
        chain.rules[rule_idx] = *rule;
        chain.rules[rule_idx].rule_id = chain.rule_count + 1;
        chain.rule_count += 1;

        FW_OK
    }

    fn remove_rule(&mut self, table_name: &[U8], chain_name: &[U8], rule_id: U32) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain = &mut self.tables[table_idx].chains[chain_idx];
        
        // Find and remove rule
        for i in 0..chain.rule_count as usize {
            if chain.rules[i].rule_id == rule_id {
                // Shift remaining rules
                for j in i..(chain.rule_count as usize - 1) {
                    chain.rules[j] = chain.rules[j + 1];
                }
                chain.rule_count -= 1;
                return FW_OK;
            }
        }

        FW_ERR_INVALID_RULE
    }

    fn get_rules(&self, table_name: &[U8], chain_name: &[U8], rules: &mut [FirewallRule], count: &mut U32) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain = &self.tables[table_idx].chains[chain_idx];
        let copy_count = chain.rule_count.min(rules.len() as U32);

        for i in 0..copy_count as usize {
            rules[i] = chain.rules[i];
        }

        *count = copy_count;
        FW_OK
    }

    fn set_rule_enabled(&mut self, table_name: &[U8], chain_name: &[U8], rule_id: U32, enabled: Bool) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain = &mut self.tables[table_idx].chains[chain_idx];
        
        for i in 0..chain.rule_count as usize {
            if chain.rules[i].rule_id == rule_id {
                chain.rules[i].enabled = enabled;
                return FW_OK;
            }
        }

        FW_ERR_INVALID_RULE
    }

    fn set_chain_policy(&mut self, table_name: &[U8], chain_name: &[U8], policy: FirewallAction) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        self.tables[table_idx].chains[chain_idx].policy = policy;
        FW_OK
    }

    fn evaluate_packet(&self, hook: FirewallHook, packet: &[U8]) -> FirewallAction {
        // Find chain for this hook
        for table_idx in 0..self.table_count as usize {
            for chain_idx in 0..self.tables[table_idx].chain_count as usize {
                let chain = &self.tables[table_idx].chains[chain_idx];
                if chain.hook == hook && chain.enabled {
                    // Evaluate rules in order
                    for rule_idx in 0..chain.rule_count as usize {
                        let rule = &chain.rules[rule_idx];
                        if rule.enabled && self.matches_rule(rule, packet) {
                            return rule.action;
                        }
                    }
                    // No rule matched, use chain policy
                    return chain.policy;
                }
            }
        }
        
        // Default: accept
        FirewallAction::Accept
    }

    fn flush_chain(&mut self, table_name: &[U8], chain_name: &[U8]) -> I32 {
        let table_idx = match self.find_table(table_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        let chain_idx = match self.find_chain(table_idx, chain_name) {
            Some(idx) => idx,
            None => return FW_ERR_CHAIN_NOT_FOUND,
        };

        self.tables[table_idx].chains[chain_idx].rule_count = 0;
        FW_OK
    }

    fn get_stats(&self) -> FirewallStats {
        self.stats
    }
}

// ─── Global Firewall Instance ─────────────────────────────────────────────

static mut GLOBAL_FIREWALL: SigmaFirewall = SigmaFirewall::new();

// ─── C-ABI Exports ─────────────────────────────────────────────────────────

/// Get global firewall
pub unsafe fn get_firewall() -> &'static mut SigmaFirewall {
    &mut GLOBAL_FIREWALL
}

/// Initialize firewall
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_init() -> I32 {
    GLOBAL_FIREWALL.init()
}

/// Add rule to firewall
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_add_rule(
    table_name: *const U8,
    table_name_len: usize,
    chain_name: *const U8,
    chain_name_len: usize,
    rule: *const FirewallRule,
) -> I32 {
    if table_name.is_null() || chain_name.is_null() || rule.is_null() {
        return FW_ERR_NULL_PTR;
    }

    let table_slice = core::slice::from_raw_parts(table_name, table_name_len);
    let chain_slice = core::slice::from_raw_parts(chain_name, chain_name_len);
    let rule_ref = &*rule;

    GLOBAL_FIREWALL.add_rule(table_slice, chain_slice, rule_ref)
}

/// Remove rule from firewall
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_remove_rule(
    table_name: *const U8,
    table_name_len: usize,
    chain_name: *const U8,
    chain_name_len: usize,
    rule_id: U32,
) -> I32 {
    if table_name.is_null() || chain_name.is_null() {
        return FW_ERR_NULL_PTR;
    }

    let table_slice = core::slice::from_raw_parts(table_name, table_name_len);
    let chain_slice = core::slice::from_raw_parts(chain_name, chain_name_len);

    GLOBAL_FIREWALL.remove_rule(table_slice, chain_slice, rule_id)
}

/// Evaluate packet against firewall
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_evaluate_packet(
    hook: U32,
    packet: *const U8,
    packet_len: usize,
) -> I32 {
    if packet.is_null() {
        return FW_ERR_NULL_PTR;
    }

    let packet_slice = core::slice::from_raw_parts(packet, packet_len);
    let firewall_hook = match hook {
        0 => FirewallHook::Prerouting,
        1 => FirewallHook::Input,
        2 => FirewallHook::Forward,
        3 => FirewallHook::Output,
        4 => FirewallHook::Postrouting,
        _ => return FW_ERR_INVALID_RULE,
    };

    let action = GLOBAL_FIREWALL.evaluate_packet(firewall_hook, packet_slice);
    match action {
        FirewallAction::Accept => 0,
        FirewallAction::Drop => 1,
        FirewallAction::Reject => 2,
        FirewallAction::Log => 3,
        FirewallAction::Jump => 4,
        FirewallAction::Return => 5,
    }
}

/// Set chain policy
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_set_chain_policy(
    table_name: *const U8,
    table_name_len: usize,
    chain_name: *const U8,
    chain_name_len: usize,
    policy: U32,
) -> I32 {
    if table_name.is_null() || chain_name.is_null() {
        return FW_ERR_NULL_PTR;
    }

    let table_slice = core::slice::from_raw_parts(table_name, table_name_len);
    let chain_slice = core::slice::from_raw_parts(chain_name, chain_name_len);
    let firewall_policy = match policy {
        0 => FirewallAction::Accept,
        1 => FirewallAction::Drop,
        2 => FirewallAction::Reject,
        _ => return FW_ERR_INVALID_RULE,
    };

    GLOBAL_FIREWALL.set_chain_policy(table_slice, chain_slice, firewall_policy)
}

/// Flush chain
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_flush_chain(
    table_name: *const U8,
    table_name_len: usize,
    chain_name: *const U8,
    chain_name_len: usize,
) -> I32 {
    if table_name.is_null() || chain_name.is_null() {
        return FW_ERR_NULL_PTR;
    }

    let table_slice = core::slice::from_raw_parts(table_name, table_name_len);
    let chain_slice = core::slice::from_raw_parts(chain_name, chain_name_len);

    GLOBAL_FIREWALL.flush_chain(table_slice, chain_slice)
}

/// Get firewall statistics
#[no_mangle]
pub unsafe extern "C" fn sigma_firewall_get_stats() -> FirewallStats {
    GLOBAL_FIREWALL.get_stats()
}
