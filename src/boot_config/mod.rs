// Boot Configuration Manager
// Inspired by Linux Mint 22.2 System Administration Boot Menu tool
// Manages boot menu visibility, timeout, and kernel parameters

use std::collections::HashMap;

/// Boot menu configuration
#[derive(Debug, Clone, PartialEq)]
pub struct BootMenuConfig {
    pub show_boot_menu: bool,
    pub timeout_seconds: u32,
    pub default_entry: String,
    pub hidden_timeout: bool,
    pub quiet_boot: bool,
}

impl BootMenuConfig {
    pub fn new() -> Self {
        Self {
            show_boot_menu: true,
            timeout_seconds: 5,
            default_entry: "0".to_string(),
            hidden_timeout: false,
            quiet_boot: false,
        }
    }

    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.timeout_seconds = timeout;
        self
    }

    pub fn with_default_entry(mut self, entry: String) -> Self {
        self.default_entry = entry;
        self
    }

    pub fn with_hidden_timeout(mut self, hidden: bool) -> Self {
        self.hidden_timeout = hidden;
        self
    }

    pub fn with_quiet_boot(mut self, quiet: bool) -> Self {
        self.quiet_boot = quiet;
        self
    }
}

impl Default for BootMenuConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Kernel boot parameter
#[derive(Debug, Clone, PartialEq)]
pub struct KernelParameter {
    pub name: String,
    pub value: Option<String>,
    pub description: String,
    pub category: ParameterCategory,
    pub temporary: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParameterCategory {
    Hardware,
    Graphics,
    Networking,
    Security,
    Performance,
    Debugging,
    PowerManagement,
    Other,
}

impl KernelParameter {
    pub fn new(name: String, value: Option<String>, description: String) -> Self {
        Self {
            name,
            value,
            description,
            category: ParameterCategory::Other,
            temporary: false,
        }
    }

    pub fn with_category(mut self, category: ParameterCategory) -> Self {
        self.category = category;
        self
    }

    pub fn with_temporary(mut self, temporary: bool) -> Self {
        self.temporary = temporary;
        self
    }

    /// Format as kernel command line parameter
    pub fn to_cmdline(&self) -> String {
        match &self.value {
            Some(v) => format!("{}={}", self.name, v),
            None => self.name.clone(),
        }
    }
}

/// Boot entry information
#[derive(Debug, Clone, PartialEq)]
pub struct BootEntry {
    pub id: String,
    pub name: String,
    pub kernel: String,
    pub initrd: Option<String>,
    pub options: Vec<String>,
    pub is_default: bool,
}

impl BootEntry {
    pub fn new(id: String, name: String, kernel: String) -> Self {
        Self {
            id,
            name,
            kernel,
            initrd: None,
            options: Vec::new(),
            is_default: false,
        }
    }

    pub fn with_initrd(mut self, initrd: String) -> Self {
        self.initrd = Some(initrd);
        self
    }

    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }

    pub fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }
}

/// Boot Configuration Manager
#[derive(Debug, Clone)]
pub struct BootConfigManager {
    menu_config: BootMenuConfig,
    kernel_parameters: Vec<KernelParameter>,
    boot_entries: Vec<BootEntry>,
}

impl BootConfigManager {
    pub fn new() -> Self {
        Self {
            menu_config: BootMenuConfig::new(),
            kernel_parameters: Vec::new(),
            boot_entries: Vec::new(),
        }
    }

    /// Set boot menu configuration
    pub fn set_menu_config(&mut self, config: BootMenuConfig) {
        self.menu_config = config;
    }

    /// Get boot menu configuration
    pub fn get_menu_config(&self) -> &BootMenuConfig {
        &self.menu_config
    }

    /// Add a kernel parameter
    pub fn add_kernel_parameter(&mut self, param: KernelParameter) {
        // Remove existing parameter with same name if not temporary
        if !param.temporary {
            self.kernel_parameters
                .retain(|p| p.name != param.name);
        }
        self.kernel_parameters.push(param);
    }

    /// Remove a kernel parameter
    pub fn remove_kernel_parameter(&mut self, name: &str) {
        self.kernel_parameters.retain(|p| p.name != name);
    }

