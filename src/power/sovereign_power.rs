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

/// Linux TLP & power-profiles-daemon Power Profile Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxPowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

#[derive(Debug, Clone)]
pub struct LinuxTlpPowerProfilesEngine {
    pub active_profile: LinuxPowerProfile,
    pub on_ac_power: bool,
    pub cpu_energy_perf_policy: String,
}

impl LinuxTlpPowerProfilesEngine {
    pub fn new() -> Self {
        Self {
            active_profile: LinuxPowerProfile::Balanced,
            on_ac_power: true,
            cpu_energy_perf_policy: String::from("balance_performance"),
        }
    }

    pub fn set_power_profile(&mut self, profile: LinuxPowerProfile) -> String {
        self.active_profile = profile;
        match profile {
            LinuxPowerProfile::Performance => {
                self.cpu_energy_perf_policy = String::from("performance");
            }
            LinuxPowerProfile::Balanced => {
                self.cpu_energy_perf_policy = String::from("balance_performance");
            }
            LinuxPowerProfile::PowerSaver => {
                self.cpu_energy_perf_policy = String::from("power");
            }
        }
        format!("Set Linux power profile to {:?} ({})", self.active_profile, self.cpu_energy_perf_policy)
    }

    pub fn handle_ac_adapter_event(&mut self, is_ac_plugged: bool) -> LinuxPowerProfile {
        self.on_ac_power = is_ac_plugged;
        if is_ac_plugged {
            self.set_power_profile(LinuxPowerProfile::Performance);
        } else {
            self.set_power_profile(LinuxPowerProfile::PowerSaver);
        }
        self.active_profile
    }
}

impl Default for LinuxTlpPowerProfilesEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// FreeBSD powerd(8) Frequency Scaling Mode Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeBsdPowerdMode {
    Adaptive,
    Min,
    Max,
    Hiadaptive,
}

#[derive(Debug, Clone)]
pub struct FreeBsdPowerdFreqEngine {
    pub active_mode: FreeBsdPowerdMode,
    pub sysctl_freq_levels: Vec<u32>,
    pub current_freq_mhz: u32,
}

impl FreeBsdPowerdFreqEngine {
    pub fn new() -> Self {
        Self {
            active_mode: FreeBsdPowerdMode::Hiadaptive,
            sysctl_freq_levels: vec![3600, 3200, 2800, 2400, 2000, 1600, 1200, 800],
            current_freq_mhz: 3600,
        }
    }

    pub fn calculate_powerd_frequency(&mut self, cpu_load_percent: f32) -> u32 {
        match self.active_mode {
            FreeBsdPowerdMode::Max => self.current_freq_mhz = 3600,
            FreeBsdPowerdMode::Min => self.current_freq_mhz = 800,
            FreeBsdPowerdMode::Adaptive => {
                if cpu_load_percent > 60.0 {
                    self.current_freq_mhz = 3200;
                } else {
                    self.current_freq_mhz = 1600;
                }
            }
            FreeBsdPowerdMode::Hiadaptive => {
                if cpu_load_percent > 40.0 {
                    self.current_freq_mhz = 3600;
                } else {
                    self.current_freq_mhz = 2000;
                }
            }
        }
        self.current_freq_mhz
    }
}

impl Default for FreeBsdPowerdFreqEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// OpenBSD apmd(8) Advanced Power Management Engine
#[derive(Debug, Clone)]
pub struct OpenBsdApmdPowerManagementEngine {
    pub battery_percent: u8,
    pub is_charging: bool,
    pub apm_auto_mode: bool,
}

impl OpenBsdApmdPowerManagementEngine {
    pub fn new() -> Self {
        Self {
            battery_percent: 85,
            is_charging: true,
            apm_auto_mode: true,
        }
    }

    pub fn query_apm_battery_status(&self) -> String {
        let state = if self.is_charging { "charging" } else { "discharging" };
        format!("OpenBSD APM: {}% ({})", self.battery_percent, state)
    }

    pub fn trigger_apm_suspend(&self) -> bool {
        self.battery_percent < 5 && !self.is_charging
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
    fn test_linux_tlp_power_profiles_engine() {
        let mut tlp = LinuxTlpPowerProfilesEngine::new();
        assert_eq!(tlp.active_profile, LinuxPowerProfile::Balanced);

        let res = tlp.set_power_profile(LinuxPowerProfile::Performance);
        assert!(res.contains("Performance"));
        assert_eq!(tlp.cpu_energy_perf_policy, "performance");

        let event = tlp.handle_ac_adapter_event(false);
        assert_eq!(event, LinuxPowerProfile::PowerSaver);
        assert!(!tlp.on_ac_power);
    }

    #[test]
    fn test_freebsd_powerd_freq_engine() {
        let mut powerd = FreeBsdPowerdFreqEngine::new();
        assert_eq!(powerd.calculate_powerd_frequency(80.0), 3600);

        powerd.active_mode = FreeBsdPowerdMode::Min;
        assert_eq!(powerd.calculate_powerd_frequency(80.0), 800);
    }

    #[test]
    fn test_openbsd_apmd_engine() {
        let mut apm = OpenBsdApmdPowerManagementEngine::new();
        assert!(apm.query_apm_battery_status().contains("85%"));
        assert!(!apm.trigger_apm_suspend());

        apm.battery_percent = 3;
        apm.is_charging = false;
        assert!(apm.trigger_apm_suspend());
    }
}
