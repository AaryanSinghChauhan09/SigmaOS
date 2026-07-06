// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/net/sigma_dnsmasq.rs — Sigma dnsmasq/Unbound DNS Resolver
//
// Implements dnsmasq/Unbound-style DNS resolver with caching,
// DHCP, DNS forwarding, and local DNS resolution.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── DNS Resolver Types ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
    PTR,
    SRV,
}

#[derive(Debug, Clone)]
pub struct DNSRecord {
    pub name: String,
    pub record_type: RecordType,
    pub ttl: u32,
    pub data: String,
    pub priority: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct DHCPLease {
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: String,
    pub lease_time: u32,
    pub expiry_time: String,
}

#[derive(Debug, Clone)]
pub struct DNSCacheEntry {
    pub domain: String,
    pub record_type: RecordType,
    pub answer: String,
    pub ttl: u32,
    pub timestamp: u64,
    pub hits: u32,
}

#[derive(Debug, Clone)]
pub struct UpstreamServer {
    pub address: String,
    pub port: u16,
    pub protocol: String,  // udp, tcp, tls
    pub enabled: bool,
}

// ─── DNS Resolver Manager ─────────────────────────────────────────────────

pub struct DnsmasqManager {
    pub local_records: HashMap<String, DNSRecord>,
    pub cache: HashMap<String, DNSCacheEntry>,
    pub dhcp_leases: HashMap<String, DHCPLease>,
    pub upstream_servers: Vec<UpstreamServer>,
    pub forward_domains: HashMap<String, String>,
    pub blocked_domains: Vec<String>,
    pub cache_enabled: bool,
    pub cache_size: u32,
    pub dhcp_enabled: bool,
    pub dhcp_range: (String, String),
    pub dhcp_lease_time: u32,
}

impl DnsmasqManager {
    pub fn new() -> Self {
        let mut manager = DnsmasqManager {
            local_records: HashMap::new(),
            cache: HashMap::new(),
            dhcp_leases: HashMap::new(),
            upstream_servers: vec![
                UpstreamServer {
                    address: "8.8.8.8".to_string(),
                    port: 53,
                    protocol: "udp".to_string(),
                    enabled: true,
                },
                UpstreamServer {
                    address: "8.8.4.4".to_string(),
                    port: 53,
                    protocol: "udp".to_string(),
                    enabled: true,
                },
                UpstreamServer {
                    address: "1.1.1.1".to_string(),
                    port: 53,
                    protocol: "udp".to_string(),
                    enabled: true,
                },
            ],
            forward_domains: HashMap::new(),
            blocked_domains: vec![
                "ads.example.com".to_string(),
                "tracker.example.com".to_string(),
            ],
            cache_enabled: true,
            cache_size: 1000,
            dhcp_enabled: false,
            dhcp_range: ("192.168.1.100".to_string(), "192.168.1.200".to_string()),
            dhcp_lease_time: 3600,
        };

        manager.init_local_records();
        manager
    }

    /// Initialize local DNS records
    fn init_local_records(&mut self) {
        // Local domain records
        self.local_records.insert("localhost.local".to_string(), DNSRecord {
            name: "localhost.local".to_string(),
            record_type: RecordType::A,
            ttl: 3600,
            data: "127.0.0.1".to_string(),
            priority: None,
        });

        self.local_records.insert("router.local".to_string(), DNSRecord {
            name: "router.local".to_string(),
            record_type: RecordType::A,
            ttl: 3600,
            data: "192.168.1.1".to_string(),
            priority: None,
        });

        self.local_records.insert("gateway.local".to_string(), DNSRecord {
            name: "gateway.local".to_string(),
            record_type: RecordType::A,
            ttl: 3600,
            data: "192.168.1.1".to_string(),
            priority: None,
        });

        // MX record
        self.local_records.insert("local".to_string(), DNSRecord {
            name: "local".to_string(),
            record_type: RecordType::MX,
            ttl: 3600,
            data: "mail.local".to_string(),
            priority: Some(10),
        });
    }

