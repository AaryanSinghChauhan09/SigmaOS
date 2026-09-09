#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ─── PSI Resource Category ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PsiResource {
    Cpu,
    Memory,
    Io,
}

// ─── PSI Stall Metric ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PsiMetric {
    pub avg10: f64,      // 10-second moving average percentage (0.0 - 100.0)
    pub avg60: f64,      // 60-second moving average percentage
    pub avg300: f64,     // 300-second moving average percentage
    pub total_us: u64,   // Total accumulated stall time in microseconds
}

impl PsiMetric {
    pub fn new() -> Self {
        PsiMetric { avg10: 0.0, avg60: 0.0, avg300: 0.0, total_us: 0 }
    }

    pub fn update(&mut self, stall_time_us: u64, elapsed_window_us: u64) {
        self.total_us = self.total_us.saturating_add(stall_time_us);
        if elapsed_window_us > 0 {
            let pct = (stall_time_us as f64 / elapsed_window_us as f64) * 100.0;
            let pct = pct.clamp(0.0, 100.0);
            // Exponential moving average approximations
            self.avg10 = (self.avg10 * 0.9) + (pct * 0.1);
            self.avg60 = (self.avg60 * 0.98) + (pct * 0.02);
            self.avg300 = (self.avg300 * 0.996) + (pct * 0.004);
        }
    }
}

// ─── Resource Pressure Record (/proc/pressure/*) ──────────────────────────────

#[derive(Debug, Clone)]
pub struct PsiRecord {
    pub resource: PsiResource,
    pub some: PsiMetric, // At least one task was stalled
    pub full: PsiMetric, // ALL tasks were stalled (complete deadlock)
}

impl PsiRecord {
    pub fn new(resource: PsiResource) -> Self {
        PsiRecord {
            resource,
            some: PsiMetric::new(),
            full: PsiMetric::new(),
        }
    }
}

// ─── Pressure Notification Trigger ───────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PsiTrigger {
    pub id: u32,
    pub resource: PsiResource,
    pub threshold_pct: f64,
    pub window_us: u64,
    pub fired_count: u64,
}

impl PsiTrigger {
    pub fn new(id: u32, resource: PsiResource, threshold_pct: f64, window_us: u64) -> Self {
        PsiTrigger {
            id,
            resource,
            threshold_pct,
            window_us,
            fired_count: 0,
        }
    }
}

// ─── Sovereign PSI Manager ────────────────────────────────────────────────────

pub struct SovereignPsiManager {
    pub cpu: PsiRecord,
    pub memory: PsiRecord,
    pub io: PsiRecord,
    pub triggers: Vec<PsiTrigger>,
    pub next_trigger_id: u32,
    pub total_samples: u64,
}

impl SovereignPsiManager {
    pub fn new() -> Self {
        SovereignPsiManager {
            cpu: PsiRecord::new(PsiResource::Cpu),
            memory: PsiRecord::new(PsiResource::Memory),
            io: PsiRecord::new(PsiResource::Io),
            triggers: Vec::new(),
            next_trigger_id: 1,
            total_samples: 0,
        }
    }

    /// Record resource stall interval
    pub fn record_stall(&mut self, resource: PsiResource, some_stalls_us: u64, full_stalls_us: u64, window_us: u64) {
        self.total_samples = self.total_samples.saturating_add(1);
        let record = match resource {
            PsiResource::Cpu => &mut self.cpu,
            PsiResource::Memory => &mut self.memory,
            PsiResource::Io => &mut self.io,
        };

        record.some.update(some_stalls_us, window_us);
        record.full.update(full_stalls_us, window_us);

        // Evaluate triggers
        let current_pct = record.some.avg10;
        for trg in &mut self.triggers {
            if trg.resource == resource && current_pct >= trg.threshold_pct {
                trg.fired_count = trg.fired_count.saturating_add(1);
            }
        }
    }

    /// Register threshold trigger
    pub fn register_trigger(&mut self, resource: PsiResource, threshold_pct: f64, window_us: u64) -> u32 {
        let id = self.next_trigger_id;
        self.next_trigger_id = self.next_trigger_id.saturating_add(1);
        self.triggers.push(PsiTrigger::new(id, resource, threshold_pct, window_us));
        id
    }

    pub fn format_proc_pressure(&self, resource: PsiResource) -> String {
        let r = match resource {
            PsiResource::Cpu => &self.cpu,
            PsiResource::Memory => &self.memory,
            PsiResource::Io => &self.io,
        };

        let mut out = String::new();
        out.push_str("some avg10=");
        out.push_str(&(r.some.avg10 as u32).to_string());
        out.push_str(" total=");
        out.push_str(&r.some.total_us.to_string());
        out.push('\n');
        if resource != PsiResource::Cpu {
            out.push_str("full avg10=");
            out.push_str(&(r.full.avg10 as u32).to_string());
            out.push_str(" total=");
            out.push_str(&r.full.total_us.to_string());
            out.push('\n');
        }
        out
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi_initial_state() {
        let mgr = SovereignPsiManager::new();
        assert_eq!(mgr.cpu.some.total_us, 0);
        assert_eq!(mgr.memory.full.total_us, 0);
        assert_eq!(mgr.total_samples, 0);
    }

    #[test]
    fn test_psi_record_stall_accumulation() {
        let mut mgr = SovereignPsiManager::new();
        mgr.record_stall(PsiResource::Memory, 2_000_000, 500_000, 10_000_000);
        assert_eq!(mgr.memory.some.total_us, 2_000_000);
        assert_eq!(mgr.memory.full.total_us, 500_000);
        assert!(mgr.memory.some.avg10 > 0.0);
    }

    #[test]
    fn test_psi_trigger_firing() {
        let mut mgr = SovereignPsiManager::new();
        let trg_id = mgr.register_trigger(PsiResource::Io, 5.0, 1_000_000);
        assert_eq!(trg_id, 1);

        // 80% stall over 10s pushes avg10 above 5% threshold
        mgr.record_stall(PsiResource::Io, 8_000_000, 4_000_000, 10_000_000);
        assert!(mgr.triggers[0].fired_count > 0);
    }

    #[test]
    fn test_psi_trigger_threshold_isolation() {
        let mut mgr = SovereignPsiManager::new();
        // High threshold: 90%
        mgr.register_trigger(PsiResource::Cpu, 90.0, 1_000_000);
        // Low stall: 5%
        mgr.record_stall(PsiResource::Cpu, 50_000, 0, 1_000_000);
        assert_eq!(mgr.triggers[0].fired_count, 0); // Should not fire
    }

    #[test]
    fn test_psi_format_proc_pressure() {
        let mut mgr = SovereignPsiManager::new();
        mgr.record_stall(PsiResource::Memory, 1000, 200, 10000);
        let s = mgr.format_proc_pressure(PsiResource::Memory);
        assert!(s.contains("some avg10="));
        assert!(s.contains("full avg10="));
        assert!(s.contains("total=1000"));
    }

    #[test]
    fn test_psi_cpu_no_full_stall() {
        // CPU pressure does not have a "full" stall line in Linux PSI
        let mgr = SovereignPsiManager::new();
        let s = mgr.format_proc_pressure(PsiResource::Cpu);
        assert!(s.contains("some avg10="));
        assert!(!s.contains("full avg10="));
    }
}
