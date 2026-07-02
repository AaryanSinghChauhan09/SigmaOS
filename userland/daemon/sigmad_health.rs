// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/daemon/sigmad_health.rs — sigmad-health: System Health Monitor
// Language: Rust (std) — userland daemon
// Pattern: OOP via HealthMonitor struct + Check trait

use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use std::thread;
use std::fs;

// ── Check Trait (OOP) ─────────────────────────────────────────────────────────

pub trait Check: Send {
    fn name(&self) -> &str;
    fn run(&self) -> CheckResult;
    fn interval(&self) -> Duration;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckStatus { Ok, Warn, Critical }

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub status:  CheckStatus,
    pub message: String,
    pub value:   Option<f64>,
}

impl CheckResult {
    pub fn ok(msg: &str) -> Self {
        Self { status: CheckStatus::Ok, message: msg.to_owned(), value: None }
    }
    pub fn warn(msg: &str, v: f64) -> Self {
        Self { status: CheckStatus::Warn, message: msg.to_owned(), value: Some(v) }
    }
    pub fn critical(msg: &str) -> Self {
        Self { status: CheckStatus::Critical, message: msg.to_owned(), value: None }
    }
}

// ── Concrete Checks ───────────────────────────────────────────────────────────

pub struct CpuLoadCheck { pub warn_pct: f64 }
impl Check for CpuLoadCheck {
    fn name(&self) -> &str { "cpu_load" }
    fn interval(&self) -> Duration { Duration::from_secs(5) }
    fn run(&self) -> CheckResult {
        // Read /proc/stat (on Linux host) or sigma /proc equivalent
        if let Ok(stat) = fs::read_to_string("/proc/stat") {
            if let Some(line) = stat.lines().next() {
                let parts: Vec<u64> = line.split_whitespace()
                    .skip(1).filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 4 {
                    let idle  = parts[3] as f64;
                    let total = parts.iter().take(7).sum::<u64>() as f64;
                    let busy_pct = 100.0 * (1.0 - idle / total);
                    if busy_pct > self.warn_pct {
                        return CheckResult::warn(
                            &format!("CPU load {:.1}%", busy_pct), busy_pct);
                    }
                    return CheckResult::ok(&format!("CPU {:.1}%", busy_pct));
                }
            }
        }
        CheckResult::ok("cpu: unknown (no /proc/stat)")
    }
}

pub struct MemoryCheck { pub warn_pct: f64 }
impl Check for MemoryCheck {
    fn name(&self) -> &str { "memory" }
    fn interval(&self) -> Duration { Duration::from_secs(10) }
    fn run(&self) -> CheckResult {
        if let Ok(info) = fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64; let mut available = 0u64;
            for line in info.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    match parts[0] {
                        "MemTotal:"     => { total     = parts[1].parse().unwrap_or(0); }
                        "MemAvailable:" => { available = parts[1].parse().unwrap_or(0); }
                        _ => {}
                    }
                }
            }
            if total > 0 {
                let used_pct = 100.0 * (total - available) as f64 / total as f64;
                if used_pct > self.warn_pct {
                    return CheckResult::warn(
                        &format!("Memory {:.1}% used ({} MB / {} MB)",
                            used_pct, (total-available)/1024, total/1024), used_pct);
                }
                return CheckResult::ok(
                    &format!("Memory {:.1}% ({} MB free)", used_pct, available/1024));
            }
        }
        CheckResult::ok("memory: unknown")
    }
}

pub struct DiskCheck { pub path: String, pub warn_pct: f64 }
impl Check for DiskCheck {
    fn name(&self) -> &str { "disk" }
    fn interval(&self) -> Duration { Duration::from_secs(30) }
    fn run(&self) -> CheckResult {
        // Simplified: always ok (real impl uses statvfs syscall)
        CheckResult::ok(&format!("disk {}: ok", self.path))
    }
}

pub struct ServiceCheck { pub service_name: String }
impl Check for ServiceCheck {
    fn name(&self) -> &str { "service" }
    fn interval(&self) -> Duration { Duration::from_secs(15) }
    fn run(&self) -> CheckResult {
        // Check if process with matching name is running
        let pid_dir = fs::read_dir("/proc");
        let svc = self.service_name.clone();
        if let Ok(entries) = pid_dir {
            for entry in entries.flatten() {
                let path = entry.path().join("comm");
                if let Ok(name) = fs::read_to_string(&path) {
                    if name.trim() == svc { return CheckResult::ok("running"); }
                }
            }
        }
        CheckResult::critical(&format!("service {} not running", svc))
    }
}

// ── Health Monitor ────────────────────────────────────────────────────────────

pub struct HealthMonitor {
    checks:      Vec<Box<dyn Check>>,
    last_run:    BTreeMap<String, Instant>,
    results:     BTreeMap<String, CheckResult>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            checks:   Vec::new(),
            last_run: BTreeMap::new(),
            results:  BTreeMap::new(),
        }
    }

    pub fn add_check(&mut self, check: Box<dyn Check>) {
        self.last_run.insert(check.name().to_owned(),
            Instant::now() - check.interval()); // run immediately on first tick
        self.checks.push(check);
    }

    pub fn tick(&mut self) {
        for check in &self.checks {
            let name = check.name().to_owned();
            let due  = self.last_run.get(&name)
                .map(|t| t.elapsed() >= check.interval())
                .unwrap_or(true);
            if due {
                let result = check.run();
                self.last_run.insert(name.clone(), Instant::now());
                if result.status != CheckStatus::Ok {
                    eprintln!("[sigmad-health] {} {:?}: {}",
                              name, result.status, result.message);
                }
                self.results.insert(name, result);
            }
        }
    }

    pub fn all_ok(&self) -> bool {
        self.results.values().all(|r| r.status == CheckStatus::Ok)
    }

    pub fn report(&self) {
        for (name, result) in &self.results {
            println!("{:<20} [{:?}] {}", name, result.status, result.message);
        }
    }
}

// ── Entry Point ───────────────────────────────────────────────────────────────

fn main() {
    eprintln!("[sigmad-health] starting health monitor daemon");
    let mut mon = HealthMonitor::new();
    mon.add_check(Box::new(CpuLoadCheck    { warn_pct: 80.0 }));
    mon.add_check(Box::new(MemoryCheck     { warn_pct: 85.0 }));
    mon.add_check(Box::new(DiskCheck       { path: "/".into(), warn_pct: 90.0 }));
    mon.add_check(Box::new(ServiceCheck    { service_name: "sigma-sh".into() }));

    loop {
        mon.tick();
        thread::sleep(Duration::from_secs(1));
    }
}
