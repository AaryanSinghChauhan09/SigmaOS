// SPDX-License-Identifier: MIT
// SigmaOS Configuration Management - Arch Linux Inspired
// Simple, text-based configuration system following Arch's KISS principle

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Configuration section (like INI sections but simpler)
#[derive(Debug, Clone, PartialEq)]
pub struct ConfigSection {
    pub name: String,
    pub values: BTreeMap<String, String>,
    pub comments: Vec<String>,
}

impl ConfigSection {
    pub fn new(name: String) -> Self {
        Self {
            name,
            values: BTreeMap::new(),
            comments: Vec::new(),
        }
    }

    /// Set a configuration value
    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    /// Get a configuration value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    /// Get a configuration value with default
    pub fn get_or(&self, key: &str, default: &str) -> String {
        self.values.get(key).cloned().unwrap_or_else(|| default.to_string())
    }

    /// Add a comment
    pub fn add_comment(&mut self, comment: String) {
        self.comments.push(comment);
    }
}

/// Main configuration file
#[derive(Debug, Clone)]
pub struct ConfigFile {
    pub path: PathBuf,
    pub sections: BTreeMap<String, ConfigSection>,
    pub global_comments: Vec<String>,
    pub global_values: BTreeMap<String, String>,
}

impl ConfigFile {
    /// Create a new configuration file
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            sections: BTreeMap::new(),
            global_comments: Vec::new(),
            global_values: BTreeMap::new(),
        }
    }

    /// Load configuration from file
    pub fn load(&mut self) -> io::Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let file = fs::File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut current_section: Option<String> = None;
        let mut current_comments: Vec<String> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            // Skip empty lines
            if trimmed.is_empty() {
                continue;
            }

            // Handle comments
            if trimmed.starts_with('#') {
                current_comments.push(trimmed.to_string());
                continue;
            }

            // Handle section headers [section]
            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                let section_name = trimmed[1..trimmed.len()-1].to_string();
                current_section = Some(section_name.clone());

                let mut section = ConfigSection::new(section_name.clone());
                section.comments = current_comments.clone();
                self.sections.insert(section_name, section);
                current_comments.clear();
                continue;
            }

            // Handle key=value pairs
            if let Some((key, value)) = trimmed.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().trim_matches('"').trim_matches('\'').to_string();

                if let Some(ref section_name) = current_section {
                    if let Some(section) = self.sections.get_mut(section_name) {
                        section.set(key, value);
                    }
                } else {
                    self.global_values.insert(key, value);
                }
            }
        }

        Ok(())
    }

    /// Save configuration to file
    pub fn save(&self) -> io::Result<()> {
        let mut file = fs::File::create(&self.path)?;

        // Write global comments
        for comment in &self.global_comments {
            writeln!(file, "{}", comment)?;
        }

        // Write global values
        for (key, value) in &self.global_values {
            writeln!(file, "{}={}", key, value)?;
        }

        // Write sections
        for (name, section) in &self.sections {
            writeln!(file)?; // Empty line before section

            // Write section comments
            for comment in &section.comments {
                writeln!(file, "{}", comment)?;
            }

            writeln!(file, "[{}]", name)?;

            // Write section values
            for (key, value) in &section.values {
                writeln!(file, "{}={}", key, value)?;
            }
        }

        Ok(())
    }

    /// Get or create a section
    pub fn section(&mut self, name: &str) -> &mut ConfigSection {
        self.sections.entry(name.to_string()).or_insert_with(|| ConfigSection::new(name.to_string()))
    }

    /// Get a section
    pub fn get_section(&self, name: &str) -> Option<&ConfigSection> {
        self.sections.get(name)
    }

    /// Get global value
    pub fn get_global(&self, key: &str) -> Option<&String> {
        self.global_values.get(key)
    }

    /// Set global value
    pub fn set_global(&mut self, key: String, value: String) {
        self.global_values.insert(key, value);
    }

    /// Add global comment
    pub fn add_global_comment(&mut self, comment: String) {
        self.global_comments.push(comment);
    }
}

/// Configuration manager for SigmaOS
#[derive(Debug, Clone)]
pub struct ConfigManager {
    pub config_dir: PathBuf,
    pub configs: BTreeMap<String, ConfigFile>,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            configs: BTreeMap::new(),
        }
    }

    /// Load all configuration files
    pub fn load_all(&mut self) -> io::Result<()> {
        if !self.config_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.config_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "conf") {
                let name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                let mut config = ConfigFile::new(path);
                config.load()?;
                self.configs.insert(name, config);
            }
        }

        Ok(())
    }

    /// Get a configuration file
    pub fn get_config(&self, name: &str) -> Option<&ConfigFile> {
        self.configs.get(name)
    }

    /// Get a mutable configuration file
    pub fn get_config_mut(&mut self, name: &str) -> Option<&mut ConfigFile> {
        self.configs.get_mut(name)
    }

    /// Create a new configuration file
    pub fn create_config(&mut self, name: &str) -> io::Result<&mut ConfigFile> {
        let path = self.config_dir.join(format!("{}.conf", name));
        let config = ConfigFile::new(path);
        self.configs.insert(name.to_string(), config);
        Ok(self.configs.get_mut(name).unwrap())
    }

    /// Save all configurations
    pub fn save_all(&self) -> io::Result<()> {
        for config in self.configs.values() {
            config.save()?;
        }
        Ok(())
    }

    /// Load a specific configuration
    pub fn load_config(&mut self, name: &str) -> io::Result<()> {
        let path = self.config_dir.join(format!("{}.conf", name));
        let mut config = ConfigFile::new(path);
        config.load()?;
        self.configs.insert(name.to_string(), config);
        Ok(())
    }

    /// Save a specific configuration
    pub fn save_config(&self, name: &str) -> io::Result<()> {
        if let Some(config) = self.configs.get(name) {
            config.save()?;
        }
        Ok(())
    }
}

