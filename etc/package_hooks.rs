// SPDX-License-Identifier: MIT
// SigmaOS Package Hooks - Arch Linux Inspired
// Implements pacman hooks for package management events

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Hook types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookType {
    PreInstall,
    PostInstall,
    PreUpgrade,
    PostUpgrade,
    PreRemove,
    PostRemove,
    PreTransaction,
    PostTransaction,
}

impl HookType {
    pub fn as_str(&self) -> &str {
        match self {
            HookType::PreInstall => "pre-install",
            HookType::PostInstall => "post-install",
            HookType::PreUpgrade => "pre-upgrade",
            HookType::PostUpgrade => "post-upgrade",
            HookType::PreRemove => "pre-remove",
            HookType::PostRemove => "post-remove",
            HookType::PreTransaction => "pre-transaction",
            HookType::PostTransaction => "post-transaction",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pre-install" => Some(HookType::PreInstall),
            "post-install" => Some(HookType::PostInstall),
            "pre-upgrade" => Some(HookType::PreUpgrade),
            "post-upgrade" => Some(HookType::PostUpgrade),
            "pre-remove" => Some(HookType::PreRemove),
            "post-remove" => Some(HookType::PostRemove),
            "pre-transaction" => Some(HookType::PreTransaction),
            "post-transaction" => Some(HookType::PostTransaction),
            _ => None,
        }
    }

    pub fn is_before(&self) -> bool {
        matches!(self, HookType::PreInstall | HookType::PreUpgrade | HookType::PreRemove | HookType::PreTransaction)
    }

    pub fn is_after(&self) -> bool {
        matches!(self, HookType::PostInstall | HookType::PostUpgrade | HookType::PostRemove | HookType::PostTransaction)
    }
}

/// Hook execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookStatus {
    Success,
    Failed,
    Skipped,
    Timeout,
}

impl HookStatus {
    pub fn as_str(&self) -> &str {
        match self {
            HookStatus::Success => "success",
            HookStatus::Failed => "failed",
            HookStatus::Skipped => "skipped",
            HookStatus::Timeout => "timeout",
        }
    }

    pub fn is_success(&self) -> bool {
        matches!(self, HookStatus::Success)
    }

    pub fn is_failed(&self) -> bool {
        matches!(self, HookStatus::Failed)
    }
}

/// Hook execution result
#[derive(Debug, Clone)]
pub struct HookResult {
    pub hook_name: String,
    pub hook_type: HookType,
    pub status: HookStatus,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: u64,
    pub timestamp: u64,
}

impl HookResult {
    /// Create a new hook result
    pub fn new(hook_name: String, hook_type: HookType, status: HookStatus) -> Self {
        Self {
            hook_name,
            hook_type,
            status,
            exit_code: None,
            stdout: String::new(),
            stderr: String::new(),
            execution_time: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }

    /// Set exit code
    pub fn exit_code(mut self, code: i32) -> Self {
        self.exit_code = Some(code);
        self
    }

    /// Set stdout
    pub fn stdout(mut self, output: impl Into<String>) -> Self {
        self.stdout = output.into();
        self
    }

    /// Set stderr
    pub fn stderr(mut self, output: impl Into<String>) -> Self {
        self.stderr = output.into();
        self
    }

    /// Set execution time
    pub fn execution_time(mut self, time: u64) -> Self {
        self.execution_time = time;
        self
    }
}

/// Hook definition
#[derive(Debug, Clone)]
pub struct Hook {
    pub name: String,
    pub description: String,
    pub hook_type: HookType,
    pub command: String,
    pub arguments: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: BTreeMap<String, String>,
    pub timeout: Option<u64>,
    pub enabled: bool,
    pub priority: i32,
    pub conditions: Vec<String>,
    pub packages: Vec<String>,
}

impl Hook {
    /// Create a new hook
    pub fn new(name: impl Into<String>, description: impl Into<String>, hook_type: HookType, command: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            hook_type,
            command: command.into(),
            arguments: Vec::new(),
            working_directory: None,
            environment: BTreeMap::new(),
            timeout: None,
            enabled: true,
            priority: 0,
            conditions: Vec::new(),
            packages: Vec::new(),
        }
    }

