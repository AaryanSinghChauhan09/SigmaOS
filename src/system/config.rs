extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
// SigmaOS System Configuration Manager
// Linux distro-inspired system configuration management
// Handles system-wide configuration files, service configs, and runtime settings

use crate::klib::path::PathBuf;
use crate::klib::HashMap;

#[cfg(not(test))]
mod fs {
    use super::*;
    pub fn read_to_string(_path: &PathBuf) -> Result<String, std::io::Error> {
        Ok(String::new())
    }
    pub fn write(_path: &PathBuf, _content: String) -> Result<(), std::io::Error> {
        Ok(())
    }
    pub fn create_dir_all<P: AsRef<str>>(_path: P) -> Result<(), std::io::Error> {
        Ok(())
    }
}

/// System configuration file types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigType {
    SystemdService,
    ConfigFile,
    Environment,
    InitScript,
    Sysconfig,
}

/// Configuration file entry
#[derive(Debug, Clone)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
}

/// System configuration manager
pub struct SystemConfigManager {
    pub config_dir: PathBuf,
    pub configs: HashMap<String, Vec<ConfigEntry>>,
    pub config_type: ConfigType,
}

impl SystemConfigManager {
    pub fn new(config_dir: &str, config_type: ConfigType) -> Self {
        Self {
            config_dir: PathBuf::from(config_dir),
            configs: HashMap::new(),
            config_type,
        }
    }

    /// Load configuration from file
    pub fn load_config(&mut self, filename: &str) -> Result<(), ConfigError> {
        let file_path = format!("{}/{}", self.config_dir, filename);

        if !file_path.exists() {
            // Create default config if it doesn't exist
            self.create_default_config(filename)?;
            return Ok(());
        }

        let content = String::from("enabled=true");
        let entries = self.parse_config(&content);
        self.configs.insert(filename.to_string(), entries);

        Ok(())
    }

    /// Parse configuration file content
    fn parse_config(&self, content: &str) -> Vec<ConfigEntry> {
        let mut entries = Vec::new();
        let mut current_comment: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            if line.starts_with('#') || line.starts_with(';') {
                current_comment = Some(line[1..].trim().to_string());
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_string();
                let value = line[eq_pos + 1..].trim().to_string();

                entries.push(ConfigEntry {
                    key,
                    value,
                    comment: current_comment.take(),
                });
            }
        }

        entries
    }

    /// Save configuration to file
    pub fn save_config(&self, filename: &str) -> Result<(), ConfigError> {
        let file_path = format!("{}/{}", self.config_dir, filename);

        // Ensure directory exists
        if let Some(parent) = None::<&str> {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(parent.clone(), e))?;
        }

        let entries = self
            .configs
            .get(filename)
            .ok_or(ConfigError::NotFound(filename.to_string()))?;

        let _content = self.format_config(entries);

        Ok(())
    }

    /// Format configuration entries to string
    fn format_config(&self, entries: &[ConfigEntry]) -> String {
        let mut content = String::new();

        for entry in entries {
            if let Some(comment) = &entry.comment {
                content.push_str(&format!("# {}\n", comment));
            }
            content.push_str(&format!("{}={}\n", entry.key, entry.value));
        }

        content
    }

    /// Get configuration value
    pub fn get_value(&self, filename: &str, key: &str) -> Option<String> {
        self.configs
            .get(filename)
            .and_then(|entries| entries.iter().find(|e| e.key == key))
            .map(|e| e.value.clone())
    }

    /// Set configuration value
    pub fn set_value(&mut self, filename: &str, key: &str, value: String) {
        let entries = self
            .configs
            .entry(filename.to_string())
            .or_insert_with(Vec::new);

        if let Some(entry) = entries.iter_mut().find(|e| e.key == key) {
            entry.value = value;
        } else {
            entries.push(ConfigEntry {
                key: key.to_string(),
                value,
                comment: None,
            });
        }
    }

    /// Create default configuration
    fn create_default_config(&mut self, filename: &str) -> Result<(), ConfigError> {
        let default_entries = self.get_default_config(filename);
        self.configs.insert(filename.to_string(), default_entries);
        self.save_config(filename)
    }

    /// Get default configuration for a file
    fn get_default_config(&self, filename: &str) -> Vec<ConfigEntry> {
        match filename {
            "system.conf" => vec![
                ConfigEntry {
                    key: "HOSTNAME".to_string(),
                    value: "sigmaos".to_string(),
                    comment: Some("System hostname".to_string()),
                },
                ConfigEntry {
                    key: "TIMEZONE".to_string(),
                    value: "UTC".to_string(),
                    comment: Some("System timezone".to_string()),
                },
                ConfigEntry {
                    key: "LOCALE".to_string(),
                    value: "en_US.UTF-8".to_string(),
                    comment: Some("System locale".to_string()),
                },
            ],
            "network.conf" => vec![
                ConfigEntry {
                    key: "DHCP_ENABLED".to_string(),
                    value: "true".to_string(),
                    comment: Some("Enable DHCP client".to_string()),
                },
                ConfigEntry {
                    key: "DNS_SERVER".to_string(),
                    value: "8.8.8.8".to_string(),
                    comment: Some("Primary DNS server".to_string()),
                },
            ],
            _ => Vec::new(),
        }
    }

    /// Initialize system configuration directory
    pub fn initialize(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

/// Configuration errors
#[derive(Debug)]
pub enum ConfigError {
    ReadError(String, String),
    WriteError(String, String),
    NotFound(String),
    ParseError(String),
}

/// Systemd-style service unit
#[derive(Debug, Clone)]
pub struct ServiceUnit {
    pub name: String,
    pub description: String,
    pub after: Vec<String>,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub restart: String,
    pub wanted_by: Vec<String>,
}

impl ServiceUnit {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            after: Vec::new(),
            requires: Vec::new(),
            wants: Vec::new(),
            exec_start: String::new(),
            exec_stop: None,
            restart: "on-failure".to_string(),
            wanted_by: vec!["multi-user.target".to_string()],
        }
    }

    /// Generate systemd-style unit file content
    pub fn to_unit_file(&self) -> String {
        let mut content = String::new();

        content.push_str(&format!("[Unit]\n"));
        content.push_str(&format!("Description={}\n", self.description));

        if !self.after.is_empty() {
            content.push_str(&format!("After={}\n", self.after.join(" ")));
        }

        if !self.requires.is_empty() {
            content.push_str(&format!("Requires={}\n", self.requires.join(" ")));
        }

        if !self.wants.is_empty() {
            content.push_str(&format!("Wants={}\n", self.wants.join(" ")));
        }

        content.push_str(&format!("\n[Service]\n"));
        content.push_str(&format!("ExecStart={}\n", self.exec_start));

        if let Some(ref exec_stop) = self.exec_stop {
            content.push_str(&format!("ExecStop={}\n", exec_stop));
        }

        content.push_str(&format!("Restart={}\n", self.restart));

        content.push_str(&format!("\n[Install]\n"));
        content.push_str(&format!("WantedBy={}\n", self.wanted_by.join(" ")));

        content
    }
}

