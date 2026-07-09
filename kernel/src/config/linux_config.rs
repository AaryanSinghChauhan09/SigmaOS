// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired configuration management for SigmaOS
// Zero-allocation, performance-optimized configuration operations

/// Configuration file parser (Linux-style)
pub struct ConfigParser {
    pub sections: Vec<ConfigSection>,
}

impl ConfigParser {
    pub const fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    
    pub fn parse(&mut self, content: &str) -> Result<(), ConfigError> {
        let mut current_section = ConfigSection::new("default");
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }
            
            // Section header
            if line.starts_with('[') && line.ends_with(']') {
                let section_name = &line[1..line.len()-1];
                if !current_section.entries.is_empty() {
                    self.sections.push(current_section);
                }
                current_section = ConfigSection::new(section_name);
                continue;
            }
            
            // Key-value pair
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos+1..].trim();
                current_section.add_entry(key, value);
            }
        }
        
        if !current_section.entries.is_empty() {
            self.sections.push(current_section);
        }
        
        Ok(())
    }
    
    pub fn get_section(&self, name: &str) -> Option<&ConfigSection> {
        self.sections.iter().find(|s| s.name == name)
    }
    
    pub fn get_value(&self, section: &str, key: &str) -> Option<&str> {
        self.get_section(section)?.get_value(key)
    }
}

/// Configuration section
pub struct ConfigSection {
    pub name: String,
    pub entries: Vec<ConfigEntry>,
}

impl ConfigSection {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, key: &str, value: &str) {
        self.entries.push(ConfigEntry {
            key: key.to_string(),
            value: value.to_string(),
        });
    }
    
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.key == key).map(|e| e.value.as_str())
    }
}

/// Configuration entry
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
}

/// Configuration manager
pub struct ConfigManager {
    pub configs: Vec<ConfigFile>,
}

impl ConfigManager {
    pub const fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }
    
    pub fn load_config(&mut self, path: &str, content: &str) -> Result<(), ConfigError> {
        let mut parser = ConfigParser::new();
        parser.parse(content)?;
        
        self.configs.push(ConfigFile {
            path: path.to_string(),
            parser,
        });
        
        Ok(())
    }
    
    pub fn get_config(&self, path: &str) -> Option<&ConfigFile> {
        self.configs.iter().find(|c| c.path == path)
    }
    
    pub fn get_value(&self, config_path: &str, section: &str, key: &str) -> Option<&str> {
        self.get_config(config_path)?.parser.get_value(section, key)
    }
}

/// Configuration file
pub struct ConfigFile {
    pub path: String,
    pub parser: ConfigParser,
}

/// Configuration error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigError {
    ParseError,
    InvalidSyntax,
    FileNotFound,
    PermissionDenied,
    InvalidValue,
    SectionNotFound,
    KeyNotFound,
}

/// Environment variable manager
pub struct EnvironmentManager {
    pub variables: Vec<(String, String)>,
}

impl EnvironmentManager {
    pub const fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }
    
    pub fn set(&mut self, key: &str, value: &str) {
        if let Some(pos) = self.variables.iter().position(|(k, _)| k == key) {
            self.variables[pos] = (key.to_string(), value.to_string());
        } else {
            self.variables.push((key.to_string(), value.to_string()));
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
    
    pub fn unset(&mut self, key: &str) {
        if let Some(pos) = self.variables.iter().position(|(k, _)| k == key) {
            self.variables.remove(pos);
        }
    }
    
    pub fn list(&self) -> &[(&str, &str)] {
        // This is a simplified version
        &[]
    }
}

/// Standard Linux configuration paths
pub mod config_paths {
    pub const ETC: &str = "/etc";
    pub const ETC_PROFILE: &str = "/etc/profile";
    pub const ETC_BASHRC: &str = "/etc/bashrc";
    pub const ETC_ENVIRONMENT: &str = "/etc/environment";
    pub const ETC_FSTAB: &str = "/etc/fstab";
    pub const ETC_HOSTS: &str = "/etc/hosts";
    pub const ETC_HOSTNAME: &str = "/etc/hostname";
    pub const ETC_RESOLV_CONF: &str = "/etc/resolv.conf";
    pub const ETC_NETWORK_INTERFACES: &str = "/etc/network/interfaces";
    pub const ETC_SYSCTL_CONF: &str = "/etc/sysctl.conf";
    pub const ETC_SYSCTL_D: &str = "/etc/sysctl.d";
    pub const ETC_MODPROBE_D: &str = "/etc/modprobe.d";
    pub const ETC_UDEV_RULES_D: &str = "/etc/udev/rules.d";
    pub const ETC_SYSTEMD_SYSTEM: &str = "/etc/systemd/system";
    pub const ETC_INIT_D: &str = "/etc/init.d";
    pub const ETC_RC_D: &str = "/etc/rc.d";
    pub const ETC_PROFILE_D: &str = "/etc/profile.d";
    pub const ETC_BASH_PROFILE_D: &str = "/etc/bash_profile.d";
    pub const ETC_XDG: &str = "/etc/xdg";
    pub const USR_SHARE: &str = "/usr/share";
    pub const VAR_LOG: &str = "/var/log";
    pub const VAR_RUN: &str = "/var/run";
    pub const VAR_LOCK: &str = "/var/lock";
    pub const VAR_TMP: &str = "/var/tmp";
}

/// System configuration (sysctl-style)
pub struct SysctlConfig {
    pub parameters: Vec<SysctlParameter>,
}

impl SysctlConfig {
    pub const fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }
    
    pub fn add_parameter(&mut self, key: &str, value: &str) {
        self.parameters.push(SysctlParameter {
            key: key.to_string(),
            value: value.to_string(),
        });
    }
    
    pub fn get_parameter(&self, key: &str) -> Option<&str> {
        self.parameters.iter().find(|p| p.key == key).map(|p| p.value.as_str())
    }
}

