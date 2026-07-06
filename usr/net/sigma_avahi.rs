// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/net/sigma_avahi.rs — Sigma Avahi Service Discovery
//
// Implements Avahi-style mDNS/DNS-SD service discovery with
// service registration, browsing, and hostname resolution.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Avahi Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceState {
    Unregistered,
    Registering,
    Registered,
    Collision,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ServiceType {
    _http_tcp,
    _https_tcp,
    _ssh_tcp,
    _ftp_tcp,
    _ipp_tcp,
    _printer_tcp,
    _smb_tcp,
    _raop_tcp,
    _airplay_tcp,
    _chromecast_tcp,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub service_type: ServiceType,
    pub port: u16,
    pub hostname: String,
    pub addresses: Vec<String>,
    pub txt_records: HashMap<String, String>,
    pub state: ServiceState,
    pub interface: String,
}

#[derive(Debug, Clone)]
pub struct ServiceBrowser {
    pub service_type: ServiceType,
    pub browsing: bool,
    pub discovered_services: Vec<String>,  // Service IDs
}

#[derive(Debug, Clone)]
pub struct HostNameResolver {
    pub hostname: String,
    pub addresses: Vec<String>,
    pub resolved: bool,
}

// ─── Avahi Manager ───────────────────────────────────────────────────────

pub struct AvahiManager {
    pub services: HashMap<String, Service>,
    pub browsers: HashMap<String, ServiceBrowser>,
    pub resolvers: HashMap<String, HostNameResolver>,
    pub local_hostname: String,
    pub domain: String,
    pub daemon_running: bool,
}

impl AvahiManager {
    pub fn new() -> Self {
        let mut manager = AvahiManager {
            services: HashMap::new(),
            browsers: HashMap::new(),
            resolvers: HashMap::new(),
            local_hostname: "sigmaos-host".to_string(),
            domain: "local".to_string(),
            daemon_running: true,
        };

        manager.init_default_services();
        manager
    }

    /// Initialize default services
    fn init_default_services(&mut self) {
        // Register SSH service
        self.register_service(
            "ssh".to_string(),
            "SigmaOS SSH".to_string(),
            ServiceType::_ssh_tcp,
            22,
            vec![],
        ).ok();

        // Register HTTP service
        self.register_service(
            "http".to_string(),
            "SigmaOS Web".to_string(),
            ServiceType::_http_tcp,
            80,
            vec![("path".to_string(), "/".to_string())],
        ).ok();
    }

    /// Register a service
    pub fn register_service(&mut self, id: String, name: String, service_type: ServiceType, port: u16, txt_records: Vec<(String, String)>) -> Result<Service, String> {
        if self.services.contains_key(&id) {
            return Err("Service already registered".to_string());
        }

        let mut txt_map = HashMap::new();
        for (key, value) in txt_records {
            txt_map.insert(key, value);
        }

        let service = Service {
            id: id.clone(),
            name,
            service_type,
            port,
            hostname: format!("{}.{}", self.local_hostname, self.domain),
            addresses: vec!["192.168.1.100".to_string()],
            txt_records: txt_map,
            state: ServiceState::Registering,
            interface: "eth0".to_string(),
        };

        self.services.insert(id.clone(), service.clone());
        
        // Simulate registration
        if let Some(s) = self.services.get_mut(&id) {
            s.state = ServiceState::Registered;
        }

        Ok(service)
    }

