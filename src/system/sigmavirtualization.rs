//! SigmaVirtualization - Virtual Machine Management System
//!
//! Provides comprehensive virtual machine management inspired by KVM, QEMU, and VirtualBox.
//! Supports VM lifecycle, resource allocation, disk management, and network configuration.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// VM state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VMState {
    Stopped,
    Running,
    Paused,
    PoweredOff,
    Suspended,
    Error,
    Unknown,
}

/// VM architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    X86_64,
    ARM64,
    RISCV64,
    Unknown,
}

/// Disk interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskInterface {
    IDE,
    SATA,
    SCSI,
    NVMe,
    VirtIO,
    Unknown,
}

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkInterface {
    E1000,
    VirtIO,
    RTL8139,
    Unknown,
}

/// Virtual disk
#[derive(Debug, Clone)]
pub struct VirtualDisk {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub interface: DiskInterface,
    pub bootable: bool,
}

impl VirtualDisk {
    pub fn new(id: String, path: String, size: u64, interface: DiskInterface) -> Self {
        Self {
            id,
            path,
            size,
            interface,
            bootable: false,
        }
    }

    pub fn with_bootable(mut self, bootable: bool) -> Self {
        self.bootable = bootable;
        self
    }
}

/// Virtual network adapter
#[derive(Debug, Clone)]
pub struct VirtualNetwork {
    pub id: String,
    pub interface_type: NetworkInterface,
    pub mac_address: Option<String>,
    pub bridge: Option<String>,
}

impl VirtualNetwork {
    pub fn new(id: String, interface_type: NetworkInterface) -> Self {
        Self {
            id,
            interface_type,
            mac_address: None,
            bridge: None,
        }
    }

    pub fn with_mac_address(mut self, mac: String) -> Self {
        self.mac_address = Some(mac);
        self
    }

    pub fn with_bridge(mut self, bridge: String) -> Self {
        self.bridge = Some(bridge);
        self
    }
}

/// VM resource allocation
#[derive(Debug, Clone)]
pub struct VMResources {
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub vcpus: u32,
}

impl VMResources {
    pub fn new(cpu_cores: u32, memory_mb: u64) -> Self {
        Self {
            cpu_cores,
            memory_mb,
            vcpus: cpu_cores,
        }
    }

    pub fn with_vcpus(mut self, vcpus: u32) -> Self {
        self.vcpus = vcpus;
        self
    }
}

/// Virtual machine
#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    pub state: VMState,
    pub architecture: Architecture,
    pub resources: VMResources,
    pub disks: Vec<VirtualDisk>,
    pub networks: Vec<VirtualNetwork>,
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub stopped_at: Option<u64>,
}

impl VirtualMachine {
    pub fn new(
        id: String,
        name: String,
        architecture: Architecture,
        resources: VMResources,
    ) -> Self {
        Self {
            id,
            name,
            state: VMState::Stopped,
            architecture,
            resources,
            disks: Vec::new(),
            networks: Vec::new(),
            created_at: 0,
            started_at: None,
            stopped_at: None,
        }
    }

    pub fn with_disks(mut self, disks: Vec<VirtualDisk>) -> Self {
        self.disks = disks;
        self
    }

    pub fn with_networks(mut self, networks: Vec<VirtualNetwork>) -> Self {
        self.networks = networks;
        self
    }

    pub fn add_disk(&mut self, disk: VirtualDisk) {
        self.disks.push(disk);
    }

    pub fn add_network(&mut self, network: VirtualNetwork) {
        self.networks.push(network);
    }

    pub fn start(&mut self) {
        self.state = VMState::Running;
        self.started_at = Some(0);
    }

    pub fn stop(&mut self) {
        self.state = VMState::Stopped;
        self.stopped_at = Some(0);
    }

    pub fn pause(&mut self) {
        self.state = VMState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = VMState::Running;
    }

    pub fn power_off(&mut self) {
        self.state = VMState::PoweredOff;
        self.stopped_at = Some(0);
    }

    pub fn suspend(&mut self) {
        self.state = VMState::Suspended;
    }
}

/// Virtualization manager
pub struct SigmaVirtualization {
    vms: BTreeMap<String, VirtualMachine>,
    next_vm_id: u64,
}

impl SigmaVirtualization {
    pub fn new() -> Self {
        Self {
            vms: BTreeMap::new(),
            next_vm_id: 1,
        }
    }

    pub fn create_vm(
        &mut self,
        name: String,
        architecture: Architecture,
        resources: VMResources,
    ) -> String {
        let id = format!("vm-{}", self.next_vm_id);
        self.next_vm_id += 1;
        let vm = VirtualMachine::new(id.clone(), name, architecture, resources);
        self.vms.insert(id.clone(), vm);
        id
    }

    pub fn get_vm(&self, id: &str) -> Option<&VirtualMachine> {
        self.vms.get(id)
    }

