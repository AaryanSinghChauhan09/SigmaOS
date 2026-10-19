//! SigmaContainer - Container Management System
//!
//! Provides comprehensive container management inspired by Docker, Podman, and LXC.
//! Supports container lifecycle, resource limits, networking, and storage management.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Container state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
    Restarting,
    Removing,
    Exited,
    Dead,
    Unknown,
;

/// Container networking mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkMode {
    Bridge,
    Host,
    None,
    Container,
    Unknown,
;

/// Container restart policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    OnFailure,
    Always,
    UnlessStopped,
    Unknown,
;

/// Resource limits
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub cpu_shares: Option<u32>,
    pub memory_limit: Option<u64>,
    pub memory_reservation: Option<u64>,
    pub memory_swap: Option<u64>,
    pub pids_limit: Option<u64>,
;

impl ResourceLimits {
    pub fn new() -> Self {
        Self {
            cpu_shares: None,
            memory_limit: None,
            memory_reservation: None,
            memory_swap: None,
            pids_limit: None,
        }
    }

    pub fn with_cpu_shares(mut self, shares: u32) -> Self {
        self.cpu_shares = Some(shares);
        self
    }

    pub fn with_memory_limit(mut self, limit: u64) -> Self {
        self.memory_limit = Some(limit);
        self
    }

    pub fn with_memory_reservation(mut self, reservation: u64) -> Self {
        self.memory_reservation = Some(reservation);
        self
    }

    pub fn with_pids_limit(mut self, limit: u64) -> Self {
        self.pids_limit = Some(limit);
        self
    }
;

impl Default for ResourceLimits {
    fn default() -> Self {
        Self::new()
    }
;

/// Container
#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub network_mode: NetworkMode,
    pub restart_policy: RestartPolicy,
    pub resource_limits: ResourceLimits,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub exited_at: Option<u64>,
    pub exit_code: Option<i32>,
    pub command: String,
    pub environment: Vec<String>,
    pub volumes: Vec<String>,
    pub ports: Vec<String>,
;

impl Container {
    pub fn new(id: String, name: String, image: String) -> Self {
        Self {
            id,
            name,
            image,
            state: ContainerState::Created,
            network_mode: NetworkMode::Bridge,
            restart_policy: RestartPolicy::No,
            resource_limits: ResourceLimits::new(),
            created_at: 0,
            started_at: None,
            exited_at: None,
            exit_code: None,
            command: String::new(),
            environment: Vec::new(),
            volumes: Vec::new(),
            ports: Vec::new(),
        }
    }

    pub fn with_network_mode(mut self, mode: NetworkMode) -> Self {
        self.network_mode = mode;
        self
    }

    pub fn with_restart_policy(mut self, policy: RestartPolicy) -> Self {
        self.restart_policy = policy;
        self
    }

    pub fn with_resource_limits(mut self, limits: ResourceLimits) -> Self {
        self.resource_limits = limits;
        self
    }

    pub fn with_command(mut self, command: String) -> Self {
        self.command = command;
        self
    }

    pub fn with_environment(mut self, env: Vec<String>) -> Self {
        self.environment = env;
        self
    }

    pub fn with_volumes(mut self, volumes: Vec<String>) -> Self {
        self.volumes = volumes;
        self
    }

    pub fn with_ports(mut self, ports: Vec<String>) -> Self {
        self.ports = ports;
        self
    }

    pub fn start(&mut self) {
        self.state = ContainerState::Running;
        self.started_at = Some(0);
    }

    pub fn stop(&mut self, exit_code: i32) {
        self.state = ContainerState::Stopped;
        self.exited_at = Some(0);
        self.exit_code = Some(exit_code);
    }

    pub fn pause(&mut self) {
        self.state = ContainerState::Paused;
    }

    pub fn unpause(&mut self) {
        self.state = ContainerState::Running;
    }

    pub fn restart(&mut self) {
        self.state = ContainerState::Restarting;
    }

    pub fn remove(&mut self) {
        self.state = ContainerState::Removing;
    }
;

/// Container manager
pub struct SigmaContainer {
    containers: BTreeMap<String, Container>,
    next_container_id: u64,
;

impl SigmaContainer {
    pub fn new() -> Self {
        Self {
            containers: BTreeMap::new(),
            next_container_id: 1,
        }
    }

