#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// SigmaOS Power Management Subsystem
/// CPUfreq governors, thermal throttling, suspend/resume lifecycle
use core::sync::atomic::{AtomicUsize, Ordering};
use std::string::{String, ToString};
use std::vec::Vec;

// ── CPUfreq ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpufreqGovernor {
    Performance,  // Always max freq
    Powersave,    // Always min freq
    OnDemand,     // Scale based on load
    Conservative, // Slow steps up/down
    Schedutil,    // Coupled to scheduler load (Linux 4.7+)
}

#[derive(Debug, Clone)]
pub struct CpufreqPolicy {
    pub cpu: u32,
    pub min_freq_khz: u32,
    pub max_freq_khz: u32,
    pub cur_freq_khz: u32,
    pub governor: CpufreqGovernor,
    pub transition_latency_ns: u32,
}

impl CpufreqPolicy {
    pub fn new(cpu: u32, min_khz: u32, max_khz: u32) -> Self {
        CpufreqPolicy {
            cpu,
            min_freq_khz: min_khz,
            max_freq_khz: max_khz,
            cur_freq_khz: max_khz,
            governor: CpufreqGovernor::Schedutil,
            transition_latency_ns: 50_000, // 50µs typical
        }
    }

    /// Update frequency based on governor and load percentage (0–100)
    pub fn update_freq(&mut self, load_percent: u8) {
        let load = load_percent.min(100) as u32;
        self.cur_freq_khz = match self.governor {
            CpufreqGovernor::Performance => self.max_freq_khz,
            CpufreqGovernor::Powersave => self.min_freq_khz,
            CpufreqGovernor::OnDemand | CpufreqGovernor::Schedutil => {
                let range = self.max_freq_khz - self.min_freq_khz;
                self.min_freq_khz + range * load / 100
            }
            CpufreqGovernor::Conservative => {
                // Step 10% at a time
                let step = (self.max_freq_khz - self.min_freq_khz) / 10;
                if load > 70 {
                    (self.cur_freq_khz + step).min(self.max_freq_khz)
                } else if load < 30 {
                    self.cur_freq_khz
                        .saturating_sub(step)
                        .max(self.min_freq_khz)
                } else {
                    self.cur_freq_khz
                }
            }
        };
    }
}

pub struct CpufreqManager {
    pub policies: Vec<CpufreqPolicy>,
}

impl CpufreqManager {
    pub fn new(num_cpus: u32) -> Self {
        // Typical modern CPU: 800 MHz – 3.6 GHz
        let policies = (0..num_cpus)
            .map(|cpu| CpufreqPolicy::new(cpu, 800_000, 3_600_000))
            .collect();
        CpufreqManager { policies }
    }

    pub fn set_governor_all(&mut self, gov: CpufreqGovernor) {
        for p in &mut self.policies {
            p.governor = gov;
        }
    }

    pub fn update_all(&mut self, loads: &[u8]) {
        for (i, p) in self.policies.iter_mut().enumerate() {
            if let Some(&load) = loads.get(i) {
                p.update_freq(load);
            }
        }
    }
}

// ── Thermal Management ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalZoneType {
    Cpu,
    Gpu,
    Battery,
    Ambient,
    Pch,
}

#[derive(Debug, Clone)]
pub struct ThermalZone {
    pub name: String,
    pub zone_type: ThermalZoneType,
    pub current_temp_mdegc: i32, // millidegrees C
    pub trip_critical: i32,      // Emergency shutdown
    pub trip_hot: i32,           // Throttle hard
    pub trip_warm: i32,          // Throttle lightly
}

impl ThermalZone {
    pub fn cpu_zone(name: &str) -> Self {
        ThermalZone {
            name: name.to_string(),
            zone_type: ThermalZoneType::Cpu,
            current_temp_mdegc: 45_000, // 45°C idle
            trip_critical: 105_000,     // 105°C → shutdown
            trip_hot: 95_000,           // 95°C → throttle
            trip_warm: 80_000,          // 80°C → light throttle
        }
    }

    pub fn is_throttled(&self) -> bool {
        self.current_temp_mdegc >= self.trip_warm
    }
    pub fn is_emergency(&self) -> bool {
        self.current_temp_mdegc >= self.trip_critical
    }

    pub fn throttle_percent(&self) -> u8 {
        if self.current_temp_mdegc < self.trip_warm {
            return 100;
        }
        if self.current_temp_mdegc >= self.trip_critical {
            return 0;
        }
        let hot_range = self.trip_critical - self.trip_warm;
        let excess = self.current_temp_mdegc - self.trip_warm;
        (100 - (excess * 100 / hot_range)) as u8
    }
}

