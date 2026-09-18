// FreeBSD Capsicum-Inspired Capability-Based Security Framework
// Fine-grained capability restriction for processes, limiting access to system resources


use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

/// Capsicum-inspired capability rights
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapRights(u64);

impl CapRights {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn read() -> Self {
        Self(1 << 0)
    }

    pub const fn write() -> Self {
        Self(1 << 1)
    }

    pub const fn execute() -> Self {
        Self(1 << 2)
    }

    pub const fn seek() -> Self {
        Self(1 << 3)
    }

    pub const fn mmap() -> Self {
        Self(1 << 4)
    }

    pub const fn create() -> Self {
        Self(1 << 5)
    }

    pub const fn delete() -> Self {
        Self(1 << 6)
    }

    pub const fn setattr() -> Self {
        Self(1 << 7)
    }

    pub const fn getattr() -> Self {
        Self(1 << 8)
    }

    pub const fn all() -> Self {
        Self(0xFFFFFFFFFFFFFFFF)
    }

    pub fn contains(&self, other: CapRights) -> bool {
        (self.0 & other.0) == other.0
    }

    pub fn union(&self, other: CapRights) -> CapRights {
        CapRights(self.0 | other.0)
    }

    pub fn intersection(&self, other: CapRights) -> CapRights {
        CapRights(self.0 & other.0)
    }

    pub fn remove(&self, other: CapRights) -> CapRights {
        CapRights(self.0 & !other.0)
    }
}

/// Capsicum-inspired capability mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapMode {
    /// No capability restrictions
    None,
    /// Basic capability mode enabled
    Basic,
    /// Strict capability mode (all operations require capabilities)
    Strict,
}

/// Capsicum-inspired capability descriptor
#[derive(Debug, Clone)]
pub struct CapDescriptor {
    pub id: u64,
    pub rights: CapRights,
    pub resource_type: CapResourceType,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapResourceType {
    File,
    Directory,
    Socket,
    Pipe,
    Device,
    Process,
    SharedMemory,
    Semaphore,
}

impl CapDescriptor {
    pub fn new(id: u64, rights: CapRights, resource_type: CapResourceType) -> Self {
        Self {
            id,
            rights,
            resource_type,
            description: String::new(),
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn has_rights(&self, required: CapRights) -> bool {
        self.rights.contains(required)
    }
}

/// Capsicum-inspired process sandbox
#[derive(Debug, Clone)]
pub struct CapSandbox {
    pub pid: u32,
    pub mode: CapMode,
    pub descriptors: BTreeMap<u64, CapDescriptor>,
    pub next_descriptor_id: u64,
    pub name: String,
}

impl CapSandbox {
    pub fn new(pid: u32, name: String) -> Self {
        Self {
            pid,
            mode: CapMode::None,
            descriptors: BTreeMap::new(),
            next_descriptor_id: 1,
            name,
        }
    }

    pub fn set_mode(&mut self, mode: CapMode) {
        self.mode = mode;
    }

    pub fn enter_capability_mode(&mut self) -> Result<(), &'static str> {
        if self.mode == CapMode::Strict {
            return Err("Already in strict capability mode");
        }
        self.mode = CapMode::Basic;
        Ok(())
    }

    pub fn enter_strict_mode(&mut self) -> Result<(), &'static str> {
        self.mode = CapMode::Strict;
        Ok(())
    }

    pub fn add_descriptor(&mut self, descriptor: CapDescriptor) -> u64 {
        let id = descriptor.id;
        self.descriptors.insert(id, descriptor);
        id
    }

    pub fn create_descriptor(&mut self, rights: CapRights, resource_type: CapResourceType) -> u64 {
        let id = self.next_descriptor_id;
        self.next_descriptor_id += 1;

        let descriptor = CapDescriptor::new(id, rights, resource_type);
        self.add_descriptor(descriptor)
    }

    pub fn get_descriptor(&self, id: u64) -> Option<&CapDescriptor> {
        self.descriptors.get(&id)
    }

    pub fn get_descriptor_mut(&mut self, id: u64) -> Option<&mut CapDescriptor> {
        self.descriptors.get_mut(&id)
    }

