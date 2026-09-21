// SPDX-License-Identifier: MIT
// SigmaOS Omarchy System Sleep, Power Profiles & Hibernation Engine (`src/power/omarchy_power.rs`)
// Inspired by Omarchy Linux (AC vs. battery profile memory, omarchy powerprofiles, omarchy toggle suspend,
// omarchy hibernation setup/remove, Btrfs /swap allocation, Limine bootloader resume cmdline parameters).

use std::format;
use std::string::{String, ToString};

/// Power Profile Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl PowerProfile {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "performance" => Some(Self::Performance),
            "balanced" => Some(Self::Balanced),
            "power-saver" | "powersaver" => Some(Self::PowerSaver),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Balanced => "balanced",
            Self::PowerSaver => "power-saver",
        }
    }
}

/// Power Source Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    AcPlugged,
    Battery,
}

/// Btrfs Swap Subvolume & Hibernation Config
#[derive(Debug, Clone)]
pub struct HibernationSubvolumeConfig {
    pub is_enabled: bool,
    pub swap_subvolume_path: String,
    pub ram_size_gb: u32,
    pub swap_file_size_gb: u32,
    pub root_device_uuid: String,
    pub swap_file_offset: u64,
}

/// Omarchy Power Profiles, Suspend & Hibernation Subsystem Engine
pub struct OmarchyPowerEngine {
    pub current_power_source: PowerSource,
    pub ac_profile: PowerProfile,
    pub battery_profile: PowerProfile,
    pub is_suspend_ui_visible: bool,
    pub is_hibernate_ui_visible: bool,
    pub hibernation_config: HibernationSubvolumeConfig,
}

impl OmarchyPowerEngine {
    pub fn new() -> Self {
        Self {
            current_power_source: PowerSource::AcPlugged,
            ac_profile: PowerProfile::Performance,
            battery_profile: PowerProfile::Balanced,
            is_suspend_ui_visible: true,
            is_hibernate_ui_visible: false,
            hibernation_config: HibernationSubvolumeConfig {
                is_enabled: false,
                swap_subvolume_path: String::from("/swap/swapfile"),
                ram_size_gb: 32,
                swap_file_size_gb: 32,
                root_device_uuid: String::from("a1b2c3d4-e5f6-7890-1234-56789abcdef0"),
                swap_file_offset: 34816,
            },
        }
    }

    /// Get current active power profile based on charger plug status
    pub fn get_active_profile(&self) -> PowerProfile {
        match self.current_power_source {
            PowerSource::AcPlugged => self.ac_profile,
            PowerSource::Battery => self.battery_profile,
        }
    }

    /// Update power source (e.g. charger plugged/unplugged event)
    pub fn set_power_source(&mut self, source: PowerSource) -> PowerProfile {
        self.current_power_source = source;
        self.get_active_profile()
    }

    /// Execute `omarchy powerprofiles` CLI command dispatcher
    pub fn execute_powerprofiles_command(&mut self, args: &[&str]) -> Result<String, &'static str> {
        if args.is_empty() || args[0] == "list" {
            return Ok(format!(
                "Omarchy Power Profiles:\n  Current State: {:?} -> Active Profile: '{}'\n  AC Power Profile: '{}'\n  Battery Power Profile: '{}'\n  Available: performance, balanced, power-saver",
                self.current_power_source,
                self.get_active_profile().as_str(),
                self.ac_profile.as_str(),
                self.battery_profile.as_str()
            ));
        }

        if args[0] == "set" {
            if args.len() < 3 {
                return Err("Usage: omarchy powerprofiles set [autodetect|ac|battery] [profile]");
            }
            let target_state = args[1];
            let profile_str = args[2];
            let new_profile = PowerProfile::from_str(profile_str).ok_or("Invalid profile. Choose performance, balanced, or power-saver")?;

            match target_state {
                "autodetect" => match self.current_power_source {
                    PowerSource::AcPlugged => self.ac_profile = new_profile,
                    PowerSource::Battery => self.battery_profile = new_profile,
                },
                "ac" => self.ac_profile = new_profile,
                "battery" => self.battery_profile = new_profile,
                _ => return Err("Target state must be autodetect, ac, or battery"),
            }

            return Ok(format!(
                "Updated power profile for target '{}' to '{}'.",
                target_state, new_profile.as_str()
            ));
        }

