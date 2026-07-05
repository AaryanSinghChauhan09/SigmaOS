// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/observability/sigma_cockpit.rs — Sigma Cockpit Web Admin
//
// Implements Cockpit-style web administration interface with
// system monitoring, service management, user management, and
// remote administration capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Cockpit Types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime: String,
    pub status: SystemStatus,
    pub cpu_count: u32,
    pub total_memory: u64,
    pub total_disk: u64,
}

#[derive(Debug, Clone)]
pub struct CPUUsage {
    pub user: f64,
    pub system: f64,
    pub idle: f64,
    pub iowait: f64,
    pub total: f64,
}

#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub cached: u64,
    pub buffers: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

#[derive(Debug, Clone)]
pub struct DiskUsage {
    pub device: String,
    pub mountpoint: String,
    pub fstype: String,
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub usage_percent: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub netmask: String,
    pub mac_address: String,
    pub mtu: u32,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub description: String,
    pub state: String,
    pub enabled: bool,
    pub active_since: String,
    pub main_pid: Option<u32>,
    pub memory: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub username: String,
    pub uid: u32,
    pub gid: u32,
    pub home: String,
    pub shell: String,
    pub groups: Vec<String>,
    pub last_login: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub service: String,
    pub level: String,
    pub message: String,
}

// ─── Cockpit Manager ─────────────────────────────────────────────────────

pub struct CockpitManager {
    pub system_info: SystemInfo,
    pub cpu_usage: CPUUsage,
    pub memory_usage: MemoryUsage,
    pub disk_usage: Vec<DiskUsage>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub services: HashMap<String, ServiceInfo>,
    pub users: HashMap<String, UserInfo>,
    pub logs: Vec<LogEntry>,
    pub web_port: u16,
    pub ssl_enabled: bool,
    pub auth_required: bool,
}

impl CockpitManager {
    pub fn new() -> Self {
        let mut manager = CockpitManager {
            system_info: SystemInfo {
                hostname: "sigmaos-host".to_string(),
                os_version: "SigmaOS 1.0".to_string(),
                kernel_version: "6.0.0-sigma".to_string(),
                uptime: "0d 0h 0m".to_string(),
                status: SystemStatus::Healthy,
                cpu_count: 4,
                total_memory: 16 * 1024 * 1024 * 1024,  // 16GB
                total_disk: 500 * 1024 * 1024 * 1024,  // 500GB
            },
            cpu_usage: CPUUsage {
                user: 5.0,
                system: 2.0,
                idle: 92.0,
                iowait: 1.0,
                total: 8.0,
            },
            memory_usage: MemoryUsage {
                total: 16 * 1024 * 1024 * 1024,
                used: 8 * 1024 * 1024 * 1024,
                free: 8 * 1024 * 1024 * 1024,
                cached: 2 * 1024 * 1024 * 1024,
                buffers: 1 * 1024 * 1024 * 1024,
                swap_total: 4 * 1024 * 1024 * 1024,
                swap_used: 0,
            },
            disk_usage: vec![],
            network_interfaces: vec![],
            services: HashMap::new(),
            users: HashMap::new(),
            logs: vec![],
            web_port: 9090,
            ssl_enabled: true,
            auth_required: true,
        };

        manager.init_default_services();
        manager.init_default_users();
        manager.init_default_disks();
        manager.init_default_network();
        manager
    }

    /// Initialize default services
    fn init_default_services(&mut self) {
        self.services.insert("network".to_string(), ServiceInfo {
            name: "network".to_string(),
            description: "Network service".to_string(),
            state: "running".to_string(),
            enabled: true,
            active_since: "now".to_string(),
            main_pid: Some(100),
            memory: Some(10 * 1024 * 1024),
        });

        self.services.insert("sshd".to_string(), ServiceInfo {
            name: "sshd".to_string(),
            description: "SSH daemon".to_string(),
            state: "running".to_string(),
            enabled: true,
            active_since: "now".to_string(),
            main_pid: Some(101),
            memory: Some(5 * 1024 * 1024),
        });

        self.services.insert("cockpit".to_string(), ServiceInfo {
            name: "cockpit".to_string(),
            description: "Web administration".to_string(),
            state: "running".to_string(),
            enabled: true,
            active_since: "now".to_string(),
            main_pid: Some(102),
            memory: Some(50 * 1024 * 1024),
        });
    }

