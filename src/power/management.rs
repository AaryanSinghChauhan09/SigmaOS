/// Advanced Power Management Stack for SigmaOS
/// Inspired by Linux Intel/AMD P-State drivers, FreeBSD ACPI powerd, and OpenBSD apmd.

use core::sync::atomic::{AtomicUsize, Ordering};

pub type PowerProfileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance = 0,
    Balanced = 1,
    PowerSaver = 2,
    Custom = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CPUGovernor {
    Performance = 0,
    Ondemand = 1,
    Conservative = 2,
    Powersave = 3,
    Userspace = 4,
    Schedutil = 5,
}

/// Linux-style Energy Performance Preference (EPP) for P-State scaling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EppMode {
    Performance = 0,
    BalancePerformance = 1,
    BalancePower = 2,
    Power = 3,
}

/// FreeBSD-style CPU Sleep C-States
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuCState {
    C0Active = 0,
    C1Halt = 1,
    C2StopGrant = 2,
    C3Sleep = 3,
}

/// ACPI System Sleep States
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiSleepState {
    S0Working = 0,
    S3SuspendToRam = 3,
    S4HibernateToDisk = 4,
    S5SoftOff = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerError {
    Success = 0,
    InvalidProfile = 1,
    GovernorFailed = 2,
    ThermalCritical = 3,
    SleepStateUnsupported = 4,
}

pub trait PowerProfileTrait {
    fn id(&self) -> PowerProfileID;
    fn name(&self) -> &[u8];
    fn profile_type(&self) -> PowerProfile;
    fn cpu_governor(&self) -> CPUGovernor;
    fn epp_mode(&self) -> EppMode;
    fn max_cpu_freq(&self) -> usize;
    fn min_cpu_freq(&self) -> usize;
}

pub struct SimplePowerProfile {
    pub id: PowerProfileID,
    pub name: [u8; 32],
    pub profile_type: AtomicUsize,
    pub cpu_governor: AtomicUsize,
    pub epp_mode: AtomicUsize,
    pub max_cpu_freq: AtomicUsize,
    pub min_cpu_freq: AtomicUsize,
}

impl SimplePowerProfile {
    pub fn new(id: PowerProfileID, name: &[u8], profile_type: PowerProfile, governor: CPUGovernor, epp: EppMode) -> Self {
        let mut name_array = [0u8; 32];
        let name_len = name.len().min(31);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimplePowerProfile {
            id,
            name: name_array,
            profile_type: AtomicUsize::new(profile_type as usize),
            cpu_governor: AtomicUsize::new(governor as usize),
            epp_mode: AtomicUsize::new(epp as usize),
            max_cpu_freq: AtomicUsize::new(3500000),
            min_cpu_freq: AtomicUsize::new(800000),
        }
    }
}

impl PowerProfileTrait for SimplePowerProfile {
    fn id(&self) -> PowerProfileID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }
    fn profile_type(&self) -> PowerProfile {
        match self.profile_type.load(Ordering::SeqCst) {
            0 => PowerProfile::Performance,
            1 => PowerProfile::Balanced,
            2 => PowerProfile::PowerSaver,
            _ => PowerProfile::Custom,
        }
    }
    fn cpu_governor(&self) -> CPUGovernor {
        match self.cpu_governor.load(Ordering::SeqCst) {
            0 => CPUGovernor::Performance,
            1 => CPUGovernor::Ondemand,
            2 => CPUGovernor::Conservative,
            3 => CPUGovernor::Powersave,
            4 => CPUGovernor::Userspace,
            _ => CPUGovernor::Schedutil,
        }
    }
    fn epp_mode(&self) -> EppMode {
        match self.epp_mode.load(Ordering::SeqCst) {
            0 => EppMode::Performance,
            1 => EppMode::BalancePerformance,
            2 => EppMode::BalancePower,
            _ => EppMode::Power,
        }
    }
    fn max_cpu_freq(&self) -> usize {
        self.max_cpu_freq.load(Ordering::SeqCst)
    }
    fn min_cpu_freq(&self) -> usize {
        self.min_cpu_freq.load(Ordering::SeqCst)
    }
}

pub struct SimpleThermalManager {
    pub current_temp: AtomicUsize,
    pub warning_threshold: AtomicUsize,
    pub critical_threshold: AtomicUsize,
    pub fan_speed_percent: AtomicUsize,
}

