// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/observability/sigma_logging.rs — Sigma Logging (ELK Stack)
//
// Implements ELK Stack-style logging with log collection, parsing,
// indexing, and search capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Logging Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub service: String,
    pub host: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct LogIndex {
    pub name: String,
    pub pattern: String,
    pub fields: Vec<String>,
    pub retention_days: u32,
}

#[derive(Debug, Clone)]
pub struct LogQuery {
    pub query_string: String,
    pub time_range_start: u64,
    pub time_range_end: u64,
    pub level_filter: Option<LogLevel>,
    pub source_filter: Option<String>,
}

// ─── Logging Manager ────────────────────────────────────────────────────

pub struct LoggingManager {
    pub logs: Vec<LogEntry>,
    pub indices: HashMap<String, LogIndex>,
    pub pipelines: Vec<String>,
    pub ingestion_enabled: bool,
}

impl LoggingManager {
    pub fn new() -> Self {
        let mut manager = LoggingManager {
            logs: Vec::new(),
            indices: HashMap::new(),
            pipelines: Vec::new(),
            ingestion_enabled: true,
        };
        
        manager.init_sample_logs();
        manager.init_sample_indices();
        manager
    }

    /// Initialize sample logs
    fn init_sample_logs(&mut self) {
        let mut fields1 = HashMap::new();
        fields1.insert("user_id".to_string(), "12345".to_string());
        fields1.insert("ip".to_string(), "192.168.1.100".to_string());
        
        self.logs.push(LogEntry {
            id: "log_001".to_string(),
            timestamp: 1704067200,
            level: LogLevel::Info,
            message: "User login successful".to_string(),
            source: "auth_service".to_string(),
            service: "sigma_auth".to_string(),
            host: "server-01".to_string(),
            fields: fields1,
        });

        let mut fields2 = HashMap::new();
        fields2.insert("error_code".to_string(), "500".to_string());
        fields2.insert("endpoint".to_string(), "/api/data".to_string());
        
        self.logs.push(LogEntry {
            id: "log_002".to_string(),
            timestamp: 1704067260,
            level: LogLevel::Error,
            message: "Database connection failed".to_string(),
            source: "api_service".to_string(),
            service: "sigma_api".to_string(),
            host: "server-01".to_string(),
            fields: fields2,
        });

        let mut fields3 = HashMap::new();
        fields3.insert("duration_ms".to_string(), "234".to_string());
        
        self.logs.push(LogEntry {
            id: "log_003".to_string(),
            timestamp: 1704067320,
            level: LogLevel::Debug,
            message: "Query executed successfully".to_string(),
            source: "database".to_string(),
            service: "sigma_db".to_string(),
            host: "server-02".to_string(),
            fields: fields3,
        });
    }

    /// Initialize sample indices
    fn init_sample_indices(&mut self) {
        self.indices.insert("sigma_logs".to_string(), LogIndex {
            name: "sigma_logs".to_string(),
            pattern: "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}".to_string(),
            fields: vec!["timestamp".to_string(), "level".to_string(), "message".to_string()],
            retention_days: 30,
        });

        self.indices.insert("auth_logs".to_string(), LogIndex {
            name: "auth_logs".to_string(),
            pattern: "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}".to_string(),
            fields: vec!["timestamp".to_string(), "level".to_string(), "message".to_string()],
            retention_days: 90,
        });
    }

    /// Add log entry
    pub fn add_log(&mut self, log: LogEntry) {
        self.logs.push(log);
    }

    /// Ingest log from string
    pub fn ingest_log(&mut self, message: String, level: LogLevel, source: String, service: String) {
        let log = LogEntry {
            id: format!("log_{}", self.logs.len()),
            timestamp: current_timestamp(),
            level,
            message,
            source,
            service,
            host: "localhost".to_string(),
            fields: HashMap::new(),
        };
        self.logs.push(log);
    }

    /// Search logs
    pub fn search(&self, query: LogQuery) -> Vec<&LogEntry> {
        self.logs.iter()
            .filter(|log| {
                // Time range filter
                if log.timestamp < query.time_range_start || log.timestamp > query.time_range_end {
                    return false;
                }
                
                // Level filter
                if let Some(level) = query.level_filter {
                    if log.level != level {
                        return false;
                    }
                }
                
                // Source filter
                if let Some(source) = &query.source_filter {
                    if log.source != *source {
                        return false;
                    }
                }
                
                // Query string filter
                if !query.query_string.is_empty() {
                    if !log.message.to_lowercase().contains(&query.query_string.to_lowercase()) {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }

    /// Get logs by level
    pub fn get_logs_by_level(&self, level: LogLevel) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.level == level).collect()
    }

    /// Get logs by source
    pub fn get_logs_by_source(&self, source: &str) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.source == source).collect()
    }