    /// Initialize default users
    fn init_default_users(&mut self) {
        self.users.insert("root".to_string(), UserInfo {
            username: "root".to_string(),
            uid: 0,
            gid: 0,
            home: "/root".to_string(),
            shell: "/bin/bash".to_string(),
            groups: vec!["root".to_string()],
            last_login: None,
        });

        self.users.insert("sigma".to_string(), UserInfo {
            username: "sigma".to_string(),
            uid: 1000,
            gid: 1000,
            home: "/home/sigma".to_string(),
            shell: "/bin/bash".to_string(),
            groups: vec!["sigma".to_string(), "wheel".to_string()],
            last_login: Some("now".to_string()),
        });
    }

    /// Initialize default disks
    fn init_default_disks(&mut self) {
        self.disk_usage.push(DiskUsage {
            device: "/dev/sda1".to_string(),
            mountpoint: "/".to_string(),
            fstype: "ext4".to_string(),
            total: 500 * 1024 * 1024 * 1024,
            used: 200 * 1024 * 1024 * 1024,
            free: 300 * 1024 * 1024 * 1024,
            usage_percent: 40.0,
        });
    }

    /// Initialize default network
    fn init_default_network(&mut self) {
        self.network_interfaces.push(NetworkInterface {
            name: "eth0".to_string(),
            ip_address: "192.168.1.100".to_string(),
            netmask: "255.255.255.0".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            mtu: 1500,
            rx_bytes: 1024 * 1024,
            tx_bytes: 512 * 1024,
            rx_packets: 1000,
            tx_packets: 500,
            status: "up".to_string(),
        });
    }

    /// Update system status
    pub fn update_status(&mut self) -> SystemStatus {
        // Simulate status check
        let cpu_high = self.cpu_usage.total > 80.0;
        let memory_high = (self.memory_usage.used as f64 / self.memory_usage.total as f64) > 0.9;
        let disk_high = self.disk_usage.iter().any(|d| d.usage_percent > 90.0);

        self.system_info.status = if cpu_high || memory_high || disk_high {
            SystemStatus::Critical
        } else if cpu_high || memory_high || disk_high {
            SystemStatus::Warning
        } else {
            SystemStatus::Healthy
        };

        self.system_info.status
    }

