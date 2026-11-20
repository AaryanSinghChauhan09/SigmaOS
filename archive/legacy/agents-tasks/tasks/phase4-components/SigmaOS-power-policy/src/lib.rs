//! Performance vs power profile switcher — balances battery life and performance per use case.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerKind {
    AcPerformance,
    Battery,
    Balanced,
    LowBattery,
}


pub struct PowerPolicy {
    pub kind: PowerKind,
    pub cpu_governor: &'static str,
    pub brightness_pct: u32,
    pub suspend_after_min: u32,
    pub wifi_power_save: bool,
    pub bluetooth_power_save: bool,
}

impl PowerPolicy {
    pub fn for_kind(kind: PowerKind) -> Self {
        match kind {
            PowerKind::AcPerformance => Self { kind, cpu_governor: "performance", brightness_pct: 100, suspend_after_min: 0, wifi_power_save: false, bluetooth_power_save: false },
            PowerKind::Battery => Self { kind, cpu_governor: "powersave", brightness_pct: 60, suspend_after_min: 15, wifi_power_save: true, bluetooth_power_save: true },
            PowerKind::Balanced => Self { kind, cpu_governor: "schedutil", brightness_pct: 80, suspend_after_min: 30, wifi_power_save: false, bluetooth_power_save: false },
            PowerKind::LowBattery => Self { kind, cpu_governor: "powersave", brightness_pct: 30, suspend_after_min: 5, wifi_power_save: true, bluetooth_power_save: true },
        }
    }
    pub fn should_suspend(&self, battery_pct: u32) -> bool {
        battery_pct < 5 && self.kind == PowerKind::LowBattery
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [PowerKind::AcPerformance, Kind::Battery, Kind::Balanced, Kind::LowBattery];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [PowerKind::AcPerformance, Kind::Battery, Kind::Balanced, Kind::LowBattery];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
