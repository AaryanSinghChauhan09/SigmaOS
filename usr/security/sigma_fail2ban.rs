// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_fail2ban.rs — Sigma Fail2Ban Intrusion Prevention
//
// Implements Fail2Ban-style intrusion prevention with log monitoring,
// IP banning, jail management, and automated protection rules.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Fail2Ban Types ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BanAction {
    Block,
    Reject,
    Tarpit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JailState {
    Active,
    Stopped,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub log_path: String,
    pub regex_patterns: Vec<String>,
    pub max_retry: u32,
    pub find_time: u64,  // seconds
    pub ignore_ip: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BanInfo {
    pub ip: String,
    pub jail: String,
    pub ban_time: String,
    pub attempts: u32,
    pub ban_duration: u64,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct Jail {
    pub name: String,
    pub state: JailState,
    pub enabled: bool,
    pub filter: Filter,
    pub ban_action: BanAction,
    pub ban_time: u64,  // seconds
    pub max_retry: u32,
    pub find_time: u64,
    pub banned_ips: HashMap<String, BanInfo>,
    pub total_bans: u32,
    pub ignore_cache: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub ip: String,
    pub service: String,
    pub message: String,
    pub matched_pattern: Option<String>,
}

// ─── Fail2Ban Manager ─────────────────────────────────────────────────────

pub struct Fail2BanManager {
    pub jails: HashMap<String, Jail>,
    pub global_bans: HashMap<String, BanInfo>,
    pub log_entries: Vec<LogEntry>,
    pub whitelist: Vec<String>,
    pub blacklist: Vec<String>,
    pub auto_ban_enabled: bool,
    pub ban_email_enabled: bool,
    pub notification_email: String,
}

impl Fail2BanManager {
    pub fn new() -> Self {
        let mut manager = Fail2BanManager {
            jails: HashMap::new(),
            global_bans: HashMap::new(),
            log_entries: Vec::new(),
            whitelist: vec![
                "127.0.0.1".to_string(),
                "::1".to_string(),
                "192.168.0.0/16".to_string(),
                "10.0.0.0/8".to_string(),
            ],
            blacklist: vec![],
            auto_ban_enabled: true,
            ban_email_enabled: false,
            notification_email: String::new(),
        };
        
        manager.init_default_jails();
        manager
    }

    /// Initialize default security jails
    fn init_default_jails(&mut self) {
        // SSH jail
        let ssh_filter = Filter {
            name: "sshd".to_string(),
            log_path: "/var/log/auth.log".to_string(),
            regex_patterns: vec![
                r"Failed password for .* from (\d+\.\d+\.\d+\.\d+)".to_string(),
                r"Invalid user .* from (\d+\.\d+\.\d+\.\d+)".to_string(),
                r"User .* from (\d+\.\d+\.\d+\.\d+) not allowed because not listed in AllowUsers".to_string(),
            ],
            max_retry: 5,
            find_time: 600,  // 10 minutes
            ignore_ip: vec![],
        };

        self.jails.insert("sshd".to_string(), Jail {
            name: "sshd".to_string(),
            state: JailState::Active,
            enabled: true,
            filter: ssh_filter,
            ban_action: BanAction::Block,
            ban_time: 3600,  // 1 hour
            max_retry: 5,
            find_time: 600,
            banned_ips: HashMap::new(),
            total_bans: 0,
            ignore_cache: HashMap::new(),
        });

        // HTTP jail
        let http_filter = Filter {
            name: "apache-auth".to_string(),
            log_path: "/var/log/apache2/error.log".to_string(),
            regex_patterns: vec![
                r"client (\d+\.\d+\.\d+\.\d+) authentication failure".to_string(),
                r"client (\d+\.\d+\.\d+\.\d+) user .* authentication failure".to_string(),
            ],
            max_retry: 10,
            find_time: 600,
            ignore_ip: vec![],
        };

        self.jails.insert("apache-auth".to_string(), Jail {
            name: "apache-auth".to_string(),
            state: JailState::Active,
            enabled: true,
            filter: http_filter,
            ban_action: BanAction::Reject,
            ban_time: 1800,  // 30 minutes
            max_retry: 10,
            find_time: 600,
            banned_ips: HashMap::new(),
            total_bans: 0,
            ignore_cache: HashMap::new(),
        });

        // FTP jail
        let ftp_filter = Filter {
            name: "vsftpd".to_string(),
            log_path: "/var/log/vsftpd.log".to_string(),
            regex_patterns: vec![
                r"FAIL LOGIN: Client (\d+\.\d+\.\d+\.\d+)".to_string(),
                r"authentication failure; logname=.* uid=.* euid=.* tty=.* ruser=.* rhost=(\d+\.\d+\.\d+\.\d+)".to_string(),
            ],
            max_retry: 5,
            find_time: 600,
            ignore_ip: vec![],
        };

        self.jails.insert("vsftpd".to_string(), Jail {
            name: "vsftpd".to_string(),
            state: JailState::Active,
            enabled: true,
            filter: ftp_filter,
            ban_action: BanAction::Block,
            ban_time: 3600,
            max_retry: 5,
            find_time: 600,
            banned_ips: HashMap::new(),
            total_bans: 0,
            ignore_cache: HashMap::new(),
        });
    }

    /// Create a new jail
    pub fn create_jail(&mut self, name: String, log_path: String, max_retry: u32, find_time: u64, ban_time: u64) -> Result<Jail, String> {
        if self.jails.contains_key(&name) {
            return Err("Jail already exists".to_string());
        }

        let filter = Filter {
            name: name.clone(),
            log_path,
            regex_patterns: vec![],
            max_retry,
            find_time,
            ignore_ip: vec![],
        };

        let jail = Jail {
            name: name.clone(),
            state: JailState::Active,
            enabled: true,
            filter,
            ban_action: BanAction::Block,
            ban_time,
            max_retry,
            find_time,
            banned_ips: HashMap::new(),
            total_bans: 0,
            ignore_cache: HashMap::new(),
        };

        self.jails.insert(name.clone(), jail.clone());
        Ok(jail)
    }

    /// Add regex pattern to jail filter
    pub fn add_regex_pattern(&mut self, jail: &str, pattern: String) -> Result<(), String> {
        if let Some(j) = self.jails.get_mut(jail) {
            j.filter.regex_patterns.push(pattern);
            Ok(())
        } else {
            Err("Jail not found".to_string())
        }
    }

    /// Ban an IP address
    pub fn ban_ip(&mut self, jail_name: &str, ip: String, reason: String) -> Result<(), String> {
        // Check whitelist
        if self.is_whitelisted(&ip) {
            return Err("IP is whitelisted".to_string());
        }

        if let Some(jail) = self.jails.get_mut(jail_name) {
            let ban_info = BanInfo {
                ip: ip.clone(),
                jail: jail_name.to_string(),
                ban_time: "now".to_string(),
                attempts: 1,
                ban_duration: jail.ban_time,
                reason,
            };

            jail.banned_ips.insert(ip.clone(), ban_info.clone());
            jail.total_bans += 1;
            self.global_bans.insert(ip.clone(), ban_info);

            Ok(())
        } else {
            Err("Jail not found".to_string())
        }
    }

    /// Unban an IP address
    pub fn unban_ip(&mut self, jail_name: &str, ip: &str) -> Result<(), String> {
        if let Some(jail) = self.jails.get_mut(jail_name) {
            if jail.banned_ips.remove(ip).is_some() {
                self.global_bans.remove(ip);
                Ok(())
            } else {
                Err("IP not banned in this jail".to_string())
            }
        } else {
            Err("Jail not found".to_string())
        }
    }

    /// Check if IP is whitelisted
    pub fn is_whitelisted(&self, ip: &str) -> bool {
        self.whitelist.iter().any(|entry| {
            if entry.contains('/') {
                // CIDR check (simplified)
                let parts: Vec<&str> = entry.split('/').collect();
                if parts.len() == 2 {
                    let network = parts[0];
                    ip.starts_with(network)
                } else {
                    false
                }
            } else {
                entry == ip
            }
        })
    }

    /// Add IP to whitelist
    pub fn add_whitelist(&mut self, ip: String) {
        if !self.whitelist.contains(&ip) {
            self.whitelist.push(ip);
        }
    }

    /// Remove IP from whitelist
    pub fn remove_whitelist(&mut self, ip: &str) -> bool {
        let original_len = self.whitelist.len();
        self.whitelist.retain(|x| x != ip);
        self.whitelist.len() < original_len
    }

    /// Add IP to blacklist
    pub fn add_blacklist(&mut self, ip: String) {
        if !self.blacklist.contains(&ip) {
            self.blacklist.push(ip);
        }
    }

    /// Process log entries for a jail
    pub fn process_logs(&mut self, jail_name: &str) -> Result<u32, String> {
        if let Some(jail) = self.jails.get_mut(jail_name) {
            let mut new_bans = 0;

            // Simulate log processing
            let simulated_logs = vec![
                LogEntry {
                    timestamp: "now".to_string(),
                    ip: "192.168.1.100".to_string(),
                    service: "sshd".to_string(),
                    message: "Failed password for root from 192.168.1.100".to_string(),
                    matched_pattern: Some(r"Failed password for .* from (\d+\.\d+\.\d+\.\d+)".to_string()),
                },
                LogEntry {
                    timestamp: "now".to_string(),
                    ip: "10.0.0.50".to_string(),
                    service: "sshd".to_string(),
                    message: "Invalid user admin from 10.0.0.50".to_string(),
                    matched_pattern: Some(r"Invalid user .* from (\d+\.\d+\.\d+\.\d+)".to_string()),
                },
            ];

            for entry in simulated_logs {
                self.log_entries.push(entry.clone());
                
                // Check if IP should be banned
                let attempt_count = jail.ignore_cache.entry(entry.ip.clone()).or_insert(0);
                *attempt_count += 1;

                if *attempt_count >= jail.max_retry {
                    if !jail.banned_ips.contains_key(&entry.ip) {
                        let _ = self.ban_ip(jail_name, entry.ip.clone(), "Too many failed attempts".to_string());
                        new_bans += 1;
                    }
                }
            }

            Ok(new_bans)
        } else {
            Err("Jail not found".to_string())
        }
    }

    /// Start a jail
    pub fn start_jail(&mut self, jail_name: &str) -> Result<(), String> {
        if let Some(jail) = self.jails.get_mut(jail_name) {
            jail.state = JailState::Active;
            jail.enabled = true;
            Ok(())
        } else {
            Err("Jail not found".to_string())
        }
        }

    /// Stop a jail
    pub fn stop_jail(&mut self, jail_name: &str) -> Result<(), String> {
        if let Some(jail) = self.jails.get_mut(jail_name) {
            jail.state = JailState::Stopped;
            Ok(())
        } else {
            Err("Jail not found".to_string())
        }
    }

    /// Get jail status
    pub fn get_jail_status(&self, jail_name: &str) -> Option<&Jail> {
        self.jails.get(jail_name)
    }

    /// List all jails
    pub fn list_jails(&self) -> Vec<&Jail> {
        self.jails.values().collect()
    }

    /// Get banned IPs for a jail
    pub fn get_banned_ips(&self, jail_name: &str) -> Vec<&BanInfo> {
        if let Some(jail) = self.jails.get(jail_name) {
            jail.banned_ips.values().collect()
        } else {
            vec![]
        }
    }

    /// Get all banned IPs globally
    pub fn get_all_banned_ips(&self) -> Vec<&BanInfo> {
        self.global_bans.values().collect()
    }

    /// Clear expired bans
    pub fn clear_expired_bans(&mut self) -> u32 {
        let mut cleared = 0;
        let now = "now".to_string();  // Simplified time check

        for jail in self.jails.values_mut() {
            let mut to_remove = Vec::new();
            for (ip, ban_info) in &jail.banned_ips {
                // Simplified expiration check
                if ban_info.ban_duration > 0 {
                    to_remove.push(ip.clone());
                }
            }

            for ip in to_remove {
                jail.banned_ips.remove(&ip);
                self.global_bans.remove(&ip);
                cleared += 1;
            }
        }

        cleared
    }

    /// Get statistics
    pub fn get_statistics(&self) -> HashMap<String, u32> {
        let mut stats = HashMap::new();
        stats.insert("total_jails".to_string(), self.jails.len() as u32);
        stats.insert("active_jails".to_string(), self.jails.values().filter(|j| j.state == JailState::Active).count() as u32);
        stats.insert("total_bans".to_string(), self.global_bans.len() as u32);
        
        let total_jail_bans: u32 = self.jails.values().map(|j| j.total_bans).sum();
        stats.insert("total_jail_bans".to_string(), total_jail_bans);
        stats.insert("whitelist_entries".to_string(), self.whitelist.len() as u32);
        stats.insert("blacklist_entries".to_string(), self.blacklist.len() as u32);
        
        stats
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut fail2ban = Fail2BanManager::new();
    
    println!("Sigma Fail2Ban v0.1 - Intrusion Prevention System");
    
    loop {
        println!("\n--- Fail2Ban Commands ---");
        println!("jails              - List all jails");
        println!("jail <name>        - Get jail status");
        println!("create <name> <log> <retry> <find> <ban> - Create jail");
        println!("start <name>       - Start jail");
        println!("stop <name>        - Stop jail");
        println!("ban <jail> <ip>    - Ban IP");
        println!("unban <jail> <ip>   - Unban IP");
        println!("banned <jail>      - List banned IPs for jail");
        println!("banned_all         - List all banned IPs");
        println!("whitelist <ip>     - Add to whitelist");
        println!("blacklist <ip>     - Add to blacklist");
        println!("process <jail>    - Process logs for jail");
        println!("clear_expired      - Clear expired bans");
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
            "jails" => {
                println!("--- Jails ---");
                for jail in fail2ban.list_jails() {
                    println!("{} - {:?} - {} banned - {} total bans", 
                        jail.name, jail.state, jail.banned_ips.len(), jail.total_bans);
                }
            }
            "jail" => {
                if let Some(name) = parts.get(1) {
                    if let Some(jail) = fail2ban.get_jail_status(name) {
                        println!("--- Jail ---");
                        println!("Name: {}", jail.name);
                        println!("State: {:?}", jail.state);
                        println!("Enabled: {}", jail.enabled);
                        println!("Max Retry: {}", jail.max_retry);
                        println!("Find Time: {}s", jail.find_time);
                        println!("Ban Time: {}s", jail.ban_time);
                        println!("Banned IPs: {}", jail.banned_ips.len());
                        println!("Total Bans: {}", jail.total_bans);
                    }
                }
            }
            "create" => {
                if parts.len() >= 6 {
                    let name = parts[1].to_string();
                    let log_path = parts[2].to_string();
                    let max_retry = parts[3].parse::<u32>().unwrap_or(5);
                    let find_time = parts[4].parse::<u64>().unwrap_or(600);
                    let ban_time = parts[5].parse::<u64>().unwrap_or(3600);
                    match fail2ban.create_jail(name, log_path, max_retry, find_time, ban_time) {
                        Ok(_) => println!("Jail created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "start" => {
                if let Some(name) = parts.get(1) {
                    match fail2ban.start_jail(name) {
                        Ok(_) => println!("Jail started"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "stop" => {
                if let Some(name) = parts.get(1) {
                    match fail2ban.stop_jail(name) {
                        Ok(_) => println!("Jail stopped"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "ban" => {
                if parts.len() >= 3 {
                    let jail = parts[1];
                    let ip = parts[2].to_string();
                    match fail2ban.ban_ip(jail, ip, "Manual ban".to_string()) {
                        Ok(_) => println!("IP banned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "unban" => {
                if parts.len() >= 3 {
                    let jail = parts[1];
                    let ip = parts[2];
                    match fail2ban.unban_ip(jail, ip) {
                        Ok(_) => println!("IP unbanned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "banned" => {
                if let Some(jail) = parts.get(1) {
                    println!("--- Banned IPs for {} ---", jail);
                    for ban in fail2ban.get_banned_ips(jail) {
                        println!("{} - {} - {}", ban.ip, ban.ban_time, ban.reason);
                    }
                }
            }
            "banned_all" => {
                println!("--- All Banned IPs ---");
                for ban in fail2ban.get_all_banned_ips() {
                    println!("{} - {} - {}", ban.ip, ban.jail, ban.ban_time);
                }
            }
            "whitelist" => {
                if let Some(ip) = parts.get(1) {
                    fail2ban.add_whitelist(ip.to_string());
                    println!("Added to whitelist");
                }
            }
            "blacklist" => {
                if let Some(ip) = parts.get(1) {
                    fail2ban.add_blacklist(ip.to_string());
                    println!("Added to blacklist");
                }
            }
            "process" => {
                if let Some(jail) = parts.get(1) {
                    match fail2ban.process_logs(jail) {
                        Ok(new_bans) => println!("Processed logs, {} new bans", new_bans),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "clear_expired" => {
                let cleared = fail2ban.clear_expired_bans();
                println!("Cleared {} expired bans", cleared);
            }
            "stats" => {
                println!("--- Statistics ---");
                for (key, value) in fail2ban.get_statistics() {
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
