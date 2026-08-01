// SigmaOS Container Runtime
// OOP-based container management with Docker and Podman support

use crate::klib::HashMap;
use std::path::PathBuf;

/// Container configuration
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub name: String,
    pub image: String,
    pub command: Option<String>,
    pub env_vars: HashMap<String, String>,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMapping>,
    pub network_mode: NetworkMode,
    pub restart_policy: RestartPolicy,
    pub resource_limits: ResourceLimits,
}

/// Port mapping
#[derive(Debug, Clone)]
pub struct PortMapping {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: PortProtocol,
}

/// Port protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortProtocol {
    Tcp,
    Udp,
}

/// Volume mapping
#[derive(Debug, Clone)]
pub struct VolumeMapping {
    pub host_path: PathBuf,
    pub container_path: PathBuf,
    pub read_only: bool,
}

/// Network mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkMode {
    Bridge,
    Host,
    None,
    Container(String),
}

/// Restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
    UnlessStopped,
}

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub cpu_shares: u32,
    pub memory_mb: u64,
    pub memory_swap_mb: u64,
}

/// Container state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Restarting,
    Exited,
    Dead,
}

/// Container info
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub created_at: u64,
    pub started_at: Option<u64>,
}

/// Container stats
#[derive(Debug, Clone)]
pub struct ContainerStats {
    pub cpu_percent: f64,
    pub memory_mb: u64,
    pub memory_limit_mb: u64,
    pub memory_percent: f64,
    pub network_rx_mb: u64,
    pub network_tx_mb: u64,
    pub block_read_mb: u64,
    pub block_write_mb: u64,
}

/// OOP trait for container runtimes
pub trait ContainerRuntime {
    /// Create container
    fn create_container(&mut self, config: &ContainerConfig) -> Result<String, ContainerError>;
    /// Start container
    fn start_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Stop container
    fn stop_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Pause container
    fn pause_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Resume container
    fn resume_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Restart container
    fn restart_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Remove container
    fn remove_container(&mut self, container_id: &str) -> Result<(), ContainerError>;
    /// Get container info
    fn get_container_info(&self, container_id: &str) -> Result<ContainerInfo, ContainerError>;
    /// Get container stats
    fn get_container_stats(&self, container_id: &str) -> Result<ContainerStats, ContainerError>;
    /// List containers
    fn list_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError>;
    /// Pull image
    fn pull_image(&mut self, image: &str) -> Result<(), ContainerError>;
    /// Get runtime name
    fn name(&self) -> &str;
}

/// Docker runtime
pub struct DockerRuntime {
    containers: HashMap<String, ContainerConfig>,
    container_states: HashMap<String, ContainerState>,
}

impl DockerRuntime {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            container_states: HashMap::new(),
        }
    }
}

impl Default for DockerRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerRuntime for DockerRuntime {
    fn create_container(&mut self, config: &ContainerConfig) -> Result<String, ContainerError> {
        let container_id = format!("container_{}", self.containers.len());
        self.containers.insert(container_id.clone(), config.clone());
        self.container_states
            .insert(container_id.clone(), ContainerState::Created);
        Ok(container_id)
    }

    fn start_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn stop_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Exited);
        Ok(())
    }

    fn pause_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Paused);
        Ok(())
    }

    fn resume_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn restart_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Restarting);
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn remove_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.remove(container_id).is_some() {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states.remove(container_id);
        Ok(())
    }

    fn get_container_info(&self, container_id: &str) -> Result<ContainerInfo, ContainerError> {
        let config = self
            .containers
            .get(container_id)
            .ok_or_else(|| ContainerError::ContainerNotFound(container_id.to_string()))?;
        let state = self
            .container_states
            .get(container_id)
            .copied()
            .unwrap_or(ContainerState::Dead);

        Ok(ContainerInfo {
            id: container_id.to_string(),
            name: config.name.clone(),
            image: config.image.clone(),
            state,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: if state == ContainerState::Running {
                Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
            } else {
                None
            },
        })
    }

    fn get_container_stats(&self, container_id: &str) -> Result<ContainerStats, ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }

        Ok(ContainerStats {
            cpu_percent: 15.0,
            memory_mb: 512,
            memory_limit_mb: 1024,
            memory_percent: 50.0,
            network_rx_mb: 10,
            network_tx_mb: 5,
            block_read_mb: 20,
            block_write_mb: 10,
        })
    }

    fn list_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError> {
        let mut infos = Vec::new();
        for (id, config) in &self.containers {
            let state = self
                .container_states
                .get(id)
                .copied()
                .unwrap_or(ContainerState::Dead);
            infos.push(ContainerInfo {
                id: id.clone(),
                name: config.name.clone(),
                image: config.image.clone(),
                state,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                started_at: None,
            });
        }
        Ok(infos)
    }

    fn pull_image(&mut self, _image: &str) -> Result<(), ContainerError> {
        // Simulated image pull
        Ok(())
    }

    fn name(&self) -> &str {
        "Docker"
    }
}

