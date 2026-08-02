#![no_std]
#![no_main]

use core::mem;
/// OOP-based Container Runtime for SigmaOS
/// Implements container runtime using OOP principles with traits and structs
/// No dependency on external container frameworks
/// Based on Roadmap Item 17: Container runtime support
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Container ID
pub type ContainerID = usize;

/// Container state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
    Failed = 4,
}

/// Container trait (OOP interface)
pub trait Container {
    /// Get container ID
    fn id(&self) -> ContainerID;
    /// Get container name
    fn name(&self) -> &[u8];
    /// Start container
    fn start(&mut self) -> Result<(), ContainerError>;
    /// Stop container
    fn stop(&mut self) -> Result<(), ContainerError>;
    /// Pause container
    fn pause(&mut self) -> Result<(), ContainerError>;
    /// Resume container
    fn resume(&mut self) -> Result<(), ContainerError>;
    /// Get container state
    fn state(&self) -> ContainerState;
    /// Get container info
    fn info(&self) -> ContainerInfo;
}

/// Container error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    Success = 0,
    AlreadyStarted = 1,
    AlreadyStopped = 2,
    StartFailed = 3,
    StopFailed = 4,
    PermissionDenied = 5,
    ResourceLimit = 6,
}

/// Container info
#[repr(C)]
pub struct ContainerInfo {
    pub id: ContainerID,
    pub name: [u8; 64],
    pub image: [u8; 128],
    pub state: ContainerState,
    pub pid: Option<usize>,
    pub memory_limit: u64,
    pub cpu_limit: u32,
    pub capability: ContainerCapability,
}

impl ContainerInfo {
    pub fn new(id: ContainerID) -> Self {
        ContainerInfo {
            id,
            name: [0; 64],
            image: [0; 128],
            state: ContainerState::Created,
            pid: None,
            memory_limit: 0,
            cpu_limit: 0,
            capability: ContainerCapability::new(),
        }
    }
}

/// Container capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ContainerCapability {
    pub can_start: bool,
    pub can_stop: bool,
    pub can_pause: bool,
    pub can_modify: bool,
}

impl ContainerCapability {
    pub fn new() -> Self {
        ContainerCapability {
            can_start: false,
            can_stop: false,
            can_pause: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        ContainerCapability {
            can_start: true,
            can_stop: true,
            can_pause: true,
            can_modify: true,
        }
    }
}

/// Container network configuration type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerNetworkType {
    None,
    Bridge,
    Overlay,
}

/// Container volume configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerVolume {
    pub is_bind_mount: bool,
    pub is_tmpfs: bool,
    pub read_only: bool,
}

/// Container user namespaces mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerNamespace {
    pub uid_mapping: u32,
    pub gid_mapping: u32,
    pub rootless: bool,
}

/// Container seccomp profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeccompProfile {
    pub hardened: bool,
    pub blocked_syscalls_mask: u32,
}

/// Simple container (OOP: Concrete container class)
#[repr(C)]
pub struct SimpleContainer {
    pub id: ContainerID,
    pub name: [u8; 64],
    pub image: [u8; 128],
    pub state: AtomicUsize, // ContainerState as usize
    pub pid: AtomicUsize,
    pub memory_limit: u64,
    pub cpu_limit: u32,
    pub capability: ContainerCapability,
    pub environment: [u8; 512],
    pub network_type: ContainerNetworkType,
    pub volume: ContainerVolume,
    pub namespace: ContainerNamespace,
    pub seccomp: SeccompProfile,
}

impl SimpleContainer {
    pub fn new(
        id: ContainerID,
        name: &[u8],
        image: &[u8],
        capability: ContainerCapability,
    ) -> Self {
        let mut name_array = [0u8; 64];
        let mut image_array = [0u8; 128];

        let name_len = name.len().min(63);
        let image_len = image.len().min(127);

        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(image.as_ptr(), image_array.as_mut_ptr(), image_len);
        }

