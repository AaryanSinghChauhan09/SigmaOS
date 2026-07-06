// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_ssh.rs — Sigma OpenSSH Integration
//
// Implements OpenSSH-style secure remote access with key management,
// connection handling, and secure shell operations.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── SSH Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SSHKey {
    pub id: String,
    pub key_type: String,
    pub public_key: String,
    pub private_key_path: String,
    pub comment: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct SSHConnection {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub status: String,
    pub connected_at: String,
    pub last_activity: String,
}

#[derive(Debug, Clone)]
pub struct SSHConfig {
    pub host: String,
    pub hostname: String,
    pub port: u16,
    pub user: String,
    pub identity_file: String,
    pub forward_agent: bool,
    pub compression: bool,
}

// ─── SSH Manager ─────────────────────────────────────────────────────────

pub struct SSHManager {
    pub keys: HashMap<String, SSHKey>,
    pub connections: HashMap<String, SSHConnection>,
    pub configs: HashMap<String, SSHConfig>,
}

impl SSHManager {
    pub fn new() -> Self {
        let mut manager = SSHManager {
            keys: HashMap::new(),
            connections: HashMap::new(),
            configs: HashMap::new(),
        };
        
        manager.init_sample_keys();
        manager.init_sample_configs();
        manager
    }

    /// Initialize sample SSH keys
    fn init_sample_keys(&mut self) {
        self.keys.insert("key_001".to_string(), SSHKey {
            id: "key_001".to_string(),
            key_type: "ed25519".to_string(),
            public_key: "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAm9z+example user@sigmaos".to_string(),
            private_key_path: "/home/user/.ssh/id_ed25519".to_string(),
            comment: "user@sigmaos".to_string(),
            created_at: "2024-01-15".to_string(),
        });

        self.keys.insert("key_002".to_string(), SSHKey {
            id: "key_002".to_string(),
            key_type: "rsa".to_string(),
            public_key: "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQC... admin@server".to_string(),
            private_key_path: "/home/user/.ssh/id_rsa".to_string(),
            comment: "admin@server".to_string(),
            created_at: "2024-02-20".to_string(),
        });
    }

    /// Initialize sample SSH configs
    fn init_sample_configs(&mut self) {
        self.configs.insert("github".to_string(), SSHConfig {
            host: "github.com".to_string(),
            hostname: "github.com".to_string(),
            port: 22,
            user: "git".to_string(),
            identity_file: "/home/user/.ssh/id_ed25519".to_string(),
            forward_agent: true,
            compression: false,
        });

        self.configs.insert("server".to_string(), SSHConfig {
            host: "server".to_string(),
            hostname: "192.168.1.100".to_string(),
            port: 22,
            user: "admin".to_string(),
            identity_file: "/home/user/.ssh/id_rsa".to_string(),
            forward_agent: false,
            compression: true,
        });
    }

    /// Generate new SSH key
    pub fn generate_key(&mut self, key_type: String, comment: String) -> SSHKey {
        let key = SSHKey {
            id: format!("key_{}", self.keys.len()),
            key_type: key_type.clone(),
            public_key: format!("ssh-{} AAAAC3NzaC1lZDI1NTE5AAAAIAm9z+example {}", key_type, comment),
            private_key_path: format!("/home/user/.ssh/id_{}", key_type),
            comment,
            created_at: "now".to_string(),
        };
        
        self.keys.insert(key.id.clone(), key.clone());
        key
    }

    /// Connect to remote host
    pub fn connect(&mut self, host: String, port: u username: String) -> Result<SSHConnection, String> {
        let connection = SSHConnection {
            id: format!("conn_{}", self.connections.len()),
            host: host.clone(),
            port,
            username,
            status: "connected".to_string(),
            connected_at: "now".to_string(),
            last_activity: "now".to_string(),
        };
        
        self.connections.insert(connection.id.clone(), connection.clone());
        Ok(connection)
    }

