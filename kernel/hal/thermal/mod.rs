// =============================================================================
// SIGMAOS: THERMAL & POWER HAL DAEMON (sigma-thermal)
// =============================================================================
// Sovereign thermal management and power optimization daemon.
// Provides: CPU/GPU thermal zones, fan control, DVFS (Dynamic Voltage
// Frequency Scaling), power profiles, battery management, and thermal
// throttling with configurable trip points.
// =============================================================================

#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Temperature is stored in milli-Celsius (°C × 1000) for precision.
pub const TEMP_UNIT: i64 = 1000;

pub const THERMAL_ZONE_CPU: u8 = 0;
pub const THERMAL_ZONE_GPU: u8 = 1;
pub const THERMAL_ZONE_NVME: u8 = 2;
pub const THERMAL_ZONE_BATTERY: u8 = 3;
pub const THERMAL_ZONE_AMBIENT: u8 = 4;

// ---------------------------------------------------------------------------
// Power Profiles
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum PowerProfile {
    /// Minimum energy: aggressive idle, low clocks.
    PowerSaver,
    /// Balanced performance/efficiency.
    Balanced,
    /// Maximum throughput: boost clocks, fans at full.
    Performance,
    /// Quiet operation: fan suppressed, moderate clocks.
    Silent,
    /// Custom profile with explicit parameters.
    Custom {
        max_cpu_freq_mhz: u32,
        min_cpu_freq_mhz: u32,
        fan_target_rpm: Option<u32>,
        tdp_limit_mw: u32,
    },
}

impl PowerProfile {
    /// Returns the maximum CPU frequency in MHz for this profile.
    pub fn max_cpu_freq_mhz(&self) -> u32 {
        match self {
            PowerProfile::PowerSaver => 800,
            PowerProfile::Balanced   => 2400,
            PowerProfile::Performance=> 5200,
            PowerProfile::Silent     => 1800,
            PowerProfile::Custom { max_cpu_freq_mhz, .. } => *max_cpu_freq_mhz,
        }
    }

    /// Returns the TDP limit in milliwatts.
    pub fn tdp_limit_mw(&self) -> u32 {
        match self {
            PowerProfile::PowerSaver => 15_000,
            PowerProfile::Balanced   => 45_000,
            PowerProfile::Performance=> 125_000,
            PowerProfile::Silent     => 25_000,
            PowerProfile::Custom { tdp_limit_mw, .. } => *tdp_limit_mw,
        }
    }
}

// ---------------------------------------------------------------------------
// Thermal Zone
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThermalSeverity {
    Normal,
    Warning,    // approaching throttle threshold
    Critical,   // active throttling
    Emergency,  // immediate shutdown to prevent damage
}

/// A single thermal trip point (temperature threshold + action).
#[derive(Debug, Clone)]
pub struct TripPoint {
    /// Temperature in milli-Celsius at which this trip fires.
    pub temp_mc: i64,
    pub severity: ThermalSeverity,
    /// Action label (for logging/policy hooks).
    pub action: String,
}

/// One thermal zone (CPU, GPU, NVMe, Battery, etc.)
pub struct ThermalZone {
    pub id:           u8,
    pub name:         String,
    pub current_temp_mc: i64,
    pub trip_points:  Vec<TripPoint>,
    /// Last active trip severity.
    pub severity:     ThermalSeverity,
    /// Cumulative throttle events.
    pub throttle_events: u64,
}

impl ThermalZone {
    pub fn new(id: u8, name: &str) -> Self {
        ThermalZone {
            id,
            name: String::from(name),
            current_temp_mc: 35_000,  // 35 °C initial
            trip_points: Vec::new(),
            severity: ThermalSeverity::Normal,
            throttle_events: 0,
        }
    }

    /// Add a trip point.
    pub fn add_trip(&mut self, temp_celsius: i64, severity: ThermalSeverity, action: &str) {
        self.trip_points.push(TripPoint {
            temp_mc: temp_celsius * TEMP_UNIT,
            severity,
            action: String::from(action),
        });
        // Keep sorted ascending.
        self.trip_points.sort_by_key(|t| t.temp_mc);
    }