    pub fn get_vm_mut(&mut self, id: &str) -> Option<&mut VirtualMachine> {
        self.vms.get_mut(id)
    }

    pub fn list_vms(&self) -> Vec<&VirtualMachine> {
        self.vms.values().collect()
    }

    pub fn list_vms_by_state(&self, state: VMState) -> Vec<&VirtualMachine> {
        self.vms.values().filter(|v| v.state == state).collect()
    }

    pub fn start_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.start();
            true
        } else {
            false
        }
    }

    pub fn stop_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.stop();
            true
        } else {
            false
        }
    }

    pub fn pause_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.pause();
            true
        } else {
            false
        }
    }

    pub fn resume_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.resume();
            true
        } else {
            false
        }
    }

    pub fn power_off_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.power_off();
            true
        } else {
            false
        }
    }

    pub fn suspend_vm(&mut self, id: &str) -> bool {
        if let Some(vm) = self.vms.get_mut(id) {
            vm.suspend();
            true
        } else {
            false
        }
    }

    pub fn remove_vm(&mut self, id: &str) -> bool {
        self.vms.remove(id).is_some()
    }

    pub fn add_disk_to_vm(&mut self, vm_id: &str, disk: VirtualDisk) -> bool {
        if let Some(vm) = self.vms.get_mut(vm_id) {
            vm.add_disk(disk);
            true
        } else {
            false
        }
    }

    pub fn add_network_to_vm(&mut self, vm_id: &str, network: VirtualNetwork) -> bool {
        if let Some(vm) = self.vms.get_mut(vm_id) {
            vm.add_network(network);
            true
        } else {
            false
        }
    }

    pub fn vm_count(&self) -> usize {
        self.vms.len()
    }

    pub fn running_vm_count(&self) -> usize {
        self.vms
            .values()
            .filter(|v| v.state == VMState::Running)
            .count()
    }
}

impl Default for SigmaVirtualization {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAVIRTUALIZATION_INSTANCE: Option<SigmaVirtualization> = None;

pub fn init_sigmavirtualization() {
    unsafe {
        SIGMAVIRTUALIZATION_INSTANCE = Some(SigmaVirtualization::new());
    }
}

pub fn get_sigmavirtualization() -> &'static SigmaVirtualization {
    unsafe {
        SIGMAVIRTUALIZATION_INSTANCE
            .as_ref()
            .expect("SigmaVirtualization not initialized")
    }
}

pub fn get_sigmavirtualization_mut() -> &'static mut SigmaVirtualization {
    unsafe {
        SIGMAVIRTUALIZATION_INSTANCE
            .as_mut()
            .expect("SigmaVirtualization not initialized")
    }
}

