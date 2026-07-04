// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/tools/sigma_monitor.rs — sigma-monitor: System Resource Monitor
// Language: Rust (std) — OOP via SystemMonitor + Collector trait

use std::collections::VecDeque;
use std::fs;
use std::time::{Duration, Instant};

// ── Sample Point ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct SystemSample {
    pub cpu_pct:    f64,
    pub mem_used:   u64,    // kB
    pub mem_total:  u64,    // kB
    pub rx_bytes:   u64,
    pub tx_bytes:   u64,
    pub disk_read:  u64,
    pub disk_write: u64,
    pub load_avg:   [f64; 3],
    pub process_count: usize,
    pub timestamp:  Instant,
}

impl SystemSample {
    pub fn cpu_pct_str(&self) -> String { format!("{:.1}%", self.cpu_pct) }
    pub fn mem_pct(&self) -> f64 {
        if self.mem_total == 0 { return 0.0; }
        100.0 * self.mem_used as f64 / self.mem_total as f64
    }
    pub fn mem_str(&self) -> String {
        format!("{} MB / {} MB ({:.0}%)",
            self.mem_used / 1024, self.mem_total / 1024, self.mem_pct())
    }
}

// ── Process Info ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid:     u32,
    pub name:    String,
    pub state:   char,
    pub cpu_pct: f64,
    pub mem_kb:  u64,
    pub threads: u32,
    pub user:    String,
}

// ── Collector Trait (OOP) ─────────────────────────────────────────────────────

pub trait Collector: Send {
    fn name(&self) -> &'static str;
    fn sample(&mut self) -> SystemSample;
    fn processes(&mut self) -> Vec<ProcessInfo>;
}

// ── Linux /proc Collector ─────────────────────────────────────────────────────

pub struct ProcCollector {
    prev_cpu:  [u64; 7],
    prev_rx:   u64,
    prev_tx:   u64,
}

impl ProcCollector {
    pub fn new() -> Self { Self { prev_cpu: [0u64; 7], prev_rx: 0, prev_tx: 0 } }

    fn read_cpu() -> [u64; 7] {
        let mut out = [0u64; 7];
        if let Ok(stat) = fs::read_to_string("/proc/stat") {
            if let Some(line) = stat.lines().next() {
                let nums: Vec<u64> = line.split_whitespace()
                    .skip(1).filter_map(|s| s.parse().ok()).collect();
                for i in 0..7.min(nums.len()) { out[i] = nums[i]; }
            }
        }
        out
    }

    fn parse_meminfo() -> (u64, u64) {
        let mut total = 0u64; let mut avail = 0u64;
        if let Ok(info) = fs::read_to_string("/proc/meminfo") {
            for line in info.lines() {
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.len() >= 2 {
                    match p[0] {
                        "MemTotal:"     => { total = p[1].parse().unwrap_or(0); }
                        "MemAvailable:" => { avail = p[1].parse().unwrap_or(0); }
                        _ => {}
                    }
                }
            }
        }
        (total, total.saturating_sub(avail))
    }

    fn parse_net_stats() -> (u64, u64) {
        let mut rx = 0u64; let mut tx = 0u64;
        if let Ok(data) = fs::read_to_string("/proc/net/dev") {
            for line in data.lines().skip(2) {
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.len() >= 10 {
                    let iface = p[0].trim_end_matches(':');
                    if iface != "lo" {
                        rx += p[1].parse().unwrap_or(0);
                        tx += p[9].parse().unwrap_or(0);
                    }
                }
            }
        }
        (rx, tx)
    }

    fn parse_loadavg() -> [f64; 3] {
        let mut la = [0.0f64; 3];
        if let Ok(data) = fs::read_to_string("/proc/loadavg") {
            let p: Vec<&str> = data.split_whitespace().collect();
            for i in 0..3.min(p.len()) { la[i] = p[i].parse().unwrap_or(0.0); }
        }
        la
    }
}

impl Collector for ProcCollector {
    fn name(&self) -> &'static str { "procfs" }