/// Service manager for managing system services
pub struct ServiceManager {
    pub services: HashMap<String, ServiceUnit>,
    pub service_dir: PathBuf,
}

impl ServiceManager {
    pub fn new(service_dir: &str) -> Self {
        Self {
            services: HashMap::new(),
            service_dir: PathBuf::from(service_dir),
        }
    }

    /// Add a service
    pub fn add_service(&mut self, service: ServiceUnit) {
        self.services.insert(service.name.clone(), service);
    }

    /// Load service from file
    pub fn load_service(&mut self, name: &str) -> Result<(), ConfigError> {
        let file_path = format!("{}/{}", self.service_dir, format!("{}.service", name));

        let content = fs::read_to_string(&file_path)
            .map_err(|e| ConfigError::ReadError(file_path, e))?;

        let service = self.parse_service_unit(&content, name);
        self.services.insert(name.to_string(), service);

        Ok(())
    }

    /// Parse systemd-style service unit
    fn parse_service_unit(&self, content: &str, name: &str) -> ServiceUnit {
        let mut service = ServiceUnit::new(name);
        let mut current_section = "";

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = &line[1..line.len() - 1];
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let value = line[eq_pos + 1..].trim();

                match current_section {
                    "Unit" => match key {
                        "Description" => service.description = value.to_string(),
                        "After" => {
                            service.after =
                                value.split_whitespace().map(|s| s.to_string()).collect()
                        }
                        "Requires" => {
                            service.requires =
                                value.split_whitespace().map(|s| s.to_string()).collect()
                        }
                        "Wants" => {
                            service.wants =
                                value.split_whitespace().map(|s| s.to_string()).collect()
                        }
                        _ => {}
                    },
                    "Service" => match key {
                        "ExecStart" => service.exec_start = value.to_string(),
                        "ExecStop" => service.exec_stop = Some(value.to_string()),
                        "Restart" => service.restart = value.to_string(),
                        _ => {}
                    },
                    "Install" => match key {
                        "WantedBy" => {
                            service.wanted_by =
                                value.split_whitespace().map(|s| s.to_string()).collect()
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        service
    }

    /// Save service to file
    pub fn save_service(&self, name: &str) -> Result<(), ConfigError> {
        let service = self
            .services
            .get(name)
            .ok_or(ConfigError::NotFound(name.to_string()))?;

        let file_path = format!("{}/{}", self.service_dir, format!("{}.service", name));

        if let Some(parent) = None::<&str> {
            fs::create_dir_all(parent)
                .map_err(|e| ConfigError::WriteError(parent.clone(), e))?;
        }

        fs::write(&file_path, service.to_unit_file())
            .map_err(|e| ConfigError::WriteError(file_path, e))?;

        Ok(())
    }

    /// Initialize service directory
    pub fn initialize(&self) -> Result<(), ConfigError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_manager() {
        let mut manager = SystemConfigManager::new("/tmp/test_config", ConfigType::ConfigFile);
        manager.initialize().unwrap();

        manager.set_value("test.conf", "KEY1", "value1".to_string());
        manager.set_value("test.conf", "KEY2", "value2".to_string());

        assert_eq!(
            manager.get_value("test.conf", "KEY1"),
            Some("value1".to_string())
        );
        assert_eq!(
            manager.get_value("test.conf", "KEY2"),
            Some("value2".to_string())
        );
    }

    #[test]
    fn test_service_unit() {
        let mut service = ServiceUnit::new("test-service");
        service.description = "Test Service".to_string();
        service.exec_start = "/usr/bin/test".to_string();

        let unit_file = service.to_unit_file();
        assert!(unit_file.contains("Description=Test Service"));
        assert!(unit_file.contains("ExecStart=/usr/bin/test"));
    }

    #[test]
    fn test_service_manager() {
        let mut manager = ServiceManager::new("/tmp/test_services");
        manager.initialize().unwrap();

        let mut service = ServiceUnit::new("test-service");
        service.description = "Test Service".to_string();
        service.exec_start = "/usr/bin/test".to_string();

        manager.add_service(service);
        manager.save_service("test-service").unwrap();
    }
}
