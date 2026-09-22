//! Declarative System Configuration Management inspired by NixOS, Guix, FreeBSD rc.conf, and OpenBSD pf.conf
//! Atomic upgrades, system generation tracking, configuration modules, key-value rc.conf parsers, and instant rollbacks.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

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
            generations: std::vec![initial_gen],
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

// ============================================================================
// Linux & BSD Declarative Configuration Engine (rc.conf, pf.conf, NixOS schema)
// ============================================================================

pub struct LinuxBsdDeclarativeConfigEngine {
    pub rc_conf_entries: BTreeMap<String, String>,
    pub pf_rules: Vec<String>,
}

impl LinuxBsdDeclarativeConfigEngine {
    pub fn new() -> Self {
        let mut rc = BTreeMap::new();
        rc.insert("hostname".to_string(), "sigmaos-node".to_string());
        rc.insert("sshd_enable".to_string(), "YES".to_string());
        rc.insert("pf_enable".to_string(), "YES".to_string());

        let pf = vec![
            "set skip on lo".to_string(),
            "block in all".to_string(),
            "pass out all keep state".to_string(),
            "pass in proto tcp to port 22 keep state".to_string(),
        ];

        Self {
            rc_conf_entries: rc,
            pf_rules: pf,
        }
    }

    pub fn parse_rc_conf(&mut self, rc_conf_content: &str) {
        for line in rc_conf_content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = trimmed.split_once('=') {
                let clean_k = k.trim().to_string();
                let clean_v = v.trim().trim_matches('"').trim_matches('\'').to_string();
                self.rc_conf_entries.insert(clean_k, clean_v);
            }
        }
    }

    pub fn generate_pf_conf(&self) -> String {
        self.pf_rules.join("\n")
    }
}

pub struct DeclarativeStateReconciler {
    pub active_config: ConfigManager,
}

impl DeclarativeStateReconciler {
    pub fn new(profile: &str) -> Self {
        Self {
            active_config: ConfigManager::new(profile),
        }
    }

    pub fn reconcile_and_stage(&mut self, module_name: &str, key: &str, value: &str) -> u32 {
        self.active_config.add_module_to_active(ConfigModule {
            module_name: module_name.to_string(),
            options: vec![(key.to_string(), value.to_string())],
            is_enabled: true,
        });

        self.active_config.commit_atomic_generation(1718920000)
    }
}

impl Default for LinuxBsdDeclarativeConfigEngine {
    fn default() -> Self {
        Self::new()
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
            options: std::vec![("enable".to_string(), "true".to_string())],
            is_enabled: true,
        });

        let gen2 = cfg.commit_atomic_generation(1718910000);
        assert_eq!(gen2, 2);
        assert_eq!(cfg.active_generation_id, 2);

        assert!(cfg.rollback(1).is_ok());
        assert_eq!(cfg.active_generation_id, 1);
        assert_eq!(cfg.generations[0].state, ConfigState::Active);
    }

    #[test]
    fn test_linux_bsd_declarative_config_engine() {
        let mut engine = LinuxBsdDeclarativeConfigEngine::new();
        engine.parse_rc_conf("zfs_enable=\"YES\"\nhostname=\"sovereign-node\"\n");
        assert_eq!(engine.rc_conf_entries.get("zfs_enable").unwrap(), "YES");
        assert_eq!(engine.rc_conf_entries.get("hostname").unwrap(), "sovereign-node");

        let pf_conf = engine.generate_pf_conf();
        assert!(pf_conf.contains("block in all"));
    }

    #[test]
    fn test_declarative_state_reconciler() {
        let mut reconciler = DeclarativeStateReconciler::new("workstation");
        let new_gen = reconciler.reconcile_and_stage("services.wireguard", "enable", "true");
        assert_eq!(new_gen, 2);
    }
}