        Err("Unknown powerprofiles subcommand. Use list or set.")
    }

    /// Toggle Suspend option under System menu (`Super + Esc`)
    pub fn toggle_suspend(&mut self) -> bool {
        self.is_suspend_ui_visible = !self.is_suspend_ui_visible;
        self.is_suspend_ui_visible
    }

    /// Execute `omarchy hibernation setup` / `remove`
    pub fn execute_hibernation_command(&mut self, action: &str, ram_gb: u32) -> Result<String, &'static str> {
        match action {
            "setup" => {
                self.hibernation_config.is_enabled = true;
                self.hibernation_config.ram_size_gb = ram_gb;
                self.hibernation_config.swap_file_size_gb = ram_gb; // 1:1 RAM sizing
                self.is_hibernate_ui_visible = true;

                Ok(format!(
                    "Hibernation Setup Complete:\n  Allocated Btrfs /swap subvolume ({}GB)\n  Limine cmdline: resume=UUID={} resume_offset={}\n  Hibernate option revealed under System menu (Super + Esc).",
                    ram_gb,
                    self.hibernation_config.root_device_uuid,
                    self.hibernation_config.swap_file_offset
                ))
            }
            "remove" => {
                self.hibernation_config.is_enabled = false;
                self.is_hibernate_ui_visible = false;

                Ok("Hibernation Removed: /swap subvolume deleted and Limine resume parameters cleared.".to_string())
            }
            _ => Err("Hibernation action must be 'setup' or 'remove'"),
        }
    }

    /// Generate Limine bootloader kernel command line resume parameters
    pub fn generate_limine_resume_cmdline(&self) -> Option<String> {
        if self.hibernation_config.is_enabled {
            Some(format!(
                "resume=UUID={} resume_offset={}",
                self.hibernation_config.root_device_uuid,
                self.hibernation_config.swap_file_offset
            ))
        } else {
            None
        }
    }
}

impl Default for OmarchyPowerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_source_and_profile_memory() {
        let mut engine = OmarchyPowerEngine::new();
        assert_eq!(engine.get_active_profile(), PowerProfile::Performance);

        // Switch to battery
        assert_eq!(engine.set_power_source(PowerSource::Battery), PowerProfile::Balanced);

        // Set battery profile explicitly to power-saver
        assert!(engine.execute_powerprofiles_command(&["set", "battery", "power-saver"]).is_ok());
        assert_eq!(engine.get_active_profile(), PowerProfile::PowerSaver);

        // Switch back to AC -> AC profile remains Performance
        assert_eq!(engine.set_power_source(PowerSource::AcPlugged), PowerProfile::Performance);
    }

    #[test]
    fn test_toggle_suspend_visibility() {
        let mut engine = OmarchyPowerEngine::new();
        assert!(engine.is_suspend_ui_visible);

        assert!(!engine.toggle_suspend());
        assert!(engine.toggle_suspend());
    }

    #[test]
    fn test_hibernation_setup_and_removal() {
        let mut engine = OmarchyPowerEngine::new();
        assert!(!engine.is_hibernate_ui_visible);

        // Setup 32GB RAM hibernation
        let setup_res = engine.execute_hibernation_command("setup", 32).unwrap();
        assert!(setup_res.contains("32GB"));
        assert!(engine.is_hibernate_ui_visible);
        assert!(engine.generate_limine_resume_cmdline().unwrap().contains("resume=UUID="));

        // Remove
        let remove_res = engine.execute_hibernation_command("remove", 32).unwrap();
        assert!(remove_res.contains("Removed"));
        assert!(!engine.is_hibernate_ui_visible);
        assert!(engine.generate_limine_resume_cmdline().is_none());
    }
}
