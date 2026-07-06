// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/it/sigma_suricata.rs — Sigma IDS/IPS (Suricata)
//
// Implements Suricata-style intrusion detection/prevention with
// signature management, alert generation, and packet analysis.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── IDS/IPS Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlertCategory {
    Malware,
    Exploit,
    Scan,
    Anomaly,
    Policy,
    DDoS,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub id: String,
    pub sid: u32,
    pub rev: u32,
    pub action: String,
    pub msg: String,
    pub category: AlertCategory,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub content: Option<String>,
    pub pcre: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub id: String,
    pub timestamp: String,
    pub signature_id: String,
    pub severity: AlertSeverity,
    pub category: AlertCategory,
    pub source_ip: String,
    pub source_port: u16,
    pub destination_ip: String,
    pub destination_port: u16,
    pub protocol: String,
    pub message: String,
    pub packet_len: u32,
}

#[derive(Debug, Clone)]
pub struct Statistics {
    pub total_alerts: u64,
    pub alerts_by_severity: HashMap<AlertSeverity, u64>,
    pub alerts_by_category: HashMap<AlertCategory, u64>,
    pub packets_analyzed: u64,
    pub packets_dropped: u64,
}

// ─── IDS/IPS Manager ────────────────────────────────────────────────────

pub struct IDSManager {
    pub signatures: HashMap<String, Signature>,
    pub alerts: Vec<Alert>,
    pub stats: Statistics,
    pub active: bool,
}

impl IDSManager {
    pub fn new() -> Self {
        let mut manager = IDSManager {
            signatures: HashMap::new(),
            alerts: Vec::new(),
            stats: Statistics {
                total_alerts: 0,
                alerts_by_severity: HashMap::new(),
                alerts_by_category: HashMap::new(),
                packets_analyzed: 0,
                packets_dropped: 0,
            },
            active: true,
        };
        
        manager.init_sample_signatures();
        manager
    }

    /// Initialize sample signatures
    fn init_sample_signatures(&mut self) {
        // SQL Injection signature
        self.signatures.insert("sig_001".to_string(), Signature {
            id: "sig_001".to_string(),
            sid: 1000001,
            rev: 1,
            action: "alert".to_string(),
            msg: "ET SQL Injection Attempt".to_string(),
            category: AlertCategory::Exploit,
            protocol: "tcp".to_string(),
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: Some(80),
            content: Some("UNION SELECT".to_string()),
            pcre: Some("/UNION.*SELECT/i".to_string()),
        });

        // Port scan signature
        self.signatures.insert("sig_002".to_string(), Signature {
            id: "sig_002".to_string(),
            sid: 1000002,
            rev: 1,
            action: "alert".to_string(),
            msg: "ET Port Scan Detected".to_string(),
            category: AlertCategory::Scan,
            protocol: "tcp".to_string(),
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: None,
            content: None,
            pcre: None,
        });

        // Malware signature
        self.signatures.insert("sig_003".to_string(), Signature {
            id: "sig_003".to_string(),
            sid: 1000003,
            rev: 1,
            action: "drop".to_string(),
            msg: "ET Malware C2 Communication".to_string(),
            category: AlertCategory::Malware,
            protocol: "tcp".to_string(),
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: Some(4444),
            content: Some("malware_payload".to_string()),
            pcre: Some("/payload.*exe/i".to_string()),
        });

        // XSS signature
        self.signatures.insert("sig_004".to_string(), Signature {
            id: "sig_004".to_string(),
            sid: 1000004,
            rev: 1,
            action: "alert".to_string(),
            msg: "ET XSS Attack Attempt".to_string(),
            category: AlertCategory::Exploit,
            protocol: "tcp".to_string(),
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: Some(443),
            content: Some("<script>".to_string()),
            pcre: Some("/<script.*>/i".to_string()),
        });
    }