/// Podman runtime
pub struct PodmanRuntime {
    containers: HashMap<String, ContainerConfig>,
    container_states: HashMap<String, ContainerState>,
}

impl PodmanRuntime {
    pub fn new() -> Self {
        Self {
            containers: HashMap::new(),
            container_states: HashMap::new(),
        }
    }
}

impl Default for PodmanRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerRuntime for PodmanRuntime {
    fn create_container(&mut self, config: &ContainerConfig) -> Result<String, ContainerError> {
        let container_id = format!("podman_{}", self.containers.len());
        self.containers.insert(container_id.clone(), config.clone());
        self.container_states
            .insert(container_id.clone(), ContainerState::Created);
        Ok(container_id)
    }

    fn start_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn stop_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Exited);
        Ok(())
    }

    fn pause_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Paused);
        Ok(())
    }

    fn resume_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn restart_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states
            .insert(container_id.to_string(), ContainerState::Restarting);
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.container_states
            .insert(container_id.to_string(), ContainerState::Running);
        Ok(())
    }

    fn remove_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        if !self.containers.remove(container_id).is_some() {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }
        self.container_states.remove(container_id);
        Ok(())
    }

    fn get_container_info(&self, container_id: &str) -> Result<ContainerInfo, ContainerError> {
        let config = self
            .containers
            .get(container_id)
            .ok_or_else(|| ContainerError::ContainerNotFound(container_id.to_string()))?;
        let state = self
            .container_states
            .get(container_id)
            .copied()
            .unwrap_or(ContainerState::Dead);

        Ok(ContainerInfo {
            id: container_id.to_string(),
            name: config.name.clone(),
            image: config.image.clone(),
            state,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            started_at: if state == ContainerState::Running {
                Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
            } else {
                None
            },
        })
    }

    fn get_container_stats(&self, container_id: &str) -> Result<ContainerStats, ContainerError> {
        if !self.containers.contains_key(container_id) {
            return Err(ContainerError::ContainerNotFound(container_id.to_string()));
        }

        Ok(ContainerStats {
            cpu_percent: 20.0,
            memory_mb: 768,
            memory_limit_mb: 2048,
            memory_percent: 37.5,
            network_rx_mb: 15,
            network_tx_mb: 8,
            block_read_mb: 25,
            block_write_mb: 12,
        })
    }

    fn list_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError> {
        let mut infos = Vec::new();
        for (id, config) in &self.containers {
            let state = self
                .container_states
                .get(id)
                .copied()
                .unwrap_or(ContainerState::Dead);
            infos.push(ContainerInfo {
                id: id.clone(),
                name: config.name.clone(),
                image: config.image.clone(),
                state,
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                started_at: None,
            });
        }
        Ok(infos)
    }

    fn pull_image(&mut self, _image: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn name(&self) -> &str {
        "Podman"
    }
}

/// OOP-based Container Runtime Manager
pub struct ContainerRuntimeManager {
    runtime: Box<dyn ContainerRuntime>,
    images: Vec<String>,
    pub events_log: Vec<String>,
}

impl ContainerRuntimeManager {
    pub fn new(runtime: Box<dyn ContainerRuntime>) -> Self {
        Self {
            runtime,
            images: Vec::new(),
            events_log: Vec::new(),
        }
    }

    /// Log a container lifecycle event
    pub fn log_event(&mut self, message: String) {
        self.events_log.push(message);
    }

    /// Perform a simulated container health check
    pub fn health_check(&self, container_id: &str) -> Result<bool, ContainerError> {
        let info = self.get_container_info(container_id)?;
        Ok(info.state == ContainerState::Running)
    }

    /// Prune unused images from the cache
    pub fn prune_images(&mut self) -> usize {
        let count = self.images.len();
        self.images.clear();
        count
    }

    /// Create container
    pub fn create_container(&mut self, config: ContainerConfig) -> Result<String, ContainerError> {
        let container_id = self.runtime.create_container(&config)?;
        self.log_event(format!(
            "[Event] Container '{}' successfully created",
            container_id
        ));
        Ok(container_id)
    }

