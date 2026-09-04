use std::vec;
use std::boxed::Box;
use std::format;
// SigmaOS Container Runtime
// OOP-based container management with Docker and Podman support
// Incorporating FreeBSD Jails (jail networking & IPC sandboxing) and Podman (rootless user namespaces) compatibility

use std::string::{String, ToString};
use std::vec::Vec;
use crate::klib::collections::HashMap;

/// FreeBSD VNET virtual network interface pair configuration (FreeBSD VNET/jails parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VnetEpairConfig {
    pub epair_id: u32,
    pub host_interface: String,    // e.g. epair0a
    pub container_interface: String, // e.g. epair0b
    pub bridge_name: String,       // e.g. bridge0
    pub ipv4_address: String,
    pub mac_address: [u8; 6],
}

impl VnetEpairConfig {
    pub fn new(epair_id: u32, bridge_name: &str, ipv4_address: &str) -> Self {
        Self {
            epair_id,
            host_interface: format!("epair{}a", epair_id),
            container_interface: format!("epair{}b", epair_id),
            bridge_name: bridge_name.to_string(),
            ipv4_address: ipv4_address.to_string(),
            mac_address: [0x02, 0x00, 0x00, 0x42, (epair_id >> 8) as u8, epair_id as u8],
        }
    }
}

/// Linux Unified Cgroups v2 Resource Controller
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CgroupV2Controller {
    pub cgroup_path: String,
    pub cpu_weight: u32,         // 1-10000 (default 100)
    pub memory_high_bytes: u64,  // Throttle limit
    pub memory_max_bytes: u64,   // OOM killer limit
    pub io_max_rbps: u64,        // Max read bytes per second
    pub io_max_wbps: u64,        // Max write bytes per second
    pub pids_max: u32,           // Maximum process count
}

impl CgroupV2Controller {
    pub fn new(cgroup_path: &str) -> Self {
        Self {
            cgroup_path: cgroup_path.to_string(),
            cpu_weight: 100,
            memory_high_bytes: u64::MAX,
            memory_max_bytes: u64::MAX,
            io_max_rbps: u64::MAX,
            io_max_wbps: u64::MAX,
            pids_max: 4096,
        }
    }
}

/// FreeBSD Jail-inspired security & network sandboxing configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignJailConfig {
    pub allow_raw_sockets: bool,
    pub sysv_ipc_isolated: bool,
    pub ip_address_bindings: Vec<String>,
    pub vnet_epair: Option<VnetEpairConfig>,
}

impl SovereignJailConfig {
    pub fn new() -> Self {
        Self {
            allow_raw_sockets: false,
            sysv_ipc_isolated: true,
            ip_address_bindings: Vec::new(),
            vnet_epair: None,
        }
    }
}

impl Default for SovereignJailConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Podman-inspired SubUID/SubGID Mapping entry for rootless container execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RootlessUidMap {
    pub container_id: u32,
    pub host_id: u32,
    pub count: u32,
}

impl RootlessUidMap {
    pub fn new(container_id: u32, host_id: u32, count: u32) -> Self {
        Self {
            container_id,
            host_id,
            count,
        }
    }

    pub fn translate_container_to_host(&self, container_uid: u32) -> Option<u32> {
        if container_uid >= self.container_id && container_uid < self.container_id + self.count {
            Some(self.host_id + (container_uid - self.container_id))
        } else {
            None
        }
    }
}

/// Docker/Podman style Event Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerEventType {
    Create,
    Start,
    Stop,
    Pause,
    Unpause,
    Restart,
    Die,
    HealthStatusChange(HealthStatus),
}

/// Event structure for Docker/Podman event stream
#[derive(Debug, Clone)]
pub struct ContainerEvent {
    pub timestamp_secs: u64,
    pub container_id: String,
    pub event_type: ContainerEventType,
    pub attributes: HashMap<String, String>,
}

/// Live Container Event Bus
pub struct ContainerEventBus {
    pub events: Vec<ContainerEvent>,
}

