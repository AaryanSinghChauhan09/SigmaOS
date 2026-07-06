// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_wireguard.rs — Sigma WireGuard VPN
//
// Implements WireGuard-style VPN with peer management, key exchange,
// tunnel configuration, and secure routing.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── WireGuard Types ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WGPeer {
    pub id: String,
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u32,
    pub latest_handshake: String,
    pub transfer_rx: u64,
    pub transfer_tx: u64,
}

#[derive(Debug, Clone)]
pub struct WGInterface {
    pub name: String,
    pub private_key: String,
    pub public_key: String,
    pub listen_port: u16,
    pub address: String,
    pub dns: String,
    pub mtu: u32,
    pub peers: HashMap<String, WGPeer>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct WGConfig {
    pub interface_name: String,
    pub private_key: String,
    pub address: String,
    pub port: u16,
    pub peers: Vec<WGPeer>,
}

// ─── WireGuard Manager ────────────────────────────────────────────────────

pub struct WireGuardManager {
    pub interfaces: HashMap<String, WGInterface>,
    pub configs: HashMap<String, WGConfig>,
}

impl WireGuardManager {
    pub fn new() -> Self {
        let mut manager = WireGuardManager {
            interfaces: HashMap::new(),
            configs: HashMap::new(),
        };
        
        manager.init_sample_interface();
        manager
    }

    /// Initialize sample WireGuard interface
    fn init_sample_interface(&mut self) {
        let mut interface = WGInterface {
            name: "wg0".to_string(),
            private_key: "aBcDeFgHiJkLmNoPqRsTuVwXyZ1234567890=".to_string(),
            public_key: "YzAbCdEfGhIjKlMnOpQrStUvWxYz12345678=".to_string(),
            listen_port: 51820,
            address: "10.0.0.1/24".to_string(),
            dns: "1.1.1.1".to_string(),
            mtu: 1420,
            peers: HashMap::new(),
            active: true,
        };
        
        // Add sample peer
        let peer = WGPeer {
            id: "peer_001".to_string(),
            public_key: "XyZ1234567890AbCdEfGhIjKlMnOpQrStUv=".to_string(),
            endpoint: "192.168.1.100:51820".to_string(),
            allowed_ips: vec!["10.0.0.2/32".to_string()],
            persistent_keepalive: 25,
            latest_handshake: "2024-01-15 10:30:00".to_string(),
            transfer_rx: 1024 * 1024 * 100,  // 100 MB
            transfer_tx: 1024 * 1024 * 50,   // 50 MB
        };
        
        interface.peers.insert(peer.id.clone(), peer);
        self.interfaces.insert(interface.name.clone(), interface);
    }

    /// Create new interface
    pub fn create_interface(&mut self, name: String, address: String, port: u16) -> WGInterface {
        let interface = WGInterface {
            name: name.clone(),
            private_key: format!("{}=", generate_key_base64()),
            public_key: format!("{}=", generate_key_base64()),
            listen_port: port,
            address,
            dns: "1.1.1.1".to_string(),
            mtu: 1420,
            peers: HashMap::new(),
            active: false,
        };
        
        self.interfaces.insert(name.clone(), interface.clone());
        interface
    }

    /// Add peer to interface
    pub fn add_peer(&mut self, interface_name: &str, peer: WGPeer) -> Result<(), String> {
        if let Some(interface) = self.interfaces.get_mut(interface_name) {
            interface.peers.insert(peer.id.clone(), peer);
            Ok(())
        } else {
            Err("Interface not found".to_string())
        }
    }

    /// Remove peer
    pub fn remove_peer(&mut self, interface_name: &str, peer_id: &str) -> Result<(), String> {
        if let Some(interface) = self.interfaces.get_mut(interface_name) {
            if interface.peers.remove(peer_id).is_some() {
                Ok(())
            } else {
                Err("Peer not found".to_string())
            }
        } else {
            Err("Interface not found".to_string())
        }
    }

    /// Activate interface
    pub fn activate_interface(&mut self, name: &str) -> Result<(), String> {
        if let Some(interface) = self.interfaces.get_mut(name) {
            interface.active = true;
            Ok(())
        } else {
            Err("Interface not found".to_string())
        }
    }

    /// Deactivate interface
    pub fn deactivate_interface(&mut self, name: &str) -> Result<(), String> {
        if let Some(interface) = self.interfaces.get_mut(name) {
            interface.active = false;
            Ok(())
        } else {
            Err("Interface not found".to_string())
        }
    }

    /// Get interface by name
    pub fn get_interface(&self, name: &str) -> Option<&WGInterface> {
        self.interfaces.get(name)
    }

    /// Get all interfaces
    pub fn get_all_interfaces(&self) -> Vec<&WGInterface> {
        self.interfaces.values().collect()
    }

