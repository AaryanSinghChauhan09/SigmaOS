// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/security/sigma_audit.rs — Sigma Audit Trail Visualizer
//
// Implements log and monitoring visualization tools to teach students
// how system logging, audit trails, and security monitoring work.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Log Types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogCategory {
    System,
    Security,
    Network,
    Application,
    Authentication,
    Filesystem,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub category: LogCategory,
    pub source: String,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub id: String,
    pub event_type: String,
    pub user: String,
    pub action: String,
    pub resource: String,
    pub timestamp: String,
    pub success: bool,
    pub ip_address: Option<String>,
}

// ─── Audit Trail Visualizer ───────────────────────────────────────────────────

pub struct AuditVisualizer {
    pub logs: Vec<LogEntry>,
    pub audit_events: Vec<AuditEvent>,
    pub filters: LogFilters,
}

#[derive(Debug, Clone)]
pub struct LogFilters {
    pub level_filter: Option<LogLevel>,
    pub category_filter: Option<LogCategory>,
    pub source_filter: Option<String>,
    pub time_range: Option<(String, String)>,
}

impl AuditVisualizer {
    pub fn new() -> Self {
        let mut visualizer = AuditVisualizer {
            logs: Vec::new(),
            audit_events: Vec::new(),
            filters: LogFilters {
                level_filter: None,
                category_filter: None,
                source_filter: None,
                time_range: None,
            },
        };
        
        visualizer.generate_sample_logs();
        visualizer.generate_sample_audit_events();
        visualizer
    }

    /// Generate sample system logs
    fn generate_sample_logs(&mut self) {
        self.logs.push(LogEntry {
            timestamp: "2024-01-15 08:00:00".to_string(),
            level: LogLevel::Info,
            category: LogCategory::System,
            source: "kernel".to_string(),
            message: "System boot completed successfully".to_string(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("boot_time".to_string(), "2.5s".to_string());
                map
            },
        });

        self.logs.push(LogEntry {
            timestamp: "2024-01-15 08:05:23".to_string(),
            level: LogLevel::Warning,
            category: LogCategory::Security,
            source: "authd".to_string(),
            message: "Failed login attempt from unknown IP".to_string(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("ip".to_string(), "192.168.1.100".to_string());
                map.insert("attempts".to_string(), "3".to_string());
                map
            },
        });

        self.logs.push(LogEntry {
            timestamp: "2024-01-15 08:10:45".to_string(),
            level: LogLevel::Info,
            category: LogCategory::Network,
            source: "networkd".to_string(),
            message: "Network interface eth0 up".to_string(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("ip".to_string(), "192.168.1.10".to_string());
                map.insert("mac".to_string(), "00:11:22:33:44:55".to_string());
                map
            },
        });

        self.logs.push(LogEntry {
            timestamp: "2024-01-15 08:15:00".to_string(),
            level: LogLevel::Error,
            category: LogCategory::Filesystem,
            source: "fsck".to_string(),
            message: "Filesystem corruption detected on /dev/sda2".to_string(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("device".to_string(), "/dev/sda2".to_string());
                map.insert("errors".to_string(), "5".to_string());
                map
            },
        });

        self.logs.push(LogEntry {
            timestamp: "2024-01-15 08:20:30".to_string(),
            level: LogLevel::Critical,
            category: LogCategory::Security,
            source: "firewalld".to_string(),
            message: "Potential intrusion detected".to_string(),
            metadata: {
                let mut map = HashMap::new();
                map.insert("signature".to_string(), "SQL_INJECTION".to_string());
                map.insert("blocked".to_string(), "true".to_string());
                map
            },
        });
    }