    /// Get all kernel parameters
    pub fn get_kernel_parameters(&self) -> &[KernelParameter] {
        &self.kernel_parameters
    }

    /// Get kernel parameters by category
    pub fn get_parameters_by_category(&self, category: ParameterCategory) -> Vec<&KernelParameter> {
        self.kernel_parameters
            .iter()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Get kernel command line string
    pub fn get_kernel_cmdline(&self) -> String {
        self.kernel_parameters
            .iter()
            .map(|p| p.to_cmdline())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Add a boot entry
    pub fn add_boot_entry(&mut self, entry: BootEntry) {
        self.boot_entries.push(entry);
    }

    /// Get all boot entries
    pub fn get_boot_entries(&self) -> &[BootEntry] {
        &self.boot_entries
    }

    /// Get default boot entry
    pub fn get_default_entry(&self) -> Option<&BootEntry> {
        self.boot_entries.iter().find(|e| e.is_default)
    }

    /// Set default boot entry
    pub fn set_default_entry(&mut self, id: &str) {
        for entry in &mut self.boot_entries {
            entry.is_default = entry.id == id;
        }
        self.menu_config.default_entry = id.to_string();
    }

    /// Generate GRUB configuration (simplified)
    pub fn generate_grub_config(&self) -> String {
        let mut config = String::new();

        config.push_str("# Boot menu configuration\n");
        config.push_str(&format!("GRUB_TIMEOUT={}\n", self.menu_config.timeout_seconds));
        config.push_str(&format!(
            "GRUB_TIMEOUT_STYLE={}\n",
            if self.menu_config.hidden_timeout {
                "hidden"
            } else {
                "menu"
            }
        ));
        config.push_str(&format!(
            "GRUB_DEFAULT={}\n",
            self.menu_config.default_entry
        ));
        config.push_str(&format!(
            "GRUB_HIDDEN_TIMEOUT_QUIET={}\n",
            if self.menu_config.quiet_boot { "true" } else { "false" }
        ));
        config.push_str("\n");

        // Kernel parameters
        if !self.kernel_parameters.is_empty() {
            config.push_str("# Kernel parameters\n");
            config.push_str("GRUB_CMDLINE_LINUX_DEFAULT=\"");
            config.push_str(&self.get_kernel_cmdline());
            config.push_str("\"\n");
            config.push_str("\n");
        }

        // Boot entries
        config.push_str("# Boot entries\n");
        for entry in &self.boot_entries {
            config.push_str(&format!("menuentry \"{}\" {{\n", entry.name));
            config.push_str(&format!("    linux /boot/{} ", entry.kernel));
            for opt in &entry.options {
                config.push_str(opt);
                config.push_str(" ");
            }
            config.push_str("\n");
            if let Some(initrd) = &entry.initrd {
                config.push_str(&format!("    initrd /boot/{}\n", initrd));
            }
            config.push_str("}\n");
        }

        config
    }

    /// Validate boot configuration
    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        // Check timeout
        if self.menu_config.timeout_seconds > 30 {
            errors.push("Boot timeout exceeds recommended maximum of 30 seconds".to_string());
        }

        // Check default entry exists
        if !self.boot_entries.iter().any(|e| e.id == self.menu_config.default_entry) {
            errors.push(format!(
                "Default entry '{}' not found in boot entries",
                self.menu_config.default_entry
            ));
        }

        // Check for duplicate parameter names
        let param_names: Vec<_> = self.kernel_parameters.iter().map(|p| &p.name).collect();
        let mut seen = HashMap::new();
        for name in param_names {
            *seen.entry(name).or_insert(0) += 1;
            if seen[name] > 1 {
                errors.push(format!("Duplicate kernel parameter: {}", name));
            }
        }

        errors
    }

    /// Get recommended parameters for common scenarios
    pub fn get_recommended_parameters(&self) -> HashMap<String, Vec<KernelParameter>> {
        let mut recommendations = HashMap::new();

        // AMD GPU issues
        recommendations.insert(
            "amd_gpu_flicker".to_string(),
            vec![
                KernelParameter::new(
                    "amdgpu.expert_support".to_string(),
                    Some("1".to_string()),
                    "Enable expert support for AMD GPU".to_string(),
                )
                .with_category(ParameterCategory::Graphics),
                KernelParameter::new(
                    "amdgpu.dc".to_string(),
                    Some("1".to_string()),
                    "Enable Display Core for AMD GPU".to_string(),
                )
                .with_category(ParameterCategory::Graphics),
            ],
        );

        // NVIDIA issues
        recommendations.insert(
            "nvidia_driver".to_string(),
            vec![
                KernelParameter::new(
                    "nomodeset".to_string(),
                    None,
                    "Disable kernel mode setting (use with problematic drivers)".to_string(),
                )
                .with_category(ParameterCategory::Graphics),
                KernelParameter::new(
                    "nvidia-drm.modeset".to_string(),
                    Some("1".to_string()),
                    "Enable NVIDIA DRM mode setting".to_string(),
                )
                .with_category(ParameterCategory::Graphics),
            ],
        );

        // WiFi issues
        recommendations.insert(
            "wifi_bluetooth".to_string(),
            vec![
                KernelParameter::new(
                    "btcoex_enable".to_string(),
                    Some("1".to_string()),
                    "Enable Bluetooth/WiFi coexistence".to_string(),
                )
                .with_category(ParameterCategory::Networking),
            ],
        );

        // Power management
        recommendations.insert(
            "power_saving".to_string(),
            vec![
                KernelParameter::new(
                    "pcie_aspm".to_string(),
                    Some("powersave".to_string()),
                    "Enable PCIe Active State Power Management".to_string(),
                )
                .with_category(ParameterCategory::PowerManagement),
                KernelParameter::new(
                    "i915.enable_psr".to_string(),
                    Some("1".to_string()),
                    "Enable Panel Self Refresh for Intel GPU".to_string(),
                )
                .with_category(ParameterCategory::PowerManagement),
            ],
        );

        recommendations
    }
}

impl Default for BootConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_config_manager_creation() {
        let manager = BootConfigManager::new();
        assert_eq!(manager.get_boot_entries().len(), 0);
        assert_eq!(manager.get_kernel_parameters().len(), 0);
    }

