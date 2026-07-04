// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_sandbox.rs — Sigma Cybersecurity Sandbox
//
// Implements safe sandboxed environments for demonstrating malware,
// firewalls, encryption, and other security concepts for IT training.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Sandbox Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SandboxType {
    MalwareAnalysis,
    FirewallSimulation,
    EncryptionDemo,
    NetworkSecurity,
    PenetrationTesting,
}

#[derive(Debug, Clone)]
pub struct SandboxEnvironment {
    pub id: String,
    pub name: String,
    pub sandbox_type: SandboxType,
    pub is_active: bool,
    pub resources: HashMap<String, String>,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityScenario {
    pub id: String,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub steps: Vec<String>,
    pub learning_objectives: Vec<String>,
}

// ─── Cybersecurity Sandbox Manager ───────────────────────────────────────────

pub struct SandboxManager {
    pub environments: HashMap<String, SandboxEnvironment>,
    pub scenarios: HashMap<String, SecurityScenario>,
    pub current_environment: Option<String>,
}

impl SandboxManager {
    pub fn new() -> Self {
        let mut manager = SandboxManager {
            environments: HashMap::new(),
            scenarios: HashMap::new(),
            current_environment: None,
        };
        
        manager.init_scenarios();
        manager
    }

    /// Initialize security scenarios
    fn init_scenarios(&mut self) {
        // Malware Analysis Scenario
        self.scenarios.insert("malware_1".to_string(), SecurityScenario {
            id: "malware_1".to_string(),
            title: "Basic Malware Analysis".to_string(),
            description: "Analyze a simple file infector malware in a safe sandbox environment".to_string(),
            difficulty: "Easy".to_string(),
            steps: vec![
                "Create isolated sandbox environment".to_string(),
                "Load sample malware file (stub)".to_string(),
                "Monitor file system changes".to_string(),
                "Analyze network connections".to_string(),
                "Document malware behavior".to_string(),
            ],
            learning_objectives: vec![
                "Understand sandbox isolation principles".to_string(),
                "Learn malware behavior analysis".to_string(),
                "Practice safe malware handling".to_string(),
            ],
        });

        // Firewall Simulation Scenario
        self.scenarios.insert("firewall_1".to_string(), SecurityScenario {
            id: "firewall_1".to_string(),
            title: "Firewall Rule Configuration".to_string(),
            description: "Configure firewall rules to block specific traffic patterns".to_string(),
            difficulty: "Medium".to_string(),
            steps: vec![
                "Set up virtual network topology".to_string(),
                "Configure inbound/outbound rules".to_string(),
                "Test rule effectiveness".to_string(),
                "Analyze blocked traffic logs".to_string(),
            ],
            learning_objectives: vec![
                "Understand firewall rule syntax".to_string(),
                "Learn traffic filtering concepts".to_string(),
                "Practice security policy implementation".to_string(),
            ],
        });

        // Encryption Demo Scenario
        self.scenarios.insert("encrypt_1".to_string(), SecurityScenario {
            id: "encrypt_1".to_string(),
            title: "Symmetric vs Asymmetric Encryption".to_string(),
            description: "Compare and demonstrate symmetric and asymmetric encryption methods".to_string(),
            difficulty: "Easy".to_string(),
            steps: vec![
                "Generate symmetric key (AES)".to_string(),
                "Encrypt sample data".to_string(),
                "Generate asymmetric key pair (RSA)".to_string(),
                "Encrypt with public key".to_string(),
                "Decrypt with private key".to_string(),
                "Compare performance and security".to_string(),
            ],
            learning_objectives: vec![
                "Understand symmetric encryption".to_string(),
                "Understand asymmetric encryption".to_string(),
                "Learn key management basics".to_string(),
            ],
        });

        // Network Security Scenario
        self.scenarios.insert("network_1".to_string(), SecurityScenario {
            id: "network_1".to_string(),
            title: "Packet Sniffing and Analysis".to_string(),
            description: "Capture and analyze network packets in a controlled environment".to_string(),
            difficulty: "Medium".to_string(),
            steps: vec![
                "Set up virtual network interface".to_string(),
                "Configure packet capture".to_string(),
                "Generate sample traffic".to_string(),
                "Analyze captured packets".to_string(),
                "Identify protocol headers".to_string(),
            ],
            learning_objectives: vec![
                "Learn packet structure".to_string(),
                "Understand protocol analysis".to_string(),
                "Practice network monitoring".to_string(),
            ],
        });
    }