    /// Add signature
    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.insert(signature.id.clone(), signature);
    }

    /// Generate alert
    pub fn generate_alert(&mut self, signature_id: &str, source_ip: String, source_port: u16, dest_ip: String, dest_port: u16, packet_len: u32) {
        if let Some(signature) = self.signatures.get(signature_id) {
            let severity = match signature.category {
                AlertCategory::Malware => AlertSeverity::Critical,
                AlertCategory::Exploit => AlertSeverity::High,
                AlertCategory::DDoS => AlertSeverity::High,
                AlertCategory::Scan => AlertSeverity::Medium,
                AlertCategory::Anomaly => AlertSeverity::Medium,
                AlertCategory::Policy => AlertSeverity::Low,
            };
            
            let alert = Alert {
                id: format!("alert_{}", self.alerts.len()),
                timestamp: "now".to_string(),
                signature_id: signature.id.clone(),
                severity,
                category: signature.category,
                source_ip,
                source_port,
                destination_ip: dest_ip,
                destination_port: dest_port,
                protocol: signature.protocol.clone(),
                message: signature.msg.clone(),
                packet_len,
            };
            
            self.alerts.push(alert.clone());
            self.stats.total_alerts += 1;
            *self.stats.alerts_by_severity.entry(severity).or_insert(0) += 1;
            *self.stats.alerts_by_category.entry(signature.category).or_insert(0) += 1;
            
            // Drop packet if action is drop
            if signature.action == "drop" {
                self.stats.packets_dropped += 1;
            }
        }
    }

    /// Analyze packet (simulated)
    pub fn analyze_packet(&mut self, source_ip: String, source_port: u16, dest_ip: String, dest_port: u16, protocol: String, payload: String) {
        self.stats.packets_analyzed += 1;
        
        // Check against signatures
        for signature in self.signatures.values() {
            if signature.protocol == protocol {
                let port_match = signature.destination_port.map(|p| p == dest_port).unwrap_or(true);
                
                if port_match {
                    if let Some(content) = &signature.content {
                        if payload.contains(content) {
                            self.generate_alert(&signature.id, source_ip.clone(), source_port, dest_ip.clone(), dest_port, payload.len() as u32);
                        }
                    } else {
                        // Pattern-based detection
                        self.generate_alert(&signature.id, source_ip.clone(), source_port, dest_ip.clone(), dest_port, 0);
                    }
                }
            }
        }
    }

    /// Get signature by ID
    pub fn get_signature(&self, id: &str) -> Option<&Signature> {
        self.signatures.get(id)
    }

    /// Get all signatures
    pub fn get_all_signatures(&self) -> Vec<&Signature> {
        self.signatures.values().collect()
    }

    /// Get recent alerts
    pub fn get_recent_alerts(&self, count: usize) -> Vec<&Alert> {
        self.alerts.iter().rev().take(count).collect()
    }

    /// Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<&Alert> {
        self.alerts.iter().filter(|a| a.severity == severity).collect()
    }

    /// Toggle active state
    pub fn toggle_active(&mut self) {
        self.active = !self.active;
    }

    /// Get severity name
    pub fn get_severity_name(&self, severity: AlertSeverity) -> &str {
        match severity {
            AlertSeverity::Low => "Low",
            AlertSeverity::Medium => "Medium",
            AlertSeverity::High => "High",
            AlertSeverity::Critical => "Critical",
        }
    }

    /// Get category name
    pub fn get_category_name(&self, category: AlertCategory) -> &str {
        match category {
            AlertCategory::Malware => "Malware",
            AlertCategory::Exploit => "Exploit",
            AlertCategory::Scan => "Scan",
            AlertCategory::Anomaly => "Anomaly",
            AlertCategory::Policy => "Policy",
            AlertCategory::DDoS => "DDoS",
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = IDSManager::new();
    
    println!("Sigma IDS/IPS v0.1 - Suricata Style");
    
    loop {
        println!("\n--- IDS Status ---");
        println!("Active: {}", manager.active);
        println!("Signatures: {}", manager.signatures.len());
        println!("Total Alerts: {}", manager.stats.total_alerts);
        println!("Packets Analyzed: {}", manager.stats.packets_analyzed);
        println!("Packets Dropped: {}", manager.stats.packets_dropped);
        
        println!("\nCommands: analyze <src_ip> <src_port> <dst_ip> <dst_port> <proto> <payload>, signatures, alerts <count>, alerts_severity <severity>, toggle, stats, quit");
        println!("Severities: low, medium, high, critical");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "analyze" => {
                if parts.len() >= 7 {
                    let src_ip = parts[1].to_string();
                    if let (Ok(src_port), dst_ip, Ok(dst_port)) = (parts[2].parse::<u16>(), parts[3].to_string(), parts[4].parse::<u16>()) {
                        let protocol = parts[5].to_string();
                        let payload = parts[6..].join(" ");
                        manager.analyze_packet(src_ip, src_port, dst_ip, dst_port, protocol, payload);
                        println!("Packet analyzed");
                    }
                }
            }
            "signatures" => {
                println!("--- Signatures ---");
                for sig in manager.get_all_signatures() {
                    println!("{} - {} (SID: {}, Action: {})", sig.id, sig.msg, sig.sid, sig.action);
                    println!("  Category: {}, Protocol: {}", manager.get_category_name(sig.category), sig.protocol);
                }
            }
            "alerts" => {
                let count = parts.get(1).and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
                println!("--- Recent Alerts ---");
                for alert in manager.get_recent_alerts(count) {
                    println!("{} - [{}] {}", alert.id, manager.get_severity_name(alert.severity), alert.message);
                    println!("  {}:{} -> {}:{} ({})", alert.source_ip, alert.source_port, alert.destination_ip, alert.destination_port, alert.protocol);
                    println!("  Timestamp: {}", alert.timestamp);
                }
            }
            "alerts_severity" => {
                if let Some(arg) = parts.get(1) {
                    let severity = match *arg {
                        "low" => AlertSeverity::Low,
                        "medium" => AlertSeverity::Medium,
                        "high" => AlertSeverity::High,
                        "critical" => AlertSeverity::Critical,
                        _ => {
                            println!("Unknown severity");
                            continue;
                        }
                    };
                    println!("--- {} Severity Alerts ---", manager.get_severity_name(severity));
                    for alert in manager.get_alerts_by_severity(severity) {
                        println!("{} - {}", alert.id, alert.message);
                        println!("  {}:{} -> {}:{}", alert.source_ip, alert.source_port, alert.destination_ip, alert.destination_port);
                    }
                }
            }
            "toggle" => {
                manager.toggle_active();
                println!("IDS {}", if manager.active { "activated" } else { "deactivated" });
            }
            "stats" => {
                println!("--- Statistics ---");
                println!("Total Alerts: {}", manager.stats.total_alerts);
                println!("\nBy Severity:");
                for (severity, count) in &manager.stats.alerts_by_severity {
                    println!("  {}: {}", manager.get_severity_name(*severity), count);
                }
                println!("\nBy Category:");
                for (category, count) in &manager.stats.alerts_by_category {
                    println!("  {}: {}", manager.get_category_name(*category), count);
                }
                println!("\nPackets Analyzed: {}", manager.stats.packets_analyzed);
                println!("Packets Dropped: {}", manager.stats.packets_dropped);
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