impl ContainerEventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn emit(&mut self, container_id: &str, event_type: ContainerEventType) {
        let event = ContainerEvent {
            timestamp_secs: 10000,
            container_id: container_id.to_string(),
            event_type,
            attributes: HashMap::new(),
        };
        self.events.push(event);
    }

    pub fn filter_by_container(&self, container_id: &str) -> Vec<ContainerEvent> {
        self.events
            .iter()
            .filter(|e| e.container_id == container_id)
            .cloned()
            .collect()
    }
}

impl Default for ContainerEventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// OCI Image Layer representing a diff tarball / filesystem layer
#[derive(Debug, Clone)]
pub struct ImageLayer {
    pub layer_id: String,
    pub parent_id: Option<String>,
    pub size_bytes: u64,
    pub media_type: String,
}

/// OCI Image Manifest combining multiple layers
#[derive(Debug, Clone)]
pub struct ImageManifest {
    pub schema_version: u32,
    pub media_type: String,
    pub config_digest: String,
    pub layers: Vec<ImageLayer>,
}

/// Copy-on-Write OverlayFS Storage Driver for OCI container images
pub struct OverlayFsStorageDriver {
    pub base_dir: String,
    pub layers: HashMap<String, ImageLayer>,
    pub manifests: HashMap<String, ImageManifest>,
}

impl OverlayFsStorageDriver {
    pub fn new(base_dir: String) -> Self {
        Self {
            base_dir,
            layers: HashMap::new(),
            manifests: HashMap::new(),
        }
    }

    pub fn register_layer(&mut self, layer: ImageLayer) {
        self.layers.insert(layer.layer_id.clone(), layer);
    }

    pub fn register_manifest(&mut self, image_ref: &str, manifest: ImageManifest) {
        self.manifests.insert(image_ref.to_string(), manifest);
    }

    pub fn calculate_image_size(&self, image_ref: &str) -> u64 {
        if let Some(manifest) = self.manifests.get(image_ref) {
            manifest.layers.iter().map(|l| l.size_bytes).sum()
        } else {
            0
        }
    }

    pub fn prepare_rw_overlay_dir(&self, container_id: &str) -> String {
        format!("{}/containers/{}/diff", self.base_dir, container_id)
    }
}

/// Container health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Starting,
    Healthy,
    Unhealthy,
    None,
}

/// Docker / Podman style Healthcheck Probe Configuration
#[derive(Debug, Clone)]
pub struct HealthcheckConfig {
    pub test_cmd: Vec<String>,
    pub interval_secs: u64,
    pub timeout_secs: u64,
    pub retries: u32,
    pub start_period_secs: u64,
}

impl HealthcheckConfig {
    pub fn new(cmd: &[&str]) -> Self {
        Self {
            test_cmd: cmd.iter().map(|s| s.to_string()).collect(),
            interval_secs: 30,
            timeout_secs: 30,
            retries: 3,
            start_period_secs: 0,
        }
    }
}

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
    // Podman-inspired rootless user namespace flag
    pub is_rootless: bool,
    // FreeBSD Jail-inspired network & capability configuration
    pub jail_config: Option<SovereignJailConfig>,
    // Docker / Podman healthcheck probe config
    pub healthcheck: Option<HealthcheckConfig>,
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
    pub host_path: String,
    pub container_path: String,
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
    pub cgroup_v2: Option<CgroupV2Controller>,
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
        std::thread::sleep(core::time::Duration::from_millis(100));
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
            created_at: 1700000000u64,
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
                created_at: 1700000000u64,
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
        std::thread::sleep(core::time::Duration::from_millis(100));
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
            created_at: 1700000000u64,
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
                created_at: 1700000000u64,
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

/// Podman-inspired Pod status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodState {
    Created,
    Running,
    Degraded,
    Stopped,
    Exited,
}

/// Podman-inspired Sovereign Pod holding multiple containers sharing network, IPC, and volume namespaces
#[derive(Debug, Clone)]
pub struct SovereignPod {
    pub id: String,
    pub name: String,
    pub state: PodState,
    pub infra_container_id: String,
    pub container_ids: Vec<String>,
    pub shared_network_mode: NetworkMode,
    pub shared_volumes: Vec<VolumeMapping>,
}

