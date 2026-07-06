// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_network.rs — Sigma Networking Simulator
//
// Implements virtual labs for TCP/IP, routing, and cloud basics
// for teaching networking concepts in a safe environment.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Network Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkLayer {
    Physical,
    DataLink,
    Network,
    Transport,
    Application,
}

#[derive(Debug, Clone)]
pub struct NetworkDevice {
    pub id: String,
    pub name: String,
    pub device_type: String,  // Router, Switch, Host, etc.
    pub ip_address: String,
    pub mac_address: String,
    pub interfaces: Vec<String>,
    pub routing_table: Vec<RouteEntry>,
}

#[derive(Debug, Clone)]
pub struct RouteEntry {
    pub destination: String,
    pub gateway: String,
    pub netmask: String,
    pub interface: String,
    pub metric: u32,
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub source_ip: String,
    pub destination_ip: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub protocol: String,
    pub payload: String,
    pub ttl: u8,
}

// ─── Networking Simulator ───────────────────────────────────────────────────

pub struct NetworkSimulator {
    pub devices: HashMap<String, NetworkDevice>,
    pub packets: Vec<Packet>,
    pub current_topology: String,
}

impl NetworkSimulator {
    pub fn new() -> Self {
        let mut simulator = NetworkSimulator {
            devices: HashMap::new(),
            packets: Vec::new(),
            current_topology: "default".to_string(),
        };
        
        simulator.init_default_topology();
        simulator
    }

    /// Initialize default network topology
    fn init_default_topology(&mut self) {
        // Router
        self.devices.insert("router1".to_string(), NetworkDevice {
            id: "router1".to_string(),
            name: "Main Router".to_string(),
            device_type: "Router".to_string(),
            ip_address: "192.168.1.1".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            interfaces: vec!["eth0".to_string(), "eth1".to_string()],
            routing_table: vec![
                RouteEntry {
                    destination: "192.168.1.0".to_string(),
                    gateway: "0.0.0.0".to_string(),
                    netmask: "255.255.255.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 0,
                },
                RouteEntry {
                    destination: "10.0.0.0".to_string(),
                    gateway: "10.0.0.2".to_string(),
                    netmask: "255.255.255.0".to_string(),
                    interface: "eth1".to_string(),
                    metric: 1,
                },
                RouteEntry {
                    destination: "0.0.0.0".to_string(),
                    gateway: "192.168.1.254".to_string(),
                    netmask: "0.0.0.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 10,
                },
            ],
        });

        // Switch
        self.devices.insert("switch1".to_string(), NetworkDevice {
            id: "switch1".to_string(),
            name: "Core Switch".to_string(),
            device_type: "Switch".to_string(),
            ip_address: "192.168.1.2".to_string(),
            mac_address: "00:11:22:33:44:56".to_string(),
            interfaces: vec!["port1".to_string(), "port2".to_string(), "port3".to_string(), "port4".to_string()],
            routing_table: Vec::new(),
        });

        // Host 1
        self.devices.insert("host1".to_string(), NetworkDevice {
            id: "host1".to_string(),
            name: "Workstation 1".to_string(),
            device_type: "Host".to_string(),
            ip_address: "192.168.1.10".to_string(),
            mac_address: "00:11:22:33:44:57".to_string(),
            interfaces: vec!["eth0".to_string()],
            routing_table: vec![
                RouteEntry {
                    destination: "192.168.1.0".to_string(),
                    gateway: "0.0.0.0".to_string(),
                    netmask: "255.255.255.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 0,
                },
                RouteEntry {
                    destination: "0.0.0.0".to_string(),
                    gateway: "192.168.1.1".to_string(),
                    netmask: "0.0.0.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 1,
                },
            ],
        });

        // Host 2
        self.devices.insert("host2".to_string(), NetworkDevice {
            id: "host2".to_string(),
            name: "Workstation 2".to_string(),
            device_type: "Host".to_string(),
            ip_address: "192.168.1.11".to_string(),
            mac_address: "00:11:22:33:44:58".to_string(),
            interfaces: vec!["eth0".to_string()],
            routing_table: vec![
                RouteEntry {
                    destination: "192.168.1.0".to_string(),
                    gateway: "0.0.0.0".to_string(),
                    netmask: "255.255.255.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 0,
                },
                RouteEntry {
                    destination: "0.0.0.0".to_string(),
                    gateway: "192.168.1.1".to_string(),
                    netmask: "0.0.0.0".to_string(),
                    interface: "eth0".to_string(),
                    metric: 1,
                },
            ],
        });
    }