    /// Start a service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.state = "running".to_string();
            service.active_since = "now".to_string();
            service.main_pid = Some(1000 + self.services.len() as u32);
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Stop a service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.state = "stopped".to_string();
            service.main_pid = None;
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Restart a service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        self.stop_service(name)?;
        self.start_service(name)
    }

    /// Enable a service
    pub fn enable_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.enabled = true;
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Disable a service
    pub fn disable_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.enabled = false;
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Add a user
    pub fn add_user(&mut self, username: String, uid: u32) -> Result<UserInfo, String> {
        if self.users.contains_key(&username) {
            return Err("User already exists".to_string());
        }

        let user = UserInfo {
            username: username.clone(),
            uid,
            gid: uid,
            home: format!("/home/{}", username),
            shell: "/bin/bash".to_string(),
            groups: vec![username.clone()],
            last_login: None,
        };

        self.users.insert(username.clone(), user.clone());
        Ok(user)
    }

    /// Remove a user
    pub fn remove_user(&mut self, username: &str) -> Result<(), String> {
        if username == "root" {
            return Err("Cannot remove root user".to_string());
        }

        if self.users.remove(username).is_some() {
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Add log entry
    pub fn add_log(&mut self, service: String, level: String, message: String) {
        self.logs.push(LogEntry {
            timestamp: "now".to_string(),
            service,
            level,
            message,
        });
    }

    /// Get logs filtered by service
    pub fn get_logs(&self, service: Option<&str>) -> Vec<&LogEntry> {
        if let Some(svc) = service {
            self.logs.iter().filter(|l| l.service == svc).collect()
        } else {
            self.logs.iter().collect()
        }
    }

    /// Set web port
    pub fn set_web_port(&mut self, port: u16) {
        self.web_port = port;
    }

    /// Toggle SSL
    pub fn toggle_ssl(&mut self) {
        self.ssl_enabled = !self.ssl_enabled;
    }

    /// Toggle auth
    pub fn toggle_auth(&mut self) {
        self.auth_required = !self.auth_required;
    }

    /// Get dashboard summary
    pub fn get_dashboard_summary(&self) -> HashMap<String, String> {
        let mut summary = HashMap::new();
        summary.insert("hostname".to_string(), self.system_info.hostname.clone());
        summary.insert("status".to_string(), format!("{:?}", self.system_info.status));
        summary.insert("cpu_usage".to_string(), format!("{}%", self.cpu_usage.total));
        summary.insert("memory_usage".to_string(), format!("{}%", 
            (self.memory_usage.used as f64 / self.memory_usage.total as f64 * 100.0) as u32));
        summary.insert("disk_usage".to_string(), format!("{}%", 
            self.disk_usage.first().map(|d| d.usage_percent as u32).unwrap_or(0)));
        summary.insert("uptime".to_string(), self.system_info.uptime.clone());
        summary.insert("services_running".to_string(), 
            self.services.values().filter(|s| s.state == "running").count().to_string());
        summary.insert("users_count".to_string(), self.users.len().to_string());
        summary
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut cockpit = CockpitManager::new();
    
    println!("Sigma Cockpit v0.1 - Web Administration Interface");
    
    loop {
        println!("\n--- Cockpit Commands ---");
        println!("status            - Show system status");
        println!("dashboard         - Show dashboard summary");
        println!("cpu               - Show CPU usage");
        println!("memory            - Show memory usage");
        println!("disk              - Show disk usage");
        println!("network           - Show network interfaces");
        println!("services          - List all services");
        println!("start <service>   - Start service");
        println!("stop <service>    - Stop service");
        println!("restart <service> - Restart service");
        println!("enable <service>  - Enable service");
        println!("disable <service> - Disable service");
        println!("users             - List all users");
        println!("add_user <name> <uid> - Add user");
        println!("remove_user <name> - Remove user");
        println!("logs [service]    - View logs");
        println!("add_log <svc> <level> <msg> - Add log entry");
        println!("port <port>       - Set web port");
        println!("toggle_ssl        - Toggle SSL");
        println!("toggle_auth       - Toggle authentication");
        println!("update            - Update system status");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "status" => {
                println!("--- System Status ---");
                println!("Hostname: {}", cockpit.system_info.hostname);
                println!("OS: {}", cockpit.system_info.os_version);
                println!("Kernel: {}", cockpit.system_info.kernel_version);
                println!("Uptime: {}", cockpit.system_info.uptime);
                println!("Status: {:?}", cockpit.system_info.status);
                println!("CPUs: {}", cockpit.system_info.cpu_count);
                println!("Memory: {} GB", cockpit.system_info.total_memory / (1024 * 1024 * 1024));
                println!("Disk: {} GB", cockpit.system_info.total_disk / (1024 * 1024 * 1024));
            }
            "dashboard" => {
                println!("--- Dashboard ---");
                for (key, value) in cockpit.get_dashboard_summary() {
                    println!("{}: {}", key, value);
                }
            }
            "cpu" => {
                println!("--- CPU Usage ---");
                println!("User: {}%", cockpit.cpu_usage.user);
                println!("System: {}%", cockpit.cpu_usage.system);
                println!("Idle: {}%", cockpit.cpu_usage.idle);
                println!("IOWait: {}%", cockpit.cpu_usage.iowait);
                println!("Total: {}%", cockpit.cpu_usage.total);
            }
            "memory" => {
                println!("--- Memory Usage ---");
                println!("Total: {} MB", cockpit.memory_usage.total / (1024 * 1024));
                println!("Used: {} MB", cockpit.memory_usage.used / (1024 * 1024));
                println!("Free: {} MB", cockpit.memory_usage.free / (1024 * 1024));
                println!("Cached: {} MB", cockpit.memory_usage.cached / (1024 * 1024));
                println!("Buffers: {} MB", cockpit.memory_usage.buffers / (1024 * 1024));
                println!("Swap Total: {} MB", cockpit.memory_usage.swap_total / (1024 * 1024));
                println!("Swap Used: {} MB", cockpit.memory_usage.swap_used / (1024 * 1024));
            }
            "disk" => {
                println!("--- Disk Usage ---");
                for disk in &cockpit.disk_usage {
                    println!("{} on {} ({})", disk.device, disk.mountpoint, disk.fstype);
                    println!("  Total: {} GB", disk.total / (1024 * 1024 * 1024));
                    println!("  Used: {} GB", disk.used / (1024 * 1024 * 1024));
                    println!("  Free: {} GB", disk.free / (1024 * 1024 * 1024));
                    println!("  Usage: {:.1}%", disk.usage_percent);
                }
            }
            "network" => {
                println!("--- Network Interfaces ---");
                for iface in &cockpit.network_interfaces {
                    println!("{} - {}", iface.name, iface.status);
                    println!("  IP: {}", iface.ip_address);
                    println!("  Netmask: {}", iface.netmask);
                    println!("  MAC: {}", iface.mac_address);
                    println!("  MTU: {}", iface.mtu);
                    println!("  RX: {} bytes ({} packets)", iface.rx_bytes, iface.rx_packets);
                    println!("  TX: {} bytes ({} packets)", iface.tx_bytes, iface.tx_packets);
                }
            }
            "services" => {
                println!("--- Services ---");
                for service in cockpit.services.values() {
                    println!("{} - {} - {} - {}", 
                        service.name, service.description, service.state, 
                        if service.enabled { "enabled" } else { "disabled" });
                }
            }
            "start" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.start_service(name) {
                        Ok(_) => println!("Service started"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stop" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.stop_service(name) {
                        Ok(_) => println!("Service stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "restart" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.restart_service(name) {
                        Ok(_) => println!("Service restarted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "enable" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.enable_service(name) {
                        Ok(_) => println!("Service enabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "disable" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.disable_service(name) {
                        Ok(_) => println!("Service disabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "users" => {
                println!("--- Users ---");
                for user in cockpit.users.values() {
                    println!("{} (UID: {}) - {} - {:?}", 
                        user.username, user.uid, user.home, user.last_login);
                }
            }
            "add_user" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let uid = parts[2].parse::<u32>().unwrap_or(1000);
                    match cockpit.add_user(name, uid) {
                        Ok(_) => println!("User added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_user" => {
                if let Some(name) = parts.get(1) {
                    match cockpit.remove_user(name) {
                        Ok(_) => println!("User removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "logs" => {
                let service = parts.get(1).copied();
                println!("--- Logs ---");
                for log in cockpit.get_logs(service) {
                    println!("[{}] {} {}: {}", log.timestamp, log.service, log.level, log.message);
                }
            }
            "add_log" => {
                if parts.len() >= 4 {
                    let service = parts[1].to_string();
                    let level = parts[2].to_string();
                    let message = parts[3..].join(" ");
                    cockpit.add_log(service, level, message);
                    println!("Log entry added");
                }
            }
            "port" => {
                if let Some(port) = parts.get(1).and_then(|p| p.parse::<u16>().ok()) {
                    cockpit.set_web_port(port);
                    println!("Web port set to {}", port);
                }
            }
            "toggle_ssl" => {
                cockpit.toggle_ssl();
                println!("SSL: {}", if cockpit.ssl_enabled { "enabled" } else { "disabled" });
            }
            "toggle_auth" => {
                cockpit.toggle_auth();
                println!("Auth: {}", if cockpit.auth_required { "required" } else { "not required" });
            }
            "update" => {
                let status = cockpit.update_status();
                println!("System status: {:?}", status);
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