    /// Update temperature reading (in milli-Celsius) and evaluate trip points.
    pub fn update_temp(&mut self, temp_mc: i64) -> &ThermalSeverity {
        self.current_temp_mc = temp_mc;
        self.severity = ThermalSeverity::Normal;
        for tp in &self.trip_points {
            if temp_mc >= tp.temp_mc {
                self.severity = tp.severity.clone();
            }
        }
        if self.severity == ThermalSeverity::Critical || self.severity == ThermalSeverity::Emergency {
            self.throttle_events += 1;
        }
        &self.severity
    }

    pub fn temp_celsius(&self) -> f32 {
        self.current_temp_mc as f32 / TEMP_UNIT as f32
    }
}

// ---------------------------------------------------------------------------
// Fan Controller
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum FanMode {
    Auto,
    Manual(u8),   // 0–100% duty cycle
    Off,
}

pub struct FanController {
    pub zone_id:     u8,
    pub mode:        FanMode,
    pub current_rpm: u32,
    pub max_rpm:     u32,
    pub min_rpm:     u32,
}

impl FanController {
    pub fn new(zone_id: u8, max_rpm: u32) -> Self {
        FanController {
            zone_id,
            mode: FanMode::Auto,
            current_rpm: 0,
            max_rpm,
            min_rpm: 500,
        }
    }

    /// Compute target RPM from a temperature reading (auto mode: PID-like curve).
    pub fn compute_rpm_auto(&self, temp_celsius: f32) -> u32 {
        // Linear curve: idle below 40 °C, max at 90 °C.
        let t_min = 40.0f32;
        let t_max = 90.0f32;
        if temp_celsius <= t_min {
            return 0;
        }
        if temp_celsius >= t_max {
            return self.max_rpm;
        }
        let ratio = (temp_celsius - t_min) / (t_max - t_min);
        (self.min_rpm as f32 + ratio * (self.max_rpm - self.min_rpm) as f32) as u32
    }

    pub fn set_rpm(&mut self, temp_celsius: f32) {
        self.current_rpm = match &self.mode {
            FanMode::Auto           => self.compute_rpm_auto(temp_celsius),
            FanMode::Manual(duty)   => (*duty as u32 * self.max_rpm) / 100,
            FanMode::Off            => 0,
        };
    }
}

// ---------------------------------------------------------------------------
// DVFS Controller (Dynamic Voltage & Frequency Scaling)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct DvfsPoint {
    pub freq_mhz: u32,
    pub voltage_mv: u32,
    pub power_mw: u32,
}

pub struct DvfsController {
    pub current_freq_mhz: u32,
    pub current_voltage_mv: u32,
    pub op_points: Vec<DvfsPoint>,
}

impl DvfsController {
    pub fn new(op_points: Vec<DvfsPoint>) -> Self {
        let freq = op_points.first().map(|p| p.freq_mhz).unwrap_or(800);
        let volt = op_points.first().map(|p| p.voltage_mv).unwrap_or(700);
        DvfsController { current_freq_mhz: freq, current_voltage_mv: volt, op_points }
    }

    /// Select the highest frequency at or below the power/temp ceiling.
    pub fn apply_ceiling(&mut self, max_freq_mhz: u32, tdp_limit_mw: u32) {
        for point in self.op_points.iter().rev() {
            if point.freq_mhz <= max_freq_mhz && point.power_mw <= tdp_limit_mw {
                self.current_freq_mhz   = point.freq_mhz;
                self.current_voltage_mv = point.voltage_mv;
                return;
            }
        }
        // Fallback: minimum
        if let Some(p) = self.op_points.first() {
            self.current_freq_mhz   = p.freq_mhz;
            self.current_voltage_mv = p.voltage_mv;
        }
    }
}

// ---------------------------------------------------------------------------
// Battery Manager
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum BatteryStatus {
    Discharging,
    Charging,
    Full,
    Unknown,
}

pub struct BatteryManager {
    pub capacity_percent: u8,
    pub status: BatteryStatus,
    pub voltage_mv: u32,
    pub current_ma: i32,   // positive = charging, negative = discharging
    pub temperature_mc: i64,
    pub cycle_count: u32,
}

impl BatteryManager {
    pub fn new() -> Self {
        BatteryManager {
            capacity_percent: 100,
            status: BatteryStatus::Full,
            voltage_mv: 12600,
            current_ma: 0,
            temperature_mc: 30_000,
            cycle_count: 0,
        }
    }