    /// Generate sample audit events
    fn generate_sample_audit_events(&mut self) {
        self.audit_events.push(AuditEvent {
            id: "evt_001".to_string(),
            event_type: "LOGIN".to_string(),
            user: "admin".to_string(),
            action: "login".to_string(),
            resource: "system".to_string(),
            timestamp: "2024-01-15 08:01:00".to_string(),
            success: true,
            ip_address: Some("192.168.1.5".to_string()),
        });

        self.audit_events.push(AuditEvent {
            id: "evt_002".to_string(),
            event_type: "FILE_ACCESS".to_string(),
            user: "student1".to_string(),
            action: "read".to_string(),
            resource: "/etc/passwd".to_string(),
            timestamp: "2024-01-15 08:10:15".to_string(),
            success: false,
            ip_address: Some("192.168.1.20".to_string()),
        });

        self.audit_events.push(AuditEvent {
            id: "evt_003".to_string(),
            event_type: "CONFIG_CHANGE".to_string(),
            user: "admin".to_string(),
            action: "modify".to_string(),
            resource: "/etc/firewall/rules.conf".to_string(),
            timestamp: "2024-01-15 08:15:30".to_string(),
            success: true,
            ip_address: Some("192.168.1.5".to_string()),
        });

        self.audit_events.push(AuditEvent {
            id: "evt_004".to_string(),
            event_type: "PRIVILEGE_ESCALATION".to_string(),
            user: "student2".to_string(),
            action: "sudo".to_string(),
            resource: "/bin/bash".to_string(),
            timestamp: "2024-01-15 08:20:45".to_string(),
            success: false,
            ip_address: Some("192.168.1.21".to_string()),
        });
    }