    pub fn remove_descriptor(&mut self, id: u64) -> Result<(), &'static str> {
        if !self.descriptors.contains_key(&id) {
            return Err("Descriptor not found");
        }
        self.descriptors.remove(&id);
        Ok(())
    }

    pub fn limit_rights(&mut self, id: u64, new_rights: CapRights) -> Result<(), &'static str> {
        if let Some(descriptor) = self.descriptors.get_mut(&id) {
            descriptor.rights = descriptor.rights.intersection(new_rights);
            Ok(())
        } else {
            Err("Descriptor not found")
        }
    }

    pub fn check_rights(&self, id: u64, required: CapRights) -> bool {
        if self.mode == CapMode::None {
            return true; // No restrictions
        }

        if let Some(descriptor) = self.descriptors.get(&id) {
            descriptor.has_rights(required)
        } else {
            false
        }
    }

    pub fn list_descriptors(&self) -> Vec<&CapDescriptor> {
        self.descriptors.values().collect()
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptors.len()
    }
}

/// Capsicum-inspired capability manager
pub struct CapManager {
    sandboxes: BTreeMap<u32, CapSandbox>,
    global_descriptors: BTreeMap<u64, CapDescriptor>,
    next_global_id: u64,
    default_mode: CapMode,
}

impl CapManager {
    pub fn new() -> Self {
        Self {
            sandboxes: BTreeMap::new(),
            global_descriptors: BTreeMap::new(),
            next_global_id: 1,
            default_mode: CapMode::None,
        }
    }

    pub fn with_default_mode(mode: CapMode) -> Self {
        let mut manager = Self::new();
        manager.default_mode = mode;
        manager
    }

    pub fn create_sandbox(&mut self, pid: u32, name: String) -> CapSandbox {
        let mut sandbox = CapSandbox::new(pid, name);
        sandbox.set_mode(self.default_mode);
        self.sandboxes.insert(pid, sandbox);
        self.sandboxes.get(&pid).unwrap().clone()
    }

    pub fn get_sandbox(&self, pid: u32) -> Option<&CapSandbox> {
        self.sandboxes.get(&pid)
    }

    pub fn get_sandbox_mut(&mut self, pid: u32) -> Option<&mut CapSandbox> {
        self.sandboxes.get_mut(&pid)
    }

    pub fn remove_sandbox(&mut self, pid: u32) -> Result<(), &'static str> {
        if !self.sandboxes.contains_key(&pid) {
            return Err("Sandbox not found");
        }
        self.sandboxes.remove(&pid);
        Ok(())
    }

    pub fn add_global_descriptor(&mut self, descriptor: CapDescriptor) -> u64 {
        let id = descriptor.id;
        self.global_descriptors.insert(id, descriptor);
        id
    }

    pub fn create_global_descriptor(&mut self, rights: CapRights, resource_type: CapResourceType) -> u64 {
        let id = self.next_global_id;
        self.next_global_id += 1;

        let descriptor = CapDescriptor::new(id, rights, resource_type);
        self.add_global_descriptor(descriptor)
    }

    pub fn get_global_descriptor(&self, id: u64) -> Option<&CapDescriptor> {
        self.global_descriptors.get(&id)
    }

    pub fn check_global_rights(&self, id: u64, required: CapRights) -> bool {
        if let Some(descriptor) = self.global_descriptors.get(&id) {
            descriptor.has_rights(required)
        } else {
            false
        }
    }

    pub fn set_default_mode(&mut self, mode: CapMode) {
        self.default_mode = mode;
    }

    /// Enter capability mode for a process
    pub fn enter_capability_mode(&mut self, pid: u32) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(&pid) {
            sandbox.enter_capability_mode()
        } else {
            Err("Sandbox not found")
        }
    }

    /// Enter strict capability mode for a process
    pub fn enter_strict_mode(&mut self, pid: u32) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(&pid) {
            sandbox.enter_strict_mode()
        } else {
            Err("Sandbox not found")
        }
    }

    /// Check if a process can perform an operation
    pub fn check_operation(&self, pid: u32, descriptor_id: u64, required_rights: CapRights) -> bool {
        if let Some(sandbox) = self.sandboxes.get(&pid) {
            sandbox.check_rights(descriptor_id, required_rights)
        } else {
            false
        }
    }

    /// Grant additional rights to a descriptor
    pub fn grant_rights(&mut self, pid: u32, descriptor_id: u64, additional_rights: CapRights) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(&pid) {
            if let Some(descriptor) = sandbox.descriptors.get_mut(&descriptor_id) {
                descriptor.rights = descriptor.rights.union(additional_rights);
                Ok(())
            } else {
                Err("Descriptor not found")
            }
        } else {
            Err("Sandbox not found")
        }
    }

    /// Revoke rights from a descriptor
    pub fn revoke_rights(&mut self, pid: u32, descriptor_id: u64, rights_to_revoke: CapRights) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(&pid) {
            sandbox.limit_rights(descriptor_id, sandbox.get_descriptor(descriptor_id).unwrap().rights.remove(rights_to_revoke))
        } else {
            Err("Sandbox not found")
        }
    }

    /// Get statistics about capability usage
    pub fn get_stats(&self) -> CapStats {
        let total_sandboxes = self.sandboxes.len();
        let total_descriptors: usize = self.sandboxes.values().map(|s| s.descriptor_count()).sum();
        let total_global_descriptors = self.global_descriptors.len();
        let strict_mode_count = self.sandboxes.values().filter(|s| s.mode == CapMode::Strict).count();
        let basic_mode_count = self.sandboxes.values().filter(|s| s.mode == CapMode::Basic).count();

        CapStats {
            total_sandboxes,
            total_descriptors,
            total_global_descriptors,
            strict_mode_count,
            basic_mode_count,
            default_mode: self.default_mode,
        }
    }
}