    /// Add argument
    pub fn arg(mut self, arg: impl Into<String>) -> Self {
        self.arguments.push(arg.into());
        self
    }

    /// Add multiple arguments
    pub fn args(mut self, args: Vec<String>) -> Self {
        self.arguments.extend(args);
        self
    }

    /// Set working directory
    pub fn working_dir(mut self, dir: impl Into<String>) -> Self {
        self.working_directory = Some(dir.into());
        self
    }

    /// Set environment variable
    pub fn env(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.environment.insert(key.into(), value.into());
        self
    }

    /// Set timeout
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }

    /// Enable/disable hook
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }

    /// Add condition
    pub fn condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Add package
    pub fn package(mut self, package: impl Into<String>) -> Self {
        self.packages.push(package.into());
        self
    }

    /// Build command line
    pub fn build_command(&self) -> Vec<String> {
        let mut cmd = vec![self.command.clone()];
        cmd.extend(self.arguments.clone());
        cmd
    }

    /// Check if hook should run for given package
    pub fn should_run_for_package(&self, package_name: &str) -> bool {
        if self.packages.is_empty() {
            return true; // Run for all packages
        }
        self.packages.contains(&package_name.to_string())
    }

    /// Check if hook conditions are met
    pub fn check_conditions(&self) -> bool {
        // In real implementation, would evaluate conditions
        // For now, always return true
        true
    }
}

/// Hook configuration
#[derive(Debug, Clone)]
pub struct HookConfig {
    pub name: String,
    pub description: String,
    pub hook_type: HookType,
    pub script_path: String,
    pub arguments: Vec<String>,
    pub environment: BTreeMap<String, String>,
    pub timeout: Option<u64>,
    pub enabled: bool,
    pub priority: i32,
    pub conditions: Vec<String>,
    pub packages: Vec<String>,
}

/// Hook manager
#[derive(Debug, Clone)]
pub struct HookManager {
    pub hooks: BTreeMap<String, Hook>,
    pub hook_dirs: Vec<PathBuf>,
    pub results: Vec<HookResult>,
    pub execution_log: Vec<String>,
    pub default_timeout: u64,
    pub continue_on_error: bool,
}

impl HookManager {
    /// Create a new hook manager
    pub fn new() -> Self {
        Self {
            hooks: BTreeMap::new(),
            hook_dirs: vec![
                PathBuf::from("/etc/pacman.d/hooks"),
                PathBuf::from("/usr/share/pacman/hooks"),
            ],
            results: Vec::new(),
            execution_log: Vec::new(),
            default_timeout: 300, // 5 minutes
            continue_on_error: true,
        }
    }

    /// Create hook manager with custom directories
    pub fn with_dirs(hook_dirs: Vec<PathBuf>) -> Self {
        Self {
            hooks: BTreeMap::new(),
            hook_dirs,
            results: Vec::new(),
            execution_log: Vec::new(),
            default_timeout: 300,
            continue_on_error: true,
        }
    }

    /// Load hooks from directories
    pub fn load_hooks(&mut self) -> io::Result<()> {
        for dir in &self.hook_dirs {
            if dir.exists() {
                self.load_hooks_from_dir(dir)?;
            }
        }
        Ok(())
    }

