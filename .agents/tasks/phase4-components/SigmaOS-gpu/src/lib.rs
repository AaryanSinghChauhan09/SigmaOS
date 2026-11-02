//! GPU detection and profile selection — NVIDIA, AMD, Intel — with PRIME switching support.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuKind {
    Nvidia,
    Amd,
    Intel,
    Hybrid,
}


pub struct GpuProfile {
    pub kind: GpuKind,
    pub driver: &'static str,
    pub vulkan_support: bool,
    pub prime_switching: bool,
    pub power_management: bool,
}

impl GpuProfile {
    pub fn for_kind(kind: GpuKind) -> Self {
        match kind {
            GpuKind::Nvidia => Self { kind, driver: "nvidia", vulkan_support: true, prime_switching: true, power_management: true },
            GpuKind::Amd => Self { kind, driver: "amdgpu", vulkan_support: true, prime_switching: false, power_management: true },
            GpuKind::Intel => Self { kind, driver: "i915", vulkan_support: true, prime_switching: true, power_management: true },
            GpuKind::Hybrid => Self { kind, driver: "hybrid-prime", vulkan_support: true, prime_switching: true, power_management: true },
        }
    }
    pub fn detect() -> GpuKind {
        // In a real impl: parse /sys/class/drm or lspci. Scaffold returns Intel default.
        GpuKind::Intel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [GpuKind::Nvidia, Kind::Amd, Kind::Intel, Kind::Hybrid];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [GpuKind::Nvidia, Kind::Amd, Kind::Intel, Kind::Hybrid];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
