//! Declarative System Configuration Management inspired by NixOS and Guix
//! Atomic upgrades, system generation tracking, configuration modules, and instant rollbacks.

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigState {
    Active,
    Staged,
    RolledBack,
}

#[derive(Debug, Clone)]
pub struct ConfigModule {
    pub module_name: String,
    pub options: Vec<(String, String)>,
    pub is_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct SystemGeneration {
    pub generation_id: u32,
    pub created_at_timestamp: u64,
    pub config_hash: [u8; 32],
    pub modules: Vec<ConfigModule>,
    pub state: ConfigState,
}

pub struct ConfigManager {
    pub generations: Vec<SystemGeneration>,
    pub active_generation_id: u32,
    pub system_profile_name: String,
}

impl ConfigManager {
    pub fn new(profile_name: &str) -> Self {
        let initial_gen = SystemGeneration {
            generation_id: 1,
            created_at_timestamp: 1718900000,
            config_hash: [0u8; 32],
            modules: Vec::new(),
            state: ConfigState::Active,
        };

        Self {
            generations: alloc::vec![initial_gen],
            active_generation_id: 1,
            system_profile_name: profile_name.to_string(),
        }
    }

    pub fn add_module_to_active(&mut self, module: ConfigModule) {
        if let Some(gen) = self.generations.iter_mut().find(|g| g.generation_id == self.active_generation_id) {
            gen.modules.push(module);
        }
    }

    pub fn commit_atomic_generation(&mut self, timestamp: u64) -> u32 {
        let new_id = self.generations.len() as u32 + 1;

        let current_modules = self.generations.iter()
            .find(|g| g.generation_id == self.active_generation_id)
            .map(|g| g.modules.clone())
            .unwrap_or_default();

        let mut hash = [0u8; 32];
        for (i, m) in current_modules.iter().enumerate() {
            for &b in m.module_name.as_bytes() {
                hash[i % 32] ^= b;
            }
        }

        // Set previous active generation state to rolled-back or inactive
        for g in &mut self.generations {
            if g.generation_id == self.active_generation_id {
                g.state = ConfigState::RolledBack;
            }
        }

        let new_gen = SystemGeneration {
            generation_id: new_id,
            created_at_timestamp: timestamp,
            config_hash: hash,
            modules: current_modules,
            state: ConfigState::Active,
        };

        self.generations.push(new_gen);
        self.active_generation_id = new_id;
        new_id
    }

    pub fn rollback(&mut self, target_generation_id: u32) -> Result<(), &'static str> {
        if let Some(target) = self.generations.iter().find(|g| g.generation_id == target_generation_id) {
            let _ = target;
            for g in &mut self.generations {
                if g.generation_id == target_generation_id {
                    g.state = ConfigState::Active;
                } else if g.generation_id == self.active_generation_id {
                    g.state = ConfigState::RolledBack;
                }
            }
            self.active_generation_id = target_generation_id;
            Ok(())
        } else {
            Err("Target generation ID does not exist")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declarative_config_generations_rollback() {
        let mut cfg = ConfigManager::new("default-workstation");
        cfg.add_module_to_active(ConfigModule {
            module_name: "services.pipewire".to_string(),
            options: alloc::vec![("enable".to_string(), "true".to_string())],
            is_enabled: true,
        });

        let gen2 = cfg.commit_atomic_generation(1718910000);
        assert_eq!(gen2, 2);
        assert_eq!(cfg.active_generation_id, 2);

        assert!(cfg.rollback(1).is_ok());
        assert_eq!(cfg.active_generation_id, 1);
        assert_eq!(cfg.generations[0].state, ConfigState::Active);
    }
}