    /// Add local DNS record
    pub fn add_record(&mut self, name: String, record_type: RecordType, data: String, ttl: u32) -> Result<(), String> {
        let record = DNSRecord {
            name: name.clone(),
            record_type,
            ttl,
            data,
            priority: None,
        };

        self.local_records.insert(name, record);
        Ok(())
    }

    /// Remove DNS record
    pub fn remove_record(&mut self, name: &str) -> Result<(), String> {
        if self.local_records.remove(name).is_some() {
            Ok(())
        } else {
            Err("Record not found".to_string())
        }
    }

    /// Query DNS (simulated)
    pub fn query(&mut self, domain: &str, record_type: RecordType) -> Result<String, String> {
        // Check blocked domains
        if self.blocked_domains.iter().any(|d| domain.ends_with(d) || domain == *d) {
            return Err("Domain blocked".to_string());
        }

        // Check local records
        let key = format!("{}:{:?}", domain, record_type);
        if let Some(record) = self.local_records.get(&key) {
            return Ok(record.data.clone());
        }

        // Check cache
        if self.cache_enabled {
            if let Some(entry) = self.cache.get(&key) {
                entry.hits += 1;
                return Ok(entry.answer.clone());
            }
        }

        // Forward to upstream (simulated)
        let answer = self.forward_query(domain, record_type)?;

        // Cache the result
        if self.cache_enabled {
            self.cache.insert(key, DNSCacheEntry {
                domain: domain.to_string(),
                record_type,
                answer: answer.clone(),
                ttl: 300,
                timestamp: 0,
                hits: 0,
            });
        }

        Ok(answer)
    }

    /// Forward query to upstream server
    fn forward_query(&self, domain: &str, record_type: RecordType) -> Result<String, String> {
        // Simulate DNS query
        match record_type {
            RecordType::A => Ok(format!("93.184.216.34")),  // example.com
            RecordType::AAAA => Ok("2606:2800:220:1:248:1893:25c8:1946".to_string()),
            RecordType::MX => Ok("10 mail.example.com".to_string()),
            RecordType::TXT => Ok("v=spf1 include:_spf.example.com ~all".to_string()),
            _ => Ok("NXDOMAIN".to_string()),
        }
    }

    /// Add DHCP lease
    pub fn add_dhcp_lease(&mut self, mac: String, ip: String, hostname: String) -> Result<DHCPLease, String> {
        let lease = DHCPLease {
            mac_address: mac.clone(),
            ip_address: ip.clone(),
            hostname,
            lease_time: self.dhcp_lease_time,
            expiry_time: "now".to_string(),
        };

        self.dhcp_leases.insert(mac.clone(), lease.clone());
        Ok(lease)
    }

    /// Remove DHCP lease
    pub fn remove_dhcp_lease(&mut self, mac: &str) -> Result<(), String> {
        if self.dhcp_leases.remove(mac).is_some() {
            Ok(())
        } else {
            Err("Lease not found".to_string())
        }
    }

    /// Add upstream DNS server
    pub fn add_upstream_server(&mut self, address: String, port: u16, protocol: String) {
        self.upstream_servers.push(UpstreamServer {
            address,
            port,
            protocol,
            enabled: true,
        });
    }

    /// Remove upstream DNS server
    pub fn remove_upstream_server(&mut self, address: &str) -> bool {
        let original_len = self.upstream_servers.len();
        self.upstream_servers.retain(|s| s.address != address);
        self.upstream_servers.len() < original_len
    }

    /// Add domain forwarder
    pub fn add_domain_forward(&mut self, domain: String, server: String) {
        self.forward_domains.insert(domain, server);
    }

    /// Add blocked domain
    pub fn add_blocked_domain(&mut self, domain: String) {
        if !self.blocked_domains.contains(&domain) {
            self.blocked_domains.push(domain);
        }
    }