pub struct ThermalManager {
    pub zones: Vec<ThermalZone>,
    throttle_events: AtomicUsize,
    emergency_events: AtomicUsize,
}

impl ThermalManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ThermalManager {
            zones: Vec::new(),
            throttle_events: AtomicUsize::new(0),
            emergency_events: AtomicUsize::new(0),
        }
    }

    pub fn register_zone(&mut self, zone: ThermalZone) {
        self.zones.push(zone);
    }

    pub fn update_temp(&mut self, zone_idx: usize, temp_mdegc: i32) {
        if let Some(zone) = self.zones.get_mut(zone_idx) {
            zone.current_temp_mdegc = temp_mdegc;
            if zone.is_emergency() {
                self.emergency_events.fetch_add(1, Ordering::Relaxed);
            } else if zone.is_throttled() {
                self.throttle_events.fetch_add(1, Ordering::Relaxed);
            }
        }
    }

    pub fn any_emergency(&self) -> bool {
        self.zones.iter().any(|z| z.is_emergency())
    }
    pub fn min_throttle_percent(&self) -> u8 {
        self.zones
            .iter()
            .map(|z| z.throttle_percent())
            .min()
            .unwrap_or(100)
    }
    pub fn throttle_events(&self) -> usize {
        self.throttle_events.load(Ordering::Relaxed)
    }
}

impl Default for ThermalManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Suspend / Resume ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SleepState {
    S0,
    S1,
    S3Suspend,
    S4Hibernate,
    S5SoftOff,
}

pub struct PowerStateManager {
    pub current_state: SleepState,
    suspend_count: AtomicUsize,
    resume_count: AtomicUsize,
    pub wake_sources: Vec<String>,
}

impl PowerStateManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PowerStateManager {
            current_state: SleepState::S0,
            suspend_count: AtomicUsize::new(0),
            resume_count: AtomicUsize::new(0),
            wake_sources: Vec::new(),
        }
    }

    pub fn add_wake_source(&mut self, name: &str) {
        self.wake_sources.push(name.to_string());
    }

    pub fn enter_sleep(&mut self, state: SleepState) -> Result<(), &'static str> {
        if state == SleepState::S0 {
            return Err("Already in S0");
        }
        self.current_state = state;
        self.suspend_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), &'static str> {
        if self.current_state == SleepState::S0 {
            return Err("Already running (S0)");
        }
        self.current_state = SleepState::S0;
        self.resume_count.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    pub fn suspend_count(&self) -> usize {
        self.suspend_count.load(Ordering::Relaxed)
    }
    pub fn resume_count(&self) -> usize {
        self.resume_count.load(Ordering::Relaxed)
    }
}

impl Default for PowerStateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpufreq_governors() {
        let mut pol = CpufreqPolicy::new(0, 800_000, 3_600_000);
        pol.governor = CpufreqGovernor::Performance;
        pol.update_freq(0);
        assert_eq!(pol.cur_freq_khz, 3_600_000);

        pol.governor = CpufreqGovernor::Powersave;
        pol.update_freq(100);
        assert_eq!(pol.cur_freq_khz, 800_000);

        pol.governor = CpufreqGovernor::OnDemand;
        pol.update_freq(50);
        let expected = 800_000 + (3_600_000 - 800_000) * 50 / 100;
        assert_eq!(pol.cur_freq_khz, expected);
    }

    #[test]
    fn test_thermal_throttle() {
        let zone = ThermalZone::cpu_zone("cpu0");
        assert!(!zone.is_throttled());
        assert_eq!(zone.throttle_percent(), 100);

        let mut zone2 = ThermalZone::cpu_zone("cpu1");
        zone2.current_temp_mdegc = 95_000; // at trip_hot
        assert!(zone2.is_throttled());
        assert!(!zone2.is_emergency());
        let pct = zone2.throttle_percent();
        assert!(pct < 100 && pct > 0);
    }

    #[test]
    fn test_suspend_resume() {
        let mut pm = PowerStateManager::new();
        pm.add_wake_source("RTC");
        pm.enter_sleep(SleepState::S3Suspend).unwrap();
        assert_eq!(pm.current_state, SleepState::S3Suspend);
        pm.resume().unwrap();
        assert_eq!(pm.current_state, SleepState::S0);
        assert_eq!(pm.suspend_count(), 1);
        assert_eq!(pm.resume_count(), 1);
    }
}