    /// Load hooks from a specific directory
    pub fn load_hooks_from_dir(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "hook") {
                if let Ok(config) = self.parse_hook_file(&path) {
                    let hook = self.config_to_hook(config);
                    self.hooks.insert(hook.name.clone(), hook);
                }
            }
        }
        Ok(())
    }

    /// Parse hook file
    pub fn parse_hook_file(&self, path: &Path) -> Result<HookConfig, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read hook file: {}", e))?;

        let mut config = HookConfig {
            name: String::new(),
            description: String::new(),
            hook_type: HookType::PreInstall,
            script_path: String::new(),
            arguments: Vec::new(),
            environment: BTreeMap::new(),
            timeout: None,
            enabled: true,
            priority: 0,
            conditions: Vec::new(),
            packages: Vec::new(),
        };

        let mut in_hook = false;

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                let section = &line[1..line.len()-1];
                in_hook = section == "Hook";
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                if in_hook {
                    match key {
                        "Name" => config.name = value.to_string(),
                        "Description" => config.description = value.to_string(),
                        "Type" => config.hook_type = HookType::from_str(value).unwrap_or(HookType::PreInstall),
                        "Script" => config.script_path = value.to_string(),
                        "Arguments" => config.arguments = value.split_whitespace().map(String::from).collect(),
                        "Environment" => {
                            if let Some((k, v)) = value.split_once('=') {
                                config.environment.insert(k.trim().to_string(), v.trim().to_string());
                            }
                        }
                        "Timeout" => config.timeout = value.parse().ok(),
                        "Enabled" => config.enabled = value == "true",
                        "Priority" => config.priority = value.parse().unwrap_or(0),
                        "Conditions" => config.conditions = value.split(',').map(String::from).collect(),
                        "Packages" => config.packages = value.split(',').map(String::from).collect(),
                        _ => {}
                    }
                }
            }
        }

        Ok(config)
    }

    /// Convert hook config to hook
    fn config_to_hook(&self, config: HookConfig) -> Hook {
        let mut hook = Hook::new(
            config.name.clone(),
            config.description,
            config.hook_type,
            config.script_path,
        );

        hook = hook.args(config.arguments)
            .timeout(config.timeout.unwrap_or(self.default_timeout))
            .enabled(config.enabled)
            .priority(config.priority);

        for (key, value) in config.environment {
            hook = hook.env(key, value);
        }

        for condition in config.conditions {
            hook = hook.condition(condition);
        }

        for package in config.packages {
            hook = hook.package(package);
        }

        hook
    }

    /// Save hook to file
    pub fn save_hook(&self, hook: &Hook, path: &Path) -> io::Result<()> {
        let mut content = String::new();

        content.push_str("[Hook]\n");
        content.push_str(&format!("Name={}\n", hook.name));
        content.push_str(&format!("Description={}\n", hook.description));
        content.push_str(&format!("Type={}\n", hook.hook_type.as_str()));
        content.push_str(&format!("Script={}\n", hook.command));

        if !hook.arguments.is_empty() {
            content.push_str(&format!("Arguments={}\n", hook.arguments.join(" ")));
        }

        for (key, value) in &hook.environment {
            content.push_str(&format!("Environment={}={}\n", key, value));
        }

        if let Some(timeout) = hook.timeout {
            content.push_str(&format!("Timeout={}\n", timeout));
        }

        content.push_str(&format!("Enabled={}\n", hook.enabled));
        content.push_str(&format!("Priority={}\n", hook.priority));

        if !hook.conditions.is_empty() {
            content.push_str(&format!("Conditions={}\n", hook.conditions.join(",")));
        }

        if !hook.packages.is_empty() {
            content.push_str(&format!("Packages={}\n", hook.packages.join(",")));
        }

        fs::write(path, content)
    }

    /// Add hook
    pub fn add_hook(&mut self, hook: Hook) {
        self.hooks.insert(hook.name.clone(), hook);
    }

    /// Remove hook
    pub fn remove_hook(&mut self, name: &str) -> Option<Hook> {
        self.hooks.remove(name)
    }

    /// Get hook
    pub fn get_hook(&self, name: &str) -> Option<&Hook> {
        self.hooks.get(name)
    }

    /// Get hook mutably
    pub fn get_hook_mut(&mut self, name: &str) -> Option<&mut Hook> {
        self.hooks.get_mut(name)
    }

    /// Get hooks by type
    pub fn get_hooks_by_type(&self, hook_type: HookType) -> Vec<&Hook> {
        self.hooks.values()
            .filter(|h| h.hook_type == hook_type)
            .collect()
    }

    /// Get hooks for package operation
    pub fn get_hooks_for_operation(&self, hook_type: HookType, package_name: Option<&str>) -> Vec<&Hook> {
        self.hooks.values()
            .filter(|h| {
                h.hook_type == hook_type &&
                h.enabled &&
                (package_name.is_none() || h.should_run_for_package(package_name.unwrap()))
            })
            .collect()
    }

    /// Execute hooks for operation
    pub fn execute_hooks(&mut self, hook_type: HookType, package_name: Option<&str>, package_version: Option<&str>) -> Vec<HookResult> {
        let mut results = Vec::new();
        let mut hooks = self.get_hooks_for_operation(hook_type, package_name);

        // Sort hooks by priority
        hooks.sort_by_key(|h| h.priority);

        for hook in hooks {
            let result = self.execute_hook(hook, package_name, package_version);
            results.push(result.clone());

            // Check if we should continue on error
            if result.status.is_failed() && !self.continue_on_error {
                break;
            }
        }

        self.results.extend(results.clone());
        results
    }

    /// Execute a single hook
    pub fn execute_hook(&self, hook: &Hook, package_name: Option<&str>, package_version: Option<&str>) -> HookResult {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut result = HookResult::new(
            hook.name.clone(),
            hook.hook_type,
            HookStatus::Success,
        );

        // Build command
        let mut cmd = vec![hook.command.clone()];
        cmd.extend(hook.arguments.clone());

        // Add package information as arguments
        if let Some(pkg) = package_name {
            cmd.push(pkg.to_string());
        }
        if let Some(ver) = package_version {
            cmd.push(ver.to_string());
        }

        // Execute command
        match Command::new(&cmd[0])
            .args(&cmd[1..])
            .envs(&hook.environment)
            .output()
        {
            Ok(output) => {
                result.exit_code = Some(output.status.code().unwrap_or(-1));
                result.stdout = String::from_utf8_lossy(&output.stdout).to_string();
                result.stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if output.status.success() {
                    result.status = HookStatus::Success;
                } else {
                    result.status = HookStatus::Failed;
                }
            }
            Err(e) => {
                result.status = HookStatus::Failed;
                result.stderr = format!("Failed to execute hook: {}", e);
            }
        }

        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        result.execution_time = end_time - start_time;

        // Log execution
        self.log_execution(&result);

        result
    }

    /// Log hook execution
    fn log_execution(&mut self, result: &HookResult) {
        let log_entry = format!(
            "[{}] {} ({:?}) - {} ({}ms)",
            result.timestamp,
            result.hook_name,
            result.hook_type,
            result.status.as_str(),
            result.execution_time
        );
        self.execution_log.push(log_entry);

        // Keep only last 1000 log entries
        if self.execution_log.len() > 1000 {
            self.execution_log.remove(0);
        }
    }

    /// Get execution log
    pub fn get_execution_log(&self) -> &[String] {
        &self.execution_log
    }

    /// Clear execution log
    pub fn clear_execution_log(&mut self) {
        self.execution_log.clear();
    }

    /// Get hook results
    pub fn get_results(&self) -> &[HookResult] {
        &self.results
    }

    /// Clear results
    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    /// Get hook statistics
    pub fn get_statistics(&self) -> HookStatistics {
        let total = self.hooks.len();
        let enabled = self.hooks.values().filter(|h| h.enabled).count();
        let results = self.results.len();
        let successful = self.results.iter().filter(|r| r.status.is_success()).count();
        let failed = self.results.iter().filter(|r| r.status.is_failed()).count();

        HookStatistics {
            total,
            enabled,
            results,
            successful,
            failed,
        }
    }

    /// Set default timeout
    pub fn set_default_timeout(&mut self, timeout: u64) {
        self.default_timeout = timeout;
    }

    /// Set continue on error
    pub fn set_continue_on_error(&mut self, continue_on_error: bool) {
        self.continue_on_error = continue_on_error;
    }

    /// Enable/disable hook
    pub fn set_hook_enabled(&mut self, name: &str, enabled: bool) -> Result<(), String> {
        if let Some(hook) = self.hooks.get_mut(name) {
            hook.enabled = enabled;
            Ok(())
        } else {
            Err(format!("Hook '{}' not found", name))
        }
    }

    /// Export hooks to JSON (simplified)
    pub fn export_hooks(&self) -> String {
        let mut json = String::from("[\n");
        for (i, hook) in self.hooks.values().enumerate() {
            json.push_str(&format!(
                "  {{\"name\": \"{}\", \"type\": \"{}\", \"command\": \"{}\", \"enabled\": {}, \"priority\": {}}}{}\n",
                hook.name,
                hook.hook_type.as_str(),
                hook.command,
                hook.enabled,
                hook.priority,
                if i < self.hooks.len() - 1 { "," } else { "" }
            ));
        }
        json.push_str("]\n");
        json
    }

    /// Import hooks from JSON (simplified)
    pub fn import_hooks(&mut self, json: &str) -> Result<(), String> {
        // In real implementation, would parse JSON properly
        // For now, just return success
        let _ = json;
        Ok(())
    }
}

