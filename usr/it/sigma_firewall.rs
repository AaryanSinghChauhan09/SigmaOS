// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_firewall.rs — Sigma Firewall (iptables/nftables)
//
// Implements iptables/nftables-style firewall with rule management,
// chain configuration, NAT, and packet filtering.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Firewall Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChainType {
    Input,
    Output,
    Forward,
    Prerouting,
    Postrouting,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Target {
    Accept,
    Drop,
    Reject,
    Log,
    Masquerade,
    DNAT,
    SNAT,
}

#[derive(Debug, Clone)]
pub struct FirewallRule {
    pub id: String,
    pub chain: ChainType,
    pub source: String,
    pub destination: String,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub protocol: String,
    pub target: Target,
    pub comment: String,
    pub packet_count: u64,
    pub byte_count: u64,
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub name: ChainType,
    pub policy: Target,
    pub rules: Vec<FirewallRule>,
}

#[derive(Debug, Clone)]
pub struct NATRule {
    pub id: String,
    pub chain_type: ChainType,
    pub source: String,
    pub destination: String,
    pub to_address: String,
    pub to_port: Option<u16>,
    pub protocol: String,
}

// ─── Firewall Manager ────────────────────────────────────────────────────

pub struct FirewallManager {
    pub chains: HashMap<ChainType, Chain>,
    pub nat_rules: Vec<NATRule>,
    pub default_policy: Target,
}

impl FirewallManager {
    pub fn new() -> Self {
        let mut manager = FirewallManager {
            chains: HashMap::new(),
            nat_rules: Vec::new(),
            default_policy: Target::Accept,
        };
        
        manager.init_default_chains();
        manager.init_sample_rules();
        manager
    }

    /// Initialize default chains
    fn init_default_chains(&mut self) {
        self.chains.insert(ChainType::Input, Chain {
            name: ChainType::Input,
            policy: Target::Accept,
            rules: Vec::new(),
        });
        
        self.chains.insert(ChainType::Output, Chain {
            name: ChainType::Output,
            policy: Target::Accept,
            rules: Vec::new(),
        });
        
        self.chains.insert(ChainType::Forward, Chain {
            name: ChainType::Forward,
            policy: Target::Drop,
            rules: Vec::new(),
        });
        
        self.chains.insert(ChainType::Prerouting, Chain {
            name: ChainType::Prerouting,
            policy: Target::Accept,
            rules: Vec::new(),
        });
        
        self.chains.insert(ChainType::Postrouting, Chain {
            name: ChainType::Postrouting,
            policy: Target::Accept,
            rules: Vec::new(),
        });
    }

    /// Initialize sample rules
    fn init_sample_rules(&mut self) {
        // Allow SSH
        let ssh_rule = FirewallRule {
            id: "rule_001".to_string(),
            chain: ChainType::Input,
            source: "0.0.0.0/0".to_string(),
            destination: "0.0.0.0/0".to_string(),
            source_port: None,
            destination_port: Some(22),
            protocol: "tcp".to_string(),
            target: Target::Accept,
            comment: "Allow SSH".to_string(),
            packet_count: 1523,
            byte_count: 1024 * 1024 * 5,
        };
        
        // Allow HTTP
        let http_rule = FirewallRule {
            id: "rule_002".to_string(),
            chain: ChainType::Input,
            source: "0.0.0.0/0".to_string(),
            destination: "0.0.0.0/0".to_string(),
            source_port: None,
            destination_port: Some(80),
            protocol: "tcp".to_string(),
            target: Target::Accept,
            comment: "Allow HTTP".to_string(),
            packet_count: 52341,
            byte_count: 1024 * 1024 * 500,
        };
        
        // Drop invalid packets
        let invalid_rule = FirewallRule {
            id: "rule_003".to_string(),
            chain: ChainType::Input,
            source: "0.0.0.0/0".to_string(),
            destination: "0.0.0.0/0".to_string(),
            source_port: None,
            destination_port: None,
            protocol: "all".to_string(),
            target: Target::Drop,
            comment: "Drop invalid packets".to_string(),
            packet_count: 23,
            byte_count: 2048,
        };
        
        if let Some(input_chain) = self.chains.get_mut(&ChainType::Input) {
            input_chain.rules.push(ssh_rule);
            input_chain.rules.push(http_rule);
            input_chain.rules.push(invalid_rule);
        }
    }

