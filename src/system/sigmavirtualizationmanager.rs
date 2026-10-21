// SPDX-License-Identifier: MIT
// SigmaOS - Virtualization Manager
// Virtualization management inspired by Arch (KVM, QEMU), Debian (libvirt), and CachyOS

#![no_std]
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// VM state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState {
    Stopped,
    Running,
    Paused,
    Suspended,
    PoweredOff,
}

/// VM config
#[derive(Debug, Clone)]
pub struct VMConfig {
    pub name: String,
    pub memory_mb: u64,
    pub cpu_cores: u32,
    pub disk_path: String,
    pub disk_size_gb: u64,
    pub network_enabled: bool,
}

/// Virtual machine
#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub id: String,
    pub config: VMConfig,
    pub state: VMState,
    pub uptime_seconds: u64,
}

/// Virtualization manager
pub struct SigmaVirtualizationManager {
    vms: Vec<VirtualMachine>,
}

impl SigmaVirtualizationManager {
    pub fn new() -> Self {
        Self { vms: Vec::new() }
    }

    pub fn create_vm(&mut self, config: VMConfig) -> VirtualMachine {
        let vm = VirtualMachine {
            id: alloc::format!("vm_{}", self.vms.len()),
            config,
            state: VMState::Stopped,
            uptime_seconds: 0,
        };
        self.vms.push(vm.clone());
        vm
    }

    pub fn get_vms(&self) -> &[VirtualMachine] {
        &self.vms
    }

    pub fn get_vm(&self, id: &str) -> Option<&VirtualMachine> {
        self.vms.iter().find(|v| v.id == id)
    }

    pub fn get_vm_mut(&mut self, id: &str) -> Option<&mut VirtualMachine> {
        self.vms.iter_mut().find(|v| v.id == id)
    }

    pub fn start_vm(&mut self, id: String) -> Result<(), String> {
        if let Some(vm) = self.get_vm_mut(&id) {
            vm.state = VMState::Running;
            Ok(())
        } else {
            Err(format!("VM {} not found", id))
        }
    }

    pub fn stop_vm(&mut self, id: String) -> Result<(), String> {
        if let Some(vm) = self.get_vm_mut(&id) {
            vm.state = VMState::Stopped;
            vm.uptime_seconds = 0;
            Ok(())
        } else {
            Err(format!("VM {} not found", id))
        }
    }

    pub fn pause_vm(&mut self, id: String) -> Result<(), String> {
        if let Some(vm) = self.get_vm_mut(&id) {
            vm.state = VMState::Paused;
            Ok(())
        } else {
            Err(format!("VM {} not found", id))
        }
    }

    pub fn resume_vm(&mut self, id: String) -> Result<(), String> {
        if let Some(vm) = self.get_vm_mut(&id) {
            vm.state = VMState::Running;
            Ok(())
        } else {
            Err(format!("VM {} not found", id))
        }
    }

    pub fn delete_vm(&mut self, id: String) -> Result<(), String> {
        if let Some(idx) = self.vms.iter().position(|v| v.id == id) {
            self.vms.remove(idx);
            Ok(())
        } else {
            Err(format!("VM {} not found", id))
        }
    }

    pub fn get_running_vms(&self) -> Vec<&VirtualMachine> {
        self.vms
            .iter()
            .filter(|v| v.state == VMState::Running)
            .collect()
    }

    pub fn get_vm_count(&self) -> usize {
        self.vms.len()
    }
}

impl Default for SigmaVirtualizationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtualization_manager_creation() {
        let manager = SigmaVirtualizationManager::new();
        assert_eq!(manager.get_vms().len(), 0);
    }

    #[test]
    fn test_create_vm() {
        let mut manager = SigmaVirtualizationManager::new();
        let config = VMConfig {
            name: "test-vm".to_string(),
            memory_mb: 2048,
            cpu_cores: 2,
            disk_path: "/var/lib/vms/test.qcow2".to_string(),
            disk_size_gb: 20,
            network_enabled: true,
        };
        manager.create_vm(config);
        assert_eq!(manager.get_vms().len(), 1);
    }

    #[test]
    fn test_start_vm() {
        let mut manager = SigmaVirtualizationManager::new();
        let config = VMConfig {
            name: "test-vm".to_string(),
            memory_mb: 2048,
            cpu_cores: 2,
            disk_path: "/var/lib/vms/test.qcow2".to_string(),
            disk_size_gb: 20,
            network_enabled: true,
        };
        let vm = manager.create_vm(config);
        manager.start_vm(vm.id).unwrap();
        assert_eq!(manager.get_vms()[0].state, VMState::Running);
    }

    #[test]
    fn test_stop_vm() {
        let mut manager = SigmaVirtualizationManager::new();
        let config = VMConfig {
            name: "test-vm".to_string(),
            memory_mb: 2048,
            cpu_cores: 2,
            disk_path: "/var/lib/vms/test.qcow2".to_string(),
            disk_size_gb: 20,
            network_enabled: true,
        };
        let vm = manager.create_vm(config);
        manager.start_vm(vm.id.clone()).unwrap();
        manager.stop_vm(vm.id).unwrap();
        assert_eq!(manager.get_vms()[0].state, VMState::Stopped);
    }

    #[test]
    fn test_delete_vm() {
        let mut manager = SigmaVirtualizationManager::new();
        let config = VMConfig {
            name: "test-vm".to_string(),
            memory_mb: 2048,
            cpu_cores: 2,
            disk_path: "/var/lib/vms/test.qcow2".to_string(),
            disk_size_gb: 20,
            network_enabled: true,
        };
        let vm = manager.create_vm(config);
        manager.delete_vm(vm.id).unwrap();
        assert_eq!(manager.get_vms().len(), 0);
    }

    #[test]
    fn test_running_vms() {
        let mut manager = SigmaVirtualizationManager::new();
        let config = VMConfig {
            name: "test-vm".to_string(),
            memory_mb: 2048,
            cpu_cores: 2,
            disk_path: "/var/lib/vms/test.qcow2".to_string(),
            disk_size_gb: 20,
            network_enabled: true,
        };
        let vm1 = manager.create_vm(config.clone());
        let _vm2 = manager.create_vm(config);
        manager.start_vm(vm1.id).unwrap();
        assert_eq!(manager.get_running_vms().len(), 1);
    }
}