    /// Get logs by service
    pub fn get_logs_by_service(&self, service: &str) -> Vec<&LogEntry> {
        self.logs.iter().filter(|l| l.service == service).collect()
    }

    /// Add index
    pub fn add_index(&mut self, index: LogIndex) {
        self.indices.insert(index.name.clone(), index);
    }

    /// Get index by name
    pub fn get_index(&self, name: &str) -> Option<&LogIndex> {
        self.indices.get(name)
    }

    /// Get all indices
    pub fn get_all_indices(&self) -> Vec<&LogIndex> {
        self.indices.values().collect()
    }

    /// Add pipeline
    pub fn add_pipeline(&mut self, pipeline: String) {
        self.pipelines.push(pipeline);
    }

    /// Get all pipelines
    pub fn get_all_pipelines(&self) -> Vec<&String> {
        self.pipelines.iter().collect()
    }

    /// Toggle ingestion
    pub fn toggle_ingestion(&mut self) {
        self.ingestion_enabled = !self.ingestion_enabled;
    }

    /// Get level name
    pub fn get_level_name(&self, level: LogLevel) -> &str {
        match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = LoggingManager::new();
    
    println!("Sigma Logging v0.1 - ELK Stack Style");
    
    loop {
        println!("\n--- Logging Status ---");
        println!("Ingestion: {}", manager.ingestion_enabled);
        println!("Logs: {}", manager.logs.len());
        println!("Indices: {}", manager.indices.len());
        println!("Pipelines: {}", manager.pipelines.len());
        
        println!("\nCommands: ingest <message> <level> <source> <service>, search <query>, logs_level <level>, logs_source <source>, logs_service <service>, indices, pipelines, toggle, quit");
        println!("Levels: debug, info, warning, error, critical");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "ingest" => {
                if parts.len() >= 4 {
                    let message = parts[1].to_string();
                    let level = match parts[2] {
                        "debug" => LogLevel::Debug,
                        "info" => LogLevel::Info,
                        "warning" => LogLevel::Warning,
                        "error" => LogLevel::Error,
                        "critical" => LogLevel::Critical,
                        _ => LogLevel::Info,
                    };
                    let source = parts[3].to_string();
                    let service = parts.get(4).map(|s| s.to_string()).unwrap_or("default".to_string());
                    manager.ingest_log(message, level, source, service);
                    println!("Log ingested");
                }
            }
            "search" => {
                if let Some(arg) = parts.get(1) {
                    let query = LogQuery {
                        query_string: arg.to_string(),
                        time_range_start: 0,
                        time_range_end: current_timestamp(),
                        level_filter: None,
                        source_filter: None,
                    };
                    let results = manager.search(query);
                    println!("--- Search Results ---");
                    for log in results {
                        println!("[{}] {} - {} ({})", manager.get_level_name(log.level), log.timestamp, log.message, log.source);
                    }
                }
            }
            "logs_level" => {
                if let Some(arg) = parts.get(1) {
                    let level = match *arg {
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
                    println!("--- Logs: {} ---", manager.get_level_name(level));
                    for log in manager.get_logs_by_level(level) {
                        println!("[{}] {} - {}", log.timestamp, log.message, log.source);
                    }
                }
            }
            "logs_source" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Logs from {} ---", arg);
                    for log in manager.get_logs_by_source(arg) {
                        println!("[{}] {} - {}", manager.get_level_name(log.level), log.message, log.service);
                    }
                }
            }
            "logs_service" => {
                if let Some(arg) = parts.get(1) {
                    println!("--- Logs from {} ---", arg);
                    for log in manager.get_logs_by_service(arg) {
                        println!("[{}] {} - {}", manager.get_level_name(log.level), log.message, log.source);
                    }
                }
            }
            "indices" => {
                println!("--- Log Indices ---");
                for index in manager.get_all_indices() {
                    println!("{} - Retention: {} days", index.name, index.retention_days);
                    println!("  Pattern: {}", index.pattern);
                }
            }
            "pipelines" => {
                println!("--- Ingest Pipelines ---");
                for pipeline in manager.get_all_pipelines() {
                    println!("{}", pipeline);
                }
            }
            "toggle" => {
                manager.toggle_ingestion();
                println!("Ingestion {}", if manager.ingestion_enabled { "enabled" } else { "disabled" });
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