/// Arch-style pacman.conf generator
pub fn generate_pacman_conf() -> String {
    let mut conf = String::new();

    conf.push_str("# SigmaOS pacman configuration\n");
    conf.push_str("# Generated by SigmaOS Config Manager\n\n");

    conf.push_str("[options]\n");
    conf.push_str("HoldPkg     = pacman glibc\n");
    conf.push_str("Architecture = auto\n");
    conf.push_str("CheckSpace\n");
    conf.push_str("ParallelDownloads = 5\n");
    conf.push_str("VerbosePkgLists\n");
    conf.push_str("Color\n");
    conf.push_str("TotalDownload\n");
    conf.push_str("CheckUpdates\n");
    conf.push_str("\n");

    conf.push_str("[core]\n");
    conf.push_str("Server = https://mirror.example.com/core\n");
    conf.push_str("\n");

    conf.push_str("[extra]\n");
    conf.push_str("Server = https://mirror.example.com/extra\n");
    conf.push_str("\n");

    conf.push_str("[community]\n");
    conf.push_str("Server = https://mirror.example.com/community\n");
    conf.push_str("\n");

    conf.push_str("[multilib]\n");
    conf.push_str("Server = https://mirror.example.com/multilib\n");
    conf.push_str("\n");

    conf
}

/// Arch-style mirrorlist generator
pub fn generate_mirrorlist(mirrors: Vec<String>) -> String {
    let mut list = String::new();

    list.push_str("# SigmaOS mirrorlist\n");
    list.push_str("# Generated by SigmaOS Config Manager\n\n");

    for mirror in mirrors {
        list.push_str(&format!("Server = {}\n", mirror));
    }

    list
}

/// Arch-style mkinitcpio.conf generator
pub fn generate_mkinitcpio_conf() -> String {
    let mut conf = String::new();

    conf.push_str("# SigmaOS mkinitcpio configuration\n");
    conf.push_str("# Generated by SigmaOS Config Manager\n\n");

    conf.push_str("MODULES=()\n");
    conf.push_str("BINARIES=()\n");
    conf.push_str("FILES=()\n");
    conf.push_str("\n");

    conf.push_str("HOOKS=(base udev autodetect modconf block filesystems fsck)\n");
    conf.push_str("\n");

    conf.push_str("# Use compressing utilities like zstd. Use uncompressed images\n");
    conf.push_str("# or another compression tool (gzip, bzip2, etc.) if compression\n");
    conf.push_str("# is not desired for speed.\n");
    conf.push_str("#COMPRESSION=\"zstd -1 -T0\"\n");
    conf.push_str("#COMPRESSION_FALLBACK=\"zlib\"\n");
    conf.push_str("COMPRESSION=\"zstd -1 -T0\"\n");
    conf.push_str("COMPRESSION_FALLBACK=\"zlib\"\n");
    conf.push_str("\n");

    conf.push_str("# Use mkinitcpio.conf.d/ to override the following options\n");
    conf.push_str("#MODULES_DEPS=()\n");
    conf.push_str("#INSTALL=\n");
    conf.push_str("#NO_MODULES=0\n");
    conf.push_str("#HOOKS=()\n");

    conf
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_section() {
        let mut section = ConfigSection::new("test".to_string());
        section.set("key".to_string(), "value".to_string());

        assert_eq!(section.get("key"), Some(&"value".to_string()));
        assert_eq!(section.get_or("missing", "default"), "default".to_string());
    }

    #[test]
    fn test_config_file() {
        let temp_dir = env::temp_dir();
        let config_path = temp_dir.join("test_config.conf");

        let mut config = ConfigFile::new(config_path.clone());
        config.set_global("global_key".to_string(), "global_value".to_string());
        config.section("section1").set("key1".to_string(), "value1".to_string());

        config.save().unwrap();

        let mut loaded_config = ConfigFile::new(config_path.clone());
        loaded_config.load().unwrap();

        assert_eq!(loaded_config.get_global("global_key"), Some(&"global_value".to_string()));
        assert_eq!(loaded_config.get_section("section1").unwrap().get("key1"), Some(&"value1".to_string()));

        // Cleanup
        let _ = fs::remove_file(config_path);
    }

    #[test]
    fn test_config_manager() {
        let temp_dir = env::temp_dir().join("sigmaos_config_test");
        let _ = fs::create_dir_all(&temp_dir);

        let mut manager = ConfigManager::new(temp_dir.clone());

        manager.create_config("test").unwrap()
            .set_global("test_key".to_string(), "test_value".to_string());

        manager.save_all().unwrap();

        let mut loaded_manager = ConfigManager::new(temp_dir.clone());
        loaded_manager.load_all().unwrap();

        let config = loaded_manager.get_config("test").unwrap();
        assert_eq!(config.get_global("test_key"), Some(&"test_value".to_string()));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_pacman_conf_generation() {
        let conf = generate_pacman_conf();
        assert!(conf.contains("[options]"));
        assert!(conf.contains("[core]"));
        assert!(conf.contains("[extra]"));
    }

    #[test]
    fn test_mirrorlist_generation() {
        let mirrors = vec![
            "https://mirror1.example.com".to_string(),
            "https://mirror2.example.com".to_string(),
        ];

        let list = generate_mirrorlist(mirrors);
        assert!(list.contains("Server = https://mirror1.example.com"));
        assert!(list.contains("Server = https://mirror2.example.com"));
    }

    #[test]
    fn test_mkinitcpio_conf_generation() {
        let conf = generate_mkinitcpio_conf();
        assert!(conf.contains("HOOKS="));
        assert!(conf.contains("COMPRESSION="));
    }
}
