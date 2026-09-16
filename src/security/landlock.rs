// Linux Landlock LSM (Unprivileged Application Sandboxing) Subsystem for SigmaOS
// Enables unprivileged processes to restrict their own access to filesystem hierarchies and resources.

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

/// Bitmask constants for Landlock Filesystem Access Rights
pub const LANDLOCK_ACCESS_FS_EXECUTE: u64 = 1 << 0;
pub const LANDLOCK_ACCESS_FS_WRITE_FILE: u64 = 1 << 1;
pub const LANDLOCK_ACCESS_FS_READ_FILE: u64 = 1 << 2;
pub const LANDLOCK_ACCESS_FS_READ_DIR: u64 = 1 << 3;
pub const LANDLOCK_ACCESS_FS_REMOVE_DIR: u64 = 1 << 4;
pub const LANDLOCK_ACCESS_FS_REMOVE_FILE: u64 = 1 << 5;
pub const LANDLOCK_ACCESS_FS_MAKE_CHAR: u64 = 1 << 6;
pub const LANDLOCK_ACCESS_FS_MAKE_DIR: u64 = 1 << 7;
pub const LANDLOCK_ACCESS_FS_MAKE_REG: u64 = 1 << 8;
pub const LANDLOCK_ACCESS_FS_MAKE_SOCK: u64 = 1 << 9;
pub const LANDLOCK_ACCESS_FS_MAKE_FIFO: u64 = 1 << 10;
pub const LANDLOCK_ACCESS_FS_MAKE_BLOCK: u64 = 1 << 11;
pub const LANDLOCK_ACCESS_FS_MAKE_SYM: u64 = 1 << 12;
pub const LANDLOCK_ACCESS_FS_REFER: u64 = 1 << 13;
pub const LANDLOCK_ACCESS_FS_TRUNCATE: u64 = 1 << 14;

/// Attribute describing a Path Beneath Landlock Rule
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandlockPathBeneathAttr {
    pub allowed_access: u64,
    pub parent_path: String,
}

/// Landlock Ruleset Definition
#[derive(Debug, Clone)]
pub struct LandlockRuleset {
    pub ruleset_id: u32,
    pub handled_access_fs: u64,
    pub path_beneath_rules: Vec<LandlockPathBeneathAttr>,
    pub is_enforced: bool,
}

impl LandlockRuleset {
    pub fn new(ruleset_id: u32, handled_access_fs: u64) -> Self {
        Self {
            ruleset_id,
            handled_access_fs,
            path_beneath_rules: Vec::new(),
            is_enforced: false,
        }
    }

    pub fn add_path_beneath_rule(&mut self, parent_path: &str, allowed_access: u64) {
        self.path_beneath_rules.push(LandlockPathBeneathAttr {
            allowed_access: allowed_access & self.handled_access_fs,
            parent_path: parent_path.to_string(),
        });
    }

    pub fn is_access_allowed(&self, path: &str, access: u64) -> bool {
        if !self.is_enforced {
            return true;
        }

        // If the access right is not handled by this ruleset, allow it by default
        if (self.handled_access_fs & access) == 0 {
            return true;
        }

        // Check matching path beneath rules
        for rule in &self.path_beneath_rules {
            if path == rule.parent_path || path.starts_with(&format!("{}/", rule.parent_path.trim_end_matches('/'))) {
                if (rule.allowed_access & access) == access {
                    return true;
                }
            }
        }

        false
    }
}

/// Linux Landlock Security Sandbox Engine
pub struct LandlockEngine {
    pub rulesets: Vec<LandlockRuleset>,
    pub next_ruleset_id: u32,
}

impl LandlockEngine {
    pub fn new() -> Self {
        Self {
            rulesets: Vec::new(),
            next_ruleset_id: 1,
        }
    }

    pub fn create_ruleset(&mut self, handled_access_fs: u64) -> u32 {
        let id = self.next_ruleset_id;
        self.next_ruleset_id += 1;
        let ruleset = LandlockRuleset::new(id, handled_access_fs);
        self.rulesets.push(ruleset);
        id
    }

    pub fn add_rule(
        &mut self,
        ruleset_id: u32,
        parent_path: &str,
        allowed_access: u64,
    ) -> Result<(), &'static str> {
        let ruleset = self
            .rulesets
            .iter_mut()
            .find(|r| r.ruleset_id == ruleset_id)
            .ok_or("LandlockEngine: Invalid ruleset ID")?;

        if ruleset.is_enforced {
            return Err("LandlockEngine: Cannot add rule to an already enforced ruleset");
        }

        ruleset.add_path_beneath_rule(parent_path, allowed_access);
        Ok(())
    }

    pub fn restrict_self(&mut self, ruleset_id: u32) -> Result<(), &'static str> {
        let ruleset = self
            .rulesets
            .iter_mut()
            .find(|r| r.ruleset_id == ruleset_id)
            .ok_or("LandlockEngine: Invalid ruleset ID")?;

        ruleset.is_enforced = true;
        Ok(())
    }

    pub fn validate_file_access(&self, ruleset_id: u32, path: &str, access: u64) -> bool {
        if let Some(ruleset) = self.rulesets.iter().find(|r| r.ruleset_id == ruleset_id) {
            ruleset.is_access_allowed(path, access)
        } else {
            true // No sandbox ruleset active
        }
    }
}

impl Default for LandlockEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landlock_sandbox_enforcement() {
        let mut engine = LandlockEngine::new();
        let handled = LANDLOCK_ACCESS_FS_READ_FILE | LANDLOCK_ACCESS_FS_WRITE_FILE | LANDLOCK_ACCESS_FS_EXECUTE;
        let ruleset_id = engine.create_ruleset(handled);

        // Grant read + execute access under /usr and read + write under /tmp
        assert!(engine.add_rule(ruleset_id, "/usr", LANDLOCK_ACCESS_FS_READ_FILE | LANDLOCK_ACCESS_FS_EXECUTE).is_ok());
        assert!(engine.add_rule(ruleset_id, "/tmp", LANDLOCK_ACCESS_FS_READ_FILE | LANDLOCK_ACCESS_FS_WRITE_FILE).is_ok());

        // Before restrict_self: access allowed everywhere
        assert!(engine.validate_file_access(ruleset_id, "/etc/shadow", LANDLOCK_ACCESS_FS_READ_FILE));

        // Restrict process
        assert!(engine.restrict_self(ruleset_id).is_ok());

        // Cannot add rules after restriction
        assert!(engine.add_rule(ruleset_id, "/home", LANDLOCK_ACCESS_FS_READ_FILE).is_err());

        // Allowed accesses
        assert!(engine.validate_file_access(ruleset_id, "/usr/bin/bash", LANDLOCK_ACCESS_FS_EXECUTE));
        assert!(engine.validate_file_access(ruleset_id, "/tmp/log.txt", LANDLOCK_ACCESS_FS_WRITE_FILE));

        // Prohibited accesses
        assert!(!engine.validate_file_access(ruleset_id, "/etc/shadow", LANDLOCK_ACCESS_FS_READ_FILE));
        assert!(!engine.validate_file_access(ruleset_id, "/usr/bin/bash", LANDLOCK_ACCESS_FS_WRITE_FILE));
    }
}