pub fn create_vm(name: String, architecture: Architecture, resources: VMResources) -> String {
    let manager = get_sigmavirtualization_mut();
    manager.create_vm(name, architecture, resources)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_disk_creation() {
        let disk = VirtualDisk::new(
            "disk-1".to_string(),
            "/path/to/disk.img".to_string(),
            10 * 1024 * 1024 * 1024,
            DiskInterface::VirtIO,
        );
        assert_eq!(disk.id, "disk-1");
        assert_eq!(disk.size, 10 * 1024 * 1024 * 1024);
        assert_eq!(disk.interface, DiskInterface::VirtIO);
        assert!(!disk.bootable);
    }

    #[test]
    fn test_virtual_disk_builder() {
        let disk = VirtualDisk::new(
            "disk-1".to_string(),
            "/path/to/disk.img".to_string(),
            10 * 1024 * 1024 * 1024,
            DiskInterface::VirtIO,
        )
        .with_bootable(true);

        assert!(disk.bootable);
    }

    #[test]
    fn test_virtual_network_creation() {
        let network = VirtualNetwork::new("net-1".to_string(), NetworkInterface::VirtIO);
        assert_eq!(network.id, "net-1");
        assert_eq!(network.interface_type, NetworkInterface::VirtIO);
        assert!(network.mac_address.is_none());
    }

    #[test]
    fn test_virtual_network_builder() {
        let network = VirtualNetwork::new("net-1".to_string(), NetworkInterface::VirtIO)
            .with_mac_address("52:54:00:00:00:01".to_string())
            .with_bridge("br0".to_string());

        assert_eq!(network.mac_address, Some("52:54:00:00:00:01".to_string()));
        assert_eq!(network.bridge, Some("br0".to_string()));
    }

    #[test]
    fn test_vm_resources() {
        let resources = VMResources::new(4, 8192).with_vcpus(8);
        assert_eq!(resources.cpu_cores, 4);
        assert_eq!(resources.memory_mb, 8192);
        assert_eq!(resources.vcpus, 8);
    }

    #[test]
    fn test_vm_creation() {
        let resources = VMResources::new(2, 4096);
        let vm = VirtualMachine::new(
            "vm-1".to_string(),
            "test-vm".to_string(),
            Architecture::X86_64,
            resources,
        );
        assert_eq!(vm.id, "vm-1");
        assert_eq!(vm.name, "test-vm");
        assert_eq!(vm.state, VMState::Stopped);
        assert_eq!(vm.architecture, Architecture::X86_64);
    }

    #[test]
    fn test_vm_state_transitions() {
        let resources = VMResources::new(2, 4096);
        let mut vm = VirtualMachine::new(
            "vm-1".to_string(),
            "test-vm".to_string(),
            Architecture::X86_64,
            resources,
        );
        assert_eq!(vm.state, VMState::Stopped);

        vm.start();
        assert_eq!(vm.state, VMState::Running);
        assert!(vm.started_at.is_some());

        vm.pause();
        assert_eq!(vm.state, VMState::Paused);

        vm.resume();
        assert_eq!(vm.state, VMState::Running);

        vm.power_off();
        assert_eq!(vm.state, VMState::PoweredOff);
    }

    #[test]
    fn test_virtualization_manager() {
        let manager = SigmaVirtualization::new();
        assert_eq!(manager.vm_count(), 0);
    }

    #[test]
    fn test_create_vm() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id = manager.create_vm("test-vm".to_string(), Architecture::X86_64, resources);
        assert_eq!(id, "vm-1");
        assert_eq!(manager.vm_count(), 1);
    }

    #[test]
    fn test_start_stop_vm() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id = manager.create_vm("test-vm".to_string(), Architecture::X86_64, resources);

        assert!(manager.start_vm(&id));
        assert_eq!(manager.get_vm(&id).unwrap().state, VMState::Running);

        assert!(manager.stop_vm(&id));
        assert_eq!(manager.get_vm(&id).unwrap().state, VMState::Stopped);
    }

    #[test]
    fn test_pause_resume_vm() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id = manager.create_vm("test-vm".to_string(), Architecture::X86_64, resources);
        manager.start_vm(&id);

        assert!(manager.pause_vm(&id));
        assert_eq!(manager.get_vm(&id).unwrap().state, VMState::Paused);

        assert!(manager.resume_vm(&id));
        assert_eq!(manager.get_vm(&id).unwrap().state, VMState::Running);
    }

    #[test]
    fn test_add_disk_to_vm() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id = manager.create_vm("test-vm".to_string(), Architecture::X86_64, resources);

        let disk = VirtualDisk::new(
            "disk-1".to_string(),
            "/path/to/disk.img".to_string(),
            10 * 1024 * 1024 * 1024,
            DiskInterface::VirtIO,
        );
        assert!(manager.add_disk_to_vm(&id, disk));
        assert_eq!(manager.get_vm(&id).unwrap().disks.len(), 1);
    }

    #[test]
    fn test_add_network_to_vm() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id = manager.create_vm("test-vm".to_string(), Architecture::X86_64, resources);

        let network = VirtualNetwork::new("net-1".to_string(), NetworkInterface::VirtIO);
        assert!(manager.add_network_to_vm(&id, network));
        assert_eq!(manager.get_vm(&id).unwrap().networks.len(), 1);
    }

    #[test]
    fn test_list_vms_by_state() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id1 = manager.create_vm("test1".to_string(), Architecture::X86_64, resources.clone());
        let id2 = manager.create_vm("test2".to_string(), Architecture::X86_64, resources);
        manager.start_vm(&id1);

        let running = manager.list_vms_by_state(VMState::Running);
        assert_eq!(running.len(), 1);

        let stopped = manager.list_vms_by_state(VMState::Stopped);
        assert_eq!(stopped.len(), 1);
    }

    #[test]
    fn test_running_vm_count() {
        let mut manager = SigmaVirtualization::new();
        let resources = VMResources::new(2, 4096);
        let id1 = manager.create_vm("test1".to_string(), Architecture::X86_64, resources.clone());
        let id2 = manager.create_vm("test2".to_string(), Architecture::X86_64, resources);
        manager.start_vm(&id1);
        manager.start_vm(&id2);

        assert_eq!(manager.running_vm_count(), 2);
    }

    #[test]
    fn test_global_instance() {
        init_sigmavirtualization();
        let manager = get_sigmavirtualization();
        assert_eq!(manager.vm_count(), 0);

        let manager_mut = get_sigmavirtualization_mut();
        let resources = VMResources::new(2, 4096);
        manager_mut.create_vm("test".to_string(), Architecture::X86_64, resources);
        assert_eq!(manager.vm_count(), 1);
    }

    #[test]
    fn test_create_vm_convenience() {
        init_sigmavirtualization();
        let resources = VMResources::new(2, 4096);
        let id = create_vm("test".to_string(), Architecture::X86_64, resources);
        assert_eq!(id, "vm-1");

        let manager = get_sigmavirtualization();
        assert!(manager.get_vm(&id).is_some());
    }
}
