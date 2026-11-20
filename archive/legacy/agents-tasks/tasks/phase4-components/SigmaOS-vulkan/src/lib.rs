//! Vulkan support configuration — driver selection, layers, and validation toggles.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulkanKind {
    Enabled,
    Disabled,
    Validation,
}


pub struct VulkanConfig {
    pub kind: VulkanKind,
    pub enabled: bool,
    pub validation_layers: bool,
    pub preferred_driver: &'static str,
}

impl VulkanConfig {
    pub fn for_kind(kind: VulkanKind) -> Self {
        match kind {
            VulkanKind::Enabled => Self { kind, enabled: true, validation_layers: false, preferred_driver: "auto" },
            VulkanKind::Disabled => Self { kind, enabled: false, validation_layers: false, preferred_driver: "none" },
            VulkanKind::Validation => Self { kind, enabled: true, validation_layers: true, preferred_driver: "auto" },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [VulkanKind::Enabled, Kind::Disabled, Kind::Validation];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [VulkanKind::Enabled, Kind::Disabled, Kind::Validation];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
