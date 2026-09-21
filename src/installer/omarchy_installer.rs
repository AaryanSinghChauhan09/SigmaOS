// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Target-Side Installer & Setup Orchestrator Engine (`src/installer/omarchy_installer.rs`)
// Inspired by Omarchy Linux (target-side setup commands, omarchy-apply-system, omarchy-apply-hardware,
// omarchy-finalize-user, omarchy-reinstall-configs, sourced leaf scripts without shebangs/exit, DKMS headers).

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

/// Installation Execution Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationCommandMode {
    ApplySystem,       // bin/omarchy-apply-system
    ApplyHardware,     // bin/omarchy-apply-hardware
    FinalizeUser,      // bin/omarchy-finalize-user
    ReinstallConfigs,  // bin/omarchy-reinstall-configs (destructive $HOME resync)
}

/// Sourced Setup Leaf Script Descriptor (`install/`)
#[derive(Debug, Clone)]
pub struct SourcedLeafScript {
    pub rel_path: String,          // e.g., "hardware/nvidia.sh", "user/mime.sh"
    pub is_user_scoped: bool,      // install/user/ vs install/hardware/
    pub has_shebang: bool,         // Must be false for sourced leaves
    pub has_direct_exit: bool,     // Should avoid 'exit' in sourced scripts
}

/// Target-Side Installation Orchestrator Engine
pub struct OmarchyInstallerEngine {
    pub omarchy_install_dir: String, // $OMARCHY_INSTALL (default: /usr/share/omarchy/install)
    pub omarchy_path_dir: String,    // $OMARCHY_PATH (default: /usr/share/omarchy)
    pub kernel_headers_installed: bool,
    pub is_iso_target_finalized: bool,
    pub execution_log: Vec<String>,
    pub leaf_scripts: BTreeMap<String, SourcedLeafScript>,
}