    #[test]
    fn test_boot_menu_config() {
        let config = BootMenuConfig::new()
            .with_timeout(10)
            .with_default_entry("1".to_string())
            .with_hidden_timeout(true)
            .with_quiet_boot(true);

        assert_eq!(config.timeout_seconds, 10);
        assert_eq!(config.default_entry, "1");
        assert!(config.hidden_timeout);
        assert!(config.quiet_boot);
    }

    #[test]
    fn test_kernel_parameter() {
        let param = KernelParameter::new(
            "quiet".to_string(),
            None,
            "Reduce kernel log output".to_string(),
        )
        .with_category(ParameterCategory::Debugging)
        .with_temporary(true);

        assert_eq!(param.name, "quiet");
        assert!(param.value.is_none());
        assert_eq!(param.to_cmdline(), "quiet");
        assert!(param.temporary);
    }

    #[test]
    fn test_kernel_parameter_with_value() {
        let param = KernelParameter::new(
            "loglevel".to_string(),
            Some("3".to_string()),
            "Set kernel log level".to_string(),
        )
        .with_category(ParameterCategory::Debugging);

        assert_eq!(param.to_cmdline(), "loglevel=3");
    }

    #[test]
    fn test_add_kernel_parameter() {
        let mut manager = BootConfigManager::new();
        let param = KernelParameter::new(
            "quiet".to_string(),
            None,
            "Reduce kernel log output".to_string(),
        );
        manager.add_kernel_parameter(param);
        assert_eq!(manager.get_kernel_parameters().len(), 1);
    }

    #[test]
    fn test_remove_kernel_parameter() {
        let mut manager = BootConfigManager::new();
        manager.add_kernel_parameter(KernelParameter::new(
            "quiet".to_string(),
            None,
            "Quiet".to_string(),
        ));
        manager.add_kernel_parameter(KernelParameter::new(
            "splash".to_string(),
            None,
            "Splash".to_string(),
        ));
        manager.remove_kernel_parameter("quiet");
        assert_eq!(manager.get_kernel_parameters().len(), 1);
        assert_eq!(manager.get_kernel_parameters()[0].name, "splash");
    }