    pub fn create_container(&mut self, name: String, image: String) -> String {
        let id = format!("sigma-{}", self.next_container_id);
        self.next_container_id += 1;
        let container = Container::new(id.clone(), name, image);
        self.containers.insert(id.clone(), container);
        id
    }

    pub fn get_container(&self, id: &str) -> Option<&Container> {
        self.containers.get(id)
    }

    pub fn get_container_mut(&mut self, id: &str) -> Option<&mut Container> {
        self.containers.get_mut(id)
    }

    pub fn list_containers(&self) -> Vec<&Container> {
        self.containers.values().collect()
    }

    pub fn list_containers_by_state(&self, state: ContainerState) -> Vec<&Container> {
        self.containers
            .values()
            .filter(|c| c.state == state)
            .collect()
    }

    pub fn start_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.start();
            true
        } else {
            false
        }
    }

    pub fn stop_container(&mut self, id: &str, exit_code: i32) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.stop(exit_code);
            true
        } else {
            false
        }
    }

    pub fn pause_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.pause();
            true
        } else {
            false
        }
    }

    pub fn unpause_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.unpause();
            true
        } else {
            false
        }
    }

    pub fn restart_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.restart();
            true
        } else {
            false
        }
    }

    pub fn remove_container(&mut self, id: &str) -> bool {
        if let Some(container) = self.containers.get_mut(id) {
            container.remove();
            self.containers.remove(id).is_some()
        } else {
            false
        }
    }

    pub fn container_count(&self) -> usize {
        self.containers.len()
    }

    pub fn running_container_count(&self) -> usize {
        self.containers
            .values()
            .filter(|c| c.state == ContainerState::Running)
            .count()
    }
;