    /// Add rule to chain
    pub fn add_rule(&mut self, chain: ChainType, rule: FirewallRule) -> Result<(), String> {
        if let Some(chain_obj) = self.chains.get_mut(&chain) {
            chain_obj.rules.push(rule);
            Ok(())
        } else {
            Err("Chain not found".to_string())
        }
    }

    /// Remove rule
    pub fn remove_rule(&mut self, chain: ChainType, rule_id: &str) -> Result<(), String> {
        if let Some(chain_obj) = self.chains.get_mut(&chain) {
            if let Some(pos) = chain_obj.rules.iter().position(|r| r.id == rule_id) {
                chain_obj.rules.remove(pos);
                Ok(())
            } else {
                Err("Rule not found".to_string())
            }
        } else {
            Err("Chain not found".to_string())
        }
    }

    /// Set chain policy
    pub fn set_policy(&mut self, chain: ChainType, policy: Target) -> Result<(), String> {
        if let Some(chain_obj) = self.chains.get_mut(&chain) {
            chain_obj.policy = policy;
            Ok(())
        } else {
            Err("Chain not found".to_string())
        }
    }

    /// Add NAT rule
    pub fn add_nat_rule(&mut self, rule: NATRule) {
        self.nat_rules.push(rule);
    }

    /// Get chain by type
    pub fn get_chain(&self, chain_type: ChainType) -> Option<&Chain> {
        self.chains.get(&chain_type)
    }

    /// Get all chains
    pub fn get_all_chains(&self) -> Vec<&Chain> {
        self.chains.values().collect()
    }

    /// Get chain name
    pub fn get_chain_name(&self, chain_type: ChainType) -> &str {
        match chain_type {
            ChainType::Input => "INPUT",
            ChainType::Output => "OUTPUT",
            ChainType::Forward => "FORWARD",
            ChainType::Prerouting => "PREROUTING",
            ChainType::Postrouting => "POSTROUTING",
        }
    }

    /// Get target name
    pub fn get_target_name(&self, target: Target) -> &str {
        match target {
            Target::Accept => "ACCEPT",
            Target::Drop => "DROP",
            Target::Reject => "REJECT",
            Target::Log => "LOG",
            Target::Masquerade => "MASQUERADE",
            Target::DNAT => "DNAT",
            Target::SNAT => "SNAT",
        }
    }

