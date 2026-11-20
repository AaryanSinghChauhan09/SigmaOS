//! SigmaOS Power Manager — battery monitoring, suspend/resume policy, and
//! performance/power profile integration.

#![allow(dead_code)]

use std::fmt;

// ---------------------------------------------------------------------------
// Power source
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerSource {
    AC,
    Battery,
    Unknown,
}

// ---------------------------------------------------------------------------
// Battery state
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    NotPresent,
}

#[derive(Debug, Clone)]
pub struct BatteryStatus {
    pub level_pct: u32,
    pub state: BatteryState,
    pub time_remaining_min: Option<u32>,
}

impl BatteryStatus {
    pub fn is_low(&self) -> bool { self.level_pct <= 20 }
    pub fn is_critical(&self) -> bool { self.level_pct <= 5 }
}

// ---------------------------------------------------------------------------
// Power policy
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
}

impl fmt::Display for PowerProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerProfile::Performance => write!(f, "performance"),
            PowerProfile::Balanced => write!(f, "balanced"),
            PowerProfile::PowerSaver => write!(f, "power-saver"),
        }
    }
}

// ---------------------------------------------------------------------------
// Power Manager
// ---------------------------------------------------------------------------

pub struct PowerManager {
    pub source: PowerSource,
    pub battery: Option<BatteryStatus>,
    pub profile: PowerProfile,
    pub suspend_after_idle_min: u32,
    low_battery_warned: bool,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            source: PowerSource::AC,
            battery: None,
            profile: PowerProfile::Balanced,
            suspend_after_idle_min: 30,
            low_battery_warned: false,
        }
    }

    pub fn update_battery(&mut self, status: BatteryStatus) {
        if status.is_low() && !self.low_battery_warned {
            self.low_battery_warned = true;
            // Real impl: emit D-Bus signal / notification
        }
        self.source = match status.state {
            BatteryState::Charging | BatteryState::Full => PowerSource::AC,
            BatteryState::Discharging => PowerSource::Battery,
            BatteryState::NotPresent => PowerSource::AC,
        };
        // Auto-select profile based on battery level
        self.profile = if self.source == PowerSource::Battery && status.is_low() {
            PowerProfile::PowerSaver
        } else {
            PowerProfile::Balanced
        };
        self.battery = Some(status);
    }

    pub fn set_profile(&mut self, profile: PowerProfile) {
        self.profile = profile;
    }

    pub fn should_suspend(&self, idle_min: u32) -> bool {
        idle_min >= self.suspend_after_idle_min
    }

    pub fn status_line(&self) -> String {
        match &self.battery {
            None => format!("AC only  profile={}", self.profile),
            Some(b) => format!(
                "{}%  {:?}  profile={}  source={:?}",
                b.level_pct, b.state, self.profile, self.source
            ),
        }
    }
}

impl Default for PowerManager {
    fn default() -> Self { Self::new() }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ac_only_defaults() {
        let pm = PowerManager::new();
        assert_eq!(pm.source, PowerSource::AC);
        assert_eq!(pm.profile, PowerProfile::Balanced);
    }

    #[test]
    fn low_battery_switches_to_power_saver() {
        let mut pm = PowerManager::new();
        pm.update_battery(BatteryStatus {
            level_pct: 15,
            state: BatteryState::Discharging,
            time_remaining_min: Some(20),
        });
        assert_eq!(pm.profile, PowerProfile::PowerSaver);
        assert_eq!(pm.source, PowerSource::Battery);
    }

    #[test]
    fn charging_stays_balanced() {
        let mut pm = PowerManager::new();
        pm.update_battery(BatteryStatus {
            level_pct: 55,
            state: BatteryState::Charging,
            time_remaining_min: None,
        });
        assert_eq!(pm.profile, PowerProfile::Balanced);
        assert_eq!(pm.source, PowerSource::AC);
    }

    #[test]
    fn suspend_threshold() {
        let pm = PowerManager::new();
        assert!(!pm.should_suspend(10));
        assert!(pm.should_suspend(30));
        assert!(pm.should_suspend(45));
    }
}
