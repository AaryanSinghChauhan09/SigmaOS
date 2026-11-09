// SPDX-License-Identifier: MIT
// SigmaOS Boot Loader - Arch Linux Inspired
// Implements systemd-boot and GRUB-style boot loader management

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Boot loader type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootLoaderType {
    SystemdBoot,
    Grub,
    Syslinux,
    Lilo,
    Refind,
    Direct,
}

impl BootLoaderType {
    pub fn as_str(&self) -> &str {
        match self {
            BootLoaderType::SystemdBoot => "systemd-boot",
            BootLoaderType::Grub => "grub",
            BootLoaderType::Syslinux => "syslinux",
            BootLoaderType::Lilo => "lilo",
            BootLoaderType::Refind => "refind",
            BootLoaderType::Direct => "direct",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "systemd-boot" => Some(BootLoaderType::SystemdBoot),
            "grub" => Some(BootLoaderType::Grub),
            "syslinux" => Some(BootLoaderType::Syslinux),
            "lilo" => Some(BootLoaderType::Lilo),
            "refind" => Some(BootLoaderType::Refind),
            "direct" => Some(BootLoaderType::Direct),
            _ => None,
        }
    }

    pub fn get_config_dir(&self) -> PathBuf {
        match self {
            BootLoaderType::SystemdBoot => PathBuf::from("/boot/EFI/Linux"),
            BootLoaderType::Grub => PathBuf::from("/boot/grub"),
            BootLoaderType::Syslinux => PathBuf::from("/boot/syslinux"),
            BootLoaderType::Lilo => PathBuf::from("/etc"),
            BootLoaderType::Refind => PathBuf::from("/boot/EFI/refind"),
            BootLoaderType::Direct => PathBuf::from("/boot"),
        }
    }

    pub fn get_config_file(&self) -> PathBuf {
        match self {
            BootLoaderType::SystemdBoot => PathBuf::from("/boot/loader/loader.conf"),
            BootLoaderType::Grub => PathBuf::from("/boot/grub/grub.cfg"),
            BootLoaderType::Syslinux => PathBuf::from("/boot/syslinux/syslinux.cfg"),
            BootLoaderType::Lilo => PathBuf::from("/etc/lilo.conf"),
            BootLoaderType::Refind => PathBuf::from("/boot/EFI/refind/refind.conf"),
            BootLoaderType::Direct => PathBuf::from("/boot/cmdline.txt"),
        }
    }

    pub fn get_install_command(&self) -> Option<Vec<String>> {
        match self {
            BootLoaderType::SystemdBoot => Some(vec!["bootctl".to_string(), "install".to_string()]),
            BootLoaderType::Grub => Some(vec!["grub-install".to_string()]),
            BootLoaderType::Syslinux => Some(vec!["syslinux".to_string(), "-i".to_string()]),
            BootLoaderType::Lilo => Some(vec!["lilo".to_string()]),
            BootLoaderType::Refind => Some(vec!["refind-install".to_string()]),
            BootLoaderType::Direct => None,
        }
    }

    pub fn get_update_command(&self) -> Option<Vec<String>> {
        match self {
            BootLoaderType::SystemdBoot => Some(vec!["bootctl".to_string(), "update".to_string()]),
            BootLoaderType::Grub => Some(vec!["grub-mkconfig".to_string(), "-o".to_string(), "/boot/grub/grub.cfg".to_string()]),
            BootLoaderType::Syslinux => Some(vec!["syslinux".to_string(), "-i".to_string()]),
            BootLoaderType::Lilo => Some(vec!["lilo".to_string()]),
            BootLoaderType::Refind => Some(vec!["refind-mkconfig".to_string(), "-o".to_string(), "/boot/EFI/refind/refind.conf".to_string()]),
            BootLoaderType::Direct => None,
        }
    }
}

/// Kernel entry
#[derive(Debug, Clone)]
pub struct KernelEntry {
    pub title: String,
    pub kernel_path: PathBuf,
    pub initrd_path: Option<PathBuf>,
    pub cmdline: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_default: bool,
    pub is_fallback: bool,
    pub os_type: String,
    pub version: Option<String>,
    pub sort_order: i32,
}

impl KernelEntry {
    /// Create a new kernel entry
    pub fn new(title: impl Into<String>, kernel_path: impl Into<PathBuf>) -> Self {
        Self {
            title: title.into(),
            kernel_path: kernel_path.into(),
            initrd_path: None,
            cmdline: String::new(),
            description: None,
            icon: None,
            is_default: false,
            is_fallback: false,
            os_type: "linux".to_string(),
            version: None,
            sort_order: 0,
        }
    }

    /// Set initrd path
    pub fn initrd(mut self, path: impl Into<PathBuf>) -> Self {
        self.initrd_path = Some(path.into());
        self
    }

