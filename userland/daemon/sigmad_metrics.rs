// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/daemon/sigmad_metrics.rs — Prometheus-compatible metrics daemon
// Language: Rust (std) — OOP via MetricsDaemon + Metric trait

use std::collections::BTreeMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Write, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// ── Metric Types ──────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub enum MetricType { Counter, Gauge, Histogram, Summary }

#[derive(Clone, Debug)]
pub struct MetricSample {
    pub name:   String,
    pub labels: BTreeMap<String, String>,
    pub value:  f64,
    pub ts_ms:  u64,
}

#[derive(Clone, Debug)]
pub struct MetricFamily {
    pub name:   String,
    pub help:   String,
    pub mtype:  MetricType,
    pub samples: Vec<MetricSample>,
}

impl MetricFamily {
    pub fn new(name: &str, help: &str, mtype: MetricType) -> Self {
        Self { name: name.to_owned(), help: help.to_owned(), mtype, samples: Vec::new() }
    }
    pub fn add(&mut self, labels: BTreeMap<String,String>, value: f64) {
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64;
        self.samples.push(MetricSample { name: self.name.clone(), labels, value, ts_ms: ts });
    }
    pub fn set_simple(&mut self, value: f64) {
        self.samples.clear();
        self.add(BTreeMap::new(), value);
    }
    pub fn format_prometheus(&self) -> String {
        let type_str = match self.mtype {
            MetricType::Counter   => "counter",
            MetricType::Gauge     => "gauge",
            MetricType::Histogram => "histogram",
            MetricType::Summary   => "summary",
        };
        let mut out = format!("# HELP {} {}\n# TYPE {} {}\n", self.name, self.help, self.name, type_str);
        for s in &self.samples {
            if s.labels.is_empty() {
                out.push_str(&format!("{} {} {}\n", s.name, s.value, s.ts_ms));
            } else {
                let lbl: String = s.labels.iter()
                    .map(|(k,v)| format!("{}=\"{}\"", k, v.replace('"', "\\\"")))
                    .collect::<Vec<_>>().join(",");
                out.push_str(&format!("{}{{{}}} {} {}\n", s.name, lbl, s.value, s.ts_ms));
            }
        }
        out
    }
}

// ── Registry ──────────────────────────────────────────────────────────────────
pub struct MetricRegistry { families: BTreeMap<String, MetricFamily> }
impl MetricRegistry {
    pub fn new() -> Self { Self { families: BTreeMap::new() } }
    pub fn register(&mut self, f: MetricFamily) { self.families.insert(f.name.clone(), f); }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut MetricFamily> { self.families.get_mut(name) }
    pub fn exposition(&self) -> String {
        self.families.values().map(|f| f.format_prometheus()).collect::<Vec<_>>().join("\n")
    }
}

// ── Collector Trait ───────────────────────────────────────────────────────────
pub trait Collector: Send {
    fn collect(&self, registry: &mut MetricRegistry);
}

pub struct SystemCollector;
impl Collector for SystemCollector {
    fn collect(&self, reg: &mut MetricRegistry) {
        // CPU
        if let Ok(stat) = std::fs::read_to_string("/proc/stat") {
            if let Some(line) = stat.lines().next() {
                let nums: Vec<u64> = line.split_whitespace().skip(1)
                    .filter_map(|s| s.parse().ok()).collect();
                if nums.len() >= 4 {
                    let total: u64 = nums.iter().take(7).sum();
                    let idle = nums[3];
                    if let Some(f) = reg.get_mut("sigma_cpu_usage_ratio") {
                        f.set_simple(if total > 0 { 1.0 - (idle as f64 / total as f64) } else { 0.0 });
                    }
                }
            }
        }
        // Memory
        if let Ok(info) = std::fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64; let mut avail = 0u64;
            for line in info.lines() {
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.len() >= 2 {
                    match p[0] { "MemTotal:" => total = p[1].parse().unwrap_or(0),
                                 "MemAvailable:" => avail = p[1].parse().unwrap_or(0), _ => {} }
                }
            }
            if let Some(f) = reg.get_mut("sigma_memory_used_bytes") {
                f.set_simple((total.saturating_sub(avail) * 1024) as f64);
            }
            if let Some(f) = reg.get_mut("sigma_memory_total_bytes") {
                f.set_simple((total * 1024) as f64);
            }
        }
    }
}

fn handle_metrics_request(mut stream: TcpStream, exposition: String) {
    let body = exposition.as_bytes();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.write_all(body);
}

// ── Metrics Daemon ────────────────────────────────────────────────────────────
pub struct MetricsDaemon {
    registry:   MetricRegistry,
    collectors: Vec<Box<dyn Collector>>,
    bind_addr:  String,
}

impl MetricsDaemon {
    pub fn new(addr: &str) -> Self {
        let mut reg = MetricRegistry::new();
        reg.register(MetricFamily::new("sigma_cpu_usage_ratio", "CPU usage 0.0-1.0", MetricType::Gauge));
        reg.register(MetricFamily::new("sigma_memory_used_bytes", "Used memory in bytes", MetricType::Gauge));
        reg.register(MetricFamily::new("sigma_memory_total_bytes", "Total memory in bytes", MetricType::Gauge));
        reg.register(MetricFamily::new("sigma_uptime_seconds", "System uptime in seconds", MetricType::Counter));
        reg.register(MetricFamily::new("sigma_process_count", "Total processes", MetricType::Gauge));
        Self {
            registry: reg,
            collectors: vec![Box::new(SystemCollector)],
            bind_addr: addr.to_owned(),
        }
    }

    pub fn add_collector(&mut self, c: Box<dyn Collector>) { self.collectors.push(c); }

    pub fn collect_all(&mut self) {
        for c in &self.collectors {
            c.collect(&mut self.registry);
        }
    }

    pub fn run(&mut self) {
        let listener = match TcpListener::bind(&self.bind_addr) {
            Ok(l) => l, Err(e) => { eprintln!("[sigmad-metrics] bind failed: {}", e); return; }
        };
        eprintln!("[sigmad-metrics] serving metrics on http://{}/metrics", self.bind_addr);
        for stream in listener.incoming().flatten() {
            self.collect_all();
            let expo = self.registry.exposition();
            // Read request (discard it — we always serve /metrics)
            let _ = BufReader::new(&stream).lines().next();
            handle_metrics_request(stream, expo);
        }
    }
}