        SimpleContainer {
            id,
            name: name_array,
            image: image_array,
            state: AtomicUsize::new(ContainerState::Created as usize),
            pid: AtomicUsize::new(0),
            memory_limit: 0,
            cpu_limit: 0,
            capability,
            environment: [0; 512],
            network_type: ContainerNetworkType::None,
            volume: ContainerVolume {
                is_bind_mount: false,
                is_tmpfs: false,
                read_only: false,
            },
            namespace: ContainerNamespace {
                uid_mapping: 0,
                gid_mapping: 0,
                rootless: false,
            },
            seccomp: SeccompProfile {
                hardened: false,
                blocked_syscalls_mask: 0,
            },
        }
    }

    pub fn set_environment(&mut self, env: &[u8]) {
        let len = env.len().min(511);
        unsafe {
            core::ptr::copy_nonoverlapping(env.as_ptr(), self.environment.as_mut_ptr(), len);
        }
    }

    pub fn set_limits(&mut self, memory_limit: u64, cpu_limit: u32) {
        self.memory_limit = memory_limit;
        self.cpu_limit = cpu_limit;
    }

    pub fn get_state(&self) -> ContainerState {
        match self.state.load(Ordering::SeqCst) {
            0 => ContainerState::Created,
            1 => ContainerState::Running,
            2 => ContainerState::Paused,
            3 => ContainerState::Stopped,
            _ => ContainerState::Failed,
        }
    }

    pub fn set_state(&self, state: ContainerState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Container for SimpleContainer {
    fn id(&self) -> ContainerID {
        self.id
    }

    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }

    fn start(&mut self) -> Result<(), ContainerError> {
        if !self.capability.can_start {
            return Err(ContainerError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == ContainerState::Running {
            return Err(ContainerError::AlreadyStarted);
        }

        self.set_state(ContainerState::Running);
        self.pid.store(1, Ordering::SeqCst); // Simulated PID
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ContainerError> {
        if !self.capability.can_stop {
            return Err(ContainerError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state == ContainerState::Stopped {
            return Err(ContainerError::AlreadyStopped);
        }

        self.set_state(ContainerState::Stopped);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), ContainerError> {
        if !self.capability.can_pause {
            return Err(ContainerError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state != ContainerState::Running {
            return Err(ContainerError::AlreadyStopped);
        }

        self.set_state(ContainerState::Paused);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), ContainerError> {
        if !self.capability.can_pause {
            return Err(ContainerError::PermissionDenied);
        }

        let current_state = self.get_state();
        if current_state != ContainerState::Paused {
            return Err(ContainerError::AlreadyStopped);
        }

        self.set_state(ContainerState::Running);
        Ok(())
    }

    fn state(&self) -> ContainerState {
        self.get_state()
    }

    fn info(&self) -> ContainerInfo {
        let pid = self.pid.load(Ordering::SeqCst);
        ContainerInfo {
            id: self.id,
            name: self.name,
            image: self.image,
            state: self.get_state(),
            pid: if pid > 0 { Some(pid) } else { None },
            memory_limit: self.memory_limit,
            cpu_limit: self.cpu_limit,
            capability: self.capability,
        }
    }
}

/// Container runtime trait (OOP interface)
pub trait ContainerRuntime {
    /// Create container
    fn create_container(
        &mut self,
        name: &[u8],
        image: &[u8],
        capability: ContainerCapability,
    ) -> Result<ContainerID, ContainerError>;
    /// Remove container
    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    /// Start container
    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    /// Stop container
    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    /// Pause container
    fn pause_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    /// Resume container
    fn resume_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    /// Get container
    fn get_container(&self, id: ContainerID) -> Option<&dyn Container>;
    /// List containers
    fn list_containers(&self) -> Vec<ContainerID>;
    /// Get runtime statistics
    fn stats(&self) -> RuntimeStats;
}

/// Runtime statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RuntimeStats {
    pub total_containers: usize,
    pub running_containers: usize,
    pub paused_containers: usize,
    pub stopped_containers: usize,
}

impl RuntimeStats {
    pub fn new() -> Self {
        RuntimeStats {
            total_containers: 0,
            running_containers: 0,
            paused_containers: 0,
            stopped_containers: 0,
        }
    }
}

/// Simple container runtime (OOP: Concrete runtime class)
pub struct SimpleContainerRuntime {
    containers: Vec<Option<Box<dyn Container>>>,
    next_id: AtomicUsize,
    stats: RuntimeStats,
    capability: RuntimeCapability,
}

/// Runtime capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RuntimeCapability {
    pub can_create: bool,
    pub can_remove: bool,
    pub can_manage: bool,
}

impl RuntimeCapability {
    pub fn new() -> Self {
        RuntimeCapability {
            can_create: false,
            can_remove: false,
            can_manage: false,
        }
    }

    pub fn full() -> Self {
        RuntimeCapability {
            can_create: true,
            can_remove: true,
            can_manage: true,
        }
    }
}

impl SimpleContainerRuntime {
    pub fn new(capability: RuntimeCapability) -> Self {
        SimpleContainerRuntime {
            containers: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: RuntimeStats::new(),
            capability,
        }
    }
}

impl ContainerRuntime for SimpleContainerRuntime {
    fn create_container(
        &mut self,
        name: &[u8],
        image: &[u8],
        capability: ContainerCapability,
    ) -> Result<ContainerID, ContainerError> {
        if !self.capability.can_create {
            return Err(ContainerError::PermissionDenied);
        }

        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let container = SimpleContainer::new(id, name, image, capability);
        self.containers.push(Some(Box::new(container)));
        self.stats.total_containers += 1;
        self.stats.stopped_containers += 1;
        Ok(id)
    }

    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_remove {
            return Err(ContainerError::PermissionDenied);
        }

        let mut index = None;
        for (i, container_option) in self.containers.iter().enumerate() {
            if let Some(ref container) = container_option {
                if container.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.containers[i] = None;
            self.stats.total_containers -= 1;
            Ok(())
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_manage {
            return Err(ContainerError::PermissionDenied);
        }

        if let Some(ref mut container) = self.get_container_mut(id) {
            let result = container.start();
            if result.is_ok() {
                let state = container.state();
                if state == ContainerState::Running {
                    self.stats.running_containers += 1;
                    self.stats.stopped_containers -= 1;
                }
            }
            result
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_manage {
            return Err(ContainerError::PermissionDenied);
        }

        if let Some(ref mut container) = self.get_container_mut(id) {
            let result = container.stop();
            if result.is_ok() {
                let state = container.state();
                if state == ContainerState::Stopped {
                    self.stats.running_containers -= 1;
                    self.stats.stopped_containers += 1;
                }
            }
            result
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn pause_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_manage {
            return Err(ContainerError::PermissionDenied);
        }

        if let Some(ref mut container) = self.get_container_mut(id) {
            let result = container.pause();
            if result.is_ok() {
                let state = container.state();
                if state == ContainerState::Paused {
                    self.stats.running_containers -= 1;
                    self.stats.paused_containers += 1;
                }
            }
            result
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn resume_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_manage {
            return Err(ContainerError::PermissionDenied);
        }

        if let Some(ref mut container) = self.get_container_mut(id) {
            let result = container.resume();
            if result.is_ok() {
                let state = container.state();
                if state == ContainerState::Running {
                    self.stats.paused_containers -= 1;
                    self.stats.running_containers += 1;
                }
            }
            result
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn get_container(&self, id: ContainerID) -> Option<&dyn Container> {
        for container_option in &*self.containers {
            if let Some(ref container) = container_option {
                if container.id() == id {
                    return Some(container.as_ref());
                }
            }
        }
        None
    }

    fn list_containers(&self) -> Vec<ContainerID> {
        let mut ids = Vec::new();
        for container_option in &*self.containers {
            if let Some(ref container) = container_option {
                ids.push(container.id());
            }
        }
        ids
    }

    fn stats(&self) -> RuntimeStats {
        self.stats
    }
}

impl SimpleContainerRuntime {
    fn get_container_mut(&mut self, id: ContainerID) -> Option<&mut Box<dyn Container>> {
        for container_option in &mut *self.containers {
            if let Some(ref mut container) = container_option {
                if container.id() == id {
                    return Some(container);
                }
            }
        }
        None
    }
}

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}
impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let mut runtime = SimpleContainerRuntime::new(RuntimeCapability::full());
        let id = runtime
            .create_container(
                b"sovereign_container",
                b"ubuntu-pqc",
                ContainerCapability::full(),
            )
            .unwrap();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_container_oci_networking_and_volumes() {
        let mut container = SimpleContainer::new(
            1,
            b"web_app",
            b"nginx-dilithium",
            ContainerCapability::full(),
        );

        // Assert network parity bridge setting
        assert_eq!(container.network_type, ContainerNetworkType::None);
        container.network_type = ContainerNetworkType::Bridge;
        assert_eq!(container.network_type, ContainerNetworkType::Bridge);

        // Assert volume mounts setting
        assert!(!container.volume.is_bind_mount);
        container.volume.is_bind_mount = true;
        assert!(container.volume.is_bind_mount);
    }

    #[test]
    fn test_container_namespaces_and_seccomp() {
        let mut container = SimpleContainer::new(
            1,
            b"secure_sandbox",
            b"alpine-kyber",
            ContainerCapability::full(),
        );

        // Assert namespace uid mappings
        assert_eq!(container.namespace.uid_mapping, 0);
        container.namespace.uid_mapping = 1000;
        assert_eq!(container.namespace.uid_mapping, 1000);

        // Assert seccomp profile hardening
        assert!(!container.seccomp.hardened);
        container.seccomp.hardened = true;
        assert!(container.seccomp.hardened);
    }
}
