// SPDX-License-Identifier: MIT OR Apache-2.0
//! SigmaOS Dynamic Power Profiles & Battery Optimization Engine
//!
//! Inspired by Omarchy's battery optimization daemons and Arch Linux's
//! power-profiles-daemon / auto-cpufreq / TLP integration.
//! Automatically switches CPU governors (performance, balanced, power-saver),
//! adjusts Wayland compositor frame rates (120Hz/60Hz down to 30Hz on low battery),
//! and manages display backlight dimming.

use std::fmt;

/// Power performance profile mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Performance => write!(f, "performance"),
            Self::Balanced => write!(f, "balanced"),
            Self::PowerSaver => write!(f, "power-saver"),
        }
    }
}

/// Power telemetry facts
#[derive(Debug, Clone, PartialEq)]
pub struct PowerTelemetry {
    pub on_ac: bool,
    pub battery_percentage: u8,
    pub active_profile: PowerProfile,
    pub target_vfr_fps: u32,
    pub backlight_pct: u8,
}

impl PowerTelemetry {
    pub fn new(on_ac: bool, battery_pct: u8) -> Self {
        let mut telemetry = Self {
            on_ac,
            battery_percentage: battery_pct.min(100),
            active_profile: PowerProfile::Balanced,
            target_vfr_fps: 60,
            backlight_pct: 80,
        };
        telemetry.reevaluate_profile();
        telemetry
    }

    /// Automatically adjust profile based on AC power and battery level
    pub fn reevaluate_profile(&mut self) {
        if self.on_ac {
            self.active_profile = PowerProfile::Performance;
            self.target_vfr_fps = 144;
            self.backlight_pct = 100;
        } else if self.battery_percentage <= 20 {
            self.active_profile = PowerProfile::PowerSaver;
            self.target_vfr_fps = 30; // Reduce compositor refresh rate to conserve power
            self.backlight_pct = 35;
        } else {
            self.active_profile = PowerProfile::Balanced;
            self.target_vfr_fps = 60;
            self.backlight_pct = 70;
        }
    }

    /// Generate powerprofilesctl command
    pub fn powerprofilesctl_command(&self) -> String {
        format!("powerprofilesctl set {}", self.active_profile)
    }

    /// Generate brightnessctl command
    pub fn brightnessctl_command(&self) -> String {
        format!("brightnessctl set {}%", self.backlight_pct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile_display_aaa() {
        assert_eq!(PowerProfile::Performance.to_string(), "performance");
        assert_eq!(PowerProfile::Balanced.to_string(), "balanced");
        assert_eq!(PowerProfile::PowerSaver.to_string(), "power-saver");
    }

    #[test]
    fn test_on_ac_performance_mode_aaa() {
        let tele = PowerTelemetry::new(true, 50);
        assert_eq!(tele.active_profile, PowerProfile::Performance);
        assert_eq!(tele.target_vfr_fps, 144);
        assert_eq!(tele.backlight_pct, 100);
        assert_eq!(tele.powerprofilesctl_command(), "powerprofilesctl set performance");
        assert_eq!(tele.brightnessctl_command(), "brightnessctl set 100%");
    }

    #[test]
    fn test_low_battery_power_saver_mode_aaa() {
        let tele = PowerTelemetry::new(false, 15);
        assert_eq!(tele.active_profile, PowerProfile::PowerSaver);
        assert_eq!(tele.target_vfr_fps, 30);
        assert_eq!(tele.backlight_pct, 35);
        assert_eq!(tele.powerprofilesctl_command(), "powerprofilesctl set power-saver");
        assert_eq!(tele.brightnessctl_command(), "brightnessctl set 35%");
    }

    #[test]
    fn test_balanced_battery_mode_aaa() {
        let tele = PowerTelemetry::new(false, 75);
        assert_eq!(tele.active_profile, PowerProfile::Balanced);
        assert_eq!(tele.target_vfr_fps, 60);
        assert_eq!(tele.backlight_pct, 70);
    }
}
