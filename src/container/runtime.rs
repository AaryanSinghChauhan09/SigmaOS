// OOP-based Container Runtime for SigmaOS
// Implements container runtime using OOP principles with traits and structs.

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ContainerID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
    Failed = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Default for ContainerCapability {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
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

pub trait Container {
    fn id(&self) -> ContainerID;
    fn name(&self) -> &[u8];
    fn start(&mut self) -> Result<(), ContainerError>;
    fn stop(&mut self) -> Result<(), ContainerError>;
    fn pause(&mut self) -> Result<(), ContainerError>;
    fn resume(&mut self) -> Result<(), ContainerError>;
    fn state(&self) -> ContainerState;
    fn info(&self) -> ContainerInfo;
}

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
}

impl Default for NamespaceConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeccompProfile {
    pub hardened: bool,
    pub blocked_syscalls_mask: u32,
}

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

#[derive(Debug, Clone)]
pub struct OverlayFS {
    pub lower_dirs: Vec<String>,
    pub upper_dir: String,
    pub work_dir: String,
    pub mounted: bool,
}

impl OverlayFS {
    pub fn new(lower_dirs: Vec<String>, upper_dir: String, work_dir: String) -> Self {
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
        Ok(())
    }

    pub fn umount(&mut self) {
        self.mounted = false;
    }
}

pub struct SimpleContainer {
    pub id: ContainerID,
    pub name: [u8; 64],
    pub image: [u8; 128],
    pub state: AtomicUsize,
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

        name_array[..name_len].copy_from_slice(&name[..name_len]);
        image_array[..image_len].copy_from_slice(&image[..image_len]);

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

    pub fn get_state(&self) -> ContainerState {
        match self.state.load(Ordering::SeqCst) {
            1 => ContainerState::Running,
            2 => ContainerState::Paused,
            3 => ContainerState::Stopped,
            4 => ContainerState::Failed,
            _ => ContainerState::Created,
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
        self.pid.store(1, Ordering::SeqCst);
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

pub trait ContainerRuntime {
    fn create_container(
        &mut self,
        name: &[u8],
        image: &[u8],
        capability: ContainerCapability,
    ) -> Result<ContainerID, ContainerError>;
    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn get_container(&self, id: ContainerID) -> Option<&dyn Container>;
    fn list_containers(&self) -> Vec<ContainerID>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeStats {
    pub total_containers: usize,
    pub running_containers: usize,
    pub paused_containers: usize,
    pub stopped_containers: usize,
}

pub struct RuntimeCapability {
    pub can_create: bool,
    pub can_remove: bool,
    pub can_manage: bool,
}

impl RuntimeCapability {
    pub fn full() -> Self {
        RuntimeCapability {
            can_create: true,
            can_remove: true,
            can_manage: true,
        }
    }
}

pub struct SimpleContainerRuntime {
    containers: Vec<Option<Box<dyn Container>>>,
    next_id: AtomicUsize,
    capability: RuntimeCapability,
}

impl SimpleContainerRuntime {
    pub fn new(capability: RuntimeCapability) -> Self {
        SimpleContainerRuntime {
            containers: Vec::new(),
            next_id: AtomicUsize::new(1),
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
        Ok(id)
    }

    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if !self.capability.can_remove {
            return Err(ContainerError::PermissionDenied);
        }

        if let Some(pos) = self.containers.iter().position(|c| match c {
            Some(cont) => cont.id() == id,
            None => false,
        }) {
            self.containers.remove(pos);
            Ok(())
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if let Some(pos) = self.containers.iter().position(|c| match c {
            Some(cont) => cont.id() == id,
            None => false,
        }) {
            if let Some(ref mut container) = self.containers[pos] {
                container.start()
            } else {
                Err(ContainerError::PermissionDenied)
            }
        } else {
            Err(ContainerError::PermissionDenied)
        }
    }

    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        if let Some(pos) = self.containers.iter().position(|c| match c {
            Some(cont) => cont.id() == id,
            None => false,
        }) {
            if let Some(ref mut container) = self.containers[pos] {
                container.stop()
            } else {
                Err(ContainerError::PermissionDenied)
            }
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
    fn test_overlayfs_stacking() {
        let mut overlay = OverlayFS::new(
            vec!["/lower1".to_string(), "/lower2".to_string()],
            "/upper".to_string(),
            "/work".to_string(),
        );
        assert!(!overlay.mounted);
        assert!(overlay.mount().is_ok());
        assert!(overlay.mounted);
        overlay.umount();
        assert!(!overlay.mounted);
    }

    #[test]
    fn test_rootless_user_namespace_mapping() {
        let ns = ContainerNamespace {
            uid_mapping: 1000,
            gid_mapping: 1000,
            rootless: true,
        };

        assert_eq!(ns.map_uid(0).unwrap(), 1000);
        assert_eq!(ns.map_gid(0).unwrap(), 1000);
        assert_eq!(ns.map_uid(10).unwrap(), 1010);
    }

    #[test]
    fn test_hardened_seccomp_syscall_filtering() {
        let mut container = SimpleContainer::new(
            1,
            b"hardened_ct",
            b"alpine",
            ContainerCapability::full(),
        );
        container.seccomp = SeccompProfile {
            hardened: true,
            blocked_syscalls_mask: 1 << 0,
        };

        assert!(container.execute_syscall(1).is_ok());
        assert_eq!(
            container.execute_syscall(0).unwrap_err(),
            ContainerError::PermissionDenied
        );
    }
}
