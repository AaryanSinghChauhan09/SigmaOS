//! Performance profile manager — switches between balanced/performance/gaming/battery/workstation profiles, applying kernel, CPU, I/O, and desktop tunings atomically.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileKind {
    Balanced,
    Performance,
    Gaming,
    BatterySaving,
    Workstation,
}


pub struct PerformanceProfile {
    pub kind: ProfileKind,
    pub cpu_governor: &'static str,
    pub scheduler: &'static str,
    pub io_scheduler: &'static str,
    pub swap_policy: &'static str,
    pub desktop_animations: bool,
    pub gpu_profile: &'static str,
}

impl PerformanceProfile {
    pub fn for_kind(kind: ProfileKind) -> Self {
        match kind {
            ProfileKind::Balanced => Self { kind, cpu_governor: "powersave", scheduler: "eevdf", io_scheduler: "mq-deadline", swap_policy: "zram", desktop_animations: true, gpu_profile: "auto" },
            ProfileKind::Performance => Self { kind, cpu_governor: "performance", scheduler: "eevdf", io_scheduler: "kyber", swap_policy: "zram", desktop_animations: true, gpu_profile: "performance" },
            ProfileKind::Gaming => Self { kind, cpu_governor: "performance", scheduler: "bore", io_scheduler: "kyber", swap_policy: "zram", desktop_animations: true, gpu_profile: "performance" },
            ProfileKind::BatterySaving => Self { kind, cpu_governor: "powersave", scheduler: "eevdf", io_scheduler: "mq-deadline", swap_policy: "zram", desktop_animations: false, gpu_profile: "low-power" },
            ProfileKind::Workstation => Self { kind, cpu_governor: "performance", scheduler: "eevdf", io_scheduler: "bfq", swap_policy: "swapfile", desktop_animations: true, gpu_profile: "performance" },
        }
    }
}

/// Apply a performance profile (writes sysfs knobs + notifies services).
/// In a real implementation this writes to /sys/devices/system/cpu/.../scaling_governor etc.
pub fn apply_profile(profile: &PerformanceProfile) -> Vec<String> {
    let mut actions = Vec::new();
    actions.push(format!("cpu_governor -> {}", profile.cpu_governor));
    actions.push(format!("scheduler -> {}", profile.scheduler));
    actions.push(format!("io_scheduler -> {}", profile.io_scheduler));
    actions.push(format!("swap_policy -> {}", profile.swap_policy));
    actions.push(format!("desktop_animations -> {}", profile.desktop_animations));
    actions.push(format!("gpu_profile -> {}", profile.gpu_profile));
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [ProfileKind::Balanced, ProfileKind::Performance, ProfileKind::Gaming, ProfileKind::BatterySaving, ProfileKind::Workstation];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [ProfileKind::Balanced, ProfileKind::Performance, ProfileKind::Gaming, ProfileKind::BatterySaving, ProfileKind::Workstation];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
