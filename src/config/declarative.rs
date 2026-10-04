//! Declarative System Configuration Management inspired by NixOS and Guix
//! Atomic upgrades, system generation tracking, configuration modules, and instant rollbacks.
use std::vec;

use std::string::{String, ToString};

/// System configuration
#[derive(Debug, Clone)]
pub struct SystemConfig {
    pub hostname: String,
    pub timezone: String,
    pub locale: String,
}

impl Default for SystemConfig {
    fn default() -> Self {
        SystemConfig {
            hostname: String::from("sigmaos"),
            timezone: String::from("UTC"),
            locale: String::from("en_US.UTF-8"),
        }
    }
}

/// Network configuration
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub hostname: String,
    pub dhcp: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            hostname: String::from("sigmaos"),
            dhcp: true,
        }
    }
}

/// Desktop configuration
#[derive(Debug, Clone)]
pub struct DesktopConfig {
    pub compositor: String,
    pub theme: String,
    pub animations: bool,
}

impl Default for DesktopConfig {
    fn default() -> Self {
        DesktopConfig {
            compositor: String::from("zenith"),
            theme: String::from("dark"),
            animations: true,
        }
    }
}

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub sandboxing: bool,
    pub firewall: bool,
    pub encryption: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            sandboxing: true,
            firewall: true,
            encryption: true,
        }
    }
}

/// Kernel configuration
#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub log_level: String,
    pub security_mitigations: bool,
    pub memory_management: String,
}

impl Default for KernelConfig {
    fn default() -> Self {
        KernelConfig {
            log_level: String::from("info"),
            security_mitigations: true,
            memory_management: String::from("auto"),
        }
    }
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    pub cpu_governor: String,
    pub iopriority: String,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        PerformanceConfig {
            cpu_governor: String::from("performance"),
            iopriority: String::from("best-effort"),
        }
    }
}

/// Declarative system configuration
#[derive(Debug, Clone)]
pub struct SigmaOsConfig {
    pub system: SystemConfig,
    pub network: NetworkConfig,
    pub desktop: DesktopConfig,
    pub generations: Vec<SystemGeneration>,
    pub active_generation_id: u32,
    pub security: SecurityConfig,
    pub kernel: KernelConfig,
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct ConfigModule {
    pub module_name: String,
    pub enabled: bool,
}

pub enum ConfigState { Active, Inactive, Pending }

#[derive(Debug, Clone)]
pub struct SystemGeneration {
    pub generation_id: u32,
    pub created_at_timestamp: u64,
    pub config_hash: [u8; 32],
    pub modules: Vec<ConfigModule>,
    pub state: ConfigState,
}

impl SigmaOsConfig {
    pub fn new() -> Self {
        SigmaOsConfig {
            system: SystemConfig::default(),
            network: NetworkConfig::default(),
            desktop: DesktopConfig::default(),
            security: SecurityConfig::default(),
            kernel: KernelConfig::default(),
            performance: PerformanceConfig::default(),
            generations: Vec::new(),
            active_generation_id: 0,
        }
    }

    pub fn add_module_to_active(&mut self, module: ConfigModule) {
        if let Some(gen) = self
            .generations
            .iter_mut()
            .find(|g| g.generation_id == self.active_generation_id)
        {
            gen.modules.push(module);
        }
    }

    pub fn commit_atomic_generation(&mut self, timestamp: u64) -> u32 {
        let new_id = self.generations.len() as u32 + 1;

        let current_modules = self
            .generations
            .iter()
            .find(|g| g.generation_id == self.active_generation_id)
            .map(|g| g.modules.clone())
            .unwrap_or_default();

        let mut hash = [0u8; 32];
        for (i, m) in current_modules.iter().enumerate() {
            for &b in m.module_name.as_bytes() {
                hash[i % 32] ^= b;
            }
        }
        cfg
    }

    pub fn rollback(&mut self, target_generation_id: u32) -> Result<(), &'static str> {
        if let Some(target) = self
            .generations
            .iter()
            .find(|g| g.generation_id == target_generation_id)
        {
            let _ = target;
            for g in &mut self.generations {
                if g.generation_id == target_generation_id {
                    g.state = ConfigState::Active;
                } else if g.generation_id == self.active_generation_id {
                    g.state = ConfigState::RolledBack;
                }
            }
        }

        config
    }

    pub fn to_config_string(&self) -> String {
        let mut result = String::new();

        result.push_str("[system]\n");
        result.push_str(&format!("hostname = \"{}\"\n", self.system.hostname));
        result.push_str(&format!("timezone = \"{}\"\n", self.system.timezone));
        result.push_str(&format!("locale = \"{}\"\n\n", self.system.locale));

        result.push_str("[network]\n");
        result.push_str(&format!("hostname = \"{}\"\n", self.network.hostname));
        result.push_str(&format!("dhcp = {}\n\n", self.network.dhcp));

        result.push_str("[desktop]\n");
        result.push_str(&format!("compositor = \"{}\"\n", self.desktop.compositor));
        result.push_str(&format!("theme = \"{}\"\n", self.desktop.theme));
        result.push_str(&format!("animations = {}\n\n", self.desktop.animations));

        result.push_str("[security]\n");
        result.push_str(&format!("sandboxing = {}\n", self.security.sandboxing));
        result.push_str(&format!("firewall = {}\n", self.security.firewall));
        result.push_str(&format!("encryption = {}\n\n", self.security.encryption));

        result.push_str("[kernel]\n");
        result.push_str(&format!("log_level = \"{}\"\n", self.kernel.log_level));
        result.push_str(&format!("security_mitigations = {}\n", self.kernel.security_mitigations));
        result.push_str(&format!("memory_management = \"{}\"\n\n", self.kernel.memory_management));

        result.push_str("[performance]\n");
        result.push_str(&format!("cpu_governor = \"{}\"\n", self.performance.cpu_governor));
        result.push_str(&format!("iopriority = \"{}\"\n", self.performance.iopriority));

        result
    }