    #[test]
    fn test_kernel_cmdline() {
        let mut manager = BootConfigManager::new();
        manager.add_kernel_parameter(KernelParameter::new(
            "quiet".to_string(),
            None,
            "Quiet".to_string(),
        ));
        manager.add_kernel_parameter(KernelParameter::new(
            "loglevel".to_string(),
            Some("3".to_string()),
            "Log level".to_string(),
        ));
        let cmdline = manager.get_kernel_cmdline();
        assert!(cmdline.contains("quiet"));
        assert!(cmdline.contains("loglevel=3"));
    }

    #[test]
    fn test_boot_entry() {
        let entry = BootEntry::new(
            "0".to_string(),
            "SigmaOS Linux".to_string(),
            "/vmlinuz-linux".to_string(),
        )
        .with_initrd("/initramfs-linux.img".to_string())
        .with_options(vec!["root=/dev/sda2".to_string(), "rw".to_string()])
        .with_default(true);

        assert_eq!(entry.id, "0");
        assert!(entry.is_default);
        assert_eq!(entry.options.len(), 2);
    }

    #[test]
    fn test_add_boot_entry() {
        let mut manager = BootConfigManager::new();
        let entry = BootEntry::new(
            "0".to_string(),
            "SigmaOS".to_string(),
            "/vmlinuz".to_string(),
        );
        manager.add_boot_entry(entry);
        assert_eq!(manager.get_boot_entries().len(), 1);
    }

    #[test]
    fn test_set_default_entry() {
        let mut manager = BootConfigManager::new();
        manager.add_boot_entry(BootEntry::new("0".to_string(), "Entry 0".to_string(), "/vmlinuz".to_string()));
        manager.add_boot_entry(BootEntry::new("1".to_string(), "Entry 1".to_string(), "/vmlinuz".to_string()));
        manager.set_default_entry("1");
        assert_eq!(manager.get_menu_config().default_entry, "1");
        assert!(manager.get_boot_entries()[1].is_default);
        assert!(!manager.get_boot_entries()[0].is_default);
    }

    #[test]
    fn test_generate_grub_config() {
        let mut manager = BootConfigManager::new();
        manager.set_menu_config(BootMenuConfig::new().with_timeout(5));
        manager.add_kernel_parameter(KernelParameter::new(
            "quiet".to_string(),
            None,
            "Quiet".to_string(),
        ));
        manager.add_boot_entry(
            BootEntry::new("0".to_string(), "SigmaOS".to_string(), "/vmlinuz".to_string())
                .with_default(true),
        );

        let config = manager.generate_grub_config();
        assert!(config.contains("GRUB_TIMEOUT=5"));
        assert!(config.contains("quiet"));
        assert!(config.contains("menuentry \"SigmaOS\""));
    }

    #[test]
    fn test_validate() {
        let mut manager = BootConfigManager::new();
        manager.set_menu_config(BootMenuConfig::new().with_timeout(35));
        let errors = manager.validate();
        assert!(!errors.is_empty());
        assert!(errors[0].contains("timeout"));
    }

    #[test]
    fn test_get_parameters_by_category() {
        let mut manager = BootConfigManager::new();
        manager.add_kernel_parameter(
            KernelParameter::new("quiet".to_string(), None, "Quiet".to_string())
                .with_category(ParameterCategory::Debugging),
        );
        manager.add_kernel_parameter(
            KernelParameter::new("splash".to_string(), None, "Splash".to_string())
                .with_category(ParameterCategory::Debugging),
        );
        manager.add_kernel_parameter(
            KernelParameter::new("nomodeset".to_string(), None, "No modeset".to_string())
                .with_category(ParameterCategory::Graphics),
        );

        let debug_params = manager.get_parameters_by_category(ParameterCategory::Debugging);
        assert_eq!(debug_params.len(), 2);
    }

    #[test]
    fn test_recommended_parameters() {
        let manager = BootConfigManager::new();
        let recommendations = manager.get_recommended_parameters();
        assert!(recommendations.contains_key("amd_gpu_flicker"));
        assert!(recommendations.contains_key("nvidia_driver"));
        assert!(recommendations.contains_key("wifi_bluetooth"));
    }
}
