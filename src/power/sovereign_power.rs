// SigmaOS Sovereign Power & Thermal Governance Engine
// ACPI S0-S5, C0-C6, P-States, TLP/powerd frequency governors, Intel RAPL energy capping,
// thermal throttling curves, and NVMe APST / PCIe ASPM link power management



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiSystemPowerState {
    S0Working,
    S3SuspendToRam,
    S4Hibernation,
    S5SoftOff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuCState {
    C0Active,
    C1Halt,
    C2StopClock,
    C3Sleep,
    C6DeepPowerDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignCpuGovernor {
    Performance,
    Powersave,
    SchedutilAdaptive,
}

#[derive(Debug, Clone)]
pub struct RaplPowerLimits {
    pub package_limit_watts: f32,
    pub dram_limit_watts: f32,
    pub is_enforced: bool,
}

pub struct SovereignPowerThermalGovernance {
    pub current_system_state: AcpiSystemPowerState,
    pub active_governor: SovereignCpuGovernor,
    pub rapl_limits: RaplPowerLimits,
    pub cpu_temperature_celsius: f32,
    pub is_throttled: bool,
    pub nvme_apst_enabled: bool,
    pub pcie_aspm_l1_enabled: bool,
}

impl SovereignPowerThermalGovernance {
    pub fn new() -> Self {
        Self {
            current_system_state: AcpiSystemPowerState::S0Working,
            active_governor: SovereignCpuGovernor::SchedutilAdaptive,
            rapl_limits: RaplPowerLimits {
                package_limit_watts: 45.0,
                dram_limit_watts: 15.0,
                is_enforced: true,
            },
            cpu_temperature_celsius: 42.0,
            is_throttled: false,
            nvme_apst_enabled: true,
            pcie_aspm_l1_enabled: true,
        }
    }

    pub fn set_system_power_state(&mut self, state: AcpiSystemPowerState) -> Result<(), &'static str> {
        self.current_system_state = state;
        match state {
            AcpiSystemPowerState::S3SuspendToRam => {
                self.nvme_apst_enabled = true;
                self.pcie_aspm_l1_enabled = true;
            }
            AcpiSystemPowerState::S0Working => {
                self.is_throttled = false;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn evaluate_thermal_throttling(&mut self, current_temp: f32) -> SovereignCpuGovernor {
        self.cpu_temperature_celsius = current_temp;

        if current_temp >= 90.0 {
            // Severe thermal threshold reached -> force Powersave governor and throttle RAPL
            self.is_throttled = true;
            self.active_governor = SovereignCpuGovernor::Powersave;
            self.rapl_limits.package_limit_watts = 15.0;
        } else if current_temp <= 70.0 && self.is_throttled {
            // Recovered from thermal surge
            self.is_throttled = false;
            self.active_governor = SovereignCpuGovernor::SchedutilAdaptive;
            self.rapl_limits.package_limit_watts = 45.0;
        }

        self.active_governor
    }

    pub fn compute_target_frequency_mhz(&self, current_load_percent: f32, max_freq_mhz: u32) -> u32 {
        match self.active_governor {
            SovereignCpuGovernor::Performance => max_freq_mhz,
            SovereignCpuGovernor::Powersave => (max_freq_mhz as f32 * 0.4) as u32,
            SovereignCpuGovernor::SchedutilAdaptive => {
                let scaled = (max_freq_mhz as f32 * (current_load_percent / 100.0).clamp(0.2, 1.0)) as u32;
                scaled.max(800)
            }
        }
    }
}

impl Default for SovereignPowerThermalGovernance {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// LINUX TLP & POWER-PROFILES-DAEMON ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxPowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

pub struct LinuxTlpPowerProfilesEngine {
    pub active_profile: LinuxPowerProfile,
    pub on_ac_power: bool,
    pub tlp_enabled: bool,
}

impl LinuxTlpPowerProfilesEngine {
    pub fn new() -> Self {
        Self {
            active_profile: LinuxPowerProfile::Balanced,
            on_ac_power: true,
            tlp_enabled: true,
        }
    }

    pub fn set_power_profile(&mut self, profile: LinuxPowerProfile) {
        self.active_profile = profile;
    }

    pub fn handle_ac_event(&mut self, is_ac: bool) -> LinuxPowerProfile {
        self.on_ac_power = is_ac;
        if self.tlp_enabled {
            if is_ac {
                self.active_profile = LinuxPowerProfile::Performance;
            } else {
                self.active_profile = LinuxPowerProfile::PowerSaver;
            }
        }
        self.active_profile
    }
}

impl Default for LinuxTlpPowerProfilesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// FREEBSD POWERD FREQUENCY SCALING ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeBsdPowerdMode {
    Maximum,
    Minimum,
    Adaptive,
    HiAdaptive,
}

pub struct FreeBsdPowerdFreqEngine {
    pub active_mode: FreeBsdPowerdMode,
    pub current_freq_mhz: u32,
    pub max_freq_mhz: u32,
}

impl FreeBsdPowerdFreqEngine {
    pub fn new(max_freq_mhz: u32) -> Self {
        Self {
            active_mode: FreeBsdPowerdMode::HiAdaptive,
            current_freq_mhz: max_freq_mhz,
            max_freq_mhz,
        }
    }

    pub fn evaluate_frequency(&mut self, cpu_load_pct: f32) -> u32 {
        let freq = match self.active_mode {
            FreeBsdPowerdMode::Maximum => self.max_freq_mhz,
            FreeBsdPowerdMode::Minimum => (self.max_freq_mhz as f32 * 0.3) as u32,
            FreeBsdPowerdMode::Adaptive => {
                (self.max_freq_mhz as f32 * (cpu_load_pct / 100.0).clamp(0.3, 1.0)) as u32
            }
            FreeBsdPowerdMode::HiAdaptive => {
                let load_factor = if cpu_load_pct > 50.0 { 1.0 } else { cpu_load_pct / 100.0 };
                (self.max_freq_mhz as f32 * load_factor.clamp(0.4, 1.0)) as u32
            }
        };

        self.current_freq_mhz = freq;
        freq
    }
}

// =========================================================================
// OPENBSD APMD POWER MANAGEMENT ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenBsdApmdMode {
    Manual,
    Auto,
    CoolRun,
}

pub struct OpenBsdApmdPowerManagementEngine {
    pub mode: OpenBsdApmdMode,
    pub battery_percent: u8,
    pub time_remaining_mins: u32,
}

impl OpenBsdApmdPowerManagementEngine {
    pub fn new() -> Self {
        Self {
            mode: OpenBsdApmdMode::Auto,
            battery_percent: 100,
            time_remaining_mins: 240,
        }
    }

    pub fn update_battery_status(&mut self, percent: u8, remaining_mins: u32) {
        self.battery_percent = percent;
        self.time_remaining_mins = remaining_mins;
    }
}

impl Default for OpenBsdApmdPowerManagementEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_state_transitions_and_throttling() {
        let mut gov = SovereignPowerThermalGovernance::new();
        assert_eq!(gov.current_system_state, AcpiSystemPowerState::S0Working);

        // Calculate frequency under normal load
        let freq_normal = gov.compute_target_frequency_mhz(50.0, 4000);
        assert_eq!(freq_normal, 2000);

        // Trigger high thermal event (95°C)
        let gov_state = gov.evaluate_thermal_throttling(95.0);
        assert_eq!(gov_state, SovereignCpuGovernor::Powersave);
        assert!(gov.is_throttled);
        assert_eq!(gov.rapl_limits.package_limit_watts, 15.0);

        // Frequency under throttled powersave
        let freq_throttled = gov.compute_target_frequency_mhz(100.0, 4000);
        assert_eq!(freq_throttled, 1600);

        // Temperature cools down to 60°C
        gov.evaluate_thermal_throttling(60.0);
        assert!(!gov.is_throttled);
        assert_eq!(gov.active_governor, SovereignCpuGovernor::SchedutilAdaptive);
    }

    #[test]
    fn test_linux_tlp_power_profiles() {
        let mut tlp = LinuxTlpPowerProfilesEngine::new();
        assert_eq!(tlp.active_profile, LinuxPowerProfile::Balanced);

        // Switch to battery power
        let battery_prof = tlp.handle_ac_event(false);
        assert_eq!(battery_prof, LinuxPowerProfile::PowerSaver);
        assert!(!tlp.on_ac_power);

        // Switch to AC power
        let ac_prof = tlp.handle_ac_event(true);
        assert_eq!(ac_prof, LinuxPowerProfile::Performance);
        assert!(tlp.on_ac_power);
    }

    #[test]
    fn test_freebsd_powerd_freq_scaling() {
        let mut powerd = FreeBsdPowerdFreqEngine::new(3200);
        assert_eq!(powerd.active_mode, FreeBsdPowerdMode::HiAdaptive);

        let freq_low = powerd.evaluate_frequency(20.0);
        assert_eq!(freq_low, 1280);

        let freq_high = powerd.evaluate_frequency(80.0);
        assert_eq!(freq_high, 3200);
    }

    #[test]
    fn test_openbsd_apmd_power_management() {
        let mut apmd = OpenBsdApmdPowerManagementEngine::new();
        assert_eq!(apmd.mode, OpenBsdApmdMode::Auto);

        apmd.update_battery_status(85, 180);
        assert_eq!(apmd.battery_percent, 85);
        assert_eq!(apmd.time_remaining_mins, 180);
    }
}