    pub fn set_hostname(&mut self, hostname: String) {
        self.system.hostname = hostname.clone();
        self.network.hostname = hostname;
    }

    pub fn set_timezone(&mut self, timezone: String) {
        self.system.timezone = timezone;
    }

    pub fn set_locale(&mut self, locale: String) {
        self.system.locale = locale;
    }

    pub fn set_desktop_theme(&mut self, theme: String) {
        self.desktop.theme = theme;
    }

    pub fn enable_security_mitigations(&mut self, enabled: bool) {
        self.kernel.security_mitigations = enabled;
    }
}

impl Default for SigmaOsConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = SigmaOsConfig::new();
        assert_eq!(config.system.hostname, "sigmaos");
        assert_eq!(config.system.timezone, "UTC");
        assert_eq!(config.network.dhcp, true);
        assert_eq!(config.desktop.compositor, "zenith");
        assert_eq!(config.security.sandboxing, true);
    }

    #[test]
    fn test_parse_config() {
        let config_str = r#"
[system]
hostname = "sigmaos-desktop"
timezone = "America/New_York"
locale = "en_US.UTF-8"

[network]
hostname = "sigmaos-desktop"
dhcp = true

[desktop]
compositor = "zenith"
theme = "dark"
animations = true

[security]
sandboxing = true
firewall = true
encryption = true

[kernel]
log_level = "info"
security_mitigations = true
memory_management = "auto"

[performance]
cpu_governor = "performance"
iopriority = "best-effort"
"#;

        let config = SigmaOsConfig::parse_config(config_str);
        assert_eq!(config.system.hostname, "sigmaos-desktop");
        assert_eq!(config.system.timezone, "America/New_York");
        assert_eq!(config.system.locale, "en_US.UTF-8");
        assert_eq!(config.network.dhcp, true);
        assert_eq!(config.desktop.theme, "dark");
        assert_eq!(config.security.sandboxing, true);
        assert_eq!(config.kernel.log_level, "info");
        assert_eq!(config.performance.cpu_governor, "performance");
    }

    #[test]
    fn test_to_config_string() {
        let config = SigmaOsConfig::new();
        let config_str = config.to_config_string();

        assert!(config_str.contains("[system]"));
        assert!(config_str.contains("[network]"));
        assert!(config_str.contains("[desktop]"));
        assert!(config_str.contains("[security]"));
        assert!(config_str.contains("[kernel]"));
        assert!(config_str.contains("[performance]"));
        assert!(config_str.contains("hostname = \"sigmaos\""));
    }

    #[test]
    fn test_set_hostname() {
        let mut config = SigmaOsConfig::new();
        config.set_hostname(String::from("testhost"));
        assert_eq!(config.system.hostname, "testhost");
        assert_eq!(config.network.hostname, "testhost");
    }

    #[test]
    fn test_set_timezone() {
        let mut config = SigmaOsConfig::new();
        config.set_timezone(String::from("Europe/London"));
        assert_eq!(config.system.timezone, "Europe/London");
    }

    #[test]
    fn test_set_locale() {
        let mut config = SigmaOsConfig::new();
        config.set_locale(String::from("de_DE.UTF-8"));
        assert_eq!(config.system.locale, "de_DE.UTF-8");
    }

    #[test]
    fn test_set_desktop_theme() {
        let mut config = SigmaOsConfig::new();
        config.set_desktop_theme(String::from("light"));
        assert_eq!(config.desktop.theme, "light");
    }

    #[test]
    fn test_enable_security_mitigations() {
        let mut config = SigmaOsConfig::new();
        config.enable_security_mitigations(false);
        assert!(!config.kernel.security_mitigations);
    }

    #[test]
    fn test_default_configs() {
        assert_eq!(SystemConfig::default().hostname, "sigmaos");
        assert_eq!(NetworkConfig::default().dhcp, true);
        assert_eq!(DesktopConfig::default().compositor, "zenith");
        assert_eq!(SecurityConfig::default().sandboxing, true);
        assert_eq!(KernelConfig::default().log_level, "info");
        assert_eq!(PerformanceConfig::default().cpu_governor, "performance");
    }

    #[test]
    fn test_round_trip() {
        let original = SigmaOsConfig::new();
        let config_str = original.to_config_string();
        let parsed = SigmaOsConfig::parse_config(&config_str);

        assert_eq!(original.system.hostname, parsed.system.hostname);
        assert_eq!(original.system.timezone, parsed.system.timezone);
        assert_eq!(original.network.dhcp, parsed.network.dhcp);
        assert_eq!(original.desktop.compositor, parsed.desktop.compositor);
    }
}