    /// Format bytes
    fn format_bytes(&self, bytes: u64) -> String {
        if bytes >= 1024 * 1024 * 1024 {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if bytes >= 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = FirewallManager::new();
    
    println!("Sigma Firewall v0.1 - iptables/nftables Style");
    
    loop {
        println!("\n--- Firewall Status ---");
        let total_rules: usize = manager.chains.values().map(|c| c.rules.len()).sum();
        println!("Chains: {}", manager.chains.len());
        println!("Rules: {}", total_rules);
        println!("NAT Rules: {}", manager.nat_rules.len());
        
        println!("\nCommands: add_rule <chain> <src> <dst> <proto> <dport> <target> <comment>, remove_rule <chain> <rule_id>, set_policy <chain> <policy>, chains, chain <chain>, nat_rules, add_nat, quit");
        println!("Chains: input, output, forward, prerouting, postrouting");
        println!("Targets: accept, drop, reject, log");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "add_rule" => {
                if parts.len() >= 8 {
                    let chain = match parts[1] {
                        "input" => ChainType::Input,
                        "output" => ChainType::Output,
                        "forward" => ChainType::Forward,
                        "prerouting" => ChainType::Prerouting,
                        "postrouting" => ChainType::Postrouting,
                        _ => {
                            println!("Unknown chain");
                            continue;
                        }
                    };
                    let source = parts[2].to_string();
                    let destination = parts[3].to_string();
                    let protocol = parts[4].to_string();
                    let dest_port = parts[5].parse::<u16>().ok();
                    let target = match parts[6] {
                        "accept" => Target::Accept,
                        "drop" => Target::Drop,
                        "reject" => Target::Reject,
                        "log" => Target::Log,
                        _ => {
                            println!("Unknown target");
                            continue;
                        }
                    };
                    let comment = parts[7..].join(" ");
                    
                    let rule = FirewallRule {
                        id: format!("rule_{}", rand_id()),
                        chain,
                        source,
                        destination,
                        source_port: None,
                        destination_port: dest_port,
                        protocol,
                        target,
                        comment,
                        packet_count: 0,
                        byte_count: 0,
                    };
                    
                    match manager.add_rule(chain, rule) {
                        Ok(_) => println!("Rule added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_rule" => {
                if parts.len() >= 3 {
                    let chain = match parts[1] {
                        "input" => ChainType::Input,
                        "output" => ChainType::Output,
                        "forward" => ChainType::Forward,
                        "prerouting" => ChainType::Prerouting,
                        "postrouting" => ChainType::Postrouting,
                        _ => {
                            println!("Unknown chain");
                            continue;
                        }
                    };
                    match manager.remove_rule(chain, parts[2]) {
                        Ok(_) => println!("Rule removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_policy" => {
                if parts.len() >= 3 {
                    let chain = match parts[1] {
                        "input" => ChainType::Input,
                        "output" => ChainType::Output,
                        "forward" => ChainType::Forward,
                        "prerouting" => ChainType::Prerouting,
                        "postrouting" => ChainType::Postrouting,
                        _ => {
                            println!("Unknown chain");
                            continue;
                        }
                    };
                    let policy = match parts[2] {
                        "accept" => Target::Accept,
                        "drop" => Target::Drop,
                        "reject" => Target::Reject,
                        _ => {
                            println!("Unknown policy");
                            continue;
                        }
                    };
                    match manager.set_policy(chain, policy) {
                        Ok(_) => println!("Policy updated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "chains" => {
                println!("--- All Chains ---");
                for chain in manager.get_all_chains() {
                    println!("{} - Policy: {} ({} rules)", 
                        manager.get_chain_name(chain.name),
                        manager.get_target_name(chain.policy),
                        chain.rules.len()
                    );
                }
            }
            "chain" => {
                if let Some(arg) = parts.get(1) {
                    let chain = match *arg {
                        "input" => ChainType::Input,
                        "output" => ChainType::Output,
                        "forward" => ChainType::Forward,
                        "prerouting" => ChainType::Prerouting,
                        "postrouting" => ChainType::Postrouting,
                        _ => {
                            println!("Unknown chain");
                            continue;
                        }
                    };
                    if let Some(chain_obj) = manager.get_chain(chain) {
                        println!("--- Chain Details ---");
                        println!("Name: {}", manager.get_chain_name(chain_obj.name));
                        println!("Policy: {}", manager.get_target_name(chain_obj.policy));
                        println!("\n--- Rules ---");
                        for (i, rule) in chain_obj.rules.iter().enumerate() {
                            println!("{}. {} - {} -> {} ({})", 
                                i + 1,
                                rule.protocol,
                                rule.source,
                                rule.destination,
                                manager.get_target_name(rule.target)
                            );
                            if let Some(port) = rule.destination_port {
                                println!("   dport: {}", port);
                            }
                            println!("   Comment: {}", rule.comment);
                            println!("   Packets: {}, Bytes: {}", rule.packet_count, manager.format_bytes(rule.byte_count));
                        }
                    }
                }
            }
            "nat_rules" => {
                println!("--- NAT Rules ---");
                for rule in &manager.nat_rules {
                    println!("{} - {} -> {} ({})", rule.id, rule.source, rule.destination, rule.protocol);
                    println!("  To: {}", rule.to_address);
                }
            }
            "add_nat" => {
                println!("Enter chain type (prerouting/postrouting):");
                let mut chain_str = String::new();
                std::io::stdin().read_line(&mut chain_str).unwrap();
                
                let chain_type = match chain_str.trim() {
                    "prerouting" => ChainType::Prerouting,
                    "postrouting" => ChainType::Postrouting,
                    _ => {
                        println!("Invalid chain");
                        continue;
                    }
                };
                
                println!("Enter source:");
                let mut source = String::new();
                std::io::stdin().read_line(&mut source).unwrap();
                
                println!("Enter destination:");
                let mut destination = String::new();
                std::io::stdin().read_line(&mut destination).unwrap();
                
                println!("Enter to address:");
                let mut to_address = String::new();
                std::io::stdin().read_line(&mut to_address).unwrap();
                
                println!("Enter protocol:");
                let mut protocol = String::new();
                std::io::stdin().read_line(&mut protocol).unwrap();
                
                let rule = NATRule {
                    id: format!("nat_{}", rand_id()),
                    chain_type,
                    source: source.trim().to_string(),
                    destination: destination.trim().to_string(),
                    to_address: to_address.trim().to_string(),
                    to_port: None,
                    protocol: protocol.trim().to_string(),
                };
                
                manager.add_nat_rule(rule);
                println!("NAT rule added");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u32
}