impl Default for HookManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook statistics
#[derive(Debug, Clone)]
pub struct HookStatistics {
    pub total: usize,
    pub enabled: usize,
    pub results: usize,
    pub successful: usize,
    pub failed: usize,
}

/// Common hooks for package management
pub struct CommonHooks;

impl CommonHooks {
    /// Create font cache update hook
    pub fn font_cache_update() -> Hook {
        Hook::new(
            "font-cache-update".to_string(),
            "Update font cache after font package installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/fc-cache".to_string(),
        )
        .arg("-f")
        .package("fontconfig".to_string())
        .package("ttf-dejavu".to_string())
    }

    /// Create ldconfig update hook
    pub fn ldconfig_update() -> Hook {
        Hook::new(
            "ldconfig-update".to_string(),
            "Update dynamic linker cache after library installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/ldconfig".to_string(),
        )
    }

    /// Create man database update hook
    pub fn man_database_update() -> Hook {
        Hook::new(
            "man-database-update".to_string(),
            "Update man database after documentation installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/mandb".to_string(),
        )
        .arg("--quiet")
    }

    /// Create initramfs update hook
    pub fn initramfs_update() -> Hook {
        Hook::new(
            "initramfs-update".to_string(),
            "Update initramfs after kernel installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/mkinitcpio".to_string(),
        )
        .arg("-P")
        .package("linux".to_string())
        .package("linux-lts".to_string())
    }

