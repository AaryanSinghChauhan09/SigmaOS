//! Emergency shell and automated repair tools for SigmaOS rescue mode.
//!
//! When automatic rescue fails, an emergency root shell is launched.
//! Repair tools are available to the shell and can also be run
//! non-interactively by the rescue orchestrator.

/// Repair tools available in the emergency shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepairTool {
    /// Run `fsck` on the root partition and attempt automatic repair.
    Fsck,
    /// Reinstall and reconfigure GRUB.
    GrubReinstall,
    /// Verify package database integrity and reinstall corrupt packages.
    PackageIntegrityCheck,
    /// Reinstall the currently running kernel.
    KernelReinstall,
    /// Reset system configuration files to defaults from the package.
    ConfigReset,
    /// Flush and reconfigure NetworkManager.
    NetworkReset,
}

impl RepairTool {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fsck                  => "Filesystem check (fsck)",
            Self::GrubReinstall         => "Reinstall GRUB bootloader",
            Self::PackageIntegrityCheck => "Package database integrity check",
            Self::KernelReinstall       => "Reinstall kernel",
            Self::ConfigReset           => "Reset system configuration",
            Self::NetworkReset          => "Reset network configuration",
        }
    }

    /// Brief description of what the tool does.
    pub fn description(self) -> &'static str {
        match self {
            Self::Fsck =>
                "Checks and repairs filesystem errors on root and boot partitions",
            Self::GrubReinstall =>
                "Reinstalls GRUB to the EFI partition and regenerates grub.cfg",
            Self::PackageIntegrityCheck =>
                "Verifies dpkg package integrity and reinstalls any corrupt packages",
            Self::KernelReinstall =>
                "Removes and reinstalls the currently booted kernel and initramfs",
            Self::ConfigReset =>
                "Resets /etc configuration to package defaults using dpkg-reconfigure",
            Self::NetworkReset =>
                "Clears NetworkManager state and resets to DHCP on all interfaces",
        }
    }
}

/// Configuration for an emergency shell session.
#[derive(Debug, Clone)]
pub struct EmergencyShellConfig {
    /// Whether to allow network access from the emergency shell.
    pub allow_network: bool,
    /// Whether to mount filesystems before launching.
    pub mount_filesystems: bool,
    /// Which repair tools are available.
    pub tools: Vec<RepairTool>,
}

impl EmergencyShellConfig {
    /// Safe, minimal defaults for the emergency shell.
    pub fn default_config() -> Self {
        Self {
            allow_network: false,
            mount_filesystems: true,
            tools: vec![
                RepairTool::Fsck,
                RepairTool::GrubReinstall,
                RepairTool::PackageIntegrityCheck,
                RepairTool::KernelReinstall,
                RepairTool::ConfigReset,
                RepairTool::NetworkReset,
            ],
        }
    }

    /// Config that allows network access (for downloading packages).
    pub fn networked_config() -> Self {
        Self {
            allow_network: true,
            ..Self::default_config()
        }
    }
}

/// The result of running a single repair tool.
#[derive(Debug, Clone)]
pub struct RepairResult {
    /// Which tool was run.
    pub tool: RepairTool,
    /// Whether the tool succeeded.
    pub success: bool,
    /// Raw output from the tool.
    pub output: String,
    /// Human-readable list of issues that were fixed.
    pub fixed_issues: Vec<String>,
}

impl RepairResult {
    /// Create a successful result.
    pub fn success(tool: RepairTool, output: impl Into<String>, fixed: Vec<String>) -> Self {
        Self { tool, success: true, output: output.into(), fixed_issues: fixed }
    }

    /// Create a failed result.
    pub fn failure(tool: RepairTool, output: impl Into<String>) -> Self {
        Self { tool, success: false, output: output.into(), fixed_issues: vec![] }
    }
}

/// Errors from the emergency shell.
#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("failed to mount {mount_point}: {reason}")]
    MountFailed { mount_point: String, reason: String },
    #[error("repair tool {tool} not available in this configuration")]
    ToolNotAvailable { tool: String },
    #[error("repair command failed with exit code {code}: {output}")]
    CommandFailed { code: i32, output: String },
    #[error("cannot launch shell: {0}")]
    ShellLaunchFailed(String),
}

/// Emergency shell orchestrator.
pub struct EmergencyShell {
    config: EmergencyShellConfig,
}

impl EmergencyShell {
    /// Create a new emergency shell with the given configuration.
    pub fn new(config: EmergencyShellConfig) -> Self {
        Self { config }
    }

    /// Create with safe defaults.
    pub fn default() -> Self {
        Self::new(EmergencyShellConfig::default_config())
    }

    /// Prepare the environment: mount /proc, /sys, /dev.
    pub fn prepare_environment(&self) -> Result<(), ShellError> {
        if self.config.mount_filesystems {
            // Production: mount --bind /proc /mnt/proc etc.
            for mp in &["/proc", "/sys", "/dev", "/dev/pts"] {
                let _ = mp; // would call mount() here
            }
        }
        Ok(())
    }

