// SPDX-License-Identifier: GPL-2.0-or-later
// Integration tests for Sigma Dev Studio

use sigma_dev_studio::{DevStudio, DevStudioConfig};

#[test]
fn test_dev_studio_creation() {
    let config = DevStudioConfig::default();
    let result = DevStudio::new(config);
    assert!(result.is_ok());
}

#[test]
fn test_git_operations() {
    let config = DevStudioConfig::default();
    let mut studio = DevStudio::new(config).unwrap();
    
    // Test repository initialization
    let repo_id = studio.git_manager.init_repository("/tmp/test_repo");
    assert!(repo_id.is_ok());
    
    // Test commit
    let commit_id = studio.git_manager.commit(&repo_id.unwrap(), "Initial commit");
    assert!(commit_id.is_ok());
}

#[test]
fn test_docker_operations() {
    let config = DevStudioConfig::default();
    let mut studio = DevStudio::new(config).unwrap();
    
    let config = sigma_dev_studio::ContainerConfig {
        name: "test-container".to_string(),
        image: "nginx:latest".to_string(),
        ports: vec!["80:80".to_string()],
        environment: vec![],
    };
    
    let container_id = studio.docker_manager.create_container(config);
    assert!(container_id.is_ok());
}

#[test]
fn test_environment_management() {
    let config = DevStudioConfig::default();
    let mut studio = DevStudio::new(config).unwrap();
    
    // Test environment activation
    let result = studio.environment_manager.activate_environment("rust");
    assert!(result.is_ok());
    
    assert_eq!(studio.environment_manager.get_active_environment(), "rust");
}

#[test]
fn test_ai_assistant() {
    let mut config = DevStudioConfig::default();
    config.enable_ai = true;
    
    let studio = DevStudio::new(config).unwrap();
    let ai = studio.ai_assistant();
    assert!(ai.is_some());
}

#[test]
fn test_build_manager() {
    let config = DevStudioConfig::default();
    let mut studio = DevStudio::new(config).unwrap();
    
    let build_config = sigma_dev_studio::BuildConfig {
        project: "test-project".to_string(),
        branch: "main".to_string(),
    };
    
    let build_id = studio.build_manager.start_build(build_config);
    assert!(build_id.is_ok());
}
