extern crate alloc;
// Sovereign ConfigFS - "Everything is a File" and "Principle of Least Astonishment" (POLA) Implementation
// Inspired by Linux sysfs/configfs and BSD sysctl, exposing kernel state, scheduler, and security configurations as virtual files.


use crate::filesystem::vfs::FsError;
use crate::kernel::bore::BoreScheduler;
use crate::security::securelevels::{Securelevel, SovereignSecurelevelManager};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFileType {
    SchedulerBorePenalty,
    SecuritySecurelevel,
    SystemUptime,
}

/// A virtual file node representing a system configuration parameter
pub struct ConfigFileNode {
    pub name: String,
    pub path: String,
    pub file_type: ConfigFileType,
    pub is_writable: bool,
}

/// Sovereign ConfigFS manager executing file-based reads and writes to kernel parameters
pub struct SovereignConfigFS {
    pub files: Vec<ConfigFileNode>,
    pub bore_penalty: AtomicU64, // Virtualized scheduler parameter
    pub securelevel_manager: SovereignSecurelevelManager,
    pub system_uptime_secs: AtomicU64,
}

impl SovereignConfigFS {
    pub fn new() -> Self {
        let mut files = Vec::new();

        // 1. /cfg/scheduler/bore_penalty - controls BORE scheduler deadline penalty scaling
        files.push(ConfigFileNode {
            name: String::from("bore_penalty"),
            path: String::from("/cfg/scheduler/bore_penalty"),
            file_type: ConfigFileType::SchedulerBorePenalty,
            is_writable: true,
        });

        // 2. /cfg/security/securelevel - represents BSD system securelevel status
        files.push(ConfigFileNode {
            name: String::from("securelevel"),
            path: String::from("/cfg/security/securelevel"),
            file_type: ConfigFileType::SecuritySecurelevel,
            is_writable: true, // Only raising allowed (POLA)
        });

        // 3. /cfg/system/uptime - system uptime monitor (read-only)
        files.push(ConfigFileNode {
            name: String::from("uptime"),
            path: String::from("/cfg/system/uptime"),
            file_type: ConfigFileType::SystemUptime,
            is_writable: false,
        });

        SovereignConfigFS {
            files,
            bore_penalty: AtomicU64::new(5), // Default BORE latency multiplier penalty
            securelevel_manager: SovereignSecurelevelManager::new(),
            system_uptime_secs: AtomicU64::new(42), // Simulated initial uptime
        }
    }

    /// Read the virtual configuration file's contents as a String (Everything is a File)
    pub fn read_file(&self, path: &str) -> Result<String, FsError> {
        let node = self
            .files
            .iter()
            .find(|f| f.path == path)
            .ok_or(FsError::NotFound)?;

        match node.file_type {
            ConfigFileType::SchedulerBorePenalty => {
                let val = self.bore_penalty.load(Ordering::SeqCst);
                Ok(alloc::format!("{}\n", val))
            }
            ConfigFileType::SecuritySecurelevel => {
                let lvl = self.securelevel_manager.securelevel();
                Ok(alloc::format!("{:?}\n", lvl))
            }
            ConfigFileType::SystemUptime => {
                let upt = self.system_uptime_secs.load(Ordering::SeqCst);
                Ok(alloc::format!("{}s\n", upt))
            }
        }
    }

    /// Write to the virtual configuration file (POLA: validate syntax and apply changes dynamically)
    pub fn write_file(&mut self, path: &str, content: &str) -> Result<(), FsError> {
        let node = self
            .files
            .iter()
            .find(|f| f.path == path)
            .ok_or(FsError::NotFound)?;

        if !node.is_writable {
            return Err(FsError::PermissionDenied);
        }

        let clean_content = content.trim();

        match node.file_type {
            ConfigFileType::SchedulerBorePenalty => {
                // Parse and validate numeric value (POLA: block invalid formatting)
                let val = clean_content
                    .parse::<u64>()
                    .map_err(|_| FsError::PermissionDenied)?;
                if val > 100 {
                    return Err(FsError::PermissionDenied); // Block excessive penalty latency
                }
                self.bore_penalty.store(val, Ordering::SeqCst);
                Ok(())
            }
            ConfigFileType::SecuritySecurelevel => {
                // Parse securelevel name
                let level = match clean_content {
                    "0" | "Permissive" => Securelevel::Permissive,
                    "1" | "Secure" => Securelevel::Secure,
                    "2" | "HighlySecure" => Securelevel::HighlySecure,
                    "3" | "NetworkSecure" => Securelevel::NetworkSecure,
                    _ => return Err(FsError::PermissionDenied),
                };

                // Raising only (POLA: raise securelevel monotonically, never allow lowering)
                self.securelevel_manager
                    .raise_securelevel(level)
                    .map_err(|_| FsError::PermissionDenied)?;
                Ok(())
            }
            ConfigFileType::SystemUptime => Err(FsError::PermissionDenied),
        }
    }

    /// Simulate a clock tick, incrementing uptime
    pub fn tick(&self) {
        self.system_uptime_secs.fetch_add(1, Ordering::SeqCst);
    }
}

impl Default for SovereignConfigFS {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configfs_everything_is_a_file() {
        let mut cfgfs = SovereignConfigFS::new();

        // 1. Read /cfg/system/uptime
        let uptime_str = cfgfs.read_file("/cfg/system/uptime").unwrap();
        assert_eq!(uptime_str, "42s\n");

        // 2. Read default bore penalty value
        let penalty_str = cfgfs.read_file("/cfg/scheduler/bore_penalty").unwrap();
        assert_eq!(penalty_str, "5\n");

        // 3. Write new bore penalty value and verify update
        cfgfs
            .write_file("/cfg/scheduler/bore_penalty", "15")
            .unwrap();
        let new_penalty = cfgfs.read_file("/cfg/scheduler/bore_penalty").unwrap();
        assert_eq!(new_penalty, "15\n");
    }

    #[test]
    fn test_configfs_pola_input_validation() {
        let mut cfgfs = SovereignConfigFS::new();

        // Try writing non-numeric string to numeric bore_penalty (should return PermissionDenied)
        let res_invalid_str = cfgfs.write_file("/cfg/scheduler/bore_penalty", "abc");
        assert_eq!(res_invalid_str, Err(FsError::PermissionDenied));

        // Try writing excessive value (should return PermissionDenied)
        let res_too_large = cfgfs.write_file("/cfg/scheduler/bore_penalty", "200");
        assert_eq!(res_too_large, Err(FsError::PermissionDenied));

        // Try writing to read-only file (should return PermissionDenied)
        let res_readonly = cfgfs.write_file("/cfg/system/uptime", "100");
        assert_eq!(res_readonly, Err(FsError::PermissionDenied));
    }

    #[test]
    fn test_configfs_securelevel_monotonically_raise_only() {
        let mut cfgfs = SovereignConfigFS::new();

        // Read initial level
        assert_eq!(
            cfgfs.read_file("/cfg/security/securelevel").unwrap(),
            "Permissive\n"
        );

        // Raise to HighlySecure
        cfgfs
            .write_file("/cfg/security/securelevel", "HighlySecure")
            .unwrap();
        assert_eq!(
            cfgfs.read_file("/cfg/security/securelevel").unwrap(),
            "HighlySecure\n"
        );

        // Attempting to lower back to Secure or Permissive should fail (POLA / AccessDenied)
        let res_lower = cfgfs.write_file("/cfg/security/securelevel", "Secure");
        assert_eq!(res_lower, Err(FsError::PermissionDenied));
    }
}
