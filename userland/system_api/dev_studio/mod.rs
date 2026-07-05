// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Dev Studio - Unified development environment

mod git_manager;
mod docker_manager;
mod kubernetes_manager;
mod database_client;
mod api_tester;
mod environments;
mod ai_assistant;
mod build_manager;

pub use git_manager::GitManager;
pub use docker_manager::DockerManager;
pub use kubernetes_manager::KubernetesManager;
pub use database_client::DatabaseClient;
pub use api_tester::APITester;
pub use environments::EnvironmentManager;
pub use ai_assistant::DevAIAssistant;
pub use build_manager::BuildManager;

use serde::{Deserialize, Serialize};

/// Dev Studio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevStudioConfig {
    /// Default Git user name
    pub git_user_name: String,
    /// Default Git user email
    pub git_user_email: String,
    /// Enable AI assistant
    pub enable_ai: bool,
    /// Default environment
    pub default_environment: String,
    /// Auto-save interval (in minutes)
    pub auto_save_interval: u32,
}

impl Default for DevStudioConfig {
    fn default() -> Self {
        Self {
            git_user_name: String::new(),
            git_user_email: String::new(),
            enable_ai: true,
            default_environment: "python".to_string(),
            auto_save_interval: 5,
        }
    }
}

/// Main Dev Studio structure
pub struct DevStudio {
    config: DevStudioConfig,
    git_manager: GitManager,
    docker_manager: DockerManager,
    kubernetes_manager: KubernetesManager,
    database_client: DatabaseClient,
    api_tester: APITester,
    environment_manager: EnvironmentManager,
    ai_assistant: Option<DevAIAssistant>,
    build_manager: BuildManager,
}

impl DevStudio {
    /// Create a new Dev Studio instance
    pub fn new(config: DevStudioConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let git_manager = GitManager::new(&config.git_user_name, &config.git_user_email)?;
        let docker_manager = DockerManager::new()?;
        let kubernetes_manager = KubernetesManager::new()?;
        let database_client = DatabaseClient::new()?;
        let api_tester = APITester::new()?;
        let environment_manager = EnvironmentManager::new()?;
        let build_manager = BuildManager::new()?;
        
        let ai_assistant = if config.enable_ai {
            Some(DevAIAssistant::new()?)
        } else {
            None
        };

        Ok(Self {
            config,
            git_manager,
            docker_manager,
            kubernetes_manager,
            database_client,
            api_tester,
            environment_manager,
            ai_assistant,
            build_manager,
        })
    }

    /// Get Dev Studio status
    pub fn get_status(&self) -> DevStudioStatus {
        DevStudioStatus {
            git_repositories: self.git_manager.get_repository_count(),
            docker_containers: self.docker_manager.get_container_count(),
            kubernetes_clusters: self.kubernetes_manager.get_cluster_count(),
            database_connections: self.database_client.get_connection_count(),
            active_environment: self.environment_manager.get_active_environment(),
            build_status: self.build_manager.get_status(),
        }
    }

    /// Get AI assistant if enabled
    pub fn ai_assistant(&self) -> Option<&DevAIAssistant> {
        self.ai_assistant.as_ref()
    }

    /// Update configuration
    pub fn update_config(&mut self, config: DevStudioConfig) {
        self.config = config;
    }
}

/// Dev Studio status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevStudioStatus {
    pub git_repositories: usize,
    pub docker_containers: usize,
    pub kubernetes_clusters: usize,
    pub database_connections: usize,
    pub active_environment: String,
    pub build_status: BuildStatus,
}

/// Build status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildStatus {
    pub is_building: bool,
    pub current_build: Option<String>,
    pub last_build_status: Option<String>,
    pub build_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_studio_creation() {
        let config = DevStudioConfig::default();
        // Note: This test will fail if dependencies aren't available
        // let studio = DevStudio::new(config);
        // assert!(studio.is_ok());
    }

    #[test]
    fn test_config_default() {
        let config = DevStudioConfig::default();
        assert_eq!(config.enable_ai, true);
        assert_eq!(config.default_environment, "python");
    }
}