    /// Unregister a service
    pub fn unregister_service(&mut self, id: &str) -> Result<(), String> {
        if self.services.remove(id).is_some() {
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Update service
    pub fn update_service(&mut self, id: &str, txt_records: HashMap<String, String>) -> Result<(), String> {
        if let Some(service) = self.services.get_mut(id) {
            service.txt_records = txt_records;
            Ok(())
        } else {
            Err("Service not found".to_string())
        }
    }

    /// Browse for services
    pub fn browse_services(&mut self, service_type: ServiceType) -> Result<ServiceBrowser, String> {
        let browser_id = format!("browser_{}", self.browsers.len());
        
        let browser = ServiceBrowser {
            service_type: service_type.clone(),
            browsing: true,
            discovered_services: vec![],
        };

        self.browsers.insert(browser_id.clone(), browser.clone());
        
        // Simulate discovering services
        self.simulate_service_discovery(&browser_id, &service_type);
        
        Ok(browser)
    }

    /// Simulate service discovery
    fn simulate_service_discovery(&mut self, browser_id: &str, service_type: &ServiceType) {
        // Add some simulated discovered services
        let discovered_id = format!("discovered_{}", self.services.len());
        
        let service = Service {
            id: discovered_id.clone(),
            name: "Remote Device".to_string(),
            service_type: service_type.clone(),
            port: 8080,
            hostname: "remote-device.local".to_string(),
            addresses: vec!["192.168.1.200".to_string()],
            txt_records: HashMap::new(),
            state: ServiceState::Registered,
            interface: "eth0".to_string(),
        };

        self.services.insert(discovered_id.clone(), service.clone());
        
        if let Some(browser) = self.browsers.get_mut(browser_id) {
            browser.discovered_services.push(discovered_id);
        }
    }

    /// Stop browsing
    pub fn stop_browsing(&mut self, browser_id: &str) -> Result<(), String> {
        if let Some(browser) = self.browsers.get_mut(browser_id) {
            browser.browsing = false;
            Ok(())
        } else {
            Err("Browser not found".to_string())
        }
    }

    /// Resolve hostname
    pub fn resolve_hostname(&mut self, hostname: String) -> Result<HostNameResolver, String> {
        let resolver_id = format!("resolver_{}", self.resolvers.len());
        
        let resolver = HostNameResolver {
            hostname: hostname.clone(),
            addresses: vec!["192.168.1.100".to_string()],
            resolved: true,
        };

        self.resolvers.insert(resolver_id.clone(), resolver.clone());
        Ok(resolver)
    }

    /// Get service by ID
    pub fn get_service(&self, id: &str) -> Option<&Service> {
        self.services.get(id)
    }

    /// List all registered services
    pub fn list_services(&self) -> Vec<&Service> {
        self.services.values().collect()
    }

    /// List services by type
    pub fn list_services_by_type(&self, service_type: &ServiceType) -> Vec<&Service> {
        self.services.values()
            .filter(|s| match (&s.service_type, service_type) {
                (ServiceType::Custom(a), ServiceType::Custom(b)) => a == b,
                (a, b) => std::mem::discriminant(a) == std::mem::discriminant(b),
            })
            .collect()
    }

    /// Get browser by ID
    pub fn get_browser(&self, id: &str) -> Option<&ServiceBrowser> {
        self.browsers.get(id)
    }

    /// Set local hostname
    pub fn set_hostname(&mut self, hostname: String) {
        self.local_hostname = hostname;
    }

    /// Set domain
    pub fn set_domain(&mut self, domain: String) {
        self.domain = domain;
    }

    /// Start daemon
    pub fn start_daemon(&mut self) {
        self.daemon_running = true;
    }

    /// Stop daemon
    pub fn stop_daemon(&mut self) {
        self.daemon_running = false;
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("registered_services".to_string(), self.services.len() as u32);
        stats.insert("active_browsers".to_string(), self.browsers.values().filter(|b| b.browsing).count() as u32);
        stats.insert("active_resolvers".to_string(), self.resolvers.len() as u32);
        stats.insert("daemon_running".to_string(), if self.daemon_running { 1 } else { 0 });
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut avahi = AvahiManager::new();
    
    println!("Sigma Avahi v0.1 - Service Discovery");
    
    loop {
        println!("\n--- Avahi Commands ---");
        println!("services          - List all services");
        println!("register <id> <name> <type> <port> - Register service");
        println!("unregister <id>   - Unregister service");
        println!("service_info <id>  - Show service details");
        println!("browse <type>      - Browse for services");
        println!("browsers          - List active browsers");
        println!("stop_browse <id>  - Stop browsing");
        println!("resolve <hostname> - Resolve hostname");
        println!("set_hostname <name> - Set local hostname");
        println!("set_domain <domain> - Set domain");
        println!("stats             - Show statistics");
        println!("start_daemon      - Start daemon");
        println!("stop_daemon       - Stop daemon");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "services" => {
                println!("--- Services ---");
                for service in avahi.list_services() {
                    println!("{} - {} - {:?} - {} - {:?}", 
                        service.id, service.name, service.service_type, service.port, service.state);
                }
            }
            "register" => {
                if parts.len() >= 5 {
                    let id = parts[1].to_string();
                    let name = parts[2].to_string();
                    let service_type = match parts[3] {
                        "_http._tcp" => ServiceType::_http_tcp,
                        "_https._tcp" => ServiceType::_https_tcp,
                        "_ssh._tcp" => ServiceType::_ssh_tcp,
                        "_ftp._tcp" => ServiceType::_ftp_tcp,
                        "_ipp._tcp" => ServiceType::_ipp_tcp,
                        "_printer._tcp" => ServiceType::_printer_tcp,
                        "_smb._tcp" => ServiceType::_smb_tcp,
                        "_raop._tcp" => ServiceType::_raop_tcp,
                        "_airplay._tcp" => ServiceType::_airplay_tcp,
                        "_chromecast._tcp" => ServiceType::_chromecast_tcp,
                        other => ServiceType::Custom(other.to_string()),
                    };
                    let port = parts[4].parse::<u16>().unwrap_or(8080);
                    match avahi.register_service(id, name, service_type, port, vec![]) {
                        Ok(_) => println!("Service registered"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "unregister" => {
                if let Some(id) = parts.get(1) {
                    match avahi.unregister_service(id) {
                        Ok(_) => println!("Service unregistered"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "service_info" => {
                if let Some(id) = parts.get(1) {
                    if let Some(service) = avahi.get_service(id) {
                        println!("--- Service Info ---");
                        println!("ID: {}", service.id);
                        println!("Name: {}", service.name);
                        println!("Type: {:?}", service.service_type);
                        println!("Port: {}", service.port);
                        println!("Hostname: {}", service.hostname);
                        println!("Addresses: {:?}", service.addresses);
                        println!("State: {:?}", service.state);
                        println!("Interface: {}", service.interface);
                        println!("TXT Records: {:?}", service.txt_records);
                    }
                }
            }
            "browse" => {
                if let Some(type_str) = parts.get(1) {
                    let service_type = match *type_str {
                        "_http._tcp" => ServiceType::_http_tcp,
                        "_https._tcp" => ServiceType::_https_tcp,
                        "_ssh._tcp" => ServiceType::_ssh_tcp,
                        "_ftp._tcp" => ServiceType::_ftp_tcp,
                        "_ipp._tcp" => ServiceType::_ipp_tcp,
                        "_printer._tcp" => ServiceType::_printer_tcp,
                        "_smb._tcp" => ServiceType::_smb_tcp,
                        "_raop._tcp" => ServiceType::_raop_tcp,
                        "_airplay._tcp" => ServiceType::_airplay_tcp,
                        "_chromecast._tcp" => ServiceType::_chromecast_tcp,
                        other => ServiceType::Custom(other.to_string()),
                    };
                    match avahi.browse_services(service_type) {
                        Ok(_) => println!("Browsing started"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "browsers" => {
                println!("--- Active Browsers ---");
                for (id, browser) in &avahi.browsers {
                    println!("{} - {:?} - {} - {} discovered", 
                        id, browser.service_type, browser.browsing, browser.discovered_services.len());
                }
            }
            "stop_browse" => {
                if let Some(id) = parts.get(1) {
                    match avahi.stop_browsing(id) {
                        Ok(_) => println!("Browsing stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "resolve" => {
                if let Some(hostname) = parts.get(1) {
                    match avahi.resolve_hostname(hostname.to_string()) {
                        Ok(resolver) => println!("Resolved: {} -> {:?}", resolver.hostname, resolver.addresses),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "set_hostname" => {
                if let Some(name) = parts.get(1) {
                    avahi.set_hostname(name.to_string());
                    println!("Hostname set");
                }
            }
            "set_domain" => {
                if let Some(domain) = parts.get(1) {
                    avahi.set_domain(domain.to_string());
                    println!("Domain set");
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in avahi.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "start_daemon" => {
                avahi.start_daemon();
                println!("Daemon started");
            }
            "stop_daemon" => {
                avahi.stop_daemon();
                println!("Daemon stopped");
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