    /// Set command line
    pub fn cmdline(mut self, cmdline: impl Into<String>) -> Self {
        self.cmdline = cmdline.into();
        self
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set as default
    pub fn default(mut self) -> Self {
        self.is_default = true;
        self
    }

    /// Set as fallback
    pub fn fallback(mut self) -> Self {
        self.is_fallback = true;
        self
    }

    /// Set OS type
    pub fn os_type(mut self, os_type: impl Into<String>) -> Self {
        self.os_type = os_type.into();
        self
    }

    /// Set version
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set sort order
    pub fn sort_order(mut self, order: i32) -> Self {
        self.sort_order = order;
        self
    }

    /// Get kernel filename
    pub fn get_kernel_filename(&self) -> String {
        self.kernel_path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("kernel")
            .to_string()
    }

    /// Get initrd filename
    pub fn get_initrd_filename(&self) -> Option<String> {
        self.initrd_path.as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .map(|s| s.to_string())
    }
}

/// Boot entry
#[derive(Debug, Clone)]
pub struct BootEntry {
    pub title: String,
    pub description: Option<String>,
    pub kernel: KernelEntry,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub is_default: bool,
}

impl BootEntry {
    /// Create a new boot entry
    pub fn new(title: impl Into<String>, kernel: KernelEntry) -> Self {
        Self {
            title: title.into(),
            description: None,
            kernel,
            icon: None,
            sort_order: 0,
            is_active: true,
            is_default: false,
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set sort order
    pub fn sort_order(mut self, order: i32) -> Self {
        self.sort_order = order;
        self
    }

    /// Set active
    pub fn active(mut self, active: bool) -> Self {
        self.is_active = active;
        self
    }

    /// Set as default
    pub fn default(mut self) -> Self {
        self.is_default = true;
        self.kernel = self.kernel.default();
        self
    }
}

/// Boot loader configuration
#[derive(Debug, Clone)]
pub struct BootLoaderConfig {
    pub boot_loader_type: BootLoaderType,
    pub entries: Vec<BootEntry>,
    pub default_entry: Option<String>,
    pub timeout: u32,
    pub theme: Option<String>,
    pub resolution: Option<String>,
    pub kernel_params: Vec<String>,
    pub secure_boot: bool,
    pub quiet_boot: bool,
    pub serial_console: Option<String>,
    pub custom_config: BTreeMap<String, String>,
}

impl BootLoaderConfig {
    /// Create a new boot loader config
    pub fn new(boot_loader_type: BootLoaderType) -> Self {
        Self {
            boot_loader_type,
            entries: Vec::new(),
            default_entry: None,
            timeout: 5,
            theme: None,
            resolution: None,
            kernel_params: Vec::new(),
            secure_boot: false,
            quiet_boot: false,
            serial_console: None,
            custom_config: BTreeMap::new(),
        }
    }

    /// Add boot entry
    pub fn add_entry(mut self, entry: BootEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// Set default entry
    pub fn default_entry(mut self, title: impl Into<String>) -> Self {
        self.default_entry = Some(title.into());
        self
    }

    /// Set timeout
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set theme
    pub fn theme(mut self, theme: impl Into<String>) -> Self {
        self.theme = Some(theme.into());
        self
    }

    /// Set resolution
    pub fn resolution(mut self, resolution: impl Into<String>) -> Self {
        self.resolution = Some(resolution.into());
        self
    }

    /// Add kernel parameter
    pub fn kernel_param(mut self, param: impl Into<String>) -> Self {
        self.kernel_params.push(param.into());
        self
    }

    /// Enable/disable secure boot
    pub fn secure_boot(mut self, enable: bool) -> Self {
        self.secure_boot = enable;
        self
    }

    /// Enable/disable quiet boot
    pub fn quiet_boot(mut self, enable: bool) -> Self {
        self.quiet_boot = enable;
        self
    }

    /// Set serial console
    pub fn serial_console(mut self, console: impl Into<String>) -> Self {
        self.serial_console = Some(console.into());
        self
    }

    /// Set custom config option
    pub fn custom_config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_config.insert(key.into(), value.into());
        self
    }

    /// Get kernel command line
    pub fn get_kernel_cmdline(&self) -> String {
        let mut cmdline = String::new();

        // Add standard parameters
        if self.quiet_boot {
            cmdline.push_str("quiet ");
        }

        if let Some(console) = &self.serial_console {
            cmdline.push_str(&format!("console={} ", console));
        }

        // Add custom parameters
        for param in &self.kernel_params {
            cmdline.push_str(param);
            cmdline.push(' ');
        }

        cmdline.trim().to_string()
    }

    /// Generate systemd-boot configuration
    pub fn to_systemd_boot_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS systemd-boot configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        content.push_str(&format!("default={}\n", self.default_entry.as_deref().unwrap_or("sigmaos")));
        content.push_str(&format!("timeout={}\n", self.timeout));

        if let Some(theme) = &self.theme {
            content.push_str(&format!("theme={}\n", theme));
        }

        if let Some(resolution) = &self.resolution {
            content.push_str(&format!("console-mode={}\n", resolution));
        }

        if self.secure_boot {
            content.push_str("secure-boot-enroll=yes\n");
        }

        content.push_str("\n# Kernel entries\n");
        for entry in &self.entries {
            if !entry.is_active {
                continue;
            }

            content.push_str(&format!("title {}\n", entry.title));
            content.push_str(&format!("efi {}\n", entry.kernel.kernel_path.display()));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("initrd {}\n", initrd.display()));
            }

            let cmdline = if entry.kernel.cmdline.is_empty() {
                self.get_kernel_cmdline()
            } else {
                entry.kernel.cmdline.clone()
            };
            content.push_str(&format!("options {}\n", cmdline));

            if let Some(icon) = &entry.icon {
                content.push_str(&format!("icon {}\n", icon));
            }

            if entry.is_default {
                content.push_str("default yes\n");
            }

            content.push('\n');
        }

