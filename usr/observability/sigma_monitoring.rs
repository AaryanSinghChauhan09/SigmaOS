// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/observability/sigma_monitoring.rs — Sigma Nagios/Zabbix Monitoring
//
// Implements Nagios/Zabbix-style monitoring with host management,
// service checks, alerts, and performance data collection.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Monitoring Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HostState {
    Up,
    Down,
    Unreachable,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceState {
    Ok,
    Warning,
    Critical,
    Unknown,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CheckType {
    Ping,
    Http,
    Tcp,
    Cpu,
    Memory,
    Disk,
    Process,
    Custom,
}

#[derive(Debug, Clone)]
pub struct Host {
    pub id: String,
    pub name: String,
    pub address: String,
    pub state: HostState,
    pub last_check: String,
    pub next_check: String,
    pub check_interval: u32,
    pub retry_interval: u32,
    pub max_attempts: u32,
    pub current_attempt: u32,
    pub services: Vec<String>,  // Service IDs
}

#[derive(Debug, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub host_id: String,
    pub check_type: CheckType,
    pub state: ServiceState,
    pub last_check: String,
    pub next_check: String,
    pub check_interval: u32,
    pub retry_interval: u32,
    pub max_attempts: u32,
    pub current_attempt: u32,
    pub performance_data: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub service_id: String,
    pub host_id: String,
    pub state: ServiceState,
    pub message: String,
    pub created: String,
    pub acknowledged: bool,
}

#[derive(Debug, Clone)]
pub struct CheckCommand {
    pub name: String,
    pub command: String,
    pub arguments: Vec<String>,
}

// ─── Monitoring Manager ─────────────────────────────────────────────────────

pub struct MonitoringManager {
    pub hosts: HashMap<String, Host>,
    pub services: HashMap<String, Service>,
    pub alerts: HashMap<String, Alert>,
    pub check_commands: HashMap<String, CheckCommand>,
    pub monitoring_enabled: bool,
    pub check_interval: u32,
}

impl MonitoringManager {
    pub fn new() -> Self {
        let mut manager = MonitoringManager {
            hosts: HashMap::new(),
            services: HashMap::new(),
            alerts: HashMap::new(),
            check_commands: HashMap::new(),
            monitoring_enabled: true,
            check_interval: 60,
        };

        manager.init_default_hosts();
        manager.init_default_commands();
        manager
    }

    /// Initialize default hosts
    fn init_default_hosts(&mut self) {
        let host = Host {
            id: "host_localhost".to_string(),
            name: "localhost".to_string(),
            address: "127.0.0.1".to_string(),
            state: HostState::Up,
            last_check: "now".to_string(),
            next_check: "now".to_string(),
            check_interval: 60,
            retry_interval: 30,
            max_attempts: 3,
            current_attempt: 0,
            services: vec![],
        };

        let host_id = host.id.clone();
        self.hosts.insert(host_id.clone(), host);
        
        // Add default services
        self.add_service("service_cpu".to_string(), host_id.clone(), "CPU Usage".to_string(), CheckType::Cpu);
        self.add_service("service_memory".to_string(), host_id.clone(), "Memory Usage".to_string(), CheckType::Memory);
        self.add_service("service_disk".to_string(), host_id.clone(), "Disk Usage".to_string(), CheckType::Disk);
    }

    /// Initialize default check commands
    fn init_default_commands(&mut self) {
        self.check_commands.insert("check_ping".to_string(), CheckCommand {
            name: "check_ping".to_string(),
            command: "/usr/lib/nagios/plugins/check_ping".to_string(),
            arguments: vec!["-H".to_string(), "$HOSTADDRESS$".to_string()],
        });

        self.check_commands.insert("check_http".to_string(), CheckCommand {
            name: "check_http".to_string(),
            command: "/usr/lib/nagios/plugins/check_http".to_string(),
            arguments: vec!["-H".to_string(), "$HOSTADDRESS$".to_string()],
        });

        self.check_commands.insert("check_cpu".to_string(), CheckCommand {
            name: "check_cpu".to_string(),
            command: "/usr/lib/nagios/plugins/check_cpu".to_string(),
            arguments: vec!["-w".to_string(), "80".to_string(), "-c".to_string(), "90".to_string()],
        });
    }

    /// Add a host
    pub fn add_host(&mut self, name: String, address: String) -> Result<Host, String> {
        let host_id = format!("host_{}", name);
        
        if self.hosts.contains_key(&host_id) {
            return Err("Host already exists".to_string());
        }

        let host = Host {
            id: host_id.clone(),
            name: name.clone(),
            address,
            state: HostState::Pending,
            last_check: "now".to_string(),
            next_check: "now".to_string(),
            check_interval: 60,
            retry_interval: 30,
            max_attempts: 3,
            current_attempt: 0,
            services: vec![],
        };

        self.hosts.insert(host_id.clone(), host.clone());
        Ok(host)
    }