    /// Add network device
    pub fn add_device(&mut self, device: NetworkDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&NetworkDevice> {
        self.devices.get(id)
    }

    /// Get all devices
    pub fn get_all_devices(&self) -> Vec<&NetworkDevice> {
        self.devices.values().collect()
    }

    /// Simulate packet transmission
    pub fn transmit_packet(&mut self, source: String, destination: String, protocol: String, payload: String) -> Result<String, String> {
        if let (Some(src_device), Some(dst_device)) = (self.devices.get(&source), self.devices.get(&destination)) {
            let packet = Packet {
                source_ip: src_device.ip_address.clone(),
                destination_ip: dst_device.ip_address.clone(),
                source_port: 12345,
                destination_port: 80,
                protocol,
                payload,
                ttl: 64,
            };
            
            self.packets.push(packet.clone());
            
            // Simulate routing
            let route = self.find_route(&src_device.ip_address, &dst_device.ip_address)?;
            
            Ok(format!("Packet transmitted via: {}", route))
        } else {
            Err("Source or destination device not found".to_string())
        }
    }

    /// Find route between two IPs
    fn find_route(&self, source: &str, destination: &str) -> Result<String, String> {
        // Simplified routing logic
        for device in self.devices.values() {
            if device.device_type == "Router" {
                for route in &device.routing_table {
                    if destination.starts_with(&route.destination) {
                        return Ok(format!("{} -> {} -> {}", source, device.name, route.gateway));
                    }
                }
            }
        }
        Ok(format!("{} -> {}", source, destination))
    }

    /// Add route to device
    pub fn add_route(&mut self, device_id: &str, route: RouteEntry) -> Result<(), String> {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.routing_table.push(route);
            Ok(())
        } else {
            Err("Device not found".to_string())
        }
    }

    /// Get routing table for device
    pub fn get_routing_table(&self, device_id: &str) -> Result<&[RouteEntry], String> {
        if let Some(device) = self.devices.get(device_id) {
            Ok(&device.routing_table)
        } else {
            Err("Device not found".to_string())
        }
    }

    /// Simulate TCP handshake
    pub fn simulate_tcp_handshake(&mut self, client: String, server: String) -> Vec<String> {
        let mut steps = Vec::new();
        
        steps.push(format!("SYN: {} -> {} (SYN flag set, seq=0)", client, server));
        steps.push(format!("SYN-ACK: {} -> {} (SYN+ACK flags, seq=0, ack=1)", server, client));
        steps.push(format!("ACK: {} -> {} (ACK flag, seq=1, ack=1)", client, server));
        steps.push("TCP connection established".to_string());
        
        steps
    }

    /// Simulate DNS resolution
    pub fn simulate_dns(&self, domain: &str) -> String {
        // Simplified DNS simulation
        let dns_records = HashMap::from([
            ("google.com".to_string(), "142.250.185.78".to_string()),
            ("example.com".to_string(), "93.184.216.34".to_string()),
            ("sigmaos.local".to_string(), "192.168.1.100".to_string()),
        ]);
        
        dns_records.get(domain)
            .cloned()
            .unwrap_or_else(|| "DNS record not found".to_string())
    }

    /// Calculate subnet information
    pub fn calculate_subnet(&self, ip: &str, cidr: u8) -> Result<(String, String, String, u32), String> {
        // Simplified subnet calculation
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return Err("Invalid IP address".to_string());
        }
        
        let first_octet: u32 = parts[0].parse().unwrap_or(0);
        let network_address = format!("{}.0.0.0", first_octet);
        let broadcast = format!("{}.255.255.255", first_octet);
        let netmask = format!("255.{}.{}.{}", 255 - (2u32.pow((32 - cidr) as u32) - 1), 0, 0);
        let hosts = 2u32.pow((32 - cidr) as u32) - 2;
        
