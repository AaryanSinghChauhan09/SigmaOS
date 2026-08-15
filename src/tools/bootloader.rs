//! Bootloader (GRUB/systemd-boot Inspiration)
//! Boot menu, boot entry management, and UEFI support

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};

/// Boot entry
#[derive(Debug, Clone)]
pub struct BootEntry {
    pub id: String,
    pub name: String,
    pub kernel: String,
    pub initrd: String,
    pub options: Vec<String>,
    pub efi_path: String,
}

impl BootEntry {
    pub fn new(name: &str, kernel: &str, initrd: &str) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            kernel: kernel.to_string(),
            initrd: initrd.to_string(),
            options: Vec::new(),
            efi_path: String::new(),
        }
    }

    fn generate_id() -> String {
        "boot_entry_001".to_string()
    }

    pub fn add_option(&mut self, option: &str) {
        self.options.push(option.to_string());
    }

    pub fn set_efi_path(&mut self, path: &str) {
        self.efi_path = path.to_string();
    }
}

/// Global settings
#[derive(Debug, Clone)]
pub struct GlobalSettings {
    pub timeout: u32,
    pub default_entry: String,
    pub graphics_mode: GraphicsMode,
    pub theme: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsMode {
    Text,
    Auto,
    Keep,
}

impl GlobalSettings {
    pub fn new() -> Self {
        Self {
            timeout: 5,
            default_entry: "0".to_string(),
            graphics_mode: GraphicsMode::Auto,
            theme: "default".to_string(),
        }
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    pub fn set_default_entry(&mut self, entry: &str) {
        self.default_entry = entry.to_string();
    }
}

/// Boot configuration
pub struct BootConfiguration {
    pub entries: Vec<BootEntry>,
    pub global_settings: GlobalSettings,
}

impl BootConfiguration {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            global_settings: GlobalSettings::new(),
        }
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        self.entries.push(entry);
    }

    pub fn get_entry(&mut self, id: &str) -> Option<&mut BootEntry> {
        self.entries.iter_mut().find(|e| e.id == id || e.name == id)
    }

    pub fn set_default_entry(&mut self, entry_id: &str) {
        self.global_settings.set_default_entry(entry_id);
    }

    pub fn set_timeout(&mut self, timeout: u32) {
        self.global_settings.set_timeout(timeout);
    }
}

/// Bootloader
pub struct Bootloader {
    pub configuration: BootConfiguration,
    pub efi_mode: bool,
    pub secure_boot: bool,
}

impl Bootloader {
    pub fn new() -> Self {
        Self {
            configuration: BootConfiguration::new(),
            efi_mode: true,
            secure_boot: false,
        }
    }

    pub fn add_entry(&mut self, entry: BootEntry) {
        self.configuration.add_entry(entry);
    }

    pub fn boot_entry(&self, entry_id: &str) -> Result<(), BootloaderError> {
        if self.configuration.entries.iter().any(|e| e.id == entry_id || e.name == entry_id) {
            // Boot the entry
            Ok(())
        } else {
            Err(BootloaderError::EntryNotFound)
        }
    }

    pub fn install(&mut self) -> Result<(), BootloaderError> {
        // Install bootloader to disk
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), BootloaderError> {
        // Update bootloader configuration
        Ok(())
    }

    pub fn set_efi_mode(&mut self, efi: bool) {
        self.efi_mode = efi;
    }

    pub fn enable_secure_boot(&mut self) {
        self.secure_boot = true;
    }

    pub fn disable_secure_boot(&mut self) {
        self.secure_boot = false;
    }

    pub fn generate_config(&self) -> String {
        let mut config = String::new();
        
        config.push_str(&format!("timeout {}\n", self.configuration.global_settings.timeout));
        config.push_str(&format!("default {}\n", self.configuration.global_settings.default_entry));
        
        for entry in &self.configuration.entries {
            config.push_str(&format!("menuentry \"{}\" {{\n", entry.name));
            config.push_str(&format!("    linux {}\n", entry.kernel));
            config.push_str(&format!("    initrd {}\n", entry.initrd));
            for option in &entry.options {
                config.push_str(&format!("    options {}\n", option));
            }
            config.push_str("}\n");
        }
        
        config
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootloaderError {
    EntryNotFound,
    InstallationFailed,
    UpdateFailed,
    SecureBootError,
}

impl Default for Bootloader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_entry() {
        let entry = BootEntry::new("SigmaOS", "/vmlinuz", "/initrd");
        assert_eq!(entry.name, "SigmaOS");
    }

    #[test]
    fn test_boot_configuration() {
        let mut config = BootConfiguration::new();
        let entry = BootEntry::new("SigmaOS", "/vmlinuz", "/initrd");
        config.add_entry(entry);
        assert_eq!(config.entries.len(), 1);
    }

    #[test]
    fn test_bootloader() {
        let mut bootloader = Bootloader::new();
        let entry = BootEntry::new("SigmaOS", "/vmlinuz", "/initrd");
        bootloader.add_entry(entry);
        assert_eq!(bootloader.configuration.entries.len(), 1);
    }

    #[test]
    fn test_generate_config() {
        let mut bootloader = Bootloader::new();
        let entry = BootEntry::new("SigmaOS", "/vmlinuz", "/initrd");
        bootloader.add_entry(entry);
        let config = bootloader.generate_config();
        assert!(config.contains("SigmaOS"));
    }
}