    /// Remove blocked domain
    pub fn remove_blocked_domain(&mut self, domain: &str) -> bool {
        let original_len = self.blocked_domains.len();
        self.blocked_domains.retain(|d| d != domain);
        self.blocked_domains.len() < original_len
    }

    /// Clear DNS cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("cache_size".to_string(), self.cache.len() as u32);
        stats.insert("max_cache_size".to_string(), self.cache_size);
        
        let total_hits: u32 = self.cache.values().map(|e| e.hits).sum();
        stats.insert("total_hits".to_string(), total_hits);
        
        stats
    }

    /// Get all local records
    pub fn get_local_records(&self) -> Vec<&DNSRecord> {
        self.local_records.values().collect()
    }

    /// Get all DHCP leases
    pub fn get_dhcp_leases(&self) -> Vec<&DHCPLease> {
        self.dhcp_leases.values().collect()
    }

    /// Get upstream servers
    pub fn get_upstream_servers(&self) -> &Vec<UpstreamServer> {
        &self.upstream_servers
    }

    /// Enable/disable DHCP
    pub fn set_dhcp_enabled(&mut self, enabled: bool) {
        self.dhcp_enabled = enabled;
    }

    /// Enable/disable cache
    pub fn set_cache_enabled(&mut self, enabled: bool) {
        self.cache_enabled = enabled;
    }