/// Capability statistics
#[derive(Debug)]
pub struct CapStats {
    pub total_sandboxes: usize,
    pub total_descriptors: usize,
    pub total_global_descriptors: usize,
    pub strict_mode_count: usize,
    pub basic_mode_count: usize,
    pub default_mode: CapMode,
}

/// Capsicum-inspired file descriptor wrapper
#[derive(Debug, Clone)]
pub struct CapFileDescriptor {
    pub descriptor_id: u64,
    pub path: String,
    pub rights: CapRights,
}

impl CapFileDescriptor {
    pub fn new(descriptor_id: u64, path: String, rights: CapRights) -> Self {
        Self {
            descriptor_id,
            path,
            rights,
        }
    }

    pub fn can_read(&self) -> bool {
        self.rights.contains(CapRights::read())
    }

    pub fn can_write(&self) -> bool {
        self.rights.contains(CapRights::write())
    }

    pub fn can_execute(&self) -> bool {
        self.rights.contains(CapRights::execute())
    }

    pub fn can_seek(&self) -> bool {
        self.rights.contains(CapRights::seek())
    }
}

/// Capsicum-inspired capability validator
pub struct CapValidator {
    allowed_operations: BTreeMap<CapResourceType, CapRights>,
}

impl CapValidator {
    pub fn new() -> Self {
        let mut allowed_operations = BTreeMap::new();

        // Default permissions for different resource types
        allowed_operations.insert(CapResourceType::File, CapRights::read() | CapRights::write() | CapRights::seek());
        allowed_operations.insert(CapResourceType::Directory, CapRights::read() | CapRights::getattr());
        allowed_operations.insert(CapResourceType::Socket, CapRights::read() | CapRights::write());
        allowed_operations.insert(CapResourceType::Pipe, CapRights::read() | CapRights::write());
        allowed_operations.insert(CapResourceType::Device, CapRights::read() | CapRights::write());
        allowed_operations.insert(CapResourceType::Process, CapRights::getattr());
        allowed_operations.insert(CapResourceType::SharedMemory, CapRights::read() | CapRights::write() | CapRights::mmap());
        allowed_operations.insert(CapResourceType::Semaphore, CapRights::read() | CapRights::write());

        Self {
            allowed_operations,
        }
    }

    pub fn validate_operation(&self, resource_type: CapResourceType, required_rights: CapRights) -> bool {
        if let Some(&allowed) = self.allowed_operations.get(&resource_type) {
            allowed.contains(required_rights)
        } else {
            false
        }
    }