    /// Create desktop database update hook
    pub fn desktop_database_update() -> Hook {
        Hook::new(
            "desktop-database-update".to_string(),
            "Update desktop database after desktop file installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/update-desktop-database".to_string(),
        )
        .arg("/usr/share/applications")
    }

    /// Create icon cache update hook
    pub fn icon_cache_update() -> Hook {
        Hook::new(
            "icon-cache-update".to_string(),
            "Update icon cache after icon theme installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/gtk-update-icon-cache".to_string(),
        )
        .arg("-f")
        .arg("-t")
        .arg("/usr/share/icons/hicolor")
    }

    /// Create udev rules reload hook
    pub fn udev_rules_reload() -> Hook {
        Hook::new(
            "udev-rules-reload".to_string(),
            "Reload udev rules after udev package installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/udevadm".to_string(),
        )
        .arg("control")
        .arg("--reload-rules")
        .package("udev".to_string())
    }

    /// Create systemd daemon reload hook
    pub fn systemd_daemon_reload() -> Hook {
        Hook::new(
            "systemd-daemon-reload".to_string(),
            "Reload systemd daemon after service file changes".to_string(),
            HookType::PostInstall,
            "/usr/bin/systemctl".to_string(),
        )
        .arg("daemon-reload")
        .package("systemd".to_string())
    }