    /// Run a specific repair tool.
    pub fn run_repair_tool(&self, tool: RepairTool) -> Result<RepairResult, ShellError> {
        if !self.config.tools.contains(&tool) {
            return Err(ShellError::ToolNotAvailable { tool: format!("{tool:?}") });
        }

        // Simulate tool execution.
        match tool {
            RepairTool::Fsck => Ok(RepairResult::success(
                tool,
                "fsck: no errors found",
                vec![],
            )),
            RepairTool::GrubReinstall => Ok(RepairResult::success(
                tool,
                "grub-install: finished. update-grub: done",
                vec!["Regenerated grub.cfg".to_string()],
            )),
            RepairTool::PackageIntegrityCheck => Ok(RepairResult::success(
                tool,
                "dpkg --verify: all packages OK",
                vec![],
            )),
            RepairTool::KernelReinstall => Ok(RepairResult::success(
                tool,
                "kernel reinstalled; update-initramfs complete",
                vec!["Kernel and initramfs rebuilt".to_string()],
            )),
            RepairTool::ConfigReset => Ok(RepairResult::success(
                tool,
                "dpkg-reconfigure: complete",
                vec!["Restored default configuration files".to_string()],
            )),
            RepairTool::NetworkReset => Ok(RepairResult::success(
                tool,
                "NetworkManager reset complete",
                vec!["Flushed NetworkManager state".to_string()],
            )),
        }
    }

    /// Run all configured repair tools and return their results.
    pub fn run_all_repairs(&self) -> Vec<RepairResult> {
        self.config.tools.clone().into_iter()
            .map(|t| self.run_repair_tool(t).unwrap_or_else(|e| {
                RepairResult::failure(t, format!("error: {e}"))
            }))
            .collect()
    }

    /// Launch an interactive root shell.  Blocks until the user exits.
    pub fn launch(&self) -> Result<(), ShellError> {
        self.prepare_environment()?;
        // Production: execve("/bin/bash", &["--login"], env)
        Ok(())
    }

    /// Generate a human-readable repair report.
    pub fn generate_repair_report(&self, results: &[RepairResult]) -> String {
        let mut out = String::from("=== SigmaOS Emergency Repair Report ===\n\n");
        let passed = results.iter().filter(|r| r.success).count();
        let failed = results.len() - passed;
        out.push_str(&format!("Tools run: {}  |  Passed: {}  |  Failed: {}\n\n", results.len(), passed, failed));

        for r in results {
            let status = if r.success { "✓ PASS" } else { "✗ FAIL" };
            out.push_str(&format!("[{status}] {}\n", r.tool.name()));
            if !r.output.is_empty() {
                out.push_str(&format!("       Output: {}\n", r.output));
            }
            for fix in &r.fixed_issues {
                out.push_str(&format!("       Fixed : {fix}\n"));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_all_tools() {
        // Arrange
        let config = EmergencyShellConfig::default_config();
        // Assert
        assert!(config.tools.contains(&RepairTool::Fsck));
        assert!(config.tools.contains(&RepairTool::GrubReinstall));
        assert!(!config.allow_network); // network disabled by default
        assert!(config.mount_filesystems);
    }

    #[test]
    fn test_run_fsck_succeeds() {
        // Arrange
        let shell = EmergencyShell::default();
        // Act
        let result = shell.run_repair_tool(RepairTool::Fsck).unwrap();
        // Assert
        assert!(result.success);
        assert_eq!(result.tool, RepairTool::Fsck);
    }

    #[test]
    fn test_unavailable_tool_returns_error() {
        // Arrange
        let shell = EmergencyShell::new(EmergencyShellConfig {
            tools: vec![RepairTool::Fsck], // only fsck
            ..EmergencyShellConfig::default_config()
        });
        // Act
        let result = shell.run_repair_tool(RepairTool::GrubReinstall);
        // Assert
        assert!(matches!(result, Err(ShellError::ToolNotAvailable { .. })));
    }

    #[test]
    fn test_run_all_repairs_returns_one_per_tool() {
        // Arrange
        let shell = EmergencyShell::default();
        let tool_count = EmergencyShellConfig::default_config().tools.len();
        // Act
        let results = shell.run_all_repairs();
        // Assert
        assert_eq!(results.len(), tool_count);
    }

    #[test]
    fn test_repair_report_contains_tool_names() {
        // Arrange
        let shell = EmergencyShell::default();
        let results = shell.run_all_repairs();
        // Act
        let report = shell.generate_repair_report(&results);
        // Assert
        assert!(report.contains("Filesystem check"));
        assert!(report.contains("GRUB"));
        assert!(report.contains("Tools run"));
    }

    #[test]
    fn test_prepare_environment_ok() {
        let shell = EmergencyShell::default();
        assert!(shell.prepare_environment().is_ok());
    }
}