    /// Add custom log entry
    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
    }

    /// Add audit event
    pub fn add_audit_event(&mut self, event: AuditEvent) {
        self.audit_events.push(event);
    }

    /// Get filtered logs
    pub fn get_filtered_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter()
            .filter(|log| {
                if let Some(level) = self.filters.level_filter {
                    if log.level != level {
                        return false;
                    }
                }
                if let Some(category) = self.filters.category_filter {
                    if log.category != category {
                        return false;
                    }
                }
                if let Some(ref source) = self.filters.source_filter {
                    if !log.source.contains(source) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    /// Get audit events by user
    pub fn get_events_by_user(&self, user: &str) -> Vec<&AuditEvent> {
        self.audit_events.iter()
            .filter(|event| event.user == user)
            .collect()
    }

    /// Get audit events by type
    pub fn get_events_by_type(&self, event_type: &str) -> Vec<&AuditEvent> {
        self.audit_events.iter()
            .filter(|event| event.event_type == event_type)
            .collect()
    }

    /// Get failed events (security incidents)
    pub fn get_failed_events(&self) -> Vec<&AuditEvent> {
        self.audit_events.iter()
            .filter(|event| !event.success)
            .collect()
    }

    /// Set level filter
    pub fn set_level_filter(&mut self, level: LogLevel) {
        self.filters.level_filter = Some(level);
    }

    /// Set category filter
    pub fn set_category_filter(&mut self, category: LogCategory) {
        self.filters.category_filter = Some(category);
    }

    /// Set source filter
    pub fn set_source_filter(&mut self, source: String) {
        self.filters.source_filter = Some(source);
    }

    /// Clear all filters
    pub fn clear_filters(&mut self) {
        self.filters = LogFilters {
            level_filter: None,
            category_filter: None,
            source_filter: None,
            time_range: None,
        };
    }

    /// Generate statistics
    pub fn get_statistics(&self) -> HashMap<String, usize> {
        let mut stats = HashMap::new();
        
        stats.insert("total_logs".to_string(), self.logs.len());
        stats.insert("total_audit_events".to_string(), self.audit_events.len());
        stats.insert("failed_events".to_string(), self.get_failed_events().len());
        
        let critical_logs = self.logs.iter()
            .filter(|l| l.level == LogLevel::Critical)
            .count();
        stats.insert("critical_logs".to_string(), critical_logs);
        
        let security_events = self.audit_events.iter()
            .filter(|e| e.event_type == "PRIVILEGE_ESCALATION" || e.event_type == "LOGIN")
            .count();
        stats.insert("security_events".to_string(), security_events);
        
        stats
    }

    /// Render logs as ASCII table
    pub fn render_logs_table(&self) -> String {
        let logs = self.get_filtered_logs();
        let mut output = String::new();
        
        output.push_str("┌─────────────────────┬─────────┬──────────────┬────────────────────────────────────┐\n");
        output.push_str("│ Timestamp          │ Level   │ Category    │ Message                            │\n");
        output.push_str("├─────────────────────┼─────────┼──────────────┼────────────────────────────────────┤\n");
        
        for log in logs {
            let level_str = match log.level {
                LogLevel::Debug => "DEBUG  ",
                LogLevel::Info => "INFO   ",
                LogLevel::Warning => "WARN   ",
                LogLevel::Error => "ERROR  ",
                LogLevel::Critical => "CRITICAL",
            };
            
            let category_str = match log.category {
                LogCategory::System => "System     ",
                LogCategory::Security => "Security   ",
                LogCategory::Network => "Network    ",
                LogCategory::Application => "Application",
                LogCategory::Authentication => "Auth       ",
                LogCategory::Filesystem => "Filesystem ",
            };
            
            output.push_str(&format!("│ {:<19} │ {:<7} │ {:<12} │ {:<34} │\n", 
                log.timestamp, level_str, category_str, log.message));
        }
        
        output.push_str("└─────────────────────┴─────────┴──────────────┴────────────────────────────────────┘\n");
        
        output
    }

    /// Render audit timeline
    pub fn render_audit_timeline(&self) -> String {
        let mut output = String::new();
        
        output.push_str("Audit Event Timeline:\n");
        output.push_str("─────────────────────────────────────────────────────────────\n");
        
        for event in &self.audit_events {
            let status = if event.success { "✓" } else { "✗" };
            output.push_str(&format!("{} [{}] {} - {} -> {} ({})\n", 
                status, event.timestamp, event.user, event.action, event.resource, event.event_type));
        }
        
        output
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut visualizer = AuditVisualizer::new();
    
    println!("Sigma Audit Trail Visualizer v0.1 - Log & Monitoring");
    
    loop {
        println!("\n--- Statistics ---");
        for (key, value) in visualizer.get_statistics() {
            println!("{}: {}", key, value);
        }
        
        println!("\nCommands: logs, audit, filter <level|category|source>, clear, stats, failed <user>, quit");
        println!("Levels: debug, info, warning, error, critical");
        println!("Categories: system, security, network, application, auth, filesystem");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "logs" => {
                println!("\n{}", visualizer.render_logs_table());
            }
            "audit" => {
                println!("\n{}", visualizer.render_audit_timeline());
            }
            "filter" => {
                if parts.len() >= 3 {
                    let filter_type = parts[1];
                    let value = parts[2];
                    
                    match filter_type {
                        "level" => {
                            let level = match value {
                                "debug" => LogLevel::Debug,
                                "info" => LogLevel::Info,
                                "warning" => LogLevel::Warning,
                                "error" => LogLevel::Error,
                                "critical" => LogLevel::Critical,
                                _ => {
                                    println!("Unknown level");
                                    continue;
                                }
                            };
                            visualizer.set_level_filter(level);
                            println!("Level filter set");
                        }
                        "category" => {
                            let category = match value {
                                "system" => LogCategory::System,
                                "security" => LogCategory::Security,
                                "network" => LogCategory::Network,
                                "application" => LogCategory::Application,
                                "auth" => LogCategory::Authentication,
                                "filesystem" => LogCategory::Filesystem,
                                _ => {
                                    println!("Unknown category");
                                    continue;
                                }
                            };
                            visualizer.set_category_filter(category);
                            println!("Category filter set");
                        }
                        "source" => {
                            visualizer.set_source_filter(value.to_string());
                            println!("Source filter set");
                        }
                        _ => {
                            println!("Unknown filter type");
                        }
                    }
                }
            }
            "clear" => {
                visualizer.clear_filters();
                println!("Filters cleared");
            }
            "stats" => {
                println!("\n--- Statistics ---");
                for (key, value) in visualizer.get_statistics() {
                    println!("{}: {}", key, value);
                }
            }
            "failed" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Failed Events for {} ---", arg);
                    for event in visualizer.get_events_by_user(arg) {
                        if !event.success {
                            println!("{} - {} -> {} ({})", event.timestamp, event.action, event.resource, event.event_type);
                        }
                    }
                } else {
                    println!("--- All Failed Events ---");
                    for event in visualizer.get_failed_events() {
                        println!("{} - {} -> {} ({})", event.timestamp, event.action, event.resource, event.event_type);
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