impl Default for SigmaContainer {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMACONTAINER_INSTANCE: Option<SigmaContainer> = None;

pub fn init_sigmacontainer() {
    unsafe {
        SIGMACONTAINER_INSTANCE = Some(SigmaContainer::new());
    }
;

pub fn get_sigmacontainer() -> &'static SigmaContainer {
    unsafe {
        SIGMACONTAINER_INSTANCE
            .as_ref()
            .expect("SigmaContainer not initialized")
    }
;

pub fn get_sigmacontainer_mut() -> &'static mut SigmaContainer {
    unsafe {
        SIGMACONTAINER_INSTANCE
            .as_mut()
            .expect("SigmaContainer not initialized")
    }
;

pub fn create_container(name: String, image: String) -> String {
    let manager = get_sigmacontainer_mut();
    manager.create_container(name, image)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let container = Container::new(
            "test-id".to_string(),
            "test-container".to_string(),
            "ubuntu:latest".to_string(),
        );
        assert_eq!(container.id, "test-id");
        assert_eq!(container.name, "test-container");
        assert_eq!(container.image, "ubuntu:latest");
        assert_eq!(container.state, ContainerState::Created);
    }

    #[test]
    fn test_container_builder() {
        let container = Container::new(
            "test-id".to_string(),
            "test-container".to_string(),
            "ubuntu:latest".to_string(),
        )
        .with_network_mode(NetworkMode::Host)
        .with_restart_policy(RestartPolicy::Always)
        .with_command("/bin/bash".to_string())
        .with_environment(vec!["PATH=/usr/bin".to_string()])
        .with_volumes(vec!["/host:/container".to_string()])
        .with_ports(vec!["80:8080".to_string()]);

        assert_eq!(container.network_mode, NetworkMode::Host);
        assert_eq!(container.restart_policy, RestartPolicy::Always);
        assert_eq!(container.command, "/bin/bash");
        assert_eq!(container.environment.len(), 1);
        assert_eq!(container.volumes.len(), 1);
        assert_eq!(container.ports.len(), 1);
    }

    #[test]
    fn test_container_state_transitions() {
        let mut container = Container::new(
            "test-id".to_string(),
            "test-container".to_string(),
            "ubuntu:latest".to_string(),
        );
        assert_eq!(container.state, ContainerState::Created);

        container.start();
        assert_eq!(container.state, ContainerState::Running);
        assert!(container.started_at.is_some());

        container.pause();
        assert_eq!(container.state, ContainerState::Paused);

        container.unpause();
        assert_eq!(container.state, ContainerState::Running);

        container.stop(0);
        assert_eq!(container.state, ContainerState::Stopped);
        assert_eq!(container.exit_code, Some(0));
    }

    #[test]
    fn test_resource_limits() {
        let limits = ResourceLimits::new()
            .with_cpu_shares(1024)
            .with_memory_limit(1024 * 1024 * 1024)
            .with_memory_reservation(512 * 1024 * 1024)
            .with_pids_limit(100);

        assert_eq!(limits.cpu_shares, Some(1024));
        assert_eq!(limits.memory_limit, Some(1024 * 1024 * 1024));
        assert_eq!(limits.memory_reservation, Some(512 * 1024 * 1024));
        assert_eq!(limits.pids_limit, Some(100));
    }

    #[test]
    fn test_container_manager() {
        let manager = SigmaContainer::new();
        assert_eq!(manager.container_count(), 0);
    }

    #[test]
    fn test_create_container() {
        let mut manager = SigmaContainer::new();
        let id = manager.create_container("test".to_string(), "ubuntu:latest".to_string());
        assert_eq!(id, "sigma-1");
        assert_eq!(manager.container_count(), 1);
    }

    #[test]
    fn test_start_stop_container() {
        let mut manager = SigmaContainer::new();
        let id = manager.create_container("test".to_string(), "ubuntu:latest".to_string());

        assert!(manager.start_container(&id));
        assert_eq!(
            manager.get_container(&id).unwrap().state,
            ContainerState::Running
        );

        assert!(manager.stop_container(&id, 0));
        assert_eq!(
            manager.get_container(&id).unwrap().state,
            ContainerState::Stopped
        );
    }

    #[test]
    fn test_pause_unpause_container() {
        let mut manager = SigmaContainer::new();
        let id = manager.create_container("test".to_string(), "ubuntu:latest".to_string());
        manager.start_container(&id);

        assert!(manager.pause_container(&id));
        assert_eq!(
            manager.get_container(&id).unwrap().state,
            ContainerState::Paused
        );

        assert!(manager.unpause_container(&id));
        assert_eq!(
            manager.get_container(&id).unwrap().state,
            ContainerState::Running
        );
    }

    #[test]
    fn test_restart_container() {
        let mut manager = SigmaContainer::new();
        let id = manager.create_container("test".to_string(), "ubuntu:latest".to_string());

        assert!(manager.restart_container(&id));
        assert_eq!(
            manager.get_container(&id).unwrap().state,
            ContainerState::Restarting
        );
    }

    #[test]
    fn test_remove_container() {
        let mut manager = SigmaContainer::new();
        let id = manager.create_container("test".to_string(), "ubuntu:latest".to_string());

        assert!(manager.remove_container(&id));
        assert_eq!(manager.container_count(), 0);
    }

    #[test]
    fn test_list_containers_by_state() {
        let mut manager = SigmaContainer::new();
        let id1 = manager.create_container("test1".to_string(), "ubuntu:latest".to_string());
        let id2 = manager.create_container("test2".to_string(), "ubuntu:latest".to_string());
        manager.start_container(&id1);

        let running = manager.list_containers_by_state(ContainerState::Running);
        assert_eq!(running.len(), 1);

        let created = manager.list_containers_by_state(ContainerState::Created);
        assert_eq!(created.len(), 1);
    }

    #[test]
    fn test_running_container_count() {
        let mut manager = SigmaContainer::new();
        let id1 = manager.create_container("test1".to_string(), "ubuntu:latest".to_string());
        let id2 = manager.create_container("test2".to_string(), "ubuntu:latest".to_string());
        manager.start_container(&id1);
        manager.start_container(&id2);

        assert_eq!(manager.running_container_count(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmacontainer();
        let manager = get_sigmacontainer();
        assert_eq!(manager.container_count(), 0);

        let manager_mut = get_sigmacontainer_mut();
        manager_mut.create_container("test".to_string(), "ubuntu:latest".to_string());
        assert_eq!(manager.container_count(), 1);
    }

    #[test]
    fn test_create_container_convenience() {
        init_sigmacontainer();
        let id = create_container("test".to_string(), "ubuntu:latest".to_string());
        assert_eq!(id, "sigma-1");

        let manager = get_sigmacontainer();
        assert!(manager.get_container(&id).is_some());
    }
;
