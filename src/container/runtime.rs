#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

use alloc::string::String;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

/// OOP-based Container Runtime for SigmaOS
/// Implements container runtime using OOP principles with traits and structs
/// No dependency on external container frameworks

/// Container ID
pub type ContainerID = usize;

/// Container state
#[repr(usize)]
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

impl ContainerNamespace {
    pub fn map_uid(&self, container_uid: u32) -> Result<u32, &'static str> {
        if self.rootless {
            if container_uid == 0 {
                Ok(self.uid_mapping)
            } else {
                Ok(self.uid_mapping + container_uid)
            }
        } else {
            Ok(container_uid)
        }
    }

    pub fn map_gid(&self, container_gid: u32) -> Result<u32, &'static str> {
        if self.rootless {
            if container_gid == 0 {
                Ok(self.gid_mapping)
            } else {
                Ok(self.gid_mapping + container_gid)
            }
        } else {
            Ok(container_gid)
        }
    }
}

/// Container seccomp profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeccompProfile {
    pub hardened: bool,
    pub blocked_syscalls_mask: u32,
}

/// Namespace configuration flags for a container
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespaceConfig {
    pub pid: bool,
    pub mnt: bool,
    pub net: bool,
    pub uts: bool,
    pub ipc: bool,
    pub user: bool,
    pub cgroup: bool,
}

impl NamespaceConfig {
    pub fn new() -> Self {
        NamespaceConfig {
            pid: false,
            mnt: false,
            net: false,
            uts: false,
            ipc: false,
            user: false,
            cgroup: false,
        }
    }

    pub fn all(&self) -> bool {
        self.pid && self.mnt && self.net && self.uts && self.ipc && self.user && self.cgroup
    }
}

/// Container seccomp profiles

impl SeccompProfile {
    pub fn is_syscall_blocked(&self, syscall_id: u32) -> bool {
        if !self.hardened {
            return false;
        }
        if syscall_id < 32 {
            (self.blocked_syscalls_mask & (1 << syscall_id)) != 0
        } else {
            false
        }
    }
}

/// Linux OverlayFS Layer Stacking (Ubuntu/Debian-style overlay)
#[derive(Debug, Clone)]
pub struct OverlayFS {
    pub lower_dirs: alloc::vec::Vec<String>,
    pub upper_dir: String,
    pub work_dir: String,
    pub mounted: bool,
}

impl OverlayFS {
    pub fn new(lower_dirs: alloc::vec::Vec<String>, upper_dir: String, work_dir: String) -> Self {
        Self {
            lower_dirs,
            upper_dir,
            work_dir,
            mounted: false,
        }
    }

    pub fn mount(&mut self) -> Result<(), &'static str> {
        if self.lower_dirs.is_empty() {
            return Err("OverlayFS mount failed: lower_dirs cannot be empty");
        }
        if self.upper_dir.is_empty() || self.work_dir.is_empty() {
            return Err("OverlayFS mount failed: upper_dir and work_dir must be specified");
        }
        self.mounted = true;
        println!(
            "OverlayFS mounted successfully: lowerdirs={:?}, upperdir={}, workdir={}",
            self.lower_dirs, self.upper_dir, self.work_dir
        );
        Ok(())
    }

    pub fn umount(&mut self) {
        self.mounted = false;
        println!("OverlayFS unmounted successfully.");
    }
}

/// Simple container (OOP: Concrete container class)
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
    pub seccomp: SeccompProfile,
}

impl SimpleContainer {
    pub fn execute_syscall(&self, syscall_id: u32) -> Result<(), ContainerError> {
        if self.seccomp.is_syscall_blocked(syscall_id) {
            println!(
                "Container Seccomp Violation: Syscall {} is strictly prohibited by security profile",
                syscall_id
            );
            return Err(ContainerError::PermissionDenied);
        }
        Ok(())
    }

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
            seccomp: SeccompProfile { hardened: false, blocked_syscalls_mask: 0 },
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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            if let Some(ref container) = *container_option {
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
        for container_option in &self.containers {
            if let Some(ref container) = *container_option {
                if container.id() == id {
                    return Some(container.as_ref());
                }
            }
        }
        None
    }

    fn list_containers(&self) -> Vec<ContainerID> {
        let mut ids = Vec::new();
        for container_option in &self.containers {
            if let Some(ref container) = *container_option {
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
        for container_option in &mut self.containers {
            if let Some(ref mut container) = *container_option {
                if container.id() == id {
                    return Some(container);
                }
            }
        }
        None
    }
}

/// Simple Vec implementation for no_std
pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

