// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/system/sigma_service.rs — Sigma Service Manager
//
// Implements systemd/OpenRC-style service management for SigmaOS
// with dependency tracking, socket activation, and logging.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Service Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceType {
    Simple,
    Forking,
    Oneshot,
    Notify,
    DBus,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub command: String,
    pub dependencies: Vec<String>,
    pub enabled: bool,
    pub auto_start: bool,
    pub restart_policy: String,
    pub restart_count: u32,
    pub memory_limit: Option<u64>,
    pub cpu_limit: Option<f64>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ServiceLog {
    pub service_name: String,
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

// ─── Service Manager ───────────────────────────────────────────────────────

pub struct ServiceManager {
    pub services: HashMap<String, Service>,
    pub logs: Vec<ServiceLog>,
    pub target_state: String,
}

impl ServiceManager {
    pub fn new() -> Self {
        let mut manager = ServiceManager {
            services: HashMap::new(),
            logs: Vec::new(),
            target_state: "multi-user".to_string(),
        };
        
        manager.init_default_services();
        manager
    }

    /// Initialize default system services
    fn init_default_services(&mut self) {
        // Network service
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin:/usr/sbin".to_string());
        self.services.insert("network".to_string(), Service {
            name: "network".to_string(),
            description: "Network connectivity".to_string(),
            service_type: ServiceType::Simple,
            state: ServiceState::Running,
            pid: Some(100),
            command: "/usr/sbin/networkd".to_string(),
            dependencies: vec![],
            enabled: true,
            auto_start: true,
            restart_policy: "always".to_string(),
            restart_count: 0,
            memory_limit: Some(64 * 1024 * 1024),  // 64MB
            cpu_limit: Some(0.5),  // 50%
            environment: env,
        });

        // SSH service
        let mut ssh_env = HashMap::new();
        ssh_env.insert("PATH".to_string(), "/usr/bin:/usr/sbin".to_string());
        self.services.insert("sshd".to_string(), Service {
            name: "sshd".to_string(),
            description: "OpenSSH server daemon".to_string(),
            service_type: ServiceType::Forking,
            state: ServiceState::Running,
            pid: Some(101),
            command: "/usr/sbin/sshd".to_string(),
            dependencies: vec!["network".to_string()],
            enabled: true,
            auto_start: true,
            restart_policy: "on-failure".to_string(),
            restart_count: 0,
            memory_limit: Some(32 * 1024 * 1024),  // 32MB
            cpu_limit: Some(0.3),  // 30%
            environment: ssh_env,
        });

        // Web server
        let mut http_env = HashMap::new();
        http_env.insert("PATH".to_string(), "/usr/bin:/usr/sbin".to_string());
        self.services.insert("httpd".to_string(), Service {
            name: "httpd".to_string(),
            description: "Apache HTTP server".to_string(),
            service_type: ServiceType::Forking,
            state: ServiceState::Stopped,
            pid: None,
            command: "/usr/sbin/httpd".to_string(),
            dependencies: vec!["network".to_string()],
            enabled: false,
            auto_start: false,
            restart_policy: "on-failure".to_string(),
            restart_count: 0,
            memory_limit: Some(128 * 1024 * 1024),  // 128MB
            cpu_limit: Some(0.7),  // 70%
            environment: http_env,
        });

        // Database service
        let mut pg_env = HashMap::new();
        pg_env.insert("PATH".to_string(), "/usr/bin:/usr/sbin".to_string());
        pg_env.insert("PGDATA".to_string(), "/var/lib/postgresql/data".to_string());
        self.services.insert("postgresql".to_string(), Service {
            name: "postgresql".to_string(),
            description: "PostgreSQL database server".to_string(),
            service_type: ServiceType::Simple,
            state: ServiceState::Stopped,
            pid: None,
            command: "/usr/bin/postgres".to_string(),
            dependencies: vec!["network".to_string()],
            enabled: false,
            auto_start: false,
            restart_policy: "always".to_string(),
            restart_count: 0,
            memory_limit: Some(256 * 1024 * 1024),  // 256MB
            cpu_limit: Some(0.8),  // 80%
            environment: pg_env,
        });

        // System logger
        let mut log_env = HashMap::new();
        log_env.insert("PATH".to_string(), "/usr/bin:/usr/sbin".to_string());
        self.services.insert("syslog".to_string(), Service {
            name: "syslog".to_string(),
            description: "System logging daemon".to_string(),
            service_type: ServiceType::Simple,
            state: ServiceState::Running,
            pid: Some(102),
            command: "/usr/sbin/syslogd".to_string(),
            dependencies: vec![],
            enabled: true,
            auto_start: true,
            restart_policy: "always".to_string(),
            restart_count: 0,
            memory_limit: Some(16 * 1024 * 1024),  // 16MB
            cpu_limit: Some(0.1),  // 10%
            environment: log_env,
        });
    }

    /// Start service
    pub fn start_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            // Check dependencies
            for dep in &service.dependencies {
                if let Some(dep_service) = self.services.get(dep) {
                    if dep_service.state != ServiceState::Running {
                        return Err(format!("Dependency {} is not running", dep));
                    }
                }
            }
            
            service.state = ServiceState::Starting;
            self.log_event(name, "INFO", "Service starting");
            
            // Simulate service start
            service.state = ServiceState::Running;
            service.pid = Some(1000 + self.services.len() as u32);
            self.log_event(name, "INFO", "Service started successfully");
            
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Stop service
    pub fn stop_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.state = ServiceState::Stopping;
            self.log_event(name, "INFO", "Service stopping");
            
            service.state = ServiceState::Stopped;
            service.pid = None;
            self.log_event(name, "INFO", "Service stopped");
            
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Restart service
    pub fn restart_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.restart_count += 1;
        }
        self.stop_service(name)?;
        self.start_service(name)?;
        self.log_event(name, "INFO", "Service restarted");
        Ok(())
    }

    /// Get service status
    pub fn get_service_status(&self, name: &str) -> Option<&Service> {
        self.services.get(name)
    }

    /// Set service memory limit
    pub fn set_memory_limit(&mut self, name: &str, limit: u64) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.memory_limit = Some(limit);
            self.log_event(name, "INFO", &format!("Memory limit set to {} MB", limit / (1024 * 1024)));
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Set service CPU limit
    pub fn set_cpu_limit(&mut self, name: &str, limit: f64) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.cpu_limit = Some(limit.min(1.0));
            self.log_event(name, "INFO", &format!("CPU limit set to {}%", limit * 100.0));
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Set environment variable for service
    pub fn set_env_var(&mut self, name: &str, key: String, value: String) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.environment.insert(key, value);
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Enable service (auto-start on boot)
    pub fn enable_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.enabled = true;
            self.log_event(name, "INFO", "Service enabled");
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Disable service
    pub fn disable_service(&mut self, name: &str) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(name) {
            service.enabled = false;
            self.log_event(name, "INFO", "Service disabled");
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Add custom service
    pub fn add_service(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    /// Get all services
    pub fn get_all_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    /// Get running services
    pub fn get_running_services(&self) -> Vec<&Service> {
        self.services.values()
            .filter(|s| s.state == ServiceState::Running)
            .collect()
    }

    /// Log event
    fn log_event(&mut self, service_name: &str, level: &str, message: &str) {
        let log = ServiceLog {
            service_name: service_name.to_string(),
            timestamp: "now".to_string(),
            level: level.to_string(),
            message: message.to_string(),
        };
        self.logs.push(log);
    }

    /// Get service logs
    pub fn get_service_logs(&self, service_name: &str) -> Vec<&ServiceLog> {
        self.logs.iter()
            .filter(|l| l.service_name == service_name)
            .collect()
    }

    /// Get state name
    pub fn get_state_name(&self, state: ServiceState) -> &str {
        match state {
            ServiceState::Stopped => "Stopped",
            ServiceState::Starting => "Starting",
            ServiceState::Running => "Running",
            ServiceState::Stopping => "Stopping",
            ServiceState::Failed => "Failed",
        }
    }

    /// Get type name
    pub fn get_type_name(&self, service_type: ServiceType) -> &str {
        match service_type {
            ServiceType::Simple => "Simple",
            ServiceType::Forking => "Forking",
            ServiceType::Oneshot => "Oneshot",
            ServiceType::Notify => "Notify",
            ServiceType::DBus => "DBus",
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = ServiceManager::new();
    
    println!("Sigma Service Manager v0.1 - systemd/OpenRC Style");
    
    loop {
        println!("\n--- System Status ---");
        println!("Target: {}", manager.target_state);
        println!("Running Services: {}", manager.get_running_services().len());
        
        println!("\nCommands: start <service>, stop <service>, restart <service>, enable <service>, disable <service>, status <service>, services, logs <service>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "start" => {
                if let Some(arg) = parts.get(1) {
                    match manager.start_service(arg) {
                        Ok(_) => println!("Service started"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stop" => {
                if let Some(arg) = parts.get(1) {
                    match manager.stop_service(arg) {
                        Ok(_) => println!("Service stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "restart" => {
                if let Some(arg) = parts.get(1) {
                    match manager.restart_service(arg) {
                        Ok(_) => println!("Service restarted"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "enable" => {
                if let Some(arg) = parts.get(1) {
                    match manager.enable_service(arg) {
                        Ok(_) => println!("Service enabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "disable" => {
                if let Some(arg) = parts.get(1) {
                    match manager.disable_service(arg) {
                        Ok(_) => println!("Service disabled"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "status" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(service) = manager.get_service_status(arg) {
                        println!("--- Service Status ---");
                        println!("Name: {}", service.name);
                        println!("Description: {}", service.description);
                        println!("State: {}", manager.get_state_name(service.state));
                        println!("Type: {}", manager.get_type_name(service.service_type));
                        println!("PID: {:?}", service.pid);
                        println!("Enabled: {}", service.enabled);
                        println!("Auto-start: {}", service.auto_start);
                        println!("Restart Policy: {}", service.restart_policy);
                        if !service.dependencies.is_empty() {
                            println!("Dependencies: {}", service.dependencies.join(", "));
                        }
                    }
                }
            }
            "services" => {
                println!("--- All Services ---");
                for service in manager.get_all_services() {
                    let status = if service.enabled { "[ENABLED]" } else { "" };
                    println!("{} - {} ({}) {}", service.name, manager.get_state_name(service.state), service.description, status);
                }
            }
            "logs" => {
                if let Some(arg) = parts.get(1) {
                    let logs = manager.get_service_logs(arg);
                    println!("--- Service Logs ---");
                    for log in logs {
                        println!("[{}] {} - {}: {}", log.timestamp, log.level, log.service_name, log.message);
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