        Ok((network_address, broadcast, netmask, hosts))
    }

    /// Get packet history
    pub fn get_packet_history(&self) -> &[Packet] {
        &self.packets
    }

    /// Clear packet history
    pub fn clear_packets(&mut self) {
        self.packets.clear();
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut simulator = NetworkSimulator::new();
    
    println!("Sigma Networking Simulator v0.1 - TCP/IP & Routing Labs");
    
    loop {
        println!("\n--- Network Topology ---");
        for device in simulator.get_all_devices() {
            println!("{} ({}) - IP: {}, MAC: {}", device.name, device.device_type, device.ip_address, device.mac_address);
        }
        
        println!("\nCommands: devices, route <id>, add_route <id> <dest> <gateway> <mask> <iface>, send <src> <dst> <proto> <payload>, tcp <client> <server>, dns <domain>, subnet <ip> <cidr>, packets, clear, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "devices" => {
                println!("--- Network Devices ---");
                for device in simulator.get_all_devices() {
                    println!("ID: {}", device.id);
                    println!("  Name: {}", device.name);
                    println!("  Type: {}", device.device_type);
                    println!("  IP: {}", device.ip_address);
                    println!("  MAC: {}", device.mac_address);
                    println!("  Interfaces: {}", device.interfaces.join(", "));
                    println!();
                }
            }
            "route" => {
                if let Some(arg) = parts.get(1) {
                    match simulator.get_routing_table(arg) {
                        Ok(routes) => {
                            println!("--- Routing Table for {} ---", arg);
                            println!("Destination      Gateway          Netmask          Interface    Metric");
                            println!("─────────────────────────────────────────────────────────────────");
                            for route in routes {
                                println!("{:<17} {:<16} {:<16} {:<12} {}", 
                                    route.destination, route.gateway, route.netmask, route.interface, route.metric);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "add_route" => {
                if parts.len() >= 6 {
                    let device_id = parts[1];
                    let route = RouteEntry {
                        destination: parts[2].to_string(),
                        gateway: parts[3].to_string(),
                        netmask: parts[4].to_string(),
                        interface: parts[5].to_string(),
                        metric: 1,
                    };
                    match simulator.add_route(device_id, route) {
                        Ok(_) => println!("Route added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "send" => {
                if parts.len() >= 5 {
                    let source = parts[1].to_string();
                    let destination = parts[2].to_string();
                    let protocol = parts[3].to_string();
                    let payload = parts[4..].join(" ");
                    match simulator.transmit_packet(source, destination, protocol, payload) {
                        Ok(result) => println!("Success: {}", result),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "tcp" => {
                if parts.len() >= 3 {
                    let client = parts[1].to_string();
                    let server = parts[2].to_string();
                    println!("--- TCP Handshake Simulation ---");
                    for step in simulator.simulate_tcp_handshake(client, server) {
                        println!("{}", step);
                    }
                }
            }
            "dns" => {
                if let Some(arg) = parts.get(1) {
                    let result = simulator.simulate_dns(arg);
                    println!("DNS Resolution: {} -> {}", arg, result);
                }
            }
            "subnet" => {
                if parts.len() >= 3 {
                    let ip = parts[1];
                    if let Ok(cidr) = parts[2].parse::<u8>() {
                        match simulator.calculate_subnet(ip, cidr) {
                            Ok((network, broadcast, netmask, hosts)) => {
                                println!("--- Subnet Information ---");
                                println!("Network Address: {}", network);
                                println!("Broadcast Address: {}", broadcast);
                                println!("Subnet Mask: {}", netmask);
                                println!("Available Hosts: {}", hosts);
                            }
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "packets" => {
                println!("--- Packet History ---");
                for packet in simulator.get_packet_history() {
                    println!("{}:{} -> {}:{} [{}] TTL: {} | {}", 
                        packet.source_ip, packet.source_port,
                        packet.destination_ip, packet.destination_port,
                        packet.protocol, packet.ttl, packet.payload);
                }
            }
            "clear" => {
                simulator.clear_packets();
                println!("Packet history cleared");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
