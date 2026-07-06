// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/net/sigma_networkmanager.rs — Sigma NetworkManager
//
// Implements NetworkManager-style unified network configuration with
// connection management, device control, and network monitoring.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Network Manager Types ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionState {
    Unknown,
    Activating,
    Activated,
    Deactivating,
    Deactivated,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    Ethernet,
    WiFi,
    Bluetooth,
    Bridge,
    Bond,
    VLAN,
    Dummy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IPMethod {
    Auto,
    Manual,
    LinkLocal,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct IPAddress {
    pub address: String,
    pub prefix: u32,
    pub gateway: Option<String>,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct WiFiSecurity {
    pub key_mgmt: String,  // wpa-psk, wpa-eap, none
    pub psk: Option<String>,
    pub identity: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub device_type: DeviceType,
    pub state: ConnectionState,
    pub auto_connect: bool,
    pub interface: String,
    pub ip_method: IPMethod,
    pub ip_address: Option<IPAddress>,
    pub wifi_security: Option<WiFiSecurity>,
    pub mac_address: Option<String>,
    pub mtu: u32,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub interface: String,
    pub device_type: DeviceType,
    pub state: ConnectionState,
    pub driver: String,
    pub hw_address: String,
    pub mtu: u32,
    pub carrier: bool,
    pub speed: u32,
}

#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub ssid: String,
    pub bssid: String,
    pub frequency: u32,
    pub signal_strength: i32,
    pub security: String,
    pub wpa_flags: u32,
    pub rsn_flags: u32,
}

// ─── Network Manager ───────────────────────────────────────────────────────

pub struct NetworkManager {
    pub connections: HashMap<String, Connection>,
    pub devices: HashMap<String, Device>,
    pub access_points: Vec<AccessPoint>,
    pub active_connections: Vec<String>,
    pub dns_servers: Vec<String>,
    pub search_domains: Vec<String>,
}

impl NetworkManager {
    pub fn new() -> Self {
        let mut manager = NetworkManager {
            connections: HashMap::new(),
            devices: HashMap::new(),
            access_points: Vec::new(),
            active_connections: Vec::new(),
            dns_servers: vec![
                "8.8.8.8".to_string(),
                "8.8.4.4".to_string(),
            ],
            search_domains: vec![],
        };

        manager.init_default_devices();
        manager.init_default_connections();
        manager
    }

    /// Initialize default network devices
    fn init_default_devices(&mut self) {
        // Ethernet device
        self.devices.insert("eth0".to_string(), Device {
            interface: "eth0".to_string(),
            device_type: DeviceType::Ethernet,
            state: ConnectionState::Activated,
            driver: "e1000e".to_string(),
            hw_address: "00:11:22:33:44:55".to_string(),
            mtu: 1500,
            carrier: true,
            speed: 1000,
        });

        // WiFi device
        self.devices.insert("wlan0".to_string(), Device {
            interface: "wlan0".to_string(),
            device_type: DeviceType::WiFi,
            state: ConnectionState::Disconnected,
            driver: "iwlwifi".to_string(),
            hw_address: "00:aa:bb:cc:dd:ee".to_string(),
            mtu: 1500,
            carrier: false,
            speed: 0,
        });

        // Loopback
        self.devices.insert("lo".to_string(), Device {
            interface: "lo".to_string(),
            device_type: DeviceType::Dummy,
            state: ConnectionState::Activated,
            driver: "loopback".to_string(),
            hw_address: "00:00:00:00:00:00".to_string(),
            mtu: 65536,
            carrier: true,
            speed: 0,
        });
    }

    /// Initialize default connections
    fn init_default_connections(&mut self) {
        // Wired connection
        let eth_ip = IPAddress {
            address: "192.168.1.100".to_string(),
            prefix: 24,
            gateway: Some("192.168.1.1".to_string()),
            dns: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
        };

        self.connections.insert("Wired connection 1".to_string(), Connection {
            id: "Wired connection 1".to_string(),
            uuid: "uuid-eth0".to_string(),
            name: "Wired connection 1".to_string(),
            device_type: DeviceType::Ethernet,
            state: ConnectionState::Activated,
            auto_connect: true,
            interface: "eth0".to_string(),
            ip_method: IPMethod::Manual,
            ip_address: Some(eth_ip),
            wifi_security: None,
            mac_address: Some("00:11:22:33:44:55".to_string()),
            mtu: 1500,
        });

        self.active_connections.push("Wired connection 1".to_string());

        // WiFi connection template
        self.connections.insert("WiFi-Home".to_string(), Connection {
            id: "WiFi-Home".to_string(),
            uuid: "uuid-wifi-home".to_string(),
            name: "WiFi-Home".to_string(),
            device_type: DeviceType::WiFi,
            state: ConnectionState::Disconnected,
            auto_connect: true,
            interface: "wlan0".to_string(),
            ip_method: IPMethod::Auto,
            ip_address: None,
            wifi_security: Some(WiFiSecurity {
                key_mgmt: "wpa-psk".to_string(),
                psk: Some("password123".to_string()),
                identity: None,
                password: None,
            }),
            mac_address: Some("00:aa:bb:cc:dd:ee".to_string()),
            mtu: 1500,
        });
    }

    /// Scan for WiFi access points
    pub fn scan_wifi(&mut self) -> Vec<&AccessPoint> {
        // Simulate WiFi scan
        self.access_points = vec![
            AccessPoint {
                ssid: "HomeNetwork".to_string(),
                bssid: "aa:bb:cc:dd:ee:ff".to_string(),
                frequency: 2412,
                signal_strength: -45,
                security: "WPA2".to_string(),
                wpa_flags: 0,
                rsn_flags: 0x00000010,
            },
            AccessPoint {
                ssid: "GuestNetwork".to_string(),
                bssid: "11:22:33:44:55:66".to_string(),
                frequency: 2437,
                signal_strength: -65,
                security: "WPA2".to_string(),
                wpa_flags: 0,
                rsn_flags: 0x00000010,
            },
            AccessPoint {
                ssid: "OpenWiFi".to_string(),
                bssid: "99:88:77:66:55:44".to_string(),
                frequency: 2462,
                signal_strength: -75,
                security: "Open".to_string(),
                wpa_flags: 0,
                rsn_flags: 0,
            },
        ];

        self.access_points.iter().collect()
    }

    /// Connect to WiFi network
    pub fn connect_wifi(&mut self, ssid: &str, password: Option<String>) -> Result<(), String> {
        let connection_id = format!("WiFi-{}", ssid);
        
        // Check if AP exists
        if !self.access_points.iter().any(|ap| ap.ssid == ssid) {
            return Err("Access point not found".to_string());
        }

        let wifi_security = if password.is_some() {
            Some(WiFiSecurity {
                key_mgmt: "wpa-psk".to_string(),
                psk: password,
                identity: None,
                password: None,
            })
        } else {
            None
        };

        let connection = Connection {
            id: connection_id.clone(),
            uuid: format!("uuid-{}", connection_id),
            name: ssid.to_string(),
            device_type: DeviceType::WiFi,
            state: ConnectionState::Activating,
            auto_connect: true,
            interface: "wlan0".to_string(),
            ip_method: IPMethod::Auto,
            ip_address: None,
            wifi_security,
            mac_address: Some("00:aa:bb:cc:dd:ee".to_string()),
            mtu: 1500,
        };

        self.connections.insert(connection_id.clone(), connection);
        
        // Simulate connection
        if let Some(conn) = self.connections.get_mut(&connection_id) {
            conn.state = ConnectionState::Activated;
            self.active_connections.push(connection_id.clone());
        }

        Ok(())
    }

    /// Disconnect from connection
    pub fn disconnect(&mut self, connection_id: &str) -> Result<(), String> {
        if let Some(conn) = self.connections.get_mut(connection_id) {
            conn.state = ConnectionState::Deactivated;
            self.active_connections.retain(|id| id != connection_id);
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }

    /// Create new connection
    pub fn create_connection(&mut self, name: String, device_type: DeviceType, interface: String, ip_method: IPMethod) -> Result<Connection, String> {
        let connection_id = name.clone();
        
        if self.connections.contains_key(&connection_id) {
            return Err("Connection already exists".to_string());
        }

        let connection = Connection {
            id: connection_id.clone(),
            uuid: format!("uuid-{}", connection_id),
            name: name.clone(),
            device_type,
            state: ConnectionState::Disconnected,
            auto_connect: false,
            interface,
            ip_method,
            ip_address: None,
            wifi_security: None,
            mac_address: None,
            mtu: 1500,
        };

        self.connections.insert(connection_id.clone(), connection.clone());
        Ok(connection)
    }

    /// Set IP address for connection
    pub fn set_ip_address(&mut self, connection_id: &str, address: String, prefix: u32, gateway: Option<String>) -> Result<(), String> {
        if let Some(conn) = self.connections.get_mut(connection_id) {
            conn.ip_address = Some(IPAddress {
                address,
                prefix,
                gateway,
                dns: self.dns_servers.clone(),
            });
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }

    /// Activate connection
    pub fn activate_connection(&mut self, connection_id: &str) -> Result<(), String> {
        if let Some(conn) = self.connections.get_mut(connection_id) {
            conn.state = ConnectionState::Activating;
            
            // Simulate activation
            conn.state = ConnectionState::Activated;
            if !self.active_connections.contains(&connection_id.to_string()) {
                self.active_connections.push(connection_id.to_string());
            }
            
            Ok(())
        } else {
            Err("Connection not found".to_string())
        }
    }

    /// Get device status
    pub fn get_device_status(&self, interface: &str) -> Option<&Device> {
        self.devices.get(interface)
    }

    /// Get connection status
    pub fn get_connection_status(&self, connection_id: &str) -> Option<&Connection> {
        self.connections.get(connection_id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&Device> {
        self.devices.values().collect()
    }

    /// List all connections
    pub fn list_connections(&self) -> Vec<&Connection> {
        self.connections.values().collect()
    }

    /// Set DNS servers
    pub fn set_dns_servers(&mut self, servers: Vec<String>) {
        self.dns_servers = servers;
    }

    /// Get DNS servers
    pub fn get_dns_servers(&self) -> &Vec<String> {
        &self.dns_servers
    }

    /// Get network statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("total_devices".to_string(), self.devices.len() as u32);
        stats.insert("total_connections".to_string(), self.connections.len() as u32);
        stats.insert("active_connections".to_string(), self.active_connections.len() as u32);
        stats.insert("wifi_aps".to_string(), self.access_points.len() as u32);
        
        let connected_devices = self.devices.values().filter(|d| d.state == ConnectionState::Activated).count();
        stats.insert("connected_devices".to_string(), connected_devices as u32);
        
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut nm = NetworkManager::new();
    
    println!("Sigma NetworkManager v0.1 - Unified Network Configuration");
    
    loop {
        println!("\n--- NetworkManager Commands ---");
        println!("devices            - List all devices");
        println!("connections        - List all connections");
        println!("device <iface>     - Get device status");
        println!("connection <id>    - Get connection status");
        println!("scan_wifi          - Scan for WiFi networks");
        println!("connect <ssid> [pass] - Connect to WiFi");
        println!("disconnect <id>    - Disconnect connection");
        println!("create <name> <type> <iface> <method> - Create connection");
        println!("set_ip <id> <addr> <prefix> [gateway] - Set IP");
        println!("activate <id>      - Activate connection");
        println!("set_dns <servers>  - Set DNS servers (comma separated)");
        println!("stats              - Show statistics");
        println!("quit               - Exit");
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
                for device in nm.list_devices() {
                    println!("{} - {:?} - {:?} - {} Mbps", 
                        device.interface, device.device_type, device.state, device.speed);
                }
            }
            "connections" => {
                println!("--- Network Connections ---");
                for conn in nm.list_connections() {
                    println!("{} - {:?} - {:?} - {}", 
                        conn.name, conn.device_type, conn.state, conn.interface);
                }
            }
            "device" => {
                if let Some(iface) = parts.get(1) {
                    if let Some(device) = nm.get_device_status(iface) {
                        println!("--- Device ---");
                        println!("Interface: {}", device.interface);
                        println!("Type: {:?}", device.device_type);
                        println!("State: {:?}", device.state);
                        println!("Driver: {}", device.driver);
                        println!("HW Address: {}", device.hw_address);
                        println!("MTU: {}", device.mtu);
                        println!("Carrier: {}", device.carrier);
                        println!("Speed: {} Mbps", device.speed);
                    }
                }
            }
            "connection" => {
                if let Some(id) = parts.get(1) {
                    if let Some(conn) = nm.get_connection_status(id) {
                        println!("--- Connection ---");
                        println!("Name: {}", conn.name);
                        println!("Type: {:?}", conn.device_type);
                        println!("State: {:?}", conn.state);
                        println!("Interface: {}", conn.interface);
                        println!("Auto-connect: {}", conn.auto_connect);
                        println!("IP Method: {:?}", conn.ip_method);
                        if let Some(ip) = &conn.ip_address {
                            println!("IP: {}/{}", ip.address, ip.prefix);
                            if let Some(gw) = &ip.gateway {
                                println!("Gateway: {}", gw);
                            }
                        }
                    }
                }
            }
            "scan_wifi" => {
                println!("--- WiFi Access Points ---");
                for ap in nm.scan_wifi() {
                    println!("{} - {} dBm - {} - {}", 
                        ap.ssid, ap.signal_strength, ap.frequency, ap.security);
                }
            }
            "connect" => {
                if parts.len() >= 2 {
                    let ssid = parts[1];
                    let password = parts.get(2).map(|p| p.to_string());
                    match nm.connect_wifi(ssid, password) {
                        Ok(_) => println!("Connected to {}", ssid),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "disconnect" => {
                if let Some(id) = parts.get(1) {
                    match nm.disconnect(id) {
                        Ok(_) => println!("Disconnected"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "create" => {
                if parts.len() >= 5 {
                    let name = parts[1].to_string();
                    let device_type = match parts[2] {
                        "ethernet" => DeviceType::Ethernet,
                        "wifi" => DeviceType::WiFi,
                        "bridge" => DeviceType::Bridge,
                        _ => DeviceType::Ethernet,
                    };
                    let interface = parts[3].to_string();
                    let ip_method = match parts[4] {
                        "auto" => IPMethod::Auto,
                        "manual" => IPMethod::Manual,
                        "disabled" => IPMethod::Disabled,
                        _ => IPMethod::Auto,
                    };
                    match nm.create_connection(name, device_type, interface, ip_method) {
                        Ok(_) => println!("Connection created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_ip" => {
                if parts.len() >= 4 {
                    let id = parts[1];
                    let address = parts[2].to_string();
                    let prefix = parts[3].parse::<u32>().unwrap_or(24);
                    let gateway = parts.get(4).map(|g| g.to_string());
                    match nm.set_ip_address(id, address, prefix, gateway) {
                        Ok(_) => println!("IP address set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "activate" => {
                if let Some(id) = parts.get(1) {
                    match nm.activate_connection(id) {
                        Ok(_) => println!("Connection activated"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_dns" => {
                if parts.len() >= 2 {
                    let servers: Vec<String> = parts[1].split(',').map(|s| s.trim().to_string()).collect();
                    nm.set_dns_servers(servers);
                    println!("DNS servers updated");
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in nm.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