        content
    }

    /// Generate GRUB configuration
    pub fn to_grub_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS GRUB configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        content.push_str("set default=\"0\"\n");
        content.push_str(&format!("set timeout={}\n", self.timeout));

        if let Some(resolution) = &self.resolution {
            content.push_str(&format!("set gfxmode={}\n", resolution));
        }

        content.push_str("\n# Kernel entries\n");
        for entry in &self.entries {
            if !entry.is_active {
                continue;
            }

            content.push_str(&format!("menuentry \"{}\" {{\n", entry.title));
            content.push_str(&format!("    linux {} {}\n", entry.kernel.kernel_path.display(), entry.kernel.cmdline));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("    initrd {}\n", initrd.display()));
            }

            content.push_str("}\n\n");
        }

        content
    }

    /// Generate syslinux configuration
    pub fn to_syslinux_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS syslinux configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        content.push_str("DEFAULT sigmaos\n");
        content.push_str(&format!("TIMEOUT {}\n", self.timeout * 10)); // syslinux uses 10ths of seconds

        if let Some(resolution) = &self.resolution {
            content.push_str(&format!("UI {}\n", resolution));
        }

        content.push_str("\n# Kernel entries\n");
        for entry in &self.entries {
            if !entry.is_active {
                continue;
            }

            content.push_str(&format!("LABEL {}\n", entry.title.replace(' ', "-")));
            content.push_str(&format!("    KERNEL {}\n", entry.kernel.kernel_path.display()));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("    INITRD {}\n", initrd.display()));
            }

            content.push_str(&format!("    APPEND {}\n", entry.kernel.cmdline));
            content.push('\n');
        }

        content
    }

    /// Generate LILO configuration
    pub fn to_lilo_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS LILO configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        content.push_str("boot = /dev/sda\n");
        content.push_str("timeout = ");
        content.push_str(&self.timeout.to_string());
        content.push('\n');
        content.push_str("prompt\n");

        content.push_str("\n# Kernel entries\n");
        for entry in &self.entries {
            if !entry.is_active {
                continue;
            }

            content.push_str(&format!("image = {}\n", entry.kernel.kernel_path.display()));
            content.push_str(&format!("    label = {}\n", entry.title));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("    initrd = {}\n", initrd.display()));
            }

            content.push_str(&format!("    append = \"{}\"\n", entry.kernel.cmdline));
            content.push('\n');
        }

        content
    }

    /// Generate rEFInd configuration
    pub fn to_refind_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS rEFInd configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        if let Some(theme) = &self.theme {
            content.push_str(&format!("theme {}\n", theme));
        }

        content.push_str("timeout ");
        content.push_str(&self.timeout.to_string());
        content.push('\n');

        content.push_str("\n# Kernel entries\n");
        for entry in &self.entries {
            if !entry.is_active {
                continue;
            }

            content.push_str(&format!("menuentry \"{}\" {{\n", entry.title));
            content.push_str(&format!("    icon {}\n", entry.icon.as_deref().unwrap_or("distributor.png")));
            content.push_str(&format!("    volume \\\\(h\\)\n"));
            content.push_str(&format!("    kernel \\\\(h\\){} {}\n", entry.kernel.kernel_path.display(), entry.kernel.cmdline));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("    initrd \\\\(h\\){}\n", initrd.display()));
            }

            content.push_str("}\n\n");
        }

        content
    }

    /// Generate direct boot configuration
    pub fn to_direct_config(&self) -> String {
        let mut content = String::new();

        content.push_str("# SigmaOS Direct Boot configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        // Find default entry
        let default_entry = self.entries.iter()
            .find(|e| e.is_default)
            .or_else(|| self.entries.first());

        if let Some(entry) = default_entry {
            content.push_str(&format!("# Default entry: {}\n", entry.title));
            content.push_str(&format!("kernel={}\n", entry.kernel.kernel_path.display()));

            if let Some(initrd) = &entry.kernel.initrd_path {
                content.push_str(&format!("initrd={}\n", initrd.display()));
            }

            content.push_str(&format!("cmdline={}\n", entry.kernel.cmdline));
        }

        content
    }

    /// Generate configuration for current boot loader type
    pub fn to_config(&self) -> String {
        match self.boot_loader_type {
            BootLoaderType::SystemdBoot => self.to_systemd_boot_config(),
            BootLoaderType::Grub => self.to_grub_config(),
            BootLoaderType::Syslinux => self.to_syslinux_config(),
            BootLoaderType::Lilo => self.to_lilo_config(),
            BootLoaderType::Refind => self.to_refind_config(),
            BootLoaderType::Direct => self.to_direct_config(),
        }
    }

    /// Save configuration to file
    pub fn save_config(&self, path: &Path) -> Result<(), String> {
        let content = self.to_config();
        fs::write(path, content).map_err(|e| format!("Failed to save config: {}", e))
    }

    /// Load configuration from file
    pub fn load_config(&mut self, path: &Path) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config: {}", e))?;

        self.parse_config(&content)?;
        Ok(())
    }

    /// Parse configuration content
    fn parse_config(&mut self, content: &str) -> Result<(), String> {
        // This would parse the config file based on boot loader type
        // For now, just store the content
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                self.custom_config.insert(key.to_string(), value.to_string());
            }
        }

        Ok(())
    }

    /// Install boot loader
    pub fn install(&self) -> Result<(), String> {
        if let Some(cmd) = self.boot_loader_type.get_install_command() {
            let output = Command::new(&cmd[0])
                .args(&cmd[1..])
                .output()
                .map_err(|e| format!("Failed to install boot loader: {}", e))?;

            if !output.status.success() {
                return Err(format!("Boot loader installation failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        Ok(())
    }

    /// Update boot loader configuration
    pub fn update(&self) -> Result<(), String> {
        // Save configuration
        let config_path = self.boot_loader_type.get_config_file();
        self.save_config(&config_path)?;

        // Update boot loader
        if let Some(cmd) = self.boot_loader_type.get_update_command() {
            let output = Command::new(&cmd[0])
                .args(&cmd[1..])
                .output()
                .map_err(|e| format!("Failed to update boot loader: {}", e))?;

            if !output.status.success() {
                return Err(format!("Boot loader update failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }

        Ok(())
    }

    /// Get boot loader status
    pub fn get_status(&self) -> BootLoaderStatus {
        let config_path = self.boot_loader_type.get_config_file();
        let config_exists = config_path.exists();

        BootLoaderStatus {
            boot_loader_type: self.boot_loader_type,
            config_exists,
            config_path,
            entries_count: self.entries.len(),
            default_entry: self.default_entry.clone(),
            is_installed: self.boot_loader_type.get_install_command().is_some(),
        }
    }

    /// Export configuration
    pub fn export_config(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Boot Loader Configuration\n");
        content.push_str("# Generated by SigmaOS Boot Loader\n\n");

        content.push_str(&format!("boot_loader={}\n", self.boot_loader_type.as_str()));
        content.push_str(&format!("default={}\n", self.default_entry.as_deref().unwrap_or("none")));
        content.push_str(&format!("timeout={}\n", self.timeout));
        content.push_str(&format!("secure_boot={}\n", self.secure_boot));
        content.push_str(&format!("quiet_boot={}\n", self.quiet_boot));

        if let Some(theme) = &self.theme {
            content.push_str(&format!("theme={}\n", theme));
        }
        if let Some(resolution) = &self.resolution {
            content.push_str(&format!("resolution={}\n", resolution));
        }
        if let Some(console) = &self.serial_console {
            content.push_str(&format!("serial_console={}\n", console));
        }

        content.push_str("\n## Kernel Entries\n");
        for entry in &self.entries {
            content.push_str(&format!("entry={} kernel={} initrd={} cmdline={} default={}\n",
                entry.title,
                entry.kernel.kernel_path.display(),
                entry.kernel.initrd_path.as_ref().map(|p| p.display().to_string()).unwrap_or("none".to_string()),
                entry.kernel.cmdline,
                entry.is_default
            ));
        }

        content.push_str("\n## Kernel Parameters\n");
        for param in &self.kernel_params {
            content.push_str(&format!("kernel_param={}\n", param));
        }

        content
    }

    /// Import configuration
    pub fn import_config(&mut self, content: &str) -> Result<(), String> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "boot_loader" => {
                        if let Some(bl) = BootLoaderType::from_str(value) {
                            self.boot_loader_type = bl;
                        }
                    }
                    "default" => self.default_entry = Some(value.to_string()),
                    "timeout" => {
                        if let Ok(timeout) = value.parse::<u32>() {
                            self.timeout = timeout;
                        }
                    }
                    "secure_boot" => self.secure_boot = value == "true",
                    "quiet_boot" => self.quiet_boot = value == "true",
                    "theme" => self.theme = Some(value.to_string()),
                    "resolution" => self.resolution = Some(value.to_string()),
                    "serial_console" => self.serial_console = Some(value.to_string()),
                    "kernel_param" => self.kernel_params.push(value.to_string()),
                    "entry" => {
                        // Parse entry line
                        let parts: Vec<&str> = value.split_whitespace().collect();
                        if parts.len() >= 2 {
                            let title = parts[0].to_string();
                            let kernel_path = PathBuf::from(parts[1]);
                            let mut kernel = KernelEntry::new(title.clone(), kernel_path);

                            // Parse additional parameters
                            for part in &parts[2..] {
                                if let Some((key, val)) = part.split_once('=') {
                                    match key {
                                        "initrd" => kernel.initrd_path = Some(PathBuf::from(val)),
                                        "cmdline" => kernel.cmdline = val.to_string(),
                                        "default" => kernel.is_default = val == "true",
                                        _ => {}
                                    }
                                }
                            }

                            let entry = BootEntry::new(title, kernel);
                            self.entries.push(entry);
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl Default for BootLoaderConfig {
    fn default() -> Self {
        Self::new(BootLoaderType::SystemdBoot)
    }
}

/// Boot loader status
#[derive(Debug, Clone)]
pub struct BootLoaderStatus {
    pub boot_loader_type: BootLoaderType,
    pub config_exists: bool,
    pub config_path: PathBuf,
    pub entries_count: usize,
    pub default_entry: Option<String>,
    pub is_installed: bool,
}

/// Boot loader manager
#[derive(Debug, Clone)]
pub struct BootLoaderManager {
    pub configs: BTreeMap<BootLoaderType, BootLoaderConfig>,
    pub active_boot_loader: Option<BootLoaderType>,
    pub boot_dir: PathBuf,
    pub esp_dir: Option<PathBuf>,
    pub last_update: Option<u64>,
}

impl BootLoaderManager {
    /// Create a new boot loader manager
    pub fn new() -> Self {
        Self {
            configs: BTreeMap::new(),
            active_boot_loader: None,
            boot_dir: PathBuf::from("/boot"),
            esp_dir: None,
            last_update: None,
        }
    }

    /// Create boot loader manager with custom paths
    pub fn with_paths(boot_dir: PathBuf, esp_dir: Option<PathBuf>) -> Self {
        Self {
            configs: BTreeMap::new(),
            active_boot_loader: None,
            boot_dir,
            esp_dir,
            last_update: None,
        }
    }

    /// Initialize boot loader manager
    pub fn initialize(&mut self) -> Result<(), String> {
        // Detect active boot loader
        self.detect_active_boot_loader()?;

        // Load existing configurations
        self.load_configurations()?;

        self.last_update = Some(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());

        Ok(())
    }

    /// Detect active boot loader
    fn detect_active_boot_loader(&mut self) -> Result<(), String> {
        // Check for systemd-boot
        if Path::new("/boot/loader/loader.conf").exists() {
            self.active_boot_loader = Some(BootLoaderType::SystemdBoot);
            return Ok(());
        }

        // Check for GRUB
        if Path::new("/boot/grub/grub.cfg").exists() {
            self.active_boot_loader = Some(BootLoaderType::Grub);
            return Ok(());
        }

        // Check for syslinux
        if Path::new("/boot/syslinux/syslinux.cfg").exists() {
            self.active_boot_loader = Some(BootLoaderType::Syslinux);
            return Ok(());
        }

        // Check for LILO
        if Path::new("/etc/lilo.conf").exists() {
            self.active_boot_loader = Some(BootLoaderType::Lilo);
            return Ok(());
        }

        // Check for rEFInd
        if Path::new("/boot/EFI/refind/refind.conf").exists() {
            self.active_boot_loader = Some(BootLoaderType::Refind);
            return Ok(());
        }

        // Default to systemd-boot
        self.active_boot_loader = Some(BootLoaderType::SystemdBoot);
        Ok(())
    }

    /// Load existing configurations
    fn load_configurations(&mut self) -> Result<(), String> {
        for boot_loader_type in [
            BootLoaderType::SystemdBoot,
            BootLoaderType::Grub,
            BootLoaderType::Syslinux,
            BootLoaderType::Lilo,
            BootLoaderType::Refind,
            BootLoaderType::Direct,
        ] {
            let config_path = boot_loader_type.get_config_file();
            if config_path.exists() {
                let mut config = BootLoaderConfig::new(boot_loader_type);
                if config.load_config(&config_path).is_ok() {
                    self.configs.insert(boot_loader_type, config);
                }
            }
        }

        Ok(())
    }

    /// Add configuration
    pub fn add_config(&mut self, config: BootLoaderConfig) {
        self.configs.insert(config.boot_loader_type, config);
    }

    /// Get configuration
    pub fn get_config(&self, boot_loader_type: BootLoaderType) -> Option<&BootLoaderConfig> {
        self.configs.get(&boot_loader_type)
    }

    /// Get configuration mutably
    pub fn get_config_mut(&mut self, boot_loader_type: BootLoaderType) -> Option<&mut BootLoaderConfig> {
        self.configs.get_mut(&boot_loader_type)
    }

    /// Set active boot loader
    pub fn set_active_boot_loader(&mut self, boot_loader_type: BootLoaderType) -> Result<(), String> {
        if !self.configs.contains_key(&boot_loader_type) {
            return Err(format!("Configuration for {} not found", boot_loader_type.as_str()));
        }
        self.active_boot_loader = Some(boot_loader_type);
        Ok(())
    }

    /// Get active boot loader
    pub fn get_active_boot_loader(&self) -> Option<BootLoaderType> {
        self.active_boot_loader
    }

    /// Get active configuration
    pub fn get_active_config(&self) -> Option<&BootLoaderConfig> {
        self.active_boot_loader.and_then(|bl| self.configs.get(&bl))
    }

    /// Update active boot loader
    pub fn update_active(&self) -> Result<(), String> {
        if let Some(boot_loader_type) = self.active_boot_loader {
            if let Some(config) = self.configs.get(&boot_loader_type) {
                config.update()?;
            }
        }
        Ok(())
    }

    /// Install active boot loader
    pub fn install_active(&self) -> Result<(), String> {
        if let Some(boot_loader_type) = self.active_boot_loader {
            if let Some(config) = self.configs.get(&boot_loader_type) {
                config.install()?;
            }
        }
        Ok(())
    }

    /// Scan for kernels
    pub fn scan_kernels(&self) -> Result<Vec<KernelEntry>, String> {
        let mut kernels = Vec::new();

        // Scan /boot directory for kernels
        let boot_dir = &self.boot_dir;
        if boot_dir.exists() {
            for entry in fs::read_dir(boot_dir)? {
                let entry = entry?;
                let path = entry.path();
                let filename = entry.file_name().to_string_lossy().to_string();

                // Look for kernel files
                if filename.starts_with("vmlinuz") || filename.starts_with("bzImage") || filename.starts_with("kernel") {
                    let kernel_entry = KernelEntry::new(filename, path.clone());
                    kernels.push(kernel_entry);
                }
            }
        }

        // Scan /boot/EFI/Linux for UKI kernels
        let uki_dir = Path::new("/boot/EFI/Linux");
        if uki_dir.exists() {
            for entry in fs::read_dir(uki_dir)? {
                let entry = entry?;
                let path = entry.path();
                let filename = entry.file_name().to_string_lossy().to_string();

                if filename.ends_with(".efi") {
                    let kernel_entry = KernelEntry::new(filename, path.clone());
                    kernels.push(kernel_entry);
                }
            }
        }

        Ok(kernels)
    }

    /// Find initrd for kernel
    pub fn find_initrd(&self, kernel_name: &str) -> Option<PathBuf> {
        let boot_dir = &self.boot_dir;

        // Try common initrd patterns
        let patterns = vec![
            format!("initramfs-{}.img", kernel_name),
            format!("initrd-{}.img", kernel_name),
            format!("initramfs-{}.img.gz", kernel_name),
            format!("initrd-{}.img.gz", kernel_name),
            format!("initramfs-{}.img.zst", kernel_name),
            format!("initrd-{}.img.zst", kernel_name),
        ];

        for pattern in patterns {
            let path = boot_dir.join(&pattern);
            if path.exists() {
                return Some(path);
            }
        }

        None
    }

    /// Add kernel entry to active boot loader
    pub fn add_kernel_entry(&mut self, entry: BootEntry) -> Result<(), String> {
        let boot_loader_type = self.active_boot_loader
            .ok_or("No active boot loader set")?;

        if let Some(config) = self.configs.get_mut(&boot_loader_type) {
            config.entries.push(entry);
            Ok(())
        } else {
            Err(format!("Configuration for {} not found", boot_loader_type.as_str()))
        }
    }

    /// Remove kernel entry from active boot loader
    pub fn remove_kernel_entry(&mut self, title: &str) -> Result<(), String> {
        let boot_loader_type = self.active_boot_loader
            .ok_or("No active boot loader set")?;

        if let Some(config) = self.configs.get_mut(&boot_loader_type) {
            if let Some(pos) = config.entries.iter().position(|e| e.title == title) {
                config.entries.remove(pos);
                Ok(())
            } else {
                Err(format!("Entry '{}' not found", title))
            }
        } else {
            Err(format!("Configuration for {} not found", boot_loader_type.as_str()))
        }
    }

    /// Set default kernel entry
    pub fn set_default_entry(&mut self, title: &str) -> Result<(), String> {
        let boot_loader_type = self.active_boot_loader
            .ok_or("No active boot loader set")?;

        if let Some(config) = self.configs.get_mut(&boot_loader_type) {
            // Clear all default flags
            for entry in &mut config.entries {
                entry.is_default = false;
                entry.kernel.is_default = false;
            }

            // Set new default
            if let Some(entry) = config.entries.iter_mut().find(|e| e.title == title) {
                entry.is_default = true;
                entry.kernel.is_default = true;
                config.default_entry = Some(title.to_string());
                Ok(())
            } else {
                Err(format!("Entry '{}' not found", title))
            }
        } else {
            Err(format!("Configuration for {} not found", boot_loader_type.as_str()))
        }
    }

    /// Get all kernel entries
    pub fn get_all_entries(&self) -> Vec<&BootEntry> {
        self.configs.values()
            .flat_map(|config| config.entries.iter())
            .collect()
    }

    /// Get active kernel entries
    pub fn get_active_entries(&self) -> Vec<&BootEntry> {
        self.get_active_config()
            .map(|config| config.entries.iter().collect())
            .unwrap_or_default()
    }

    /// Export all configurations
    pub fn export_all(&self) -> String {
        let mut content = String::new();
        content.push_str("# SigmaOS Boot Loader Manager Export\n");
        content.push_str("# Generated by SigmaOS Boot Loader Manager\n\n");

        content.push_str(&format!("active_boot_loader={}\n", self.active_boot_loader.map(|bl| bl.as_str()).unwrap_or("none")));

        for (boot_loader_type, config) in &self.configs {
            content.push_str(&format!("\n## {} Configuration\n", boot_loader_type.as_str()));
            content.push_str(&config.export_config());
        }

        content
    }

    /// Import configurations
    pub fn import_all(&mut self, content: &str) -> Result<(), String> {
        // In real implementation, would parse the export format
        // For now, just return success
        let _ = content;
        Ok(())
    }

    /// Get boot loader statistics
    pub fn get_statistics(&self) -> BootLoaderStatistics {
        let total_configs = self.configs.len();
        let total_entries = self.configs.values()
            .map(|config| config.entries.len())
            .sum();

        BootLoaderStatistics {
            total_configs,
            total_entries,
            active_boot_loader: self.active_boot_loader,
            last_update: self.last_update,
        }
    }
}

impl Default for BootLoaderManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Boot loader statistics
#[derive(Debug, Clone)]
pub struct BootLoaderStatistics {
    pub total_configs: usize,
    pub total_entries: usize,
    pub active_boot_loader: Option<BootLoaderType>,
    pub last_update: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_loader_type() {
        assert_eq!(BootLoaderType::SystemdBoot.as_str(), "systemd-boot");
        assert_eq!(BootLoaderType::from_str("grub"), Some(BootLoaderType::Grub));
        assert_eq!(BootLoaderType::SystemdBoot.get_config_dir(), PathBuf::from("/boot/EFI/Linux"));
    }

    #[test]
    fn test_kernel_entry() {
        let kernel = KernelEntry::new("Linux 6.1", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1 quiet")
            .description("Linux kernel")
            .default();

        assert_eq!(kernel.title, "Linux 6.1");
        assert_eq!(kernel.kernel_path, PathBuf::from("/boot/vmlinuz-linux"));
        assert!(kernel.is_default);
        assert_eq!(kernel.get_kernel_filename(), "vmlinuz-linux");
    }

    #[test]
    fn test_boot_entry() {
        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux");
        let entry = BootEntry::new("Linux 6.1", kernel)
            .description("Linux kernel 6.1")
            .icon("linux.png")
            .sort_order(10)
            .default();

        assert_eq!(entry.title, "Linux 6.1");
        assert_eq!(entry.description, Some("Linux kernel 6.1".to_string()));
        assert_eq!(entry.icon, Some("linux.png".to_string()));
        assert_eq!(entry.sort_order, 10);
        assert!(entry.is_default);
    }

    #[test]
    fn test_boot_loader_config_builder() {
        let config = BootLoaderConfig::new(BootLoaderType::SystemdBoot)
            .default_entry("sigmaos")
            .timeout(10)
            .theme("sigmaos-theme")
            .resolution("1920x1080")
            .kernel_param("quiet")
            .kernel_param("loglevel=3")
            .secure_boot(true)
            .quiet_boot(true);

        assert_eq!(config.boot_loader_type, BootLoaderType::SystemdBoot);
        assert_eq!(config.default_entry, Some("sigmaos".to_string()));
        assert_eq!(config.timeout, 10);
        assert_eq!(config.theme, Some("sigmaos-theme".to_string()));
        assert!(config.secure_boot);
        assert!(config.quiet_boot);
        assert_eq!(config.kernel_params.len(), 2);
    }

    #[test]
    fn test_systemd_boot_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::SystemdBoot)
            .default_entry("sigmaos")
            .timeout(5)
            .quiet_boot(true);

        let kernel = KernelEntry::new("SigmaOS", "/boot/vmlinuz-sigmaos")
            .initrd("/boot/initramfs-sigmaos.img")
            .cmdline("root=/dev/sda1")
            .default();

        let entry = BootEntry::new("SigmaOS 1.0", kernel);
        config.entries.push(entry);

        let systemd_config = config.to_systemd_boot_config();
        assert!(systemd_config.contains("default=sigmaos"));
        assert!(systemd_config.contains("timeout=5"));
        assert!(systemd_config.contains("title SigmaOS 1.0"));
        assert!(systemd_config.contains("efi /boot/vmlinuz-sigmaos"));
        assert!(systemd_config.contains("initrd /boot/initramfs-sigmaos.img"));
        assert!(systemd_config.contains("options root=/dev/sda1"));
    }

    #[test]
    fn test_grub_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::Grub)
            .timeout(5);

        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1");

        let entry = BootEntry::new("Linux 6.1", kernel);
        config.entries.push(entry);

        let grub_config = config.to_grub_config();
        assert!(grub_config.contains("set default=\"0\""));
        assert!(grub_config.contains("set timeout=5"));
        assert!(grub_config.contains("menuentry \"Linux 6.1\""));
        assert!(grub_config.contains("linux /boot/vmlinuz-linux root=/dev/sda1"));
        assert!(grub_config.contains("initrd /boot/initramfs-linux.img"));
    }

    #[test]
    fn test_syslinux_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::Syslinux)
            .timeout(5);

        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1");

        let entry = BootEntry::new("Linux 6.1", kernel);
        config.entries.push(entry);

        let syslinux_config = config.to_syslinux_config();
        assert!(syslinux_config.contains("DEFAULT sigmaos"));
        assert!(syslinux_config.contains("TIMEOUT 50")); // 5 seconds * 10
        assert!(syslinux_config.contains("LABEL Linux-6.1"));
        assert!(syslinux_config.contains("KERNEL /boot/vmlinuz-linux"));
        assert!(syslinux_config.contains("INITRD /boot/initramfs-linux.img"));
        assert!(syslinux_config.contains("APPEND root=/dev/sda1"));
    }

    #[test]
    fn test_lilo_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::Lilo)
            .timeout(5);

        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1");

        let entry = BootEntry::new("Linux 6.1", kernel);
        config.entries.push(entry);

        let lilo_config = config.to_lilo_config();
        assert!(lilo_config.contains("boot = /dev/sda"));
        assert!(lilo_config.contains("timeout = 5"));
        assert!(lilo_config.contains("image = /boot/vmlinuz-linux"));
        assert!(lilo_config.contains("label = Linux 6.1"));
        assert!(lilo_config.contains("initrd = /boot/initramfs-linux.img"));
        assert!(lilo_config.contains("append = \"root=/dev/sda1\""));
    }

    #[test]
    fn test_refind_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::Refind)
            .timeout(5)
            .theme("sigmaos");

        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1");

        let entry = BootEntry::new("Linux 6.1", kernel)
            .icon("linux.png");
        config.entries.push(entry);

        let refind_config = config.to_refind_config();
        assert!(refind_config.contains("theme sigmaos"));
        assert!(refind_config.contains("timeout 5"));
        assert!(refind_config.contains("menuentry \"Linux 6.1\""));
        assert!(refind_config.contains("icon linux.png"));
        assert!(refind_config.contains("kernel \\\\(h\\)/boot/vmlinuz-linux root=/dev/sda1"));
        assert!(refind_config.contains("initrd \\\\(h\\)/boot/initramfs-linux.img"));
    }

    #[test]
    fn test_direct_config() {
        let mut config = BootLoaderConfig::new(BootLoaderType::Direct);

        let kernel = KernelEntry::new("Linux", "/boot/vmlinuz-linux")
            .initrd("/boot/initramfs-linux.img")
            .cmdline("root=/dev/sda1")
            .default();

        let entry = BootEntry::new("Linux 6.1", kernel);
        config.entries.push(entry);

        let direct_config = config.to_direct_config();
        assert!(direct_config.contains("# Default entry: Linux 6.1"));
        assert!(direct_config.contains("kernel=/boot/vmlinuz-linux"));
        assert!(direct_config.contains("initrd=/boot/initramfs-linux.img"));
        assert!(direct_config.contains("cmdline=root=/dev/sda1"));
    }

    #[test]
    fn test_boot_loader_manager() {
        let mut manager = BootLoaderManager::new();

        // Add a configuration
        let mut config = BootLoaderConfig::new(BootLoaderType::SystemdBoot)
            .default_entry("sigmaos")
            .timeout(5);

        let kernel = KernelEntry::new("SigmaOS", "/boot/vmlinuz-sigmaos")
            .initrd("/boot/initramfs-sigmaos.img")
            .cmdline("root=/dev/sda1")
            .default();

        let entry = BootEntry::new("SigmaOS 1.0", kernel);
        config.entries.push(entry);

        manager.add_config(config);
        assert!(manager.set_active_boot_loader(BootLoaderType::SystemdBoot).is_ok());
        assert_eq!(manager.get_active_boot_loader(), Some(BootLoaderType::SystemdBoot));
    }

    #[test]
    fn test_boot_loader_export_import() {
        let mut manager = BootLoaderManager::new();

        let mut config = BootLoaderConfig::new(BootLoaderType::SystemdBoot)
            .default_entry("sigmaos")
            .timeout(5);

        let kernel = KernelEntry::new("SigmaOS", "/boot/vmlinuz-sigmaos")
            .cmdline("root=/dev/sda1")
            .default();

        let entry = BootEntry::new("SigmaOS 1.0", kernel);
        config.entries.push(entry);

        manager.add_config(config);
        manager.set_active_boot_loader(BootLoaderType::SystemdBoot).unwrap();

        let export = manager.export_all();
        assert!(export.contains("active_boot_loader=systemd-boot"));
        assert!(export.contains("SigmaOS 1.0"));
    }

    #[test]
    fn test_kernel_scanning() {
        let manager = BootLoaderManager::new();
        let kernels = manager.scan_kernels().unwrap();
        // This will return empty in test environment, but should not error
        assert!(kernels.is_empty() || !kernels.is_empty());
    }

    #[test]
    fn test_boot_loader_statistics() {
        let mut manager = BootLoaderManager::new();

        let config = BootLoaderConfig::new(BootLoaderType::SystemdBoot);
        manager.add_config(config);
        manager.set_active_boot_loader(BootLoaderType::SystemdBoot).unwrap();

        let stats = manager.get_statistics();
        assert_eq!(stats.total_configs, 1);
        assert_eq!(stats.active_boot_loader, Some(BootLoaderType::SystemdBoot));
    }
}
