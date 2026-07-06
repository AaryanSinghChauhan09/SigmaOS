// SPDX-License-Identifier: GPL-2.0-or-later
// Integration tests for Sigma Control Center

use sigma_control_center::{ControlCenter, ControlCenterConfig, SystemStatus};

#[test]
fn test_control_center_creation() {
    let config = ControlCenterConfig::default();
    let result = ControlCenter::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_system_status() {
    let config = ControlCenterConfig::default();
    let cc = ControlCenter::new(config).unwrap();
    let status = cc.get_system_status();
    
    // Verify hardware status
    assert!(status.hardware.cpu_usage >= 0.0 && status.hardware.cpu_usage <= 100.0);
    assert!(status.hardware.memory_usage >= 0.0 && status.hardware.memory_usage <= 100.0);
    
    // Verify security status
    assert!(status.security.security_score <= 100);
    
    // Verify backup status
    assert!(status.backup_count >= 0);
}

#[test]
fn test_ai_assistant_integration() {
    let mut config = ControlCenterConfig::default();
    config.enable_ai = true;
    
    let cc = ControlCenter::new(config).unwrap();
    let ai = cc.ai_assistant();
    assert!(ai.is_some());
}

#[test]
fn test_config_update() {
    let config = ControlCenterConfig::default();
    let mut cc = ControlCenter::new(config).unwrap();
    
    let new_config = ControlCenterConfig {
        monitor_interval: 10,
        enable_ai: false,
        auto_updates: true,
        auto_backups: false,
        backup_interval: 12,
        theme: "light".to_string(),
    };
    
    cc.update_config(new_config);
}