impl SovereignPod {
    pub fn new(id: String, name: String, infra_container_id: String, shared_network_mode: NetworkMode) -> Self {
        Self {
            id,
            name,
            state: PodState::Created,
            infra_container_id,
            container_ids: Vec::new(),
            shared_network_mode,
            shared_volumes: Vec::new(),
        }
    }

    pub fn add_container(&mut self, container_id: String) {
        if !self.container_ids.contains(&container_id) {
            self.container_ids.push(container_id);
        }
    }

    pub fn remove_container(&mut self, container_id: &str) -> bool {
        if let Some(pos) = self.container_ids.iter().position(|id| id == container_id) {
            self.container_ids.remove(pos);
            true
        } else {
            false
        }
    }
}

/// Manager for Podman-style pods
pub struct PodManager {
    pods: HashMap<String, SovereignPod>,
}

impl PodManager {
    pub fn new() -> Self {
        Self {
            pods: HashMap::new(),
        }
    }

    pub fn create_pod(&mut self, name: &str, shared_net: NetworkMode) -> Result<String, ContainerError> {
        let pod_id = format!("pod_{}", self.pods.len() + 1);
        let infra_id = format!("infra_{}", pod_id);
        let pod = SovereignPod::new(pod_id.clone(), name.to_string(), infra_id, shared_net);
        self.pods.insert(pod_id.clone(), pod);
        Ok(pod_id)
    }

    pub fn get_pod(&self, pod_id: &str) -> Result<&SovereignPod, ContainerError> {
        self.pods
            .get(pod_id)
            .ok_or_else(|| ContainerError::ContainerNotFound(format!("Pod '{}' not found", pod_id)))
    }

    pub fn get_pod_mut(&mut self, pod_id: &str) -> Result<&mut SovereignPod, ContainerError> {
        self.pods
            .get_mut(pod_id)
            .ok_or_else(|| ContainerError::ContainerNotFound(format!("Pod '{}' not found", pod_id)))
    }

    pub fn add_container_to_pod(&mut self, pod_id: &str, container_id: String) -> Result<(), ContainerError> {
        let pod = self.get_pod_mut(pod_id)?;
        pod.add_container(container_id);
        Ok(())
    }

    pub fn list_pods(&self) -> Vec<SovereignPod> {
        self.pods.values().cloned().collect()
    }

    pub fn remove_pod(&mut self, pod_id: &str) -> Result<(), ContainerError> {
        if self.pods.remove(pod_id).is_some() {
            Ok(())
        } else {
            Err(ContainerError::ContainerNotFound(format!("Pod '{}' not found", pod_id)))
        }
    }
}

