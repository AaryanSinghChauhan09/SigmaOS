// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/system/sigma_perf.rs — Sigma Performance Profiler
//
// Implements perf/sysstat-style performance profiling with CPU,
// memory, I/O, and network metrics collection and analysis.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Performance Types ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CPUMetrics {
    pub user_percent: f64,
    pub system_percent: f64,
    pub idle_percent: f64,
    pub iowait_percent: f64,
    pub context_switches: u64,
    pub interrupts: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryMetrics {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub page_ins: u64,
    pub page_outs: u64,
}

#[derive(Debug, Clone)]
pub struct IOMetrics {
    pub reads: u64,
    pub writes: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_time_ms: u64,
    pub write_time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    pub interface: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
}

#[derive(Debug, Clone)]
pub struct ProfileSession {
    pub id: String,
    pub duration_seconds: u32,
    pub interval_ms: u32,
    pub cpu_samples: Vec<CPUMetrics>,
    pub memory_samples: Vec<MemoryMetrics>,
    pub io_samples: Vec<IOMetrics>,
    pub network_samples: Vec<NetworkMetrics>,
}

// ─── Performance Profiler ───────────────────────────────────────────────────

pub struct PerformanceProfiler {
    pub sessions: HashMap<String, ProfileSession>,
    pub current_session: Option<String>,
    pub is_profiling: bool,
}

impl PerformanceProfiler {
    pub fn new() -> Self {
        PerformanceProfiler {
            sessions: HashMap::new(),
            current_session: None,
            is_profiling: false,
        }
    }

    /// Start profiling session
    pub fn start_session(&mut self, duration: u32, interval: u32) -> String {
        let session_id = format!("session_{}", self.sessions.len());
        let session = ProfileSession {
            id: session_id.clone(),
            duration_seconds: duration,
            interval_ms: interval,
            cpu_samples: Vec::new(),
            memory_samples: Vec::new(),
            io_samples: Vec::new(),
            network_samples: Vec::new(),
        };
        
        self.sessions.insert(session_id.clone(), session);
        self.current_session = Some(session_id.clone());
        self.is_profiling = true;
        
        session_id
    }

    /// Stop profiling session
    pub fn stop_session(&mut self) {
        self.is_profiling = false;
    }

    /// Collect metrics (simulated)
    pub fn collect_metrics(&mut self) {
        if !self.is_profiling {
            return;
        }
        
        if let Some(session_id) = &self.current_session {
            if let Some(session) = self.sessions.get_mut(session_id) {
                // Simulate CPU metrics
                let cpu = CPUMetrics {
                    user_percent: 45.0 + (rand_f64() * 10.0),
                    system_percent: 15.0 + (rand_f64() * 5.0),
                    idle_percent: 30.0 + (rand_f64() * 10.0),
                    iowait_percent: 10.0 + (rand_f64() * 5.0),
                    context_switches: 10000 + (rand_u64() % 5000),
                    interrupts: 5000 + (rand_u64() % 2000),
                };
                session.cpu_samples.push(cpu);
                
                // Simulate memory metrics
                let memory = MemoryMetrics {
                    total: 16 * 1024 * 1024 * 1024,  // 16 GB
                    used: (8 * 1024 * 1024 * 1024) + (rand_u64() % (2 * 1024 * 1024 * 1024)),
                    free: (6 * 1024 * 1024 * 1024) - (rand_u64() % (1 * 1024 * 1024 * 1024)),
                    buffers: 512 * 1024 * 1024,
                    cached: (2 * 1024 * 1024 * 1024) + (rand_u64() % (512 * 1024 * 1024)),
                    swap_total: 4 * 1024 * 1024 * 1024,
                    swap_used: rand_u64() % (512 * 1024 * 1024),
                    page_ins: rand_u64() % 1000,
                    page_outs: rand_u64() % 500,
                };
                session.memory_samples.push(memory);
                
                // Simulate I/O metrics
                let io = IOMetrics {
                    reads: 1000 + (rand_u64() % 500),
                    writes: 500 + (rand_u64() % 300),
                    read_bytes: (10 * 1024 * 1024) + (rand_u64() % (5 * 1024 * 1024)),
                    write_bytes: (5 * 1024 * 1024) + (rand_u64() % (3 * 1024 * 1024)),
                    read_time_ms: 100 + (rand_u64() % 50),
                    write_time_ms: 50 + (rand_u64() % 30),
                };
                session.io_samples.push(io);
                
                // Simulate network metrics
                let network = NetworkMetrics {
                    interface: "eth0".to_string(),
                    rx_bytes: (100 * 1024 * 1024) + (rand_u64() % (50 * 1024 * 1024)),
                    tx_bytes: (50 * 1024 * 1024) + (rand_u64() % (25 * 1024 * 1024)),
                    rx_packets: 10000 + (rand_u64() % 5000),
                    tx_packets: 5000 + (rand_u64() % 2500),
                    rx_errors: rand_u64() % 10,
                    tx_errors: rand_u64() % 5,
                };
                session.network_samples.push(network);
            }
        }
    }

    /// Get session by ID
    pub fn get_session(&self, id: &str) -> Option<&ProfileSession> {
        self.sessions.get(id)
    }