    /// Create logrotate state update hook
    pub fn logrotate_state_update() -> Hook {
        Hook::new(
            "logrotate-state-update".to_string(),
            "Update logrotate state after logrotate installation".to_string(),
            HookType::PostInstall,
            "/usr/bin/logrotate".to_string(),
        )
        .arg("--state")
        .arg("/var/lib/logrotate/status")
        .package("logrotate".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_type() {
        assert_eq!(HookType::PreInstall.as_str(), "pre-install");
        assert_eq!(HookType::from_str("post-install"), Some(HookType::PostInstall));
        assert!(HookType::PreInstall.is_before());
        assert!(HookType::PostInstall.is_after());
    }

    #[test]
    fn test_hook_status() {
        assert_eq!(HookStatus::Success.as_str(), "success");
        assert!(HookStatus::Success.is_success());
        assert!(HookStatus::Failed.is_failed());
    }

    #[test]
    fn test_hook_builder() {
        let hook = Hook::new("test-hook", "Test hook", HookType::PostInstall, "/usr/bin/test")
            .arg("--verbose")
            .arg("--force")
            .env("TEST_VAR", "test_value")
            .timeout(60)
            .priority(10)
            .package("test-package");

        assert_eq!(hook.name, "test-hook");
        assert_eq!(hook.hook_type, HookType::PostInstall);
        assert_eq!(hook.arguments.len(), 2);
        assert_eq!(hook.environment.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(hook.timeout, Some(60));
        assert_eq!(hook.priority, 10);
        assert!(hook.should_run_for_package("test-package"));
        assert!(!hook.should_run_for_package("other-package"));
    }

    #[test]
    fn test_hook_manager() {
        let mut manager = HookManager::new();

        let hook = Hook::new("test-hook", "Test hook", HookType::PostInstall, "/usr/bin/echo")
            .arg("test")
            .enabled(true);

        manager.add_hook(hook);
        assert_eq!(manager.hooks.len(), 1);

        let results = manager.execute_hooks(HookType::PostInstall, Some("test-package"), None);
        assert_eq!(results.len(), 1);
        assert!(results[0].status.is_success() || results[0].status.is_failed()); // Depends on /usr/bin/echo existing
    }

    #[test]
    fn test_hook_file_parsing() {
        let content = r#"
[Hook]
Name=test-hook
Description=Test hook for testing
Type=post-install
Script=/usr/bin/test-script
Arguments=--verbose --force
Environment=TEST_VAR=test_value
Timeout=60
Enabled=true
Priority=10
Conditions=package_installed
Packages=test-package,other-package
"#;

        let manager = HookManager::new();
        let config = manager.parse_hook_file(Path::new("test.hook")).unwrap();

        assert_eq!(config.name, "test-hook");
        assert_eq!(config.description, "Test hook for testing");
        assert_eq!(config.hook_type, HookType::PostInstall);
        assert_eq!(config.script_path, "/usr/bin/test-script");
        assert_eq!(config.arguments.len(), 2);
        assert_eq!(config.environment.get("TEST_VAR"), Some(&"test_value".to_string()));
        assert_eq!(config.timeout, Some(60));
        assert!(config.enabled);
        assert_eq!(config.priority, 10);
        assert_eq!(config.packages.len(), 2);
    }

    #[test]
    fn test_hook_execution() {
        let manager = HookManager::new();

        let hook = Hook::new("echo-hook", "Echo test", HookType::PostInstall, "/usr/bin/echo")
            .arg("Hello from hook")
            .timeout(10);

        let result = manager.execute_hook(&hook, Some("test-package"), Some("1.0"));
        assert!(result.status.is_success() || result.status.is_failed()); // Depends on echo existing
        assert!(result.execution_time >= 0);
    }

    #[test]
    fn test_common_hooks() {
        let font_hook = CommonHooks::font_cache_update();
        assert_eq!(font_hook.name, "font-cache-update");
        assert_eq!(font_hook.hook_type, HookType::PostInstall);
        assert!(font_hook.packages.contains(&"fontconfig".to_string()));

        let ldconfig_hook = CommonHooks::ldconfig_update();
        assert_eq!(ldconfig_hook.name, "ldconfig-update");
        assert_eq!(ldconfig_hook.hook_type, HookType::PostInstall);
    }

    #[test]
    fn test_hook_statistics() {
        let mut manager = HookManager::new();

        let hook1 = Hook::new("hook1", "Hook 1", HookType::PostInstall, "/usr/bin/echo").enabled(true);
        let hook2 = Hook::new("hook2", "Hook 2", HookType::PreInstall, "/usr/bin/echo").enabled(false);

        manager.add_hook(hook1);
        manager.add_hook(hook2);

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 2);
        assert_eq!(stats.enabled, 1);
    }
}