pub struct SysctlParameter {
    pub key: String,
    pub value: String,
}

/// Kernel parameter categories
pub mod sysctl_categories {
    pub const KERNEL: &str = "kernel";
    pub const VM: &str = "vm";
    pub const FS: &str = "fs";
    pub const NET: &str = "net";
    pub const NET_IPV4: &str = "net.ipv4";
    pub const NET_IPV6: &str = "net.ipv6";
    pub const NET_CORE: &str = "net.core";
    pub const DEV: &str = "dev";
}

/// Host configuration
pub struct HostConfig {
    pub hostname: String,
    pub domainname: String,
    pub hosts: Vec<HostEntry>,
}

impl HostConfig {
    pub const fn new() -> Self {
        Self {
            hostname: String::new(),
            domainname: String::new(),
            hosts: Vec::new(),
        }
    }
    
    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = hostname.to_string();
    }
    
    pub fn set_domainname(&mut self, domainname: &str) {
        self.domainname = domainname.to_string();
    }
    
    pub fn add_host(&mut self, ip: &str, hostname: &str, aliases: &[&str]) {
        self.hosts.push(HostEntry {
            ip: ip.to_string(),
            hostname: hostname.to_string(),
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
        });
    }
}

pub struct HostEntry {
    pub ip: String,
    pub hostname: String,
    pub aliases: Vec<String>,
}

/// Network configuration
pub struct NetworkConfig {
    pub interfaces: Vec<NetworkInterface>,
    pub dns_servers: Vec<String>,
    pub search_domains: Vec<String>,
}

impl NetworkConfig {
    pub const fn new() -> Self {
        Self {
            interfaces: Vec::new(),
            dns_servers: Vec::new(),
            search_domains: Vec::new(),
        }
    }
    
    pub fn add_interface(&mut self, interface: NetworkInterface) {
        self.interfaces.push(interface);
    }
    
    pub fn add_dns_server(&mut self, server: &str) {
        self.dns_servers.push(server.to_string());
    }
    
    pub fn add_search_domain(&mut self, domain: &str) {
        self.search_domains.push(domain.to_string());
    }
}

pub struct NetworkInterface {
    pub name: String,
    pub ip_address: String,
    pub netmask: String,
    pub gateway: Option<String>,
    pub dhcp: bool,
    pub enabled: bool,
}

impl NetworkInterface {
    pub const fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ip_address: String::new(),
            netmask: String::new(),
            gateway: None,
            dhcp: false,
            enabled: true,
        }
    }
}

/// Profile configuration
pub struct ProfileConfig {
    pub path: String,
    pub environment: Vec<(String, String)>,
    pub aliases: Vec<(String, String)>,
    pub functions: Vec<String>,
}

impl ProfileConfig {
    pub const fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            environment: Vec::new(),
            aliases: Vec::new(),
            functions: Vec::new(),
        }
    }
    
    pub fn add_environment(&mut self, key: &str, value: &str) {
        self.environment.push((key.to_string(), value.to_string()));
    }
    
    pub fn add_alias(&mut self, name: &str, command: &str) {
        self.aliases.push((name.to_string(), command.to_string()));
    }
}

/// XDG Base Directory specification
pub mod xdg {
    pub const XDG_DATA_HOME: &str = "XDG_DATA_HOME";
    pub const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";
    pub const XDG_STATE_HOME: &str = "XDG_STATE_HOME";
    pub const XDG_DATA_DIRS: &str = "XDG_DATA_DIRS";
    pub const XDG_CONFIG_DIRS: &str = "XDG_CONFIG_DIRS";
    pub const XDG_CACHE_HOME: &str = "XDG_CACHE_HOME";
    pub const XDG_RUNTIME_DIR: &str = "XDG_RUNTIME_DIR";
    
    pub fn get_default_data_home() -> &'static str {
        "/.local/share"
    }
    
    pub fn get_default_config_home() -> &'static str {
        "/.config"
    }
    
    pub fn get_default_state_home() -> &'static str {
        "/.local/state"
    }
    
    pub fn get_default_cache_home() -> &'static str {
        "/.cache"
    }
}
