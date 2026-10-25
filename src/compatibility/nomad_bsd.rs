// SPDX-License-Identifier: GPL-2.0-or-later
// NomadBSD live USB persistence layer for SigmaOS.
// Handles overlay/full persistence modes, hardware profile detection,
// and state save/restore for live USB environments.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Persistence mode for the NomadBSD live session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PersistenceMode {
    Volatile,
    Overlay { overlay_mb: u32 },
    Full,
}

/// Detected hardware profile for a live USB boot.
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    pub cpu_vendor: String,
    pub gpu_vendor: String,
    pub wifi_chip: String,
}

impl HardwareProfile {
    pub fn new(cpu_vendor: &str, gpu_vendor: &str, wifi_chip: &str) -> Self {
        Self {
            cpu_vendor: cpu_vendor.to_string(),
            gpu_vendor: gpu_vendor.to_string(),
            wifi_chip: wifi_chip.to_string(),
        }
    }

    pub fn unknown() -> Self {
        Self::new("unknown", "unknown", "unknown")
    }
}

/// NomadBSD live session state container.
#[derive(Debug, Clone)]
pub struct NomadState {
    pub persistence: PersistenceMode,
    pub hw_profile: HardwareProfile,
}

impl NomadState {
    pub fn new(persistence: PersistenceMode) -> Self {
        Self {
            persistence,
            hw_profile: HardwareProfile::unknown(),
        }
    }
}

/// Core NomadBSD session controller.
pub struct NomadSession {
    pub state: NomadState,
    pub mounted: bool,
    pub saved_keys: Vec<(String, String)>,
}

impl NomadSession {
    pub fn new(mode: PersistenceMode) -> Self {
        Self {
            state: NomadState::new(mode),
            mounted: false,
            saved_keys: Vec::new(),
        }
    }

    /// Detect hardware and populate hw_profile.
    pub fn detect_hardware(&mut self, cpu: &str, gpu: &str, wifi: &str) {
        self.state.hw_profile = HardwareProfile::new(cpu, gpu, wifi);
    }

    /// Mount the persistence layer (overlay or full partition).
    pub fn mount_persistence(&mut self) -> Result<(), &'static str> {
        match &self.state.persistence {
            PersistenceMode::Volatile => Err("Volatile mode: no persistence to mount"),
            PersistenceMode::Overlay { overlay_mb } => {
                if *overlay_mb == 0 {
                    return Err("Overlay size cannot be zero");
                }
                self.mounted = true;
                Ok(())
            }
            PersistenceMode::Full => {
                self.mounted = true;
                Ok(())
            }
        }
    }

    /// Persist a key-value pair to the state store.
    pub fn save_state(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if !self.mounted && self.state.persistence != PersistenceMode::Volatile {
            return Err("Persistence not mounted");
        }
        self.saved_keys.retain(|(k, _)| k != key);
        self.saved_keys.push((key.to_string(), value.to_string()));
        Ok(())
    }

    /// Restore a persisted value by key.
    pub fn restore_state(&self, key: &str) -> Option<&str> {
        self.saved_keys
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volatile_cannot_mount() {
        let mut s = NomadSession::new(PersistenceMode::Volatile);
        assert!(s.mount_persistence().is_err());
    }

    #[test]
    fn test_overlay_mount_and_state() {
        let mut s = NomadSession::new(PersistenceMode::Overlay { overlay_mb: 512 });
        s.detect_hardware("GenuineIntel", "NVIDIA", "iwlwifi");
        assert!(s.mount_persistence().is_ok());
        assert!(s.save_state("locale", "en_US.UTF-8").is_ok());
        assert_eq!(s.restore_state("locale"), Some("en_US.UTF-8"));
    }

    #[test]
    fn test_full_persistence() {
        let mut s = NomadSession::new(PersistenceMode::Full);
        assert!(s.mount_persistence().is_ok());
        s.save_state("timezone", "UTC").unwrap();
        assert_eq!(s.restore_state("timezone"), Some("UTC"));
    }
}
