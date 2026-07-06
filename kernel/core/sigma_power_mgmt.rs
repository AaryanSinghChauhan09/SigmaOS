// SigmaOS — Power Management (ACPI S-states, DVFS, thermal, battery)
// Sovereign implementation — no external dependencies
#![no_std]
#![allow(dead_code)]
use core::sync::atomic::{AtomicU32, AtomicI32, AtomicBool, Ordering};

// ─── ACPI S-States ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AcpiSState { S0=0, S1=1, S2=2, S3=3, S4=4, S5=5 }

// ─── CPU P-States (DVFS) ─────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct PState {
    pub freq_mhz:   u32,
    pub voltage_mv: u16,
    pub power_mw:   u32,
    pub latency_us: u16,
}

pub const MAX_PSTATES: usize = 16;

// ─── CPU C-States (idle) ─────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct CState {
    pub name:       [u8; 8],
    pub latency_us: u32,
    pub power_mw:   u32,
}

pub const MAX_CSTATES: usize = 8;

// ─── Governor ────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CpuGovernor { Performance, PowerSave, OnDemand, Conservative, Schedutil }

// ─── Battery ─────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BatteryStatus { Discharging, Charging, Full, NotPresent }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BatteryHealth { Good, Warm, Hot, Degraded, Critical }

pub struct Battery {
    pub present:        bool,
    pub capacity_mwh:   u32,    // design capacity
    pub remaining_mwh:  u32,    // remaining
    pub charge_rate_mw: i32,    // positive=charging, negative=discharging
    pub voltage_mv:     u16,
    pub temperature_mc: i32,    // millidegrees C
    pub status:         BatteryStatus,
    pub health:         BatteryHealth,
    pub cycle_count:    u16,
    pub manufacturer:   [u8; 32],
    pub model:          [u8; 32],
}

impl Battery {
    pub const fn new() -> Self {
        Battery {
            present: false, capacity_mwh: 0, remaining_mwh: 0,
            charge_rate_mw: 0, voltage_mv: 0, temperature_mc: 25000,
            status: BatteryStatus::NotPresent, health: BatteryHealth::Good,
            cycle_count: 0, manufacturer: [0u8; 32], model: [0u8; 32],
        }
    }
    pub fn percent(&self) -> u8 {
        if self.capacity_mwh == 0 { return 0; }
        ((self.remaining_mwh as u64 * 100 / self.capacity_mwh as u64) as u8).min(100)
    }
    pub fn time_remaining_min(&self) -> Option<u32> {
        if self.status != BatteryStatus::Discharging { return None; }
        let rate = (-self.charge_rate_mw) as u32;
        if rate == 0 { return None; }
        Some(self.remaining_mwh * 60 / rate)
    }
    pub fn is_low(&self) -> bool { self.percent() < 15 && self.status == BatteryStatus::Discharging }
    pub fn is_critical(&self) -> bool { self.percent() < 5 && self.status == BatteryStatus::Discharging }
    pub fn update_health(&mut self) {
        let tc = self.temperature_mc;
        self.health = if tc > 60000 { BatteryHealth::Critical }
                      else if tc > 45000 { BatteryHealth::Hot }
                      else if tc > 35000 { BatteryHealth::Warm }
                      else if self.cycle_count > 800 { BatteryHealth::Degraded }
                      else { BatteryHealth::Good };
    }
}

// ─── Thermal Zone ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
pub struct ThermalZone {
    pub id:        u8,
    pub name:      [u8; 16],
    pub temp_mc:   i32,       // millidegrees C
    pub trip_warn: i32,       // warning trip point
    pub trip_crit: i32,       // critical trip point (shutdown)
    pub trip_hot:  i32,       // hot trip point (throttle)
    pub cooling:   ThermalCooling,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ThermalCooling { None, FanSpeed, Throttle, Shutdown }

impl ThermalZone {
    pub const fn new(id: u8, warn: i32, hot: i32, crit: i32) -> Self {
        ThermalZone {
            id, name: [0u8; 16], temp_mc: 25000,
            trip_warn: warn, trip_hot: hot, trip_crit: crit,
            cooling: ThermalCooling::None,
        }
    }
    pub fn update_cooling(&mut self) -> ThermalCooling {
        self.cooling = if self.temp_mc >= self.trip_crit { ThermalCooling::Shutdown }
                       else if self.temp_mc >= self.trip_hot { ThermalCooling::Throttle }
                       else if self.temp_mc >= self.trip_warn { ThermalCooling::FanSpeed }
                       else { ThermalCooling::None };
        self.cooling
    }
    pub fn celsius(&self) -> i32 { self.temp_mc / 1000 }
}

// ─── CPU Core Power State ────────────────────────────────────────────────────
pub const MAX_CPUS: usize = 256;

pub struct CpuCorePower {
    pub id:          usize,
    pub online:      AtomicBool,
    pub cur_pstate:  AtomicU32,
    pub cur_cstate:  AtomicU32,
    pub freq_mhz:    AtomicU32,
    pub usage_pct:   AtomicU32,  // 0–100
    pub temp_mc:     AtomicI32,
    pub governor:    CpuGovernor,
    pub pstates:     [PState; MAX_PSTATES],
    pub n_pstates:   usize,
    pub cstates:     [CState; MAX_CSTATES],
    pub n_cstates:   usize,
}

impl CpuCorePower {
    pub fn new(id: usize) -> Self {
        CpuCorePower {
            id, online: AtomicBool::new(true),
            cur_pstate: AtomicU32::new(0), cur_cstate: AtomicU32::new(0),
            freq_mhz: AtomicU32::new(2000), usage_pct: AtomicU32::new(0),
            temp_mc: AtomicI32::new(35000), governor: CpuGovernor::Schedutil,
            pstates: [PState { freq_mhz: 0, voltage_mv: 0, power_mw: 0, latency_us: 0 }; MAX_PSTATES],
            n_pstates: 0,
            cstates: [CState { name: [0u8;8], latency_us: 0, power_mw: 0 }; MAX_CSTATES],
            n_cstates: 0,
        }
    }