    fn sample(&mut self) -> SystemSample {
        let cur_cpu = Self::read_cpu();
        let idle_prev = self.prev_cpu[3];
        let total_prev: u64 = self.prev_cpu.iter().sum();
        let idle_cur  = cur_cpu[3];
        let total_cur: u64 = cur_cpu.iter().sum();
        let dt_total = total_cur.saturating_sub(total_prev);
        let dt_idle  = idle_cur.saturating_sub(idle_prev);
        let cpu_pct = if dt_total > 0 {
            100.0 * (dt_total - dt_idle) as f64 / dt_total as f64
        } else { 0.0 };
        self.prev_cpu = cur_cpu;

        let (mem_total, mem_used) = Self::parse_meminfo();
        let (rx, tx) = Self::parse_net_stats();
        let drx = rx.saturating_sub(self.prev_rx);
        let dtx = tx.saturating_sub(self.prev_tx);
        self.prev_rx = rx; self.prev_tx = tx;

        SystemSample {
            cpu_pct, mem_used, mem_total,
            rx_bytes: drx, tx_bytes: dtx,
            disk_read: 0, disk_write: 0,
            load_avg: Self::parse_loadavg(),
            process_count: 0,
            timestamp: Instant::now(),
        }
    }

    fn processes(&mut self) -> Vec<ProcessInfo> {
        let mut procs = Vec::new();
        if let Ok(entries) = fs::read_dir("/proc") {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.chars().all(|c| c.is_ascii_digit()) {
                    let pid: u32 = name_str.parse().unwrap_or(0);
                    let comm = fs::read_to_string(
                        format!("/proc/{}/comm", pid)).unwrap_or_default()
                        .trim().to_owned();
                    let status = fs::read_to_string(
                        format!("/proc/{}/status", pid)).unwrap_or_default();
                    let mut vm_rss = 0u64; let mut threads = 0u32;
                    let mut state = 'R';
                    for line in status.lines() {
                        let p: Vec<&str> = line.splitn(2, ':').collect();
                        if p.len() == 2 {
                            match p[0].trim() {
                                "VmRSS"   => vm_rss  = p[1].trim().split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0),
                                "Threads" => threads = p[1].trim().parse().unwrap_or(1),
                                "State"   => state   = p[1].trim().chars().next().unwrap_or('R'),
                                _ => {}
                            }
                        }
                    }
                    procs.push(ProcessInfo {
                        pid, name: comm, state, cpu_pct: 0.0, mem_kb: vm_rss, threads, user: "root".to_owned(),
                    });
                }
            }
        }
        procs.sort_by_key(|p| core::cmp::Reverse(p.mem_kb));
        procs
    }
}

// ── System Monitor ────────────────────────────────────────────────────────────

pub struct SystemMonitor {
    collector: Box<dyn Collector>,
    history:   VecDeque<SystemSample>,
    interval:  Duration,
    last_tick: Instant,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            collector: Box::new(ProcCollector::new()),
            history:   VecDeque::with_capacity(60),
            interval:  Duration::from_secs(1),
            last_tick: Instant::now(),
        }
    }

    pub fn tick(&mut self) -> Option<SystemSample> {
        if self.last_tick.elapsed() < self.interval { return None; }
        self.last_tick = Instant::now();
        let sample = self.collector.sample();
        if self.history.len() >= 60 { self.history.pop_front(); }
        self.history.push_back(sample.clone());
        Some(sample)
    }

    pub fn current(&self) -> Option<&SystemSample> { self.history.back() }
    pub fn history(&self) -> &VecDeque<SystemSample> { &self.history }
    pub fn processes(&mut self) -> Vec<ProcessInfo> { self.collector.processes() }

    pub fn avg_cpu(&self, last_n: usize) -> f64 {
        let n = self.history.len().min(last_n);
        if n == 0 { return 0.0; }
        let sum: f64 = self.history.iter().rev().take(n).map(|s| s.cpu_pct).sum();
        sum / n as f64
    }

    pub fn format_report(&self) -> String {
        let s = match self.current() {
            Some(s) => s,
            None    => return "No data yet.".to_owned(),
        };
        format!(
            "CPU: {}  MEM: {}  NET: ↓{}/s ↑{}/s  Load: {:.2} {:.2} {:.2}",
            s.cpu_pct_str(), s.mem_str(),
            human_bytes(s.rx_bytes), human_bytes(s.tx_bytes),
            s.load_avg[0], s.load_avg[1], s.load_avg[2]
        )
    }
}

fn human_bytes(b: u64) -> String {
    if b >= 1_000_000 { format!("{:.1}MB", b as f64 / 1_000_000.0) }
    else if b >= 1_000 { format!("{:.0}KB", b as f64 / 1_000.0) }
    else { format!("{}B", b) }
}
