//! SigmaOS Power Profiles Manager
//!
//! Inspired by: Omarchy power profiles (omancy-powerprofiles-init, omancy-powerprofiles-list, omancy-powerprofiles-set)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, ENERGY_EFFICIENCY, POWER_MANAGEMENT, USER_CONTROL.

/// Power profile type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSave,
    ExtremePowerSave,
    Custom(String),
}

/// Power source
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerSource {
    AcAdapter,
    Battery,
    Unknown,
}

/// Power profile configuration
#[derive(Debug, Clone)]
pub struct PowerProfileConfig {
    pub profile: PowerProfile,
    pub cpu_governor: String,
    pub gpu_power_limit: Option<u32>,
    pub disk_spin_down: Option<u64>,
    pub network_powersave: bool,
    pub usb_autosuspend: bool,
    pub brightness_reduce: Option<u32>,
}

/// Power profiles manager
pub struct PowerProfilesManager {
    pub profiles: HashMap<PowerProfile, PowerProfileConfig>,
    pub active_profile: PowerProfile,
    pub power_source: PowerSource,
    pub battery_percent: Option<u32>,
}

impl PowerProfilesManager {
    pub fn new() -> Self {
        let mut profiles = HashMap::new();

        // Performance profile
        profiles.insert(
            PowerProfile::Performance,
            PowerProfileConfig {
                profile: PowerProfile::Performance,
                cpu_governor: "performance".to_string(),
                gpu_power_limit: None,
                disk_spin_down: None,
                network_powersave: false,
                usb_autosuspend: false,
                brightness_reduce: None,
            },
        );

        // Balanced profile
        profiles.insert(
            PowerProfile::Balanced,
            PowerProfileConfig {
                profile: PowerProfile::Balanced,
                cpu_governor: "schedutil".to_string(),
                gpu_power_limit: None,
                disk_spin_down: Some(600),
                network_powersave: true,
                usb_autosuspend: true,
                brightness_reduce: None,
            },
        );

        // Power save profile
        profiles.insert(
            PowerProfile::PowerSave,
            PowerProfileConfig {
                profile: PowerProfile::PowerSave,
                cpu_governor: "powersave".to_string(),
                gpu_power_limit: Some(75),
                disk_spin_down: Some(300),
                network_powersave: true,
                usb_autosuspend: true,
                brightness_reduce: Some(50),
            },
        );

        // Extreme power save profile
        profiles.insert(
            PowerProfile::ExtremePowerSave,
            PowerProfileConfig {
                profile: PowerProfile::ExtremePowerSave,
                cpu_governor: "powersave".to_string(),
                gpu_power_limit: Some(25),
                disk_spin_down: Some(120),
                network_powersave: true,
                usb_autosuspend: true,
                brightness_reduce: Some(25),
            },
        );

        Self {
            profiles,
            active_profile: PowerProfile::Balanced,
            power_source: PowerSource::AcAdapter,
            battery_percent: None,
        }
    }

    /// Set active power profile
    pub fn set_profile(&mut self, profile: PowerProfile) -> Result<(), String> {
        if !self.profiles.contains_key(&profile) {
            return Err(format!("Power profile {:?} not found", profile));
        }
        self.active_profile = profile;
        Ok(())
    }

    /// Get active profile config
    pub fn get_active_config(&self) -> Option<&PowerProfileConfig> {
        self.profiles.get(&self.active_profile)
    }

    /// List all profiles
    pub fn list_profiles(&self) -> Vec<&PowerProfile> {
        self.profiles.keys().collect()
    }

    /// Add custom profile
    pub fn add_custom_profile(&mut self, name: String, config: PowerProfileConfig) {
        self.profiles.insert(PowerProfile::Custom(name), config);
    }

    /// Remove custom profile
    pub fn remove_custom_profile(&mut self, name: &str) -> Result<(), String> {
        match &self.active_profile {
            PowerProfile::Custom(ref active_name) if active_name == name => {
                return Err("Cannot remove active custom profile".to_string());
            }
            _ => {}
        }
        self.profiles
            .iter()
            .find(|(p, _)| matches!(p, PowerProfileCustom(name2) if name2 == name))
            .map(|(p, _)| p.clone())
            .ok_or_else(|| format!("Custom profile {} not found", name))
            .and_then(|p| {
                self.profiles.remove(&p);
                Ok(())
            })
    }

    /// Set power source
    pub fn set_power_source(&mut self, source: PowerSource) {
        self.power_source = source;
    }

    /// Set battery percent
    pub fn set_battery_percent(&mut self, percent: u32) {
        self.battery_percent = Some(percent.min(100));
    }

    /// Get battery percent
    pub fn get_battery_percent(&self) -> Option<u32> {
        self.battery_percent
    }

    /// Auto-switch profile based on power source
    pub fn auto_switch(&mut self) {
        match self.power_source {
            PowerSource::Battery => {
                if let Some(pct) = self.battery_percent {
                    if pct < 20 {
                        let _ = self.set_profile(PowerProfile::ExtremePowerSave);
                    } else if pct < 50 {
                        let _ = self.set_profile(PowerProfile::PowerSave);
                    } else {
                        let _ = self.set_profile(PowerProfile::Balanced);
                    }
                }
            }
            PowerSource::AcAdapter => {
                let _ = self.set_profile(PowerProfile::Performance);
            }
            PowerSource::Unknown => {}
        }
    }

    /// Get profile count
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }
}

impl Default for PowerProfilesManager {
    fn default() -> Self {
        Self::new()
    }
}

fn PowerProfileCustom(name: &str) -> PowerProfile {
    PowerProfile::Custom(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profiles() {
        let mgr = PowerProfilesManager::new();
        assert_eq!(mgr.profile_count(), 4);
        assert_eq!(mgr.active_profile, PowerProfile::Balanced);
    }

    #[test]
    fn test_set_profile() {
        let mut mgr = PowerProfilesManager::new();
        assert!(mgr.set_profile(PowerProfile::PowerSave).is_ok());
        assert_eq!(mgr.active_profile, PowerProfile::PowerSave);
    }

    #[test]
    fn test_auto_switch_battery_low() {
        let mut mgr = PowerProfilesManager::new();
        mgr.set_power_source(PowerSource::Battery);
        mgr.set_battery_percent(10);
        mgr.auto_switch();
        assert_eq!(mgr.active_profile, PowerProfile::ExtremePowerSave);
    }

    #[test]
    fn test_auto_switch_ac() {
        let mut mgr = PowerProfilesManager::new();
        mgr.set_power_source(PowerSource::AcAdapter);
        mgr.auto_switch();
        assert_eq!(mgr.active_profile, PowerProfile::Performance);
    }

    #[test]
    fn test_add_custom_profile() {
        let mut mgr = PowerProfilesManager::new();
        let config = PowerProfileConfig {
            profile: PowerProfile::Custom("eco".to_string()),
            cpu_governor: "powersave".to_string(),
            gpu_power_limit: Some(50),
            disk_spin_down: Some(600),
            network_powersave: true,
            usb_autosuspend: true,
            brightness_reduce: Some(40),
        };
        mgr.add_custom_profile("eco".to_string(), config);
        assert_eq!(mgr.profile_count(), 5);
    }

    #[test]
    fn test_get_active_config() {
        let mgr = PowerProfilesManager::new();
        let config = mgr.get_active_config().unwrap();
        assert_eq!(config.cpu_governor, "schedutil");
    }
}