impl OmarchyInstallerEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            omarchy_install_dir: String::from("/usr/share/omarchy/install"),
            omarchy_path_dir: String::from("/usr/share/omarchy"),
            kernel_headers_installed: true, // Supplied by base install prior to hardware setup
            is_iso_target_finalized: false,
            execution_log: Vec::new(),
            leaf_scripts: BTreeMap::new(),
        };
        engine.register_default_leaf_scripts();
        engine
    }

    /// Register default setup leaf scripts under `install/`
    pub fn register_default_leaf_scripts(&mut self) {
        self.leaf_scripts.insert(
            "hardware/gpu.sh".to_string(),
            SourcedLeafScript {
                rel_path: "hardware/gpu.sh".to_string(),
                is_user_scoped: false,
                has_shebang: false,
                has_direct_exit: false,
            },
        );

        self.leaf_scripts.insert(
            "hardware/all.sh".to_string(),
            SourcedLeafScript {
                rel_path: "hardware/all.sh".to_string(),
                is_user_scoped: false,
                has_shebang: false,
                has_direct_exit: false,
            },
        );

        self.leaf_scripts.insert(
            "user/all.sh".to_string(),
            SourcedLeafScript {
                rel_path: "user/all.sh".to_string(),
                is_user_scoped: true,
                has_shebang: false,
                has_direct_exit: false,
            },
        );

        self.leaf_scripts.insert(
            "user/first-run/skills.sh".to_string(),
            SourcedLeafScript {
                rel_path: "user/first-run/skills.sh".to_string(),
                is_user_scoped: true,
                has_shebang: false,
                has_direct_exit: false,
            },
        );
    }

    /// Execute `run_logged $OMARCHY_INSTALL/path/to/script.sh`
    pub fn run_logged_leaf_script(&mut self, script_rel_path: &str) -> Result<String, &'static str> {
        let leaf = self
            .leaf_scripts
            .get(script_rel_path)
            .ok_or("Leaf script not found under install/")?;

        if leaf.has_shebang {
            return Err("Sourced leaf script violation: Contains shebang #!/bin/sh");
        }

        if leaf.has_direct_exit {
            return Err("Sourced leaf script violation: Calls direct 'exit'");
        }

        let full_path = format!("{}/{}", self.omarchy_install_dir, script_rel_path);
        let log_entry = format!("run_logged: Sourced leaf script '{}' successfully.", full_path);
        self.execution_log.push(log_entry.clone());

        Ok(log_entry)
    }

    /// Execute `bin/omarchy-apply-system` (Root-owned ISO finalization)
    pub fn execute_apply_system(&mut self) -> Result<String, &'static str> {
        self.execution_log.push("bin/omarchy-apply-system: Starting target-side root setup...".to_string());

        // Orchestrate hardware setup
        self.execute_apply_hardware()?;

        self.is_iso_target_finalized = true;
        let msg = "bin/omarchy-apply-system: Root system setup & ISO target finalization complete.".to_string();
        self.execution_log.push(msg.clone());
        Ok(msg)
    }

    /// Execute `bin/omarchy-apply-hardware` (Idempotent hardware setup)
    pub fn execute_apply_hardware(&mut self) -> Result<String, &'static str> {
        if !self.kernel_headers_installed {
            return Err("omarchy-apply-hardware: Missing kernel headers for DKMS driver compilation");
        }

        self.execution_log.push("bin/omarchy-apply-hardware: Verifying DKMS headers & running hardware/all.sh...".to_string());
        self.run_logged_leaf_script("hardware/all.sh")?;

        let msg = "bin/omarchy-apply-hardware: Idempotent hardware setup complete.".to_string();
        self.execution_log.push(msg.clone());
        Ok(msg)
    }

    /// Execute `bin/omarchy-finalize-user` (Per-user runtime finalization)
    pub fn execute_finalize_user(&mut self, username: &str) -> Result<String, &'static str> {
        self.execution_log.push(format!("bin/omarchy-finalize-user: Finalizing runtime environment for user '{}'...", username));

        // Execute user leaf script
        self.run_logged_leaf_script("user/all.sh")?;

        let msg = format!("bin/omarchy-finalize-user: Skill symlinks, xdg-user-dirs, and MIME defaults applied for '{}'.", username);
        self.execution_log.push(msg.clone());
        Ok(msg)
    }

    /// Execute `bin/omarchy-reinstall-configs` (Explicit destructive resync into $HOME)
    pub fn execute_reinstall_configs(&mut self, username: &str) -> Result<String, &'static str> {
        let msg = format!("bin/omarchy-reinstall-configs: Destructive resync of factory /etc/skel configs into '/home/{}' complete.", username);
        self.execution_log.push(msg.clone());
        Ok(msg)
    }
}

impl Default for OmarchyInstallerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_system_and_hardware_orchestration() {
        let mut engine = OmarchyInstallerEngine::new();
        assert!(!engine.is_iso_target_finalized);

        let res = engine.execute_apply_system().unwrap();
        assert!(res.contains("Root system setup"));
        assert!(engine.is_iso_target_finalized);
        assert!(engine.execution_log.iter().any(|l| l.contains("omarchy-apply-hardware")));
    }

    #[test]
    fn test_missing_kernel_headers_error() {
        let mut engine = OmarchyInstallerEngine::new();
        engine.kernel_headers_installed = false;

        let res = engine.execute_apply_hardware();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "omarchy-apply-hardware: Missing kernel headers for DKMS driver compilation");
    }

    #[test]
    fn test_finalize_user_and_reinstall_configs() {
        let mut engine = OmarchyInstallerEngine::new();

        let finalize_res = engine.execute_finalize_user("sigma_developer").unwrap();
        assert!(finalize_res.contains("sigma_developer"));

        let reinstall_res = engine.execute_reinstall_configs("sigma_developer").unwrap();
        assert!(reinstall_res.contains("Destructive resync"));
    }

    #[test]
    fn test_leaf_script_validation() {
        let mut engine = OmarchyInstallerEngine::new();

        // Add faulty leaf script with shebang
        engine.leaf_scripts.insert(
            "user/bad.sh".to_string(),
            SourcedLeafScript {
                rel_path: "user/bad.sh".to_string(),
                is_user_scoped: true,
                has_shebang: true,
                has_direct_exit: false,
            },
        );

        assert!(engine.run_logged_leaf_script("user/bad.sh").is_err());
    }
}
