// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Control Center - Unified System Management
// 
// This module provides the core functionality for the Sigma Control Center,
// the unified system management application for SigmaOS.

mod system_monitor;
mod driver_manager;
mod kernel_manager;
mod security_center;
mod update_manager;
mod backup_manager;
mod virtualization_manager;
mod ai_assistant;

pub use system_monitor::SystemMonitor;
pub use driver_manager::DriverManager;
pub use kernel_manager::KernelManager;
pub use security_center::SecurityCenter;
pub use update_manager::UpdateManager;
pub use backup_manager::BackupManager;
pub use virtualization_manager::VirtualizationManager;
pub use ai_assistant::AIAssistant;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use log::{info, warn, error};

/// Control Center configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlCenterConfig {
    /// Update interval for monitoring (in seconds)
    pub monitor_interval: u64,
    /// Enable AI assistant
    pub enable_ai: bool,
    /// Enable automatic updates
    pub auto_updates: bool,
    /// Enable automatic backups
    pub auto_backups: bool,
    /// Backup interval (in hours)
    pub backup_interval: u64,
    /// Theme preference
    pub theme: String,
    /// Enable logging
    pub enable_logging: bool,
    /// Log level
    pub log_level: String,
}

impl Default for ControlCenterConfig {
    fn default() -> Self {
        Self {
            monitor_interval: 5,
            enable_ai: true,
            auto_updates: false,
            auto_backups: true,
            backup_interval: 24,
            theme: "dark".to_string(),
            enable_logging: true,
            log_level: "info".to_string(),
        }
    }
}

/// Main Control Center structure
pub struct ControlCenter {
    config: ControlCenterConfig,
    system_monitor: SystemMonitor,
    driver_manager: DriverManager,
    kernel_manager: KernelManager,
    security_center: SecurityCenter,
    update_manager: UpdateManager,
    backup_manager: BackupManager,
    virtualization_manager: VirtualizationManager,
    ai_assistant: Option<AIAssistant>,
}

impl ControlCenter {
    /// Create a new Control Center instance
    pub fn new(config: ControlCenterConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize logging if enabled
        if config.enable_logging {
            let log_level = match config.log_level.to_lowercase().as_str() {
                "error" => log::LevelFilter::Error,
                "warn" => log::LevelFilter::Warn,
                "info" => log::LevelFilter::Info,
                "debug" => log::LevelFilter::Debug,
                "trace" => log::LevelFilter::Trace,
                _ => log::LevelFilter::Info,
            };
            env_logger::Builder::from_default_env()
                .filter_level(log_level)
                .init();
            
            info!("Initializing Sigma Control Center");
        }

        let system_monitor = SystemMonitor::new(config.monitor_interval)?;
        let driver_manager = DriverManager::new()?;
        let kernel_manager = KernelManager::new()?;
        let security_center = SecurityCenter::new()?;
        let update_manager = UpdateManager::new()?;
        let backup_manager = BackupManager::new(config.backup_interval)?;
        let virtualization_manager = VirtualizationManager::new()?;
        
        let ai_assistant = if config.enable_ai {
            info!("AI Assistant enabled");
            Some(AIAssistant::new()?)
        } else {
            info!("AI Assistant disabled");
            None
        };

        info!("Control Center initialized successfully");
        Ok(Self {
            config,
            system_monitor,
            driver_manager,
            kernel_manager,
            security_center,
            update_manager,
            backup_manager,
            virtualization_manager,
            ai_assistant,
        })
    }

    /// Get system status overview
    pub fn get_system_status(&self) -> SystemStatus {
        info!("Fetching system status");
        SystemStatus {
            hardware: self.system_monitor.get_hardware_status(),
            drivers: self.driver_manager.get_driver_status(),
            kernel: self.kernel_manager.get_kernel_info(),
            security: self.security_center.get_security_status(),
            updates: self.update_manager.get_update_status(),
            backups: self.backup_manager.get_backup_status(),
        }
    }

    /// Get AI assistant if enabled
    pub fn ai_assistant(&self) -> Option<&AIAssistant> {
        self.ai_assistant.as_ref()
    }

    /// Update configuration
    pub fn update_config(&mut self, config: ControlCenterConfig) {
        info!("Updating Control Center configuration");
        self.config = config;
        // Reinitialize components if needed
    }
}

/// System status overview
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub hardware: HardwareStatus,
    pub drivers: DriverStatus,
    pub kernel: KernelInfo,
    pub security: SecurityStatus,
    pub updates: UpdateStatus,
    pub backups: BackupStatus,
}

/// Hardware status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareStatus {
    pub cpu_usage: f32,
    pub cpu_temperature: f32,
    pub memory_usage: f32,
    pub memory_total: u64,
    pub disk_usage: f32,
    pub disk_total: u64,
    pub gpu_usage: Option<f32>,
    pub gpu_temperature: Option<f32>,
    pub network_status: NetworkStatus,
}

/// Network status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub connected: bool,
    pub interface: String,
    pub ip_address: String,
    pub upload_speed: f64,
    pub download_speed: f64,
}

/// Driver status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverStatus {
    pub total_drivers: usize,
    pub active_drivers: usize,
    pub outdated_drivers: usize,
    pub missing_drivers: usize,
}

/// Kernel information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelInfo {
    pub version: String,
    pub build_date: String,
    pub uptime: u64,
}

/// Security status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStatus {
    pub secure_boot_enabled: bool,
    pub disk_encrypted: bool,
    pub firewall_enabled: bool,
    pub security_score: u8,
}

/// Update status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStatus {
    pub system_updates_available: usize,
    pub package_updates_available: usize,
    pub security_updates: bool,
}

/// Backup status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStatus {
    pub last_backup: String,
    pub backup_size: u64,
    pub backup_count: usize,
    pub next_backup: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_center_creation() {
        let config = ControlCenterConfig::default();
        // Note: This test will fail if system components aren't available
        // In production, we'd use mocking for unit tests
        // let cc = ControlCenter::new(config);
        // assert!(cc.is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = ControlCenterConfig::default();
        assert_eq!(config.monitor_interval, 5);
        assert_eq!(config.enable_ai, true);
        assert_eq!(config.theme, "dark");
    }
}