impl Default for PodManager {
    fn default() -> Self {
        Self::new()
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
                cgroup_v2: None,
            },
            is_rootless: false,
            jail_config: None,
            healthcheck: None,
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
                cgroup_v2: None,
            },
            is_rootless: false,
            jail_config: None,
            healthcheck: None,
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
                cgroup_v2: None,
            },
            is_rootless: false,
            jail_config: None,
            healthcheck: None,
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
                cgroup_v2: None,
            },
            is_rootless: false,
            jail_config: None,
            healthcheck: None,
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

    #[test]
    fn test_vnet_epair_and_cgroups_v2() {
        let epair = VnetEpairConfig::new(0, "bridge0", "192.168.1.50/24");
        assert_eq!(epair.host_interface, "epair0a");
        assert_eq!(epair.container_interface, "epair0b");
        assert_eq!(epair.bridge_name, "bridge0");

        let cgroup = CgroupV2Controller::new("/sys/fs/cgroup/container_app");
        assert_eq!(cgroup.cpu_weight, 100);
        assert_eq!(cgroup.pids_max, 4096);
    }

    #[test]
    fn test_freebsd_jails_and_podman_rootless() {
        let jail_cfg = SovereignJailConfig {
            allow_raw_sockets: true,
            sysv_ipc_isolated: false,
            ip_address_bindings: vec!["192.168.1.100".to_string()],
            vnet_epair: None,
        };

        let config = ContainerConfig {
            name: "FreeBSD-Jail-Container".to_string(),
            image: "alpine:latest".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::No,
            resource_limits: ResourceLimits {
                cpu_shares: 512,
                memory_mb: 256,
                memory_swap_mb: 512,
                cgroup_v2: None,
            },
            is_rootless: true,
            jail_config: Some(jail_cfg.clone()),
            healthcheck: None,
        };

        assert!(config.is_rootless);
        let jail = config.jail_config.unwrap();
        assert!(jail.allow_raw_sockets);
        assert!(!jail.sysv_ipc_isolated);
        assert_eq!(jail.ip_address_bindings[0], "192.168.1.100");
    }

    #[test]
    fn test_sovereign_pod_lifecycle() {
        let mut pod_mgr = PodManager::new();
        let pod_id = pod_mgr
            .create_pod("web_app_pod", NetworkMode::Bridge)
            .unwrap();

        assert_eq!(pod_mgr.list_pods().len(), 1);

        pod_mgr
            .add_container_to_pod(&pod_id, "container_frontend".to_string())
            .unwrap();
        pod_mgr
            .add_container_to_pod(&pod_id, "container_backend".to_string())
            .unwrap();

        let pod = pod_mgr.get_pod(&pod_id).unwrap();
        assert_eq!(pod.container_ids.len(), 2);
        assert_eq!(pod.container_ids[0], "container_frontend");

        let mut bus = ContainerEventBus::new();
        bus.emit(&pod_id, ContainerEventType::Create);
        bus.emit(&pod_id, ContainerEventType::Start);

        let events = bus.filter_by_container(&pod_id);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, ContainerEventType::Create);

        assert!(pod_mgr.remove_pod(&pod_id).is_ok());
        assert_eq!(pod_mgr.list_pods().len(), 0);
    }

    #[test]
    fn test_container_healthcheck_probes() {
        let hc = HealthcheckConfig::new(&["CMD-SHELL", "curl -f https://localhost/health || exit 1"]);
        assert_eq!(hc.interval_secs, 30);
        assert_eq!(hc.retries, 3);

        let config = ContainerConfig {
            name: "ProbedContainer".to_string(),
            image: "nginx:latest".to_string(),
            command: None,
            env_vars: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::Always,
            resource_limits: ResourceLimits {
                cpu_shares: 1024,
                memory_mb: 512,
                memory_swap_mb: 1024,
                cgroup_v2: None,
            },
            is_rootless: true,
            jail_config: None,
            healthcheck: Some(hc),
        };

        assert!(config.healthcheck.is_some());
        let probe = config.healthcheck.unwrap();
        assert_eq!(probe.test_cmd[1], "curl -f https://localhost/health || exit 1");
    }

    #[test]
    fn test_overlayfs_and_rootless_subuid_translation() {
        let mut overlay = OverlayFsStorageDriver::new(String::from("/var/lib/sigmaos/overlay2"));

        let layer1 = ImageLayer {
            layer_id: "sha256:layer100".to_string(),
            parent_id: None,
            size_bytes: 10485760, // 10MB
            media_type: "application/vnd.oci.image.layer.v1.tar+gzip".to_string(),
        };
        let layer2 = ImageLayer {
            layer_id: "sha256:layer200".to_string(),
            parent_id: Some("sha256:layer100".to_string()),
            size_bytes: 5242880, // 5MB
            media_type: "application/vnd.oci.image.layer.v1.tar+gzip".to_string(),
        };

        overlay.register_layer(layer1.clone());
        overlay.register_layer(layer2.clone());

        let manifest = ImageManifest {
            schema_version: 2,
            media_type: "application/vnd.oci.image.manifest.v1+json".to_string(),
            config_digest: "sha256:config123".to_string(),
            layers: vec![layer1, layer2],
        };

        overlay.register_manifest("ubuntu:latest", manifest);
        assert_eq!(overlay.calculate_image_size("ubuntu:latest"), 15728640);

        let diff_dir = overlay.prepare_rw_overlay_dir("ctr_123");
        assert_eq!(diff_dir, String::from("/var/lib/sigmaos/overlay2/containers/ctr_123/diff"));

        let subuid_map = RootlessUidMap::new(0, 100000, 65536);
        assert_eq!(subuid_map.translate_container_to_host(0), Some(100000));
        assert_eq!(subuid_map.translate_container_to_host(1000), Some(101000));
        assert_eq!(subuid_map.translate_container_to_host(70000), None);
    }
}