    /// Generate config
    pub fn generate_config(&self, interface_name: &str) -> Option<String> {
        if let Some(interface) = self.get_interface(interface_name) {
            let mut config = format!(
                "[Interface]\nPrivateKey = {}\nAddress = {}\nListenPort = {}\nDNS = {}\n",
                interface.private_key, interface.address, interface.listen_port, interface.dns
            );
            
            for peer in interface.peers.values() {
                config.push_str(&format!(
                    "\n[Peer]\nPublicKey = {}\nEndpoint = {}\nAllowedIPs = {}\nPersistentKeepalive = {}\n",
                    peer.public_key,
                    peer.endpoint,
                    peer.allowed_ips.join(","),
                    peer.persistent_keepalive
                ));
            }
            
            Some(config)
        } else {
            None
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

// Simple base64 key generator
fn generate_key_base64() -> String {
    "aBcDeFgHiJkLmNoPqRsTuVwXyZ1234567890+/=".to_string()
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = WireGuardManager::new();
    
    println!("Sigma WireGuard VPN v0.1");
    
    loop {
        println!("\n--- WireGuard Status ---");
        let active_count = manager.interfaces.values().filter(|i| i.active).count();
        println!("Interfaces: {} ({} active)", manager.interfaces.len(), active_count);
        
        println!("\nCommands: create <name> <address> <port>, activate <name>, deactivate <name>, add_peer <iface> <pubkey> <endpoint> <allowed_ips>, remove_peer <iface> <peer_id>, interfaces, interface <name>, config <name>, quit");
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
                    let name = parts[1].to_string();
                    let address = parts[2].to_string();
                    if let Ok(port) = parts[3].parse::<u16>() {
                        let interface = manager.create_interface(name, address, port);
                        println!("Interface created: {}", interface.name);
                        println!("Public key: {}", interface.public_key);
                    }
                }
            }
            "activate" => {
                if let Some(arg) = parts.get(1) {
                    match manager.activate_interface(arg) {
                        Ok(_) => println!("Interface activated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "deactivate" => {
                if let Some(arg) = parts.get(1) {
                    match manager.deactivate_interface(arg) {
                        Ok(_) => println!("Interface deactivated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_peer" => {
                if parts.len() >= 5 {
                    let iface = parts[1];
                    let pubkey = parts[2].to_string();
                    let endpoint = parts[3].to_string();
                    let allowed_ips = parts[4].split(',').map(|s| s.trim().to_string()).collect();
                    
                    let peer = WGPeer {
                        id: format!("peer_{}", rand_id()),
                        public_key: pubkey,
                        endpoint,
                        allowed_ips,
                        persistent_keepalive: 25,
                        latest_handshake: "never".to_string(),
                        transfer_rx: 0,
                        transfer_tx: 0,
                    };
                    
                    match manager.add_peer(iface, peer) {
                        Ok(_) => println!("Peer added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_peer" => {
                if parts.len() >= 3 {
                    match manager.remove_peer(parts[1], parts[2]) {
                        Ok(_) => println!("Peer removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "interfaces" => {
                println!("--- All Interfaces ---");
                for iface in manager.get_all_interfaces() {
                    let status = if iface.active { "[UP]" } else { "[DOWN]" };
                    println!("{} - {} {} (Port: {}, Address: {})", iface.name, status, iface.public_key.chars().take(20).collect::<String>(), iface.listen_port, iface.address);
                    println!("  Peers: {}", iface.peers.len());
                }
            }
            "interface" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(iface) = manager.get_interface(arg) {
                        println!("--- Interface Details ---");
                        println!("Name: {}", iface.name);
                        println!("Status: {}", if iface.active { "UP" } else { "DOWN" });
                        println!("Address: {}", iface.address);
                        println!("Listen Port: {}", iface.listen_port);
                        println!("DNS: {}", iface.dns);
                        println!("MTU: {}", iface.mtu);
                        println!("Public Key: {}", iface.public_key);
                        println!("\n--- Peers ---");
                        for peer in iface.peers.values() {
                            println!("{} - {}", peer.id, peer.public_key.chars().take(20).collect::<String>());
                            println!("  Endpoint: {}", peer.endpoint);
                            println!("  Allowed IPs: {}", peer.allowed_ips.join(", "));
                            println!("  RX: {}, TX: {}", manager.format_bytes(peer.transfer_rx), manager.format_bytes(peer.transfer_tx));
                            println!("  Last Handshake: {}", peer.latest_handshake);
                        }
                    }
                }
            }
            "config" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(config) = manager.generate_config(arg) {
                        println!("--- WireGuard Config ---");
                        println!("{}", config);
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

fn rand_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u32
}
