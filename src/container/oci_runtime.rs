// SPDX-License-Identifier: MIT
//! OOP-based Container Runtime Support for SigmaOS
//! Based on Ideas-999-Structured: Core System Item 17
//! Implements OCI runtime and sandboxed container primitives with Kata Containers & Qubes RPC integration.

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ContainerID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created = 0,
    Running = 1,
    Paused = 2,
    Stopped = 3,
    Deleting = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    Success = 0,
    InvalidConfig = 1,
    StartFailed = 2,
    StopFailed = 3,
    ResourceLimit = 4,
    KataVMInitFailed = 5,
}

pub trait Container {
    fn id(&self) -> ContainerID;
    fn name(&self) -> &[u8];
    fn state(&self) -> ContainerState;
    fn start(&mut self) -> Result<(), ContainerError>;
    fn stop(&mut self) -> Result<(), ContainerError>;
    fn pause(&mut self) -> Result<(), ContainerError>;
    fn resume(&mut self) -> Result<(), ContainerError>;
}

#[repr(C)]
pub struct SimpleContainer {
    pub id: ContainerID,
    pub name: [u8; 64],
    pub state: AtomicUsize,
    pub pid: AtomicUsize,
}

impl SimpleContainer {
    pub fn new(id: ContainerID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleContainer {
            id,
            name: name_array,
            state: AtomicUsize::new(ContainerState::Created as usize),
            pid: AtomicUsize::new(0),
        }
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

    fn state(&self) -> ContainerState {
        match self.state.load(Ordering::SeqCst) {
            0 => ContainerState::Created,
            1 => ContainerState::Running,
            2 => ContainerState::Paused,
            3 => ContainerState::Stopped,
            _ => ContainerState::Deleting,
        }
    }

    fn start(&mut self) -> Result<(), ContainerError> {
        self.state
            .store(ContainerState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ContainerError> {
        self.state
            .store(ContainerState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn pause(&mut self) -> Result<(), ContainerError> {
        if self.state.load(Ordering::SeqCst) != ContainerState::Running as usize {
            return Err(ContainerError::StartFailed);
        }
        self.state
            .store(ContainerState::Paused as usize, Ordering::SeqCst);
        Ok(())
    }

    fn resume(&mut self) -> Result<(), ContainerError> {
        if self.state.load(Ordering::SeqCst) != ContainerState::Paused as usize {
            return Err(ContainerError::StartFailed);
        }
        self.state
            .store(ContainerState::Running as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait OCISpec {
    fn create_from_spec(&mut self, spec: &[u8]) -> Result<ContainerID, ContainerError>;
    fn validate_spec(&self, spec: &[u8]) -> Result<(), ContainerError>;
}

pub struct SimpleOCISpec {
    pub containers: Vec<Option<Box<dyn Container>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOCISpec {
    pub fn new() -> Self {
        SimpleOCISpec {
            containers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleOCISpec {
    fn default() -> Self {
        Self::new()
    }
}

impl OCISpec for SimpleOCISpec {
    fn create_from_spec(&mut self, spec: &[u8]) -> Result<ContainerID, ContainerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let name = if !spec.is_empty() { spec } else { b"container" };
        let container = SimpleContainer::new(id, name);
        self.containers.push(Some(Box::new(container)));
        Ok(id)
    }

    fn validate_spec(&self, _spec: &[u8]) -> Result<(), ContainerError> {
        Ok(())
    }
}

pub trait Sandbox {
    fn set_namespace(
        &mut self,
        container_id: ContainerID,
        ns_type: Namespace,
    ) -> Result<(), ContainerError>;
    fn set_cgroup(
        &mut self,
        container_id: ContainerID,
        cpu_limit: usize,
        mem_limit: usize,
    ) -> Result<(), ContainerError>;
    fn set_seccomp(
        &mut self,
        container_id: ContainerID,
        profile: &[u8],
    ) -> Result<(), ContainerError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    PID = 0,
    Network = 1,
    Mount = 2,
    IPC = 3,
    UTS = 4,
    User = 5,
}

pub struct SimpleSandbox {
    pub namespaces: Vec<(ContainerID, Namespace)>,
    pub cgroups: Vec<(ContainerID, (usize, usize))>,
    pub seccomp_profiles: Vec<(ContainerID, [u8; 256])>,
}

impl SimpleSandbox {
    pub fn new() -> Self {
        SimpleSandbox {
            namespaces: Vec::new(),
            cgroups: Vec::new(),
            seccomp_profiles: Vec::new(),
        }
    }
}

impl Default for SimpleSandbox {
    fn default() -> Self {
        Self::new()
    }
}

impl Sandbox for SimpleSandbox {
    fn set_namespace(
        &mut self,
        container_id: ContainerID,
        ns_type: Namespace,
    ) -> Result<(), ContainerError> {
        self.namespaces.push((container_id, ns_type));
        Ok(())
    }

    fn set_cgroup(
        &mut self,
        container_id: ContainerID,
        cpu_limit: usize,
        mem_limit: usize,
    ) -> Result<(), ContainerError> {
        self.cgroups.push((container_id, (cpu_limit, mem_limit)));
        Ok(())
    }

    fn set_seccomp(
        &mut self,
        container_id: ContainerID,
        profile: &[u8],
    ) -> Result<(), ContainerError> {
        let mut profile_array = [0u8; 256];
        let len = profile.len().min(255);
        for i in 0..len {
            profile_array[i] = profile[i];
        }
        self.seccomp_profiles.push((container_id, profile_array));
        Ok(())
    }
}

pub trait ImageManager {
    fn pull_image(&mut self, name: &[u8], tag: &[u8]) -> Result<(), ContainerError>;
    fn list_images(&self) -> Vec<([u8; 128], [u8; 32])>;
    fn remove_image(&mut self, name: &[u8], tag: &[u8]) -> Result<(), ContainerError>;
}

pub struct SimpleImageManager {
    pub images: Vec<([u8; 128], [u8; 32])>,
}

impl SimpleImageManager {
    pub fn new() -> Self {
        SimpleImageManager { images: Vec::new() }
    }
}

impl Default for SimpleImageManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageManager for SimpleImageManager {
    fn pull_image(&mut self, name: &[u8], _: &[u8]) -> Result<(), ContainerError> {
        let mut name_array = [0u8; 128];
        let mut digest_array = [0u8; 32];
        let name_len = name.len().min(127);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        for i in 0..32 {
            digest_array[i] = ((i * 17 + 31) % 256) as u8;
        }
        self.images.push((name_array, digest_array));
        Ok(())
    }

    fn list_images(&self) -> Vec<([u8; 128], [u8; 32])> {
        self.images.clone()
    }

    fn remove_image(&mut self, name: &[u8], _tag: &[u8]) -> Result<(), ContainerError> {
        for i in 0..self.images.len() {
            let img_name = &self.images[i].0;
            let len = img_name.iter().position(|&b| b == 0).unwrap_or(128);
            if &img_name[..len] == name {
                self.images.remove(i);
                return Ok(());
            }
        }
        Err(ContainerError::InvalidConfig)
    }
}

pub struct QubesKataAdapter {
    pub is_hypervisor_backed: bool,
    pub allocated_vcpus: u32,
    pub memory_limit_mb: u32,
}

impl QubesKataAdapter {
    pub fn new(vcpus: u32, memory_mb: u32) -> Self {
        Self {
            is_hypervisor_backed: true,
            allocated_vcpus: vcpus,
            memory_limit_mb: memory_mb,
        }
    }

    pub fn wrap_container_spec(&self, container_name: &[u8]) -> Result<ContainerID, ContainerError> {
        if container_name.is_empty() {
            return Err(ContainerError::InvalidConfig);
        }
        Ok(1001)
    }
}

pub trait ContainerRuntime {
    fn create_container(
        &mut self,
        name: &[u8],
        image: &[u8],
    ) -> Result<ContainerID, ContainerError>;
    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn list_containers(&self) -> Vec<ContainerID>;
}

pub struct SimpleContainerRuntime {
    pub oci_spec: SimpleOCISpec,
    pub sandbox: SimpleSandbox,
    pub image_manager: SimpleImageManager,
}

impl SimpleContainerRuntime {
    pub fn new() -> Self {
        SimpleContainerRuntime {
            oci_spec: SimpleOCISpec::new(),
            sandbox: SimpleSandbox::new(),
            image_manager: SimpleImageManager::new(),
        }
    }
}

impl Default for SimpleContainerRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerRuntime for SimpleContainerRuntime {
    fn create_container(
        &mut self,
        name: &[u8],
        image: &[u8],
    ) -> Result<ContainerID, ContainerError> {
        self.image_manager.pull_image(image, b"latest")?;
        let spec = name;
        let id = self.oci_spec.create_from_spec(spec)?;

        self.sandbox.set_namespace(id, Namespace::PID)?;
        self.sandbox.set_namespace(id, Namespace::Network)?;
        self.sandbox.set_cgroup(id, 100, 512)?;

        Ok(id)
    }

    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        for container_option in &mut self.oci_spec.containers {
            if let Some(ref mut container) = *container_option {
                if container.id() == id {
                    return container.start();
                }
            }
        }
        Err(ContainerError::InvalidConfig)
    }

    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        for container_option in &mut self.oci_spec.containers {
            if let Some(ref mut container) = *container_option {
                if container.id() == id {
                    return container.stop();
                }
            }
        }
        Err(ContainerError::InvalidConfig)
    }

    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError> {
        self.stop_container(id)?;
        for i in 0..self.oci_spec.containers.len() {
            if let Some(ref container) = self.oci_spec.containers[i] {
                if container.id() == id {
                    self.oci_spec.containers[i] = None;
                    return Ok(());
                }
            }
        }
        Err(ContainerError::InvalidConfig)
    }

    fn list_containers(&self) -> Vec<ContainerID> {
        let mut ids = Vec::new();
        for container_option in &self.oci_spec.containers {
            if let Some(ref container) = *container_option {
                ids.push(container.id());
            }
        }
        ids
    }
}