impl SimpleThermalManager {
    pub fn new() -> Self {
        SimpleThermalManager {
            current_temp: AtomicUsize::new(45),
            warning_threshold: AtomicUsize::new(75),
            critical_threshold: AtomicUsize::new(90),
            fan_speed_percent: AtomicUsize::new(30),
        }
    }

    pub fn update_temperature(&self, temp_celsius: usize) {
        self.current_temp.store(temp_celsius, Ordering::SeqCst);
        let warning = self.warning_threshold.load(Ordering::SeqCst);
        let critical = self.critical_threshold.load(Ordering::SeqCst);

        if temp_celsius >= critical {
            self.fan_speed_percent.store(100, Ordering::SeqCst);
        } else if temp_celsius >= warning {
            self.fan_speed_percent.store(75, Ordering::SeqCst);
        } else {
            self.fan_speed_percent.store(35, Ordering::SeqCst);
        }
    }

    pub fn is_critical(&self) -> bool {
        self.current_temp.load(Ordering::SeqCst) >= self.critical_threshold.load(Ordering::SeqCst)
    }
}

impl Default for SimpleThermalManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced System Power State Controller
pub struct AdvancedPowerManager {
    pub active_profile: PowerProfile,
    pub epp_mode: EppMode,
    pub lowest_cstate: CpuCState,
    pub sleep_state: AcpiSleepState,
    pub thermal: SimpleThermalManager,
}

impl AdvancedPowerManager {
    pub fn new() -> Self {
        Self {
            active_profile: PowerProfile::Balanced,
            epp_mode: EppMode::BalancePerformance,
            lowest_cstate: CpuCState::C0Active,
            sleep_state: AcpiSleepState::S0Working,
            thermal: SimpleThermalManager::new(),
        }
    }

    pub fn set_sleep_state(&mut self, state: AcpiSleepState) -> Result<(), PowerError> {
        self.sleep_state = state;
        match state {
            AcpiSleepState::S0Working => {
                self.lowest_cstate = CpuCState::C0Active;
            }
            AcpiSleepState::S3SuspendToRam => {
                self.lowest_cstate = CpuCState::C3Sleep;
            }
            AcpiSleepState::S4HibernateToDisk | AcpiSleepState::S5SoftOff => {
                self.lowest_cstate = CpuCState::C3Sleep;
            }
        }
        Ok(())
    }

    pub fn apply_profile(&mut self, profile: PowerProfile) {
        self.active_profile = profile;
        match profile {
            PowerProfile::Performance => {
                self.epp_mode = EppMode::Performance;
            }
            PowerProfile::Balanced => {
                self.epp_mode = EppMode::BalancePerformance;
            }
            PowerProfile::PowerSaver => {
                self.epp_mode = EppMode::Power;
            }
            PowerProfile::Custom => {}
        }
    }
}

impl Default for AdvancedPowerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_epp() {
        let profile = SimplePowerProfile::new(
            1,
            b"perf",
            PowerProfile::Performance,
            CPUGovernor::Performance,
            EppMode::Performance,
        );

        assert_eq!(profile.id(), 1);
        assert_eq!(profile.epp_mode(), EppMode::Performance);
        assert_eq!(profile.cpu_governor(), CPUGovernor::Performance);
    }

    #[test]
    fn test_advanced_power_manager_transitions() {
        let mut mgr = AdvancedPowerManager::new();
        assert_eq!(mgr.sleep_state, AcpiSleepState::S0Working);

        assert!(mgr.set_sleep_state(AcpiSleepState::S3SuspendToRam).is_ok());
        assert_eq!(mgr.sleep_state, AcpiSleepState::S3SuspendToRam);
        assert_eq!(mgr.lowest_cstate, CpuCState::C3Sleep);

        mgr.apply_profile(PowerProfile::PowerSaver);
        assert_eq!(mgr.epp_mode, EppMode::Power);
    }

    #[test]
    fn test_thermal_fan_control() {
        let thermal = SimpleThermalManager::new();
        assert_eq!(thermal.fan_speed_percent.load(Ordering::SeqCst), 30);

        // Update temp to warning level (80C)
        thermal.update_temperature(80);
        assert_eq!(thermal.fan_speed_percent.load(Ordering::SeqCst), 75);

        // Update temp to critical level (92C)
        thermal.update_temperature(92);
        assert!(thermal.is_critical());
        assert_eq!(thermal.fan_speed_percent.load(Ordering::SeqCst), 100);
    }
}