    /// Create sandbox environment
    pub fn create_environment(&mut self, id: String, name: String, sandbox_type: SandboxType) {
        let mut resources = HashMap::new();
        
        match sandbox_type {
            SandboxType::MalwareAnalysis => {
                resources.insert("cpu_limit".to_string(), "1 core".to_string());
                resources.insert("memory_limit".to_string(), "512MB".to_string());
                resources.insert("network_isolated".to_string(), "true".to_string());
                resources.insert("filesystem_isolated".to_string(), "true".to_string());
            }
            SandboxType::FirewallSimulation => {
                resources.insert("cpu_limit".to_string(), "2 cores".to_string());
                resources.insert("memory_limit".to_string(), "1GB".to_string());
                resources.insert("network_enabled".to_string(), "true".to_string());
                resources.insert("firewall_enabled".to_string(), "true".to_string());
            }
            SandboxType::EncryptionDemo => {
                resources.insert("cpu_limit".to_string(), "1 core".to_string());
                resources.insert("memory_limit".to_string(), "256MB".to_string());
                resources.insert("crypto_acceleration".to_string(), "true".to_string());
            }
            SandboxType::NetworkSecurity => {
                resources.insert("cpu_limit".to_string(), "2 cores".to_string());
                resources.insert("memory_limit".to_string(), "1GB".to_string());
                resources.insert("network_enabled".to_string(), "true".to_string());
                resources.insert("packet_capture".to_string(), "true".to_string());
            }
            SandboxType::PenetrationTesting => {
                resources.insert("cpu_limit".to_string(), "2 cores".to_string());
                resources.insert("memory_limit".to_string(), "1GB".to_string());
                resources.insert("network_enabled".to_string(), "true".to_string());
                resources.insert("tools_enabled".to_string(), "nmap, metasploit (stubs)".to_string());
            }
        }
        
        let environment = SandboxEnvironment {
            id: id.clone(),
            name,
            sandbox_type,
            is_active: false,
            resources,
            logs: Vec::new(),
        };
        
        self.environments.insert(id.clone(), environment);
    }

