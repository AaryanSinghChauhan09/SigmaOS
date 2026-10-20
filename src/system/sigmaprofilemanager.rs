// SPDX-License-Identifier: MIT
// SigmaOS - Profile Manager
// Profile management inspired by Omarchy profiles and Linux profile systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Profile
#[derive(Debug, Clone)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub settings: Vec<(String, String)>,
}

/// Profile manager
pub struct SigmaProfileManager {
    profiles: Vec<Profile>,
    active_profile: Option<String>,
}

impl SigmaProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            active_profile: None,
        }
    }

    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    pub fn get_profiles(&self) -> &[Profile] {
        &self.profiles
    }

    pub fn get_profile(&self, id: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.id == id)
    }

    pub fn set_active_profile(&mut self, id: String) -> Result<(), String> {
        if self.get_profile(&id).is_some() {
            self.active_profile = Some(id);
            Ok(())
        } else {
            Err(format!("Profile {} not found", id))
        }
    }

    pub fn get_active_profile(&self) -> Option<&Profile> {
        if let Some(ref id) = self.active_profile {
            self.get_profile(id)
        } else {
            None
        }
    }
}

impl Default for SigmaProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_manager_creation() {
        let manager = SigmaProfileManager::new();
        assert_eq!(manager.get_profiles().len(), 0);
    }

    #[test]
    fn test_add_profile() {
        let mut manager = SigmaProfileManager::new();
        let profile = Profile {
            id: "default".to_string(),
            name: "Default".to_string(),
            settings: Vec::new(),
        };
        manager.add_profile(profile);
        assert_eq!(manager.get_profiles().len(), 1);
    }

    #[test]
    fn test_set_active_profile() {
        let mut manager = SigmaProfileManager::new();
        let profile = Profile {
            id: "default".to_string(),
            name: "Default".to_string(),
            settings: Vec::new(),
        };
        manager.add_profile(profile);
        manager.set_active_profile("default".to_string()).unwrap();
        assert!(manager.get_active_profile().is_some());
    }
}