    pub fn update(&mut self, capacity: u8, voltage_mv: u32, current_ma: i32, temp_mc: i64) {
        self.capacity_percent = capacity;
        self.voltage_mv = voltage_mv;
        self.current_ma = current_ma;
        self.temperature_mc = temp_mc;
        self.status = if current_ma > 500 {
            BatteryStatus::Charging
        } else if current_ma < -100 {
            BatteryStatus::Discharging
        } else if capacity >= 99 {
            BatteryStatus::Full
        } else {
            BatteryStatus::Unknown
        };
    }

    /// Is the battery in a thermal danger state?
    pub fn is_thermal_unsafe(&self) -> bool {
        self.temperature_mc > 45_000  // > 45 °C
    }

    /// Should we enable battery-saver mode?
    pub fn should_power_save(&self) -> bool {
        self.capacity_percent < 20 && self.status == BatteryStatus::Discharging
    }
}

impl Default for BatteryManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Thermal HAL Daemon
// ---------------------------------------------------------------------------

pub struct ThermalDaemon {
    pub zones:   Vec<ThermalZone>,
    pub fans:    Vec<FanController>,
    pub dvfs:    DvfsController,
    pub battery: BatteryManager,
    pub profile: PowerProfile,
    /// Tick counter (each tick = one polling interval, typically 500 ms).
    pub tick: u64,
}

impl ThermalDaemon {
    pub fn new() -> Self {
        let op_points = alloc::vec![
            DvfsPoint { freq_mhz:  800, voltage_mv: 700, power_mw: 5_000  },
            DvfsPoint { freq_mhz: 1200, voltage_mv: 800, power_mw: 10_000 },
            DvfsPoint { freq_mhz: 2400, voltage_mv: 950, power_mw: 28_000 },
            DvfsPoint { freq_mhz: 3600, voltage_mv:1050, power_mw: 55_000 },
            DvfsPoint { freq_mhz: 4800, voltage_mv:1150, power_mw: 95_000 },
            DvfsPoint { freq_mhz: 5200, voltage_mv:1250, power_mw:125_000 },
        ];
        ThermalDaemon {
            zones: Vec::new(),
            fans: Vec::new(),
            dvfs: DvfsController::new(op_points),
            battery: BatteryManager::new(),
            profile: PowerProfile::Balanced,
            tick: 0,
        }
    }

    /// Register built-in zones and fans for a typical laptop configuration.
    pub fn register_laptop_config(&mut self) {
        let mut cpu = ThermalZone::new(THERMAL_ZONE_CPU, "CPU");
        cpu.add_trip(70, ThermalSeverity::Warning,   "cpu_throttle_warn");
        cpu.add_trip(85, ThermalSeverity::Critical,  "cpu_throttle_active");
        cpu.add_trip(100,ThermalSeverity::Emergency, "cpu_emergency_shutdown");
        self.zones.push(cpu);

        let mut gpu = ThermalZone::new(THERMAL_ZONE_GPU, "GPU");
        gpu.add_trip(80, ThermalSeverity::Warning,   "gpu_throttle_warn");
        gpu.add_trip(95, ThermalSeverity::Critical,  "gpu_throttle_active");
        gpu.add_trip(105,ThermalSeverity::Emergency, "gpu_emergency_shutdown");
        self.zones.push(gpu);

        let mut bat = ThermalZone::new(THERMAL_ZONE_BATTERY, "Battery");
        bat.add_trip(40, ThermalSeverity::Warning,   "battery_temp_warn");
        bat.add_trip(45, ThermalSeverity::Critical,  "battery_charge_inhibit");
        bat.add_trip(60, ThermalSeverity::Emergency, "battery_emergency_cutoff");
        self.zones.push(bat);

        self.fans.push(FanController::new(THERMAL_ZONE_CPU, 5000));
        self.fans.push(FanController::new(THERMAL_ZONE_GPU, 4000));
    }