    /// Calculate average CPU usage
    pub fn avg_cpu_usage(&self, session_id: &str) -> Option<f64> {
        if let Some(session) = self.get_session(session_id) {
            if session.cpu_samples.is_empty() {
                return None;
            }
            let total: f64 = session.cpu_samples.iter().map(|c| c.user_percent + c.system_percent).sum();
            Some(total / session.cpu_samples.len() as f64)
        } else {
            None
        }
    }

    /// Calculate average memory usage
    pub fn avg_memory_usage(&self, session_id: &str) -> Option<f64> {
        if let Some(session) = self.get_session(session_id) {
            if session.memory_samples.is_empty() {
                return None;
            }
            let total: f64 = session.memory_samples.iter().map(|m| m.used as f64 / m.total as f64 * 100.0).sum();
            Some(total / session.memory_samples.len() as f64)
        } else {
            None
        }
    }

    /// Get all sessions
    pub fn get_all_sessions(&self) -> Vec<&ProfileSession> {
        self.sessions.values().collect()
    }

    /// Format bytes
    fn format_bytes(&self, bytes: u64) -> String {
        if bytes >= 1024 * 1024 * 1024 {
            format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        } else if bytes >= 1024 * 1024 {
            format!("{:.2} MB", bytes as f64 / (1024.0 * 1024.0))
        } else if bytes >= 1024 {
            format!("{:.2} KB", bytes as f64 / 1024.0)
        } else {
            format!("{} B", bytes)
        }
    }
}

// Simple random number generators
fn rand_f64() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    (duration.subsec_nanos() as f64) / 1_000_000_000.0
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_nanos() as u64
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut profiler = PerformanceProfiler::new();
    
    println!("Sigma Performance Profiler v0.1 - perf/sysstat Style");
    
    loop {
        println!("\n--- Status ---");
        if profiler.is_profiling {
            println!("Profiling: ACTIVE");
            if let Some(session_id) = &profiler.current_session {
                if let Some(session) = profiler.get_session(session_id) {
                    println!("Session: {}", session.id);
                    println!("Samples: CPU={}, Memory={}, I/O={}, Network={}", 
                        session.cpu_samples.len(),
                        session.memory_samples.len(),
                        session.io_samples.len(),
                        session.network_samples.len()
                    );
                }
            }
        } else {
            println!("Profiling: INACTIVE");
        }
        
        println!("\nCommands: start <duration_sec> <interval_ms>, stop, collect, session <id>, sessions, avg_cpu <id>, avg_mem <id>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "start" => {
                if parts.len() >= 3 {
                    if let (Ok(duration), Ok(interval)) = (parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                        let session_id = profiler.start_session(duration, interval);
                        println!("Profiling session started: {}", session_id);
                    }
                }
            }
            "stop" => {
                profiler.stop_session();
                println!("Profiling stopped");
            }
            "collect" => {
                profiler.collect_metrics();
                println!("Metrics collected");
            }
            "session" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(session) = profiler.get_session(arg) {
                        println!("--- Session Details ---");
                        println!("ID: {}", session.id);
                        println!("Duration: {}s", session.duration_seconds);
                        println!("Interval: {}ms", session.interval_ms);
                        println!("CPU Samples: {}", session.cpu_samples.len());
                        println!("Memory Samples: {}", session.memory_samples.len());
                        println!("I/O Samples: {}", session.io_samples.len());
                        println!("Network Samples: {}", session.network_samples.len());
                        
                        if !session.cpu_samples.is_empty() {
                            let last_cpu = &session.cpu_samples[session.cpu_samples.len() - 1];
                            println!("\nLatest CPU Metrics:");
                            println!("  User: {:.1}%", last_cpu.user_percent);
                            println!("  System: {:.1}%", last_cpu.system_percent);
                            println!("  Idle: {:.1}%", last_cpu.idle_percent);
                            println!("  IOWait: {:.1}%", last_cpu.iowait_percent);
                            println!("  Context Switches: {}", last_cpu.context_switches);
                            println!("  Interrupts: {}", last_cpu.interrupts);
                        }
                        
                        if !session.memory_samples.is_empty() {
                            let last_mem = &session.memory_samples[session.memory_samples.last().unwrap_or(&0)];
                            println!("\nLatest Memory Metrics:");
                            println!("  Total: {}", profiler.format_bytes(last_mem.total));
                            println!("  Used: {}", profiler.format_bytes(last_mem.used));
                            println!("  Free: {}", profiler.format_bytes(last_mem.free));
                            println!("  Buffers: {}", profiler.format_bytes(last_mem.buffers));
                            println!("  Cached: {}", profiler.format_bytes(last_mem.cached));
                            println!("  Swap Used: {}", profiler.format_bytes(last_mem.swap_used));
                        }
                    }
                }
            }
            "sessions" => {
                println!("--- All Sessions ---");
                for session in profiler.get_all_sessions() {
                    let status = if profiler.is_profiling && profiler.current_session.as_ref() == Some(&session.id) {
                        "[ACTIVE]"
                    } else {
                        ""
                    };
                    println!("{} - {} samples {}", session.id, session.cpu_samples.len(), status);
                }
            }
            "avg_cpu" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(avg) = profiler.avg_cpu_usage(arg) {
                        println!("Average CPU Usage: {:.1}%", avg);
                    }
                }
            }
            "avg_mem" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(avg) = profiler.avg_memory_usage(arg) {
                        println!("Average Memory Usage: {:.1}%", avg);
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