    /// Start sandbox environment
    pub fn start_environment(&mut self, id: &str) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(id) {
            env.is_active = true;
            env.logs.push(format!("Sandbox {} started at {}", env.name, "now".to_string()));
            self.current_environment = Some(id.to_string());
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Stop sandbox environment
    pub fn stop_environment(&mut self, id: &str) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(id) {
            env.is_active = false;
            env.logs.push(format!("Sandbox {} stopped at {}", env.name, "now".to_string()));
            if self.current_environment.as_ref() == Some(&id.to_string()) {
                self.current_environment = None;
            }
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Add log entry to environment
    pub fn add_log(&mut self, id: &str, message: String) -> Result<(), String> {
        if let Some(env) = self.environments.get_mut(id) {
            env.logs.push(format!("[{}] {}", "now".to_string(), message));
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Get environment logs
    pub fn get_logs(&self, id: &str) -> Result<&[String], String> {
        if let Some(env) = self.environments.get(id) {
            Ok(&env.logs)
        } else {
            Err("Environment not found".to_string())
        }
    }

    /// Get scenario by ID
    pub fn get_scenario(&self, id: &str) -> Option<&SecurityScenario> {
        self.scenarios.get(id)
    }

    /// Get all scenarios
    pub fn get_all_scenarios(&self) -> Vec<&SecurityScenario> {
        self.scenarios.values().collect()
    }

    /// Get scenarios by type
    pub fn get_scenarios_by_type(&self, sandbox_type: SandboxType) -> Vec<&SecurityScenario> {
        self.scenarios.values()
            .filter(|s| match sandbox_type {
                SandboxType::MalwareAnalysis => s.id.starts_with("malware"),
                SandboxType::FirewallSimulation => s.id.starts_with("firewall"),
                SandboxType::EncryptionDemo => s.id.starts_with("encrypt"),
                SandboxType::NetworkSecurity => s.id.starts_with("network"),
                SandboxType::PenetrationTesting => s.id.starts_with("pentest"),
            })
            .collect()
    }

    /// Get current environment
    pub fn get_current_environment(&self) -> Option<&SandboxEnvironment> {
        self.current_environment.as_ref()
            .and_then(|id| self.environments.get(id))
    }

    /// Get all environments
    pub fn get_all_environments(&self) -> Vec<&SandboxEnvironment> {
        self.environments.values().collect()
    }

    /// Delete environment
    pub fn delete_environment(&mut self, id: &str) -> Result<(), String> {
        if let Some(env) = self.environments.get(id) {
            if env.is_active {
                return Err("Cannot delete active environment".to_string());
            }
            self.environments.remove(id);
            if self.current_environment.as_ref() == Some(&id.to_string()) {
                self.current_environment = None;
            }
            Ok(())
        } else {
            Err("Environment not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = SandboxManager::new();
    
    println!("Sigma Cybersecurity Sandbox v0.1 - Safe Security Training");
    
    loop {
        println!("\n--- Available Scenarios ---");
        for scenario in manager.get_all_scenarios() {
            println!("{} - {} ({})", scenario.id, scenario.title, scenario.difficulty);
        }
        
        println!("\n--- Environments ---");
        for env in manager.get_all_environments() {
            let status = if env.is_active { "RUNNING" } else { "STOPPED" };
            let marker = if manager.current_environment.as_ref() == Some(&env.id) { " >" } else { "  " };
            println!("{}[{}] {} - {}", marker, env.id, env.name, status);
        }
        
        println!("\nCommands: create <id> <name> <type>, start <id>, stop <id>, log <id> <msg>, logs <id>, delete <id>, scenario <id>, types <type>, quit");
        println!("Types: malware, firewall, encrypt, network, pentest");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create" => {
                if parts.len() >= 4 {
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let sandbox_type = match parts[3] {
                        "malware" => SandboxType::MalwareAnalysis,
                        "firewall" => SandboxType::FirewallSimulation,
                        "encrypt" => SandboxType::EncryptionDemo,
                        "network" => SandboxType::NetworkSecurity,
                        "pentest" => SandboxType::PenetrationTesting,
                        _ => {
                            println!("Unknown sandbox type");
                            continue;
                        }
                    };
                    manager.create_environment(id, name, sandbox_type);
                    println!("Environment created");
                }
            }
            "start" => {
                if let Some(arg) = parts.get(1) {
                    match manager.start_environment(arg) {
                        Ok(_) => println!("Environment started"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stop" => {
                if let Some(arg) = parts.get(1) {
                    match manager.stop_environment(arg) {
                        Ok(_) => println!("Environment stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "log" => {
                if parts.len() >= 3 {
                    let id = parts[1];
                    let message = parts[2..].join(" ");
                    match manager.add_log(id, message) {
                        Ok(_) => println!("Log added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "logs" => {
                if let Some(arg) = parts.get(1) {
                    match manager.get_logs(arg) {
                        Ok(logs) => {
                            println!("--- Logs ---");
                            for log in logs {
                                println!("{}", log);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "delete" => {
                if let Some(arg) = parts.get(1) {
                    match manager.delete_environment(arg) {
                        Ok(_) => println!("Environment deleted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "scenario" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(scenario) = manager.get_scenario(arg) {
                        println!("--- Scenario Details ---");
                        println!("Title: {}", scenario.title);
                        println!("Description: {}", scenario.description);
                        println!("Difficulty: {}", scenario.difficulty);
                        println!("\nSteps:");
                        for (i, step) in scenario.steps.iter().enumerate() {
                            println!("{}. {}", i + 1, step);
                        }
                        println!("\nLearning Objectives:");
                        for obj in &scenario.learning_objectives {
                            println!("- {}", obj);
                        }
                    }
                }
            }
            "types" => {
                if let Some(arg) = parts.get(1) {
                    let sandbox_type = match *arg {
                        "malware" => SandboxType::MalwareAnalysis,
                        "firewall" => SandboxType::FirewallSimulation,
                        "encrypt" => SandboxType::EncryptionDemo,
                        "network" => SandboxType::NetworkSecurity,
                        "pentest" => SandboxType::PenetrationTesting,
                        _ => {
                            println!("Unknown sandbox type");
                            continue;
                        }
                    };
                    println!("--- {} Scenarios ---", arg);
                    for scenario in manager.get_scenarios_by_type(sandbox_type) {
                        println!("{} - {}", scenario.id, scenario.title);
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