    /// Main polling tick. Call once per interval with fresh sensor readings.
    /// `temps_mc`: indexed by zone ID, in milli-Celsius.
    pub fn tick(&mut self, temps_mc: &[i64]) {
        self.tick += 1;

        // Update all zones.
        let mut worst = ThermalSeverity::Normal;
        for zone in &mut self.zones {
            if let Some(&t) = temps_mc.get(zone.id as usize) {
                let sev = zone.update_temp(t).clone();
                if sev > worst { worst = sev; }
            }
        }

        // Apply DVFS based on thermal severity and power profile.
        let (max_freq, tdp) = match worst {
            ThermalSeverity::Normal    => (self.profile.max_cpu_freq_mhz(), self.profile.tdp_limit_mw()),
            ThermalSeverity::Warning   => (self.profile.max_cpu_freq_mhz() * 80 / 100, self.profile.tdp_limit_mw() * 80 / 100),
            ThermalSeverity::Critical  => (2400, 45_000),
            ThermalSeverity::Emergency => (800, 10_000),
        };
        self.dvfs.apply_ceiling(max_freq, tdp);

        // Update fan speeds.
        let cpu_temp = self.zones.iter()
            .find(|z| z.id == THERMAL_ZONE_CPU)
            .map(|z| z.temp_celsius())
            .unwrap_or(35.0);
        for fan in &mut self.fans {
            fan.set_rpm(cpu_temp);
        }

        // Battery-triggered power save.
        if self.battery.should_power_save() {
            self.dvfs.apply_ceiling(1200, 15_000);
        }
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.profile = profile;
    }

    pub fn current_freq_mhz(&self) -> u32 {
        self.dvfs.current_freq_mhz
    }
}

impl Default for ThermalDaemon {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone_trip_normal() {
        let mut zone = ThermalZone::new(0, "CPU");
        zone.add_trip(70, ThermalSeverity::Warning,  "warn");
        zone.add_trip(85, ThermalSeverity::Critical, "crit");
        let sev = zone.update_temp(60_000);
        assert_eq!(*sev, ThermalSeverity::Normal);
    }

    #[test]
    fn test_zone_trip_critical() {
        let mut zone = ThermalZone::new(0, "CPU");
        zone.add_trip(70, ThermalSeverity::Warning,  "warn");
        zone.add_trip(85, ThermalSeverity::Critical, "crit");
        zone.update_temp(86_000);
        assert_eq!(zone.severity, ThermalSeverity::Critical);
        assert_eq!(zone.throttle_events, 1);
    }

    #[test]
    fn test_dvfs_ceiling() {
        let op_points = alloc::vec![
            DvfsPoint { freq_mhz: 800,  voltage_mv: 700, power_mw:  5_000 },
            DvfsPoint { freq_mhz: 2400, voltage_mv: 950, power_mw: 28_000 },
            DvfsPoint { freq_mhz: 4800, voltage_mv:1150, power_mw: 95_000 },
        ];
        let mut dvfs = DvfsController::new(op_points);
        dvfs.apply_ceiling(2400, 30_000);
        assert_eq!(dvfs.current_freq_mhz, 2400);
        dvfs.apply_ceiling(1000, 30_000);
        assert_eq!(dvfs.current_freq_mhz, 800);
    }

    #[test]
    fn test_fan_auto_curve() {
        let fan = FanController::new(0, 5000);
        assert_eq!(fan.compute_rpm_auto(35.0), 0);
        assert_eq!(fan.compute_rpm_auto(90.0), 5000);
        let mid = fan.compute_rpm_auto(65.0);
        assert!(mid > 0 && mid < 5000);
    }

    #[test]
    fn test_power_profiles() {
        assert!(PowerProfile::Performance.max_cpu_freq_mhz() > PowerProfile::PowerSaver.max_cpu_freq_mhz());
        assert!(PowerProfile::Performance.tdp_limit_mw() > PowerProfile::Balanced.tdp_limit_mw());
    }

    #[test]
    fn test_battery_status() {
        let mut bm = BatteryManager::new();
        bm.update(15, 11000, -2000, 32_000);
        assert_eq!(bm.status, BatteryStatus::Discharging);
        assert!(bm.should_power_save());
    }

    #[test]
    fn test_daemon_tick() {
        let mut daemon = ThermalDaemon::new();
        daemon.register_laptop_config();
        daemon.set_profile(PowerProfile::Balanced);
        // Normal temps
        let temps = [45_000i64, 50_000, 35_000, 30_000, 25_000];
        daemon.tick(&temps);
        assert_eq!(daemon.tick, 1);
        assert_eq!(daemon.current_freq_mhz(), 2400);
        // Critical CPU temp
        let temps2 = [90_000i64, 50_000, 35_000, 30_000, 25_000];
        daemon.tick(&temps2);
        assert!(daemon.current_freq_mhz() <= 2400);
    }
}