    /// Set DHCP range
    pub fn set_dhcp_range(&mut self, start: String, end: String) {
        self.dhcp_range = (start, end);
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("local_records".to_string(), self.local_records.len() as u32);
        stats.insert("cache_entries".to_string(), self.cache.len() as u32);
        stats.insert("dhcp_leases".to_string(), self.dhcp_leases.len() as u32);
        stats.insert("upstream_servers".to_string(), self.upstream_servers.len() as u32);
        stats.insert("blocked_domains".to_string(), self.blocked_domains.len() as u32);
        stats.insert("forward_domains".to_string(), self.forward_domains.len() as u32);
        
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut dnsmasq = DnsmasqManager::new();
    
    println!("Sigma dnsmasq/Unbound v0.1 - DNS Resolver & DHCP");
    
    loop {
        println!("\n--- DNS Resolver Commands ---");
        println!("records            - List local DNS records");
        println!("add_record <name> <type> <data> [ttl] - Add record");
        println!("remove_record <name> - Remove record");
        println!("query <domain> <type> - Query DNS");
        println!("cache              - Show cache statistics");
        println!("clear_cache        - Clear DNS cache");
        println!("dhcp_leases        - List DHCP leases");
        println!("add_lease <mac> <ip> <hostname> - Add DHCP lease");
        println!("remove_lease <mac> - Remove DHCP lease");
        println!("upstream           - List upstream servers");
        println!("add_upstream <addr> [port] [proto] - Add upstream");
        println!("remove_upstream <addr> - Remove upstream");
        println!("blocked            - List blocked domains");
        println!("block <domain>     - Block domain");
        println!("unblock <domain>   - Unblock domain");
        println!("forward <domain> <server> - Add domain forward");
        println!("dhcp <on/off>      - Enable/disable DHCP");
        println!("cache <on/off>     - Enable/disable cache");
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
            "records" => {
                println!("--- Local DNS Records ---");
                for record in dnsmasq.get_local_records() {
                    println!("{} - {:?} - {} (TTL: {})", record.name, record.record_type, record.data, record.ttl);
                }
            }
            "add_record" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let record_type = match parts[2] {
                        "A" => RecordType::A,
                        "AAAA" => RecordType::AAAA,
                        "CNAME" => RecordType::CNAME,
                        "MX" => RecordType::MX,
                        "TXT" => RecordType::TXT,
                        "NS" => RecordType::NS,
                        "PTR" => RecordType::PTR,
                        _ => RecordType::A,
                    };
                    let data = parts[3].to_string();
                    let ttl = parts.get(4).and_then(|t| t.parse::<u32>().ok()).unwrap_or(3600);
                    match dnsmasq.add_record(name, record_type, data, ttl) {
                        Ok(_) => println!("Record added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_record" => {
                if let Some(name) = parts.get(1) {
                    match dnsmasq.remove_record(name) {
                        Ok(_) => println!("Record removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "query" => {
                if parts.len() >= 3 {
                    let domain = parts[1];
                    let record_type = match parts[2] {
                        "A" => RecordType::A,
                        "AAAA" => RecordType::AAAA,
                        "MX" => RecordType::MX,
                        "TXT" => RecordType::TXT,
                        _ => RecordType::A,
                    };
                    match dnsmasq.query(domain, record_type) {
                        Ok(answer) => println!("Answer: {}", answer),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "cache" => {
                println!("--- Cache Statistics ---");
                for (key, value) in dnsmasq.get_cache_stats() {
                    println!("{}: {}", key, value);
                }
            }
            "clear_cache" => {
                dnsmasq.clear_cache();
                println!("Cache cleared");
            }
            "dhcp_leases" => {
                println!("--- DHCP Leases ---");
                for lease in dnsmasq.get_dhcp_leases() {
                    println!("{} - {} - {} ({}s)", lease.mac_address, lease.ip_address, lease.hostname, lease.lease_time);
                }
            }
            "add_lease" => {
                if parts.len() >= 4 {
                    let mac = parts[1].to_string();
                    let ip = parts[2].to_string();
                    let hostname = parts[3].to_string();
                    match dnsmasq.add_dhcp_lease(mac, ip, hostname) {
                        Ok(_) => println!("Lease added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "remove_lease" => {
                if let Some(mac) = parts.get(1) {
                    match dnsmasq.remove_dhcp_lease(mac) {
                        Ok(_) => println!("Lease removed"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "upstream" => {
                println!("--- Upstream DNS Servers ---");
                for server in dnsmasq.get_upstream_servers() {
                    println!("{}:{} ({}) - {}", server.address, server.port, server.protocol, 
                        if server.enabled { "enabled" } else { "disabled" });
                }
            }
            "add_upstream" => {
                if parts.len() >= 2 {
                    let address = parts[1].to_string();
                    let port = parts.get(2).and_then(|p| p.parse::<u16>().ok()).unwrap_or(53);
                    let protocol = parts.get(3).unwrap_or(&"udp").to_string();
                    dnsmasq.add_upstream_server(address, port, protocol);
                    println!("Upstream server added");
                }
            }
            "remove_upstream" => {
                if let Some(addr) = parts.get(1) {
                    if dnsmasq.remove_upstream_server(addr) {
                        println!("Upstream server removed");
                    } else {
                        println!("Server not found");
                    }
                }
            }
            "blocked" => {
                println!("--- Blocked Domains ---");
                for domain in &dnsmasq.blocked_domains {
                    println!("{}", domain);
                }
            }
            "block" => {
                if let Some(domain) = parts.get(1) {
                    dnsmasq.add_blocked_domain(domain.to_string());
                    println!("Domain blocked");
                }
            }
            "unblock" => {
                if let Some(domain) = parts.get(1) {
                    if dnsmasq.remove_blocked_domain(domain) {
                        println!("Domain unblocked");
                    } else {
                        println!("Domain not found");
                    }
                }
            }
            "forward" => {
                if parts.len() >= 3 {
                    let domain = parts[1].to_string();
                    let server = parts[2].to_string();
                    dnsmasq.add_domain_forward(domain, server);
                    println!("Domain forward added");
                }
            }
            "dhcp" => {
                if let Some(state) = parts.get(1) {
                    let enabled = *state == "on";
                    dnsmasq.set_dhcp_enabled(enabled);
                    println!("DHCP: {}", if enabled { "enabled" } else { "disabled" });
                }
            }
            "cache" => {
                if let Some(state) = parts.get(1) {
                    let enabled = *state == "on";
                    dnsmasq.set_cache_enabled(enabled);
                    println!("Cache: {}", if enabled { "enabled" } else { "disabled" });
                }
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in dnsmasq.get_statistics() {
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
