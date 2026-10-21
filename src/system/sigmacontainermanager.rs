// SPDX-License-Identifier: MIT
// SigmaOS - Container Manager
// Container management inspired by Arch (docker, podman), Debian (docker), and CachyOS

#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Container state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
    Restarting,
    Removing,
    Dead,
}

/// Container config
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub image: String,
    pub command: String,
    pub memory_limit_mb: u64,
    pub cpu_shares: u32,
    pub network_enabled: bool,
}

/// Container
#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub state: ContainerState,
    pub config: ContainerConfig,
    pub created_at: u64,
}

/// Container manager
pub struct SigmaContainerManager {
    containers: Vec<Container>,
}

impl SigmaContainerManager {
    pub fn new() -> Self {
        Self {
            containers: Vec::new(),
        }
    }

    pub fn create_container(&mut self, name: String, config: ContainerConfig) -> Container {
        let container = Container {
            id: alloc::format!("container_{}", self.containers.len()),
            name,
            state: ContainerState::Created,
            config,
            created_at: 0,
        };
        self.containers.push(container.clone());
        container
    }

    pub fn get_containers(&self) -> &[Container] {
        &self.containers
    }

    pub fn get_container(&self, id: &str) -> Option<&Container> {
        self.containers.iter().find(|c| c.id == id)
    }

    pub fn get_container_mut(&mut self, id: &str) -> Option<&mut Container> {
        self.containers.iter_mut().find(|c| c.id == id)
    }

    pub fn start_container(&mut self, id: String) -> Result<(), String> {
        if let Some(container) = self.get_container_mut(&id) {
            container.state = ContainerState::Running;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn stop_container(&mut self, id: String) -> Result<(), String> {
        if let Some(container) = self.get_container_mut(&id) {
            container.state = ContainerState::Stopped;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn pause_container(&mut self, id: String) -> Result<(), String> {
        if let Some(container) = self.get_container_mut(&id) {
            container.state = ContainerState::Paused;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn resume_container(&mut self, id: String) -> Result<(), String> {
        if let Some(container) = self.get_container_mut(&id) {
            container.state = ContainerState::Running;
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn remove_container(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.containers.iter().position(|c| c.id == id) {
            self.containers.remove(idx);
            Ok(())
        } else {
            Err(format!("Container {} not found", id))
        }
    }

    pub fn get_running_containers(&self) -> Vec<&Container> {
        self.containers
            .iter()
            .filter(|c| c.state == ContainerState::Running)
            .collect()
    }

    pub fn get_container_count(&self) -> usize {
        self.containers.len()
    }
}

impl Default for SigmaContainerManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_manager_creation() {
        let manager = SigmaContainerManager::new();
        assert_eq!(manager.get_containers().len(), 0);
    }

    #[test]
    fn test_create_container() {
        let mut manager = SigmaContainerManager::new();
        let config = ContainerConfig {
            image: "ubuntu".to_string(),
            command: "/bin/bash".to_string(),
            memory_limit_mb: 512,
            cpu_shares: 1024,
            network_enabled: true,
        };
        manager.create_container("test".to_string(), config);
        assert_eq!(manager.get_containers().len(), 1);
    }

    #[test]
    fn test_start_container() {
        let mut manager = SigmaContainerManager::new();
        let config = ContainerConfig {
            image: "ubuntu".to_string(),
            command: "/bin/bash".to_string(),
            memory_limit_mb: 512,
            cpu_shares: 1024,
            network_enabled: true,
        };
        let container = manager.create_container("test".to_string(), config);
        manager.start_container(container.id.clone()).unwrap();
        assert_eq!(manager.get_containers()[0].state, ContainerState::Running);
    }

    #[test]
    fn test_stop_container() {
        let mut manager = SigmaContainerManager::new();
        let config = ContainerConfig {
            image: "ubuntu".to_string(),
            command: "/bin/bash".to_string(),
            memory_limit_mb: 512,
            cpu_shares: 1024,
            network_enabled: true,
        };
        let container = manager.create_container("test".to_string(), config);
        manager.start_container(container.id.clone()).unwrap();
        manager.stop_container(container.id).unwrap();
        assert_eq!(manager.get_containers()[0].state, ContainerState::Stopped);
    }

    #[test]
    fn test_remove_container() {
        let mut manager = SigmaContainerManager::new();
        let config = ContainerConfig {
            image: "ubuntu".to_string(),
            command: "/bin/bash".to_string(),
            memory_limit_mb: 512,
            cpu_shares: 1024,
            network_enabled: true,
        };
        let container = manager.create_container("test".to_string(), config);
        manager.remove_container(container.id).unwrap();
        assert_eq!(manager.get_containers().len(), 0);
    }

    #[test]
    fn test_running_containers() {
        let mut manager = SigmaContainerManager::new();
        let config = ContainerConfig {
            image: "ubuntu".to_string(),
            command: "/bin/bash".to_string(),
            memory_limit_mb: 512,
            cpu_shares: 1024,
            network_enabled: true,
        };
        let container1 = manager.create_container("test1".to_string(), config.clone());
        let _container2 = manager.create_container("test2".to_string(), config);
        manager.start_container(container1.id).unwrap();
        assert_eq!(manager.get_running_containers().len(), 1);
    }
}