    pub fn set_allowed_operations(&mut self, resource_type: CapResourceType, rights: CapRights) {
        self.allowed_operations.insert(resource_type, rights);
    }

    pub fn get_allowed_operations(&self, resource_type: CapResourceType) -> Option<CapRights> {
        self.allowed_operations.get(&resource_type).copied()
    }
}

/// Capsicum-inspired namespace for process isolation
#[derive(Debug, Clone)]
pub struct CapNamespace {
    pub name: String,
    pub mount_points: Vec<String>,
    pub restricted: bool,
}

impl CapNamespace {
    pub fn new(name: String) -> Self {
        Self {
            name,
            mount_points: Vec::new(),
            restricted: true,
        }
    }

    pub fn add_mount_point(&mut self, path: String) {
        if !self.mount_points.contains(&path) {
            self.mount_points.push(path);
        }
    }

    pub fn remove_mount_point(&mut self, path: &str) {
        self.mount_points.retain(|p| p != path);
    }

    pub fn has_mount_point(&self, path: &str) -> bool {
        self.mount_points.contains(&path.to_string())
    }
}

/// Capsicum-inspired namespace manager
pub struct CapNamespaceManager {
    namespaces: BTreeMap<String, CapNamespace>,
    process_namespaces: BTreeMap<u32, String>, // pid -> namespace name
}

impl CapNamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            process_namespaces: BTreeMap::new(),
        }
    }

    pub fn create_namespace(&mut self, name: String) -> Result<(), &'static str> {
        if self.namespaces.contains_key(&name) {
            return Err("Namespace already exists");
        }

        let namespace = CapNamespace::new(name.clone());
        self.namespaces.insert(name, namespace);
        Ok(())
    }

    pub fn get_namespace(&self, name: &str) -> Option<&CapNamespace> {
        self.namespaces.get(name)
    }

    pub fn get_namespace_mut(&mut self, name: &str) -> Option<&mut CapNamespace> {
        self.namespaces.get_mut(name)
    }

    pub fn assign_process_to_namespace(&mut self, pid: u32, namespace_name: String) -> Result<(), &'static str> {
        if !self.namespaces.contains_key(&namespace_name) {
            return Err("Namespace does not exist");
        }

        self.process_namespaces.insert(pid, namespace_name);
        Ok(())
    }

    pub fn get_process_namespace(&self, pid: u32) -> Option<&CapNamespace> {
        if let Some(namespace_name) = self.process_namespaces.get(&pid) {
            self.namespaces.get(namespace_name)
        } else {
            None
        }
    }

    pub fn remove_process_from_namespace(&mut self, pid: u32) {
        self.process_namespaces.remove(&pid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cap_rights() {
        let read = CapRights::read();
        let write = CapRights::write();
        let both = read.union(write);

        assert!(both.contains(read));
        assert!(both.contains(write));
    }

    #[test]
    fn test_sandbox_creation() {
        let sandbox = CapSandbox::new(1234, "test".to_string());
        assert_eq!(sandbox.pid, 1234);
        assert_eq!(sandbox.mode, CapMode::None);
    }

    #[test]
    fn test_capability_mode() {
        let mut sandbox = CapSandbox::new(1234, "test".to_string());
        sandbox.enter_capability_mode().unwrap();
        assert_eq!(sandbox.mode, CapMode::Basic);
    }

    #[test]
    fn test_descriptor_operations() {
        let mut sandbox = CapSandbox::new(1234, "test".to_string());
        let id = sandbox.create_descriptor(CapRights::read(), CapResourceType::File);
        
        assert!(sandbox.check_rights(id, CapRights::read()));
        assert!(!sandbox.check_rights(id, CapRights::write()));
    }

    #[test]
    fn test_manager_operations() {
        let mut manager = CapManager::new();
        manager.create_sandbox(1234, "test".to_string());
        
        assert!(manager.get_sandbox(1234).is_some());
        assert_eq!(manager.get_stats().total_sandboxes, 1);
    }

    #[test]
    fn test_namespace_operations() {
        let mut manager = CapNamespaceManager::new();
        manager.create_namespace("restricted".to_string()).unwrap();
        
        let ns = manager.get_namespace("restricted").unwrap();
        assert_eq!(ns.name, "restricted");
    }
}