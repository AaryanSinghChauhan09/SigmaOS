// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Kernel Manager - Kernel version management

use serde::{Deserialize, Serialize};
use std::process::Command;
use crate::control_center::KernelInfo;

/// Kernel Manager for kernel version management
pub struct KernelManager {
    current_kernel: KernelInfo,
    available_kernels: Vec<KernelVersion>,
}

impl KernelManager {
    /// Create a new Kernel Manager
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let current_kernel = Self::get_current_kernel()?;
        let available_kernels = Self::fetch_available_kernels()?;
        
        Ok(Self {
            current_kernel,
            available_kernels,
        })
    }

    /// Get current kernel information
    fn get_current_kernel() -> Result<KernelInfo, Box<dyn std::error::Error>> {
        let output = Command::new("uname")
            .args(&["-r"])
            .output()?;
        
        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        // Get uptime
        let uptime_output = Command::new("cat")
            .args(&["/proc/uptime"])
            .output()?;
        
        let uptime_str = String::from_utf8_lossy(&uptime_output.stdout);
        let uptime: u64 = uptime_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        
        // Get build date from /proc/version
        let build_date = if let Ok(version_output) = Command::new("cat").args(&["/proc/version"]).output() {
            let version_str = String::from_utf8_lossy(&version_output.stdout);
            // Extract build date from kernel version string
            version_str.split('#').nth(1)
                .and_then(|s| s.split_whitespace().nth(1))
                .unwrap_or("Unknown")
                .to_string()
        } else {
            "Unknown".to_string()
        };
        
        Ok(KernelInfo {
            version,
            build_date,
            uptime,
        })
    }

    /// Fetch available kernel versions
    fn fetch_available_kernels() -> Result<Vec<KernelVersion>, Box<dyn std::error::Error>> {
        // Placeholder implementation - would query package manager
        Ok(vec![
            KernelVersion {
                version: "6.6.0-sigma1".to_string(),
                release_date: "2024-01-15".to_string(),
                is_stable: true,
                is_lts: true,
                changelog: "Initial stable release".to_string(),
            },
            KernelVersion {
                version: "6.7.0-sigma1".to_string(),
                release_date: "2024-02-01".to_string(),
                is_stable: true,
                is_lts: false,
                changelog: "Performance improvements and bug fixes".to_string(),
            },
        ])
    }

    /// Get current kernel info
    pub fn get_kernel_info(&self) -> KernelInfo {
        self.current_kernel.clone()
    }

    /// Get available kernel versions
    pub fn get_available_kernels(&self) -> Vec<KernelVersion> {
        self.available_kernels.clone()
    }

    /// Install a new kernel version
    pub fn install_kernel(&mut self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would use the package manager
        // to install the specified kernel version
        println!("Installing kernel version: {}", version);
        Ok(())
    }

    /// Switch to a different kernel version
    pub fn switch_kernel(&mut self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would update the bootloader
        // configuration to boot the specified kernel
        println!("Switching to kernel version: {}", version);
        self.current_kernel.version = version.to_string();
        Ok(())
    }

    /// Remove a kernel version
    pub fn remove_kernel(&mut self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Prevent removing the current kernel
        if version == self.current_kernel.version {
            return Err("Cannot remove the currently running kernel".into());
        }
        
        // In a real implementation, this would use the package manager
        // to remove the specified kernel version
        println!("Removing kernel version: {}", version);
        Ok(())
    }

    /// Check for kernel updates
    pub fn check_updates(&self) -> Option<KernelVersion> {
        self.available_kernels
            .iter()
            .find(|k| k.version > self.current_kernel.version && k.is_stable)
            .cloned()
    }

    /// Get kernel security advisories
    pub fn get_security_advisories(&self) -> Vec<SecurityAdvisory> {
        // Placeholder implementation - would query a security database
        vec![
            SecurityAdvisory {
                id: "CVE-2024-1234".to_string(),
                severity: AdvisorySeverity::High,
                description: "Buffer overflow in network driver".to_string(),
                affected_versions: vec!["6.5.0".to_string(), "6.6.0-sigma1".to_string()],
                fixed_in: Some("6.6.1-sigma1".to_string()),
            },
        ]
    }
}

/// Kernel version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelVersion {
    pub version: String,
    pub release_date: String,
    pub is_stable: bool,
    pub is_lts: bool,
    pub changelog: String,
}

/// Security advisory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAdvisory {
    pub id: String,
    pub severity: AdvisorySeverity,
    pub description: String,
    pub affected_versions: Vec<String>,
    pub fixed_in: Option<String>,
}

/// Advisory severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvisorySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_manager_creation() {
        let manager = KernelManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_get_kernel_info() {
        let manager = KernelManager::new().unwrap();
        let info = manager.get_kernel_info();
        assert!(!info.version.is_empty());
    }
}