    /// Start container
    pub fn start_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.start_container(container_id)?;
        self.log_event(format!(
            "[Event] Container '{}' successfully started",
            container_id
        ));
        Ok(())
    }

    /// Stop container
    pub fn stop_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.stop_container(container_id)?;
        self.log_event(format!(
            "[Event] Container '{}' successfully stopped",
            container_id
        ));
        Ok(())
    }

    /// Pause container
    pub fn pause_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.pause_container(container_id)?;
        self.log_event(format!("[Event] Container '{}' paused", container_id));
        Ok(())
    }

    /// Resume container
    pub fn resume_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.resume_container(container_id)?;
        self.log_event(format!("[Event] Container '{}' resumed", container_id));
        Ok(())
    }

    /// Restart container
    pub fn restart_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.restart_container(container_id)?;
        self.log_event(format!("[Event] Container '{}' restarted", container_id));
        Ok(())
    }

    /// Remove container
    pub fn remove_container(&mut self, container_id: &str) -> Result<(), ContainerError> {
        self.runtime.remove_container(container_id)?;
        self.log_event(format!("[Event] Container '{}' removed", container_id));
        Ok(())
    }

    /// Get container info
    pub fn get_container_info(&self, container_id: &str) -> Result<ContainerInfo, ContainerError> {
        self.runtime.get_container_info(container_id)
    }

    /// Get container stats
    pub fn get_container_stats(
        &self,
        container_id: &str,
    ) -> Result<ContainerStats, ContainerError> {
        self.runtime.get_container_stats(container_id)
    }

    /// List containers
    pub fn list_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError> {
        self.runtime.list_containers()
    }

    /// Pull image
    pub fn pull_image(&mut self, image: &str) -> Result<(), ContainerError> {
        self.runtime.pull_image(image)?;
        if !self.images.contains(&image.to_string()) {
            self.images.push(image.to_string());
        }
        Ok(())
    }

    /// List images
    pub fn list_images(&self) -> &[String] {
        &self.images
    }

    /// Get runtime name
    pub fn runtime_name(&self) -> &str {
        self.runtime.name()
    }

    /// Get running containers
    pub fn running_containers(&self) -> Result<Vec<ContainerInfo>, ContainerError> {
        let all = self.runtime.list_containers()?;
        Ok(all
            .into_iter()
            .filter(|c| c.state == ContainerState::Running)
            .collect())
    }
}

impl Default for ContainerRuntimeManager {
    fn default() -> Self {
        Self::new(Box::new(DockerRuntime::new()))
    }
}

/// Container errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerError {
    ContainerNotFound(String),
    ImageNotFound(String),
    CreateFailed(String),
    StartFailed(String),
    StopFailed(String),
    PauseFailed(String),
    ResumeFailed(String),
    RestartFailed(String),
    RemoveFailed(String),
    PullFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_config() {
        let config = ContainerConfig {
            name: "Test Container".to_string(),
            image: "nginx:latest".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::UnlessStopped,
            resource_limits: ResourceLimits {
                cpu_shares: 1024,
                memory_mb: 512,
                memory_swap_mb: 1024,
            },
        };
        assert_eq!(config.name, "Test Container");
    }

    #[test]
    fn test_docker_runtime() {
        let runtime = DockerRuntime::new();
        assert_eq!(runtime.name(), "Docker");
    }

    #[test]
    fn test_podman_runtime() {
        let runtime = PodmanRuntime::new();
        assert_eq!(runtime.name(), "Podman");
    }

    #[test]
    fn test_container_runtime_manager() {
        let manager = ContainerRuntimeManager::default();
        assert_eq!(manager.runtime_name(), "Docker");
    }

    #[test]
    fn test_create_container() {
        let mut manager = ContainerRuntimeManager::default();
        let config = ContainerConfig {
            name: "Test Container".to_string(),
            image: "nginx:latest".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::UnlessStopped,
            resource_limits: ResourceLimits {
                cpu_shares: 1024,
                memory_mb: 512,
                memory_swap_mb: 1024,
            },
        };
        let container_id = manager.create_container(config).unwrap();
        assert!(!container_id.is_empty());
    }

    #[test]
    fn test_start_container() {
        let mut manager = ContainerRuntimeManager::default();
        let config = ContainerConfig {
            name: "Test Container".to_string(),
            image: "nginx:latest".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::UnlessStopped,
            resource_limits: ResourceLimits {
                cpu_shares: 1024,
                memory_mb: 512,
                memory_swap_mb: 1024,
            },
        };
        let container_id = manager.create_container(config).unwrap();
        manager.start_container(&container_id).unwrap();
        let info = manager.get_container_info(&container_id).unwrap();
        assert_eq!(info.state, ContainerState::Running);
    }

    #[test]
    fn test_container_health_check_and_events_and_pruning() {
        let mut manager = ContainerRuntimeManager::default();
        let config = ContainerConfig {
            name: "Audit Container".to_string(),
            image: "redis:alpine".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::Always,
            resource_limits: ResourceLimits {
                cpu_shares: 512,
                memory_mb: 256,
                memory_swap_mb: 512,
            },
        };

        // Create & verify event logged
        let container_id = manager.create_container(config).unwrap();
        assert!(manager
            .events_log
            .iter()
            .any(|e| e.contains("successfully created")));

        // Start & verify health check and start event
        manager.start_container(&container_id).unwrap();
        assert!(manager.health_check(&container_id).unwrap());
        assert!(manager
            .events_log
            .iter()
            .any(|e| e.contains("successfully started")));

        // Pull and prune image verification
        manager.pull_image("redis:alpine").unwrap();
        assert_eq!(manager.list_images().len(), 1);
        let pruned = manager.prune_images();
        assert_eq!(pruned, 1);
        assert_eq!(manager.list_images().len(), 0);
    }
}
