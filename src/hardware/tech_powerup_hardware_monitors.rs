// SigmaOS Tech PowerUp & Hardware Busters Hardware Monitors
// Inspired by TechPowerUp, Hardware Busters, PCWorld, and Geeky Gadgets

use crate::klib::string::String;

/// TechPowerUp GPU VBIOS profiler and clock/voltage monitoring engine.
#[derive(Debug, Clone)]
pub struct TechPowerUpGpuProfilerEngine {
    pub gpu_name: String,
    pub core_clock_mhz: u32,
    pub memory_clock_mhz: u32,
    pub fan_speed_percent: u8,
    pub vbios_version: String,
}

impl TechPowerUpGpuProfilerEngine {
    pub fn new() -> Self {
        Self {
            gpu_name: String::from("Sovereign GPU Accelerator v1"),
            core_clock_mhz: 2500,
            memory_clock_mhz: 10000,
            fan_speed_percent: 45,
            vbios_version: String::from("90.02.0B.00.01"),
        }
    }

    pub fn is_optimal_performance(&self) -> bool {
        self.core_clock_mhz >= 2000 && self.fan_speed_percent <= 80
    }
}

impl Default for TechPowerUpGpuProfilerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware Busters power supply rail transient spike monitor engine.
#[derive(Debug, Clone)]
pub struct HardwareBustersPsuRailMonitorEngine {
    pub rail_12v_volts: f32,
    pub rail_5v_volts: f32,
    pub rail_3v3_volts: f32,
    pub transient_spike_detected: bool,
}

impl HardwareBustersPsuRailMonitorEngine {
    pub fn new() -> Self {
        Self {
            rail_12v_volts: 12.05,
            rail_5v_volts: 5.01,
            rail_3v3_volts: 3.31,
            transient_spike_detected: false,
        }
    }

    pub fn verify_power_stability(&self) -> bool {
        (self.rail_12v_volts - 12.0).abs() < 0.6
            && (self.rail_5v_volts - 5.0).abs() < 0.25
            && (self.rail_3v3_volts - 3.3).abs() < 0.16
            && !self.transient_spike_detected
    }
}

impl Default for HardwareBustersPsuRailMonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// PCWorld laptop battery health & threshold charging controller.
#[derive(Debug, Clone)]
pub struct PcWorldBatteryHealthControllerEngine {
    pub health_percentage: u8,
    pub charge_threshold_stop_percent: u8,
    pub cycle_count: u32,
}

impl PcWorldBatteryHealthControllerEngine {
    pub fn new() -> Self {
        Self {
            health_percentage: 98,
            charge_threshold_stop_percent: 80,
            cycle_count: 42,
        }
    }

    pub fn should_stop_charging(&self, current_level: u8) -> bool {
        current_level >= self.charge_threshold_stop_percent
    }
}

impl Default for PcWorldBatteryHealthControllerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Master coordinator for Tech PowerUp Hardware Monitors.
#[derive(Debug, Clone)]
pub struct SovereignTechPowerUpHardwareMonitorsSuite {
    pub gpu_profiler: TechPowerUpGpuProfilerEngine,
    pub psu_monitor: HardwareBustersPsuRailMonitorEngine,
    pub battery_controller: PcWorldBatteryHealthControllerEngine,
}

impl SovereignTechPowerUpHardwareMonitorsSuite {
    pub fn new() -> Self {
        Self {
            gpu_profiler: TechPowerUpGpuProfilerEngine::new(),
            psu_monitor: HardwareBustersPsuRailMonitorEngine::new(),
            battery_controller: PcWorldBatteryHealthControllerEngine::new(),
        }
    }

    pub fn verify_suite(&self) -> bool {
        self.gpu_profiler.is_optimal_performance() && self.psu_monitor.verify_power_stability()
    }
}

impl Default for SovereignTechPowerUpHardwareMonitorsSuite {
    fn default() -> Self {
        Self::new()
    }
}