    /// Compute new P-state based on governor + load.
    pub fn update_pstate(&self, load_pct: u32) {
        let n = self.n_pstates as u32;
        if n == 0 { return; }
        let new_pstate = match self.governor {
            CpuGovernor::Performance => 0,
            CpuGovernor::PowerSave   => n - 1,
            CpuGovernor::OnDemand    => {
                if load_pct > 80 { 0 }
                else if load_pct > 50 { n / 2 }
                else { n - 1 }
            }
            CpuGovernor::Conservative => {
                let cur = self.cur_pstate.load(Ordering::Relaxed);
                if load_pct > 80 && cur > 0 { cur - 1 }
                else if load_pct < 20 && cur < n - 1 { cur + 1 }
                else { cur }
            }
            CpuGovernor::Schedutil => {
                // Proportional to utilisation
                (n - 1).saturating_sub(load_pct * n / 100)
            }
        };
        self.cur_pstate.store(new_pstate.min(n - 1), Ordering::Relaxed);
        if self.n_pstates > 0 {
            let ps = &self.pstates[new_pstate as usize];
            self.freq_mhz.store(ps.freq_mhz, Ordering::Relaxed);
        }
    }

    pub fn is_online(&self) -> bool { self.online.load(Ordering::Acquire) }
    pub fn hotplug_off(&self) { self.online.store(false, Ordering::Release); }
    pub fn hotplug_on(&self)  { self.online.store(true, Ordering::Release); }
}

// ─── System Power Manager ────────────────────────────────────────────────────
pub const MAX_THERMAL_ZONES: usize = 8;

pub struct PowerManager {
    pub s_state:        AcpiSState,
    pub battery:        Battery,
    pub cpus:           [CpuCorePower; 8],  // up to 8 physical CPUs
    pub n_cpus:         usize,
    pub thermal_zones:  [ThermalZone; MAX_THERMAL_ZONES],
    pub n_zones:        usize,
    pub wake_events:    u32,
    pub suspend_count:  u32,
}

impl PowerManager {
    pub fn new(n_cpus: usize) -> Self {
        let cpus = core::array::from_fn(|i| CpuCorePower::new(i));
        PowerManager {
            s_state: AcpiSState::S0,
            battery: Battery::new(),
            cpus, n_cpus,
            thermal_zones: [ThermalZone::new(0, 75000, 90000, 105000); MAX_THERMAL_ZONES],
            n_zones: 1,
            wake_events: 0, suspend_count: 0,
        }
    }

    pub fn suspend(&mut self, target: AcpiSState) -> bool {
        if target == AcpiSState::S0 { return false; }
        self.s_state = target;
        self.suspend_count += 1;
        // In real impl: flush caches, park APs, call ACPI _PTS
        true
    }

    pub fn resume(&mut self) -> bool {
        self.s_state = AcpiSState::S0;
        self.wake_events += 1;
        true
    }

    pub fn tick_thermal(&mut self) {
        for z in &mut self.thermal_zones[..self.n_zones] {
            z.update_cooling();
        }
    }

    pub fn tick_battery(&mut self) {
        if self.battery.present {
            // Simulate discharge/charge based on status
            match self.battery.status {
                BatteryStatus::Discharging => {
                    let drain = (-self.battery.charge_rate_mw).max(0) as u32;
                    let per_tick = drain / 3600; // assuming 1s tick
                    self.battery.remaining_mwh = self.battery.remaining_mwh.saturating_sub(per_tick);
                }
                BatteryStatus::Charging => {
                    let charge = self.battery.charge_rate_mw.max(0) as u32;
                    let per_tick = charge / 3600;
                    self.battery.remaining_mwh =
                        (self.battery.remaining_mwh + per_tick).min(self.battery.capacity_mwh);
                }
                _ => {}
            }
            self.battery.update_health();
        }
    }

    pub fn tick_dvfs(&mut self, cpu_loads: &[u32]) {
        for (i, &load) in cpu_loads.iter().enumerate() {
            if i < self.n_cpus { self.cpus[i].update_pstate(load); }
        }
    }

    pub fn system_power_mw(&self) -> u32 {
        self.cpus[..self.n_cpus].iter().map(|c| {
            let ps = c.cur_pstate.load(Ordering::Relaxed) as usize;
            if ps < c.n_pstates { c.pstates[ps].power_mw } else { 0 }
        }).sum()
    }

    pub fn all_temps_ok(&self) -> bool {
        self.thermal_zones[..self.n_zones].iter()
            .all(|z| z.cooling != ThermalCooling::Shutdown)
    }
}

// ─── ACPI Table Parsing (minimal RSDT/MADT) ───────────────────────────────────
pub const RSDP_SIG: &[u8; 8] = b"RSD PTR ";
pub const MADT_SIG: &[u8; 4] = b"APIC";
pub const FADT_SIG: &[u8; 4] = b"FACP";

pub fn find_rsdp(scan_start: u64, scan_end: u64) -> Option<u64> {
    let mut addr = scan_start;
    while addr < scan_end {
        let sig = unsafe { core::slice::from_raw_parts(addr as *const u8, 8) };
        if sig == RSDP_SIG {
            // Validate checksum
            let bytes = unsafe { core::slice::from_raw_parts(addr as *const u8, 20) };
            let sum: u8 = bytes.iter().fold(0u8, |a, &b| a.wrapping_add(b));
            if sum == 0 { return Some(addr); }
        }
        addr += 16;
    }
    None
}