    /// Remove a host
    pub fn remove_host(&mut self, host_id: &str) -> Result<(), String> {
        if let Some(host) = self.hosts.remove(host_id) {
            // Remove associated services
            for service_id in &host.services {
                self.services.remove(service_id);
            }
            Ok(())
        } else {
            Err("Host not found".to_string())
        }
    }

    /// Add a service
    pub fn add_service(&mut self, id: String, host_id: String, name: String, check_type: CheckType) -> Result<Service, String> {
        if !self.hosts.contains_key(&host_id) {
            return Err("Host not found".to_string());
        }

        if self.services.contains_key(&id) {
            return Err("Service already exists".to_string());
        }

        let service = Service {
            id: id.clone(),
            name,
            host_id: host_id.clone(),
            check_type,
            state: ServiceState::Pending,
            last_check: "now".to_string(),
            next_check: "now".to_string(),
            check_interval: 60,
            retry_interval: 30,
            max_attempts: 3,
            current_attempt: 0,
            performance_data: HashMap::new(),
        };

        self.services.insert(id.clone(), service.clone());
        
        if let Some(host) = self.hosts.get_mut(&host_id) {
            host.services.push(id);
        }

        Ok(service)
    }

    /// Remove a service
    pub fn remove_service(&mut self, service_id: &str) -> Result<(), String> {
        if let Some(service) = self.services.remove(service_id) {
            if let Some(host) = self.hosts.get_mut(&service.host_id) {
                host.services.retain(|s| s != service_id);
            }
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Execute a check
    pub fn execute_check(&mut self, service_id: &str) -> Result<ServiceState, String> {
        if let Some(service) = self.services.get_mut(service_id) {
            service.last_check = "now".to_string();
            
            // Simulate check execution
            let state = match service.check_type {
                CheckType::Ping => ServiceState::Ok,
                CheckType::Http => ServiceState::Ok,
                CheckType::Tcp => ServiceState::Ok,
                CheckType::Cpu => {
                    let usage = 50.0 + (rand_f64() * 40.0);
                    service.performance_data.insert("cpu_usage".to_string(), usage);
                    if usage > 90.0 { ServiceState::Critical }
                    else if usage > 80.0 { ServiceState::Warning }
                    else { ServiceState::Ok }
                }
                CheckType::Memory => {
                    let usage = 40.0 + (rand_f64() * 50.0);
                    service.performance_data.insert("memory_usage".to_string(), usage);
                    if usage > 90.0 { ServiceState::Critical }
                    else if usage > 80.0 { ServiceState::Warning }
                    else { ServiceState::Ok }
                }
                CheckType::Disk => {
                    let usage = 30.0 + (rand_f64() * 60.0);
                    service.performance_data.insert("disk_usage".to_string(), usage);
                    if usage > 90.0 { ServiceState::Critical }
                    else if usage > 80.0 { ServiceState::Warning }
                    else { ServiceState::Ok }
                }
                CheckType::Process => ServiceState::Ok,
                CheckType::Custom => ServiceState::Ok,
            };

            let old_state = service.state;
            service.state = state;
            service.current_attempt = 0;
            
            // Generate alert if state changed to non-OK
            if state != ServiceState::Ok && old_state == ServiceState::Ok {
                self.generate_alert(service_id, state);
            }

            Ok(state)
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Generate an alert
    fn generate_alert(&mut self, service_id: &str, state: ServiceState) {
        if let Some(service) = self.services.get(service_id) {
            let alert_id = format!("alert_{}", self.alerts.len());
            let alert = Alert {
                id: alert_id.clone(),
                service_id: service_id.to_string(),
                host_id: service.host_id.clone(),
                state,
                message: format!("Service {} is {:?}", service.name, state),
                created: "now".to_string(),
                acknowledged: false,
            };
            self.alerts.insert(alert_id, alert);
        }
    }

    /// Acknowledge an alert
    pub fn acknowledge_alert(&mut self, alert_id: &str) -> Result<(), String> {
        if let Some(alert) = self.alerts.get_mut(alert_id) {
            alert.acknowledged = true;
            Ok(())
        } else {
            Err("Alert not found".to_string())
        }
    }

    /// Get host by ID
    pub fn get_host(&self, host_id: &str) -> Option<&Host> {
        self.hosts.get(host_id)
    }

    /// Get service by ID
    pub fn get_service(&self, service_id: &str) -> Option<&Service> {
        self.services.get(service_id)
    }

    /// List all hosts
    pub fn list_hosts(&self) -> Vec<&Host> {
        self.hosts.values().collect()
    }

    /// List all services
    pub fn list_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    /// List all alerts
    pub fn list_alerts(&self) -> Vec<&Alert> {
        self.alerts.values().collect()
    }

    /// Get services for host
    pub fn get_host_services(&self, host_id: &str) -> Vec<&Service> {
        if let Some(host) = self.hosts.get(host_id) {
            host.services.iter()
                .filter_map(|s| self.services.get(s))
                .collect()
        } else {
            vec![]
        }
    }

    /// Enable monitoring
    pub fn enable_monitoring(&mut self) {
        self.monitoring_enabled = true;
    }

    /// Disable monitoring
    pub fn disable_monitoring(&mut self) {
        self.monitoring_enabled = false;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("hosts".to_string(), self.hosts.len() as u32);
        stats.insert("services".to_string(), self.services.len() as u32);
        stats.insert("alerts".to_string(), self.alerts.len() as u32);
        stats.insert("hosts_up".to_string(), self.hosts.values().filter(|h| h.state == HostState::Up).count() as u32);
        stats.insert("hosts_down".to_string(), self.hosts.values().filter(|h| h.state == HostState::Down).count() as u32);
        stats.insert("services_ok".to_string(), self.services.values().filter(|s| s.state == ServiceState::Ok).count() as u32);
        stats.insert("services_warning".to_string(), self.services.values().filter(|s| s.state == ServiceState::Warning).count() as u32);
        stats.insert("services_critical".to_string(), self.services.values().filter(|s| s.state == ServiceState::Critical).count() as u32);
        stats.insert("alerts_unacknowledged".to_string(), self.alerts.values().filter(|a| !a.acknowledged).count() as u32);
        stats
    }
}

fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.as_nanos() as f64) / 1e18
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut monitoring = MonitoringManager::new();
    
    println!("Sigma Monitoring v0.1 - Nagios/Zabbix Style");
    
    loop {
        println!("\n--- Monitoring Commands ---");
        println!("hosts             - List all hosts");
        println!("add_host <name> <address> - Add host");
        println!("remove_host <id>  - Remove host");
        println!("host_info <id>    - Show host details");
        println!("services          - List all services");
        println!("add_service <id> <host> <name> <type> - Add service");
        println!("remove_service <id> - Remove service");
        println!("service_info <id> - Show service details");
        println!("check <id>        - Execute service check");
        println!("alerts            - List alerts");
        println!("acknowledge <id>  - Acknowledge alert");
        println!("stats             - Show statistics");
        println!("enable            - Enable monitoring");
        println!("disable           - Disable monitoring");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "hosts" => {
                println!("--- Hosts ---");
                for host in monitoring.list_hosts() {
                    println!("{} - {} - {} - {:?}", host.id, host.name, host.address, host.state);
                }
            }
            "add_host" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let address = parts[2].to_string();
                    match monitoring.add_host(name, address) {
                        Ok(_) => println!("Host added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_host" => {
                if let Some(id) = parts.get(1) {
                    match monitoring.remove_host(id) {
                        Ok(_) => println!("Host removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "host_info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(host) = monitoring.get_host(id) {
                        println!("--- Host Info ---");
                        println!("ID: {}", host.id);
                        println!("Name: {}", host.name);
                        println!("Address: {}", host.address);
                        println!("State: {:?}", host.state);
                        println!("Services: {}", host.services.len());
                        println!("Check Interval: {}s", host.check_interval);
                    }
                }
            }
            "services" => {
                println!("--- Services ---");
                for service in monitoring.list_services() {
                    println!("{} - {} - {:?} - {:?}", service.id, service.name, service.check_type, service.state);
                }
            }
            "add_service" => {
                if parts.len() >= 5 {
                    let id = parts[1].to_string();
                    let host = parts[2].to_string();
                    let name = parts[3].to_string();
                    let check_type = match parts[4] {
                        "ping" => CheckType::Ping,
                        "http" => CheckType::Http,
                        "tcp" => CheckType::Tcp,
                        "cpu" => CheckType::Cpu,
                        "memory" => CheckType::Memory,
                        "disk" => CheckType::Disk,
                        "process" => CheckType::Process,
                        _ => CheckType::Custom,
                    };
                    match monitoring.add_service(id, host, name, check_type) {
                        Ok(_) => println!("Service added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_service" => {
                if let Some(id) = parts.get(1) {
                    match monitoring.remove_service(id) {
                        Ok(_) => println!("Service removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "service_info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(service) = monitoring.get_service(id) {
                        println!("--- Service Info ---");
                        println!("ID: {}", service.id);
                        println!("Name: {}", service.name);
                        println!("Host: {}", service.host_id);
                        println!("Type: {:?}", service.check_type);
                        println!("State: {:?}", service.state);
                        println!("Performance Data: {:?}", service.performance_data);
                    }
                }
            }
            "check" => {
                if let Some(id) = parts.get(1) {
                    match monitoring.execute_check(id) {
                        Ok(state) => println!("Check result: {:?}", state),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "alerts" => {
                println!("--- Alerts ---");
                for alert in monitoring.list_alerts() {
                    let ack_status = if alert.acknowledged { "[ACK]" } else { "" };
                    println!("{} - {} - {:?} {} - {}", alert.id, alert.service_id, alert.state, ack_status, alert.message);
                }
            }
            "acknowledge" => {
                if let Some(id) = parts.get(1) {
                    match monitoring.acknowledge_alert(id) {
                        Ok(_) => println!("Alert acknowledged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in monitoring.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "enable" => {
                monitoring.enable_monitoring();
                println!("Monitoring enabled");
            }
            "disable" => {
                monitoring.disable_monitoring();
                println!("Monitoring disabled");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