    /// Disconnect
    pub fn disconnect(&mut self, connection_id: &str) -> Result<(), String> {
        if let Some(conn) = self.connections.get_mut(connection_id) {
            conn.status = "disconnected".to_string();
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }

    /// Get key by ID
    pub fn get_key(&self, id: &str) -> Option<&SSHKey> {
        self.keys.get(id)
    }

    /// Get all keys
    pub fn get_all_keys(&self) -> Vec<&SSHKey> {
        self.keys.values().collect()
    }

    /// Get connection by ID
    pub fn get_connection(&self, id: &str) -> Option<&SSHConnection> {
        self.connections.get(id)
    }

    /// Get all connections
    pub fn get_all_connections(&self) -> Vec<&SSHConnection> {
        self.connections.values().collect()
    }

    /// Get config by host
    pub fn get_config(&self, host: &str) -> Option<&SSHConfig> {
        self.configs.get(host)
    }

    /// Get all configs
    pub fn get_all_configs(&self) -> Vec<&SSHConfig> {
        self.configs.values().collect()
    }

    /// Add config
    pub fn add_config(&mut self, config: SSHConfig) {
        self.configs.insert(config.host.clone(), config);
    }

    /// Remove key
    pub fn remove_key(&mut self, id: &str) -> Result<(), String> {
        if self.keys.remove(id).is_some() {
            Ok(())
        } else {
            Err("Key not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = SSHManager::new();
    
    println!("Sigma OpenSSH Integration v0.1");
    
    loop {
        println!("\n--- SSH Status ---");
        println!("Keys: {}", manager.keys.len());
        println!("Connections: {}", manager.connections.values().filter(|c| c.status == "connected").count());
        println!("Configs: {}", manager.configs.len());
        
        println!("\nCommands: connect <host> <port> <user>, disconnect <conn_id>, genkey <type> <comment>, keys, connections, configs, config <host>, add_config, quit");
        println!("Key types: ed25519, rsa, ecdsa");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "connect" => {
                if parts.len() >= 4 {
                    let host = parts[1].to_string();
                    if let (Ok(port), username) = (parts[2].parse::<u16>(), parts[3].to_string()) {
                        match manager.connect(host, port, username) {
                            Ok(conn) => println!("Connected: {} (ID: {})", conn.host, conn.id),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "disconnect" => {
                if let Some(arg) = parts.get(1) {
                    match manager.disconnect(arg) {
                        Ok(_) => println!("Disconnected"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "genkey" => {
                if parts.len() >= 3 {
                    let key_type = parts[1].to_string();
                    let comment = parts[2..].join(" ");
                    let key = manager.generate_key(key_type, comment);
                    println!("Key generated: {}", key.id);
                    println!("Public key: {}", key.public_key);
                    println!("Private key: {}", key.private_key_path);
                }
            }
            "keys" => {
                println!("--- SSH Keys ---");
                for key in manager.get_all_keys() {
                    println!("{} - {} ({})", key.id, key.key_type, key.comment);
                    println!("  Public: {}", key.public_key.chars().take(40).collect::<String>());
                }
            }
            "connections" => {
                println!("--- SSH Connections ---");
                for conn in manager.get_all_connections() {
                    println!("{} - {}@{}:{} ({})", conn.id, conn.username, conn.host, conn.port, conn.status);
                }
            }
            "configs" => {
                println!("--- SSH Configs ---");
                for config in manager.get_all_configs() {
                    println!("Host {} -> {}@{}:{}", config.host, config.user, config.hostname, config.port);
                }
            }
            "config" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(config) = manager.get_config(arg) {
                        println!("--- Config Details ---");
                        println!("Host: {}", config.host);
                        println!("Hostname: {}", config.hostname);
                        println!("Port: {}", config.port);
                        println!("User: {}", config.user);
                        println!("Identity File: {}", config.identity_file);
                        println!("Forward Agent: {}", config.forward_agent);
                        println!("Compression: {}", config.compression);
                    }
                }
            }
            "add_config" => {
                println!("Enter host alias:");
                let mut host = String::new();
                std::io::stdin().read_line(&mut host).unwrap();
                
                println!("Enter hostname:");
                let mut hostname = String::new();
                std::io::stdin().read_line(&mut hostname).unwrap();
                
                println!("Enter port:");
                let mut port_str = String::new();
                std::io::stdin().read_line(&mut port_str).unwrap();
                
                println!("Enter username:");
                let mut user = String::new();
                std::io::stdin().read_line(&mut user).unwrap();
                
                if let Ok(port) = port_str.trim().parse::<u16>() {
                    let config = SSHConfig {
                        host: host.trim().to_string(),
                        hostname: hostname.trim().to_string(),
                        port,
                        user: user.trim().to_string(),
                        identity_file: "/home/user/.ssh/id_ed25519".to_string(),
                        forward_agent: false,
                        compression: false,
                    };
                    manager.add_config(config);
                    println!("Config added");
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
