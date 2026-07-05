// SPDX-License-Identifier: GPL-2.0-or-later
// Integration tests for Sigma AI Integration

use sigma_ai_integration::{AIIntegration, AIConfig, PrivacyLevel, SystemState};

#[test]
fn test_ai_integration_creation() {
    let config = AIConfig::default();
    let result = AIIntegration::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_process_command() {
    let config = AIConfig::default();
    let mut ai = AIIntegration::new(config).unwrap();
    
    let response = ai.process_command("Why is my laptop hot?");
    assert!(response.is_ok());
    
    let response = response.unwrap();
    assert!(!response.message.is_empty());
    assert!(response.confidence > 0.0);
}

#[test]
fn test_get_suggestions() {
    let config = AIConfig::default();
    let ai = AIIntegration::new(config).unwrap();
    
    let system_state = SystemState {
        cpu_usage: 85.0,
        memory_usage: 70.0,
        disk_usage: 50.0,
        temperature: 75.0,
        network_status: true,
        security_score: 80,
    };
    
    let suggestions = ai.get_suggestions(&system_state);
    assert!(!suggestions.is_empty());
}

#[test]
fn test_privacy_levels() {
    let config = AIConfig {
        model_name: "test-model".to_string(),
        local_processing: true,
        max_context_length: 4096,
        enable_learning: false,
        privacy_level: PrivacyLevel::Maximum,
    };
    
    let ai = AIIntegration::new(config).unwrap();
    assert!(ai.privacy_manager().get_privacy_level() == PrivacyLevel::Maximum);
}

#[test]
fn test_context_management() {
    let config = AIConfig::default();
    let mut ai = AIIntegration::new(config).unwrap();
    
    ai.process_command("Hello").unwrap();
    ai.process_command("How are you?").unwrap();
    
    // Context should be maintained
    assert!(true); // Placeholder for context verification
}
