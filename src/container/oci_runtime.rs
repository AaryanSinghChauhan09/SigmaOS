#![no_std]
#![no_main]

/// OOP-based Container Runtime Support for SigmaOS
/// Based on Ideas-999-Structured: Core System Item 17
/// Implements OCI runtime and sandboxed container primitives

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ContainerID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ContainerState { Created = 0, Running = 1, Paused = 2, Stopped = 3, Deleting = 4 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ContainerError { Success = 0, InvalidConfig = 1, StartFailed = 2, StopFailed = 3, ResourceLimit = 4 }

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
    fn id(&self) -> ContainerID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn state(&self) -> ContainerState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    
    fn start(&mut self) -> Result<(), ContainerError> {
        self.state.store(ContainerState::Running as usize, Ordering::SeqCst);
        self.pid.store(self.id + 1000, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop(&mut self) -> Result<(), ContainerError> {
        self.state.store(ContainerState::Stopped as usize, Ordering::SeqCst);
        self.pid.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn pause(&mut self) -> Result<(), ContainerError> {
        if self.state.load(Ordering::SeqCst) != ContainerState::Running as usize {
            return Err(ContainerError::StartFailed);
        }
        self.state.store(ContainerState::Paused as usize, Ordering::SeqCst);
        Ok(())
    }
    
    fn resume(&mut self) -> Result<(), ContainerError> {
        if self.state.load(Ordering::SeqCst) != ContainerState::Paused as usize {
            return Err(ContainerError::StartFailed);
        }
        self.state.store(ContainerState::Running as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait OCISpec {
    fn create_from_spec(&mut self, spec: &[u8]) -> Result<ContainerID, ContainerError>;
    fn validate_spec(&self, spec: &[u8]) -> Result<(), ContainerError>;
}

#[repr(C)]
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

impl OCISpec for SimpleOCISpec {
    fn create_from_spec(&mut self, spec: &[u8]) -> Result<ContainerID, ContainerError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let name = if spec.len() > 0 { spec } else { b"container" };
        let container = SimpleContainer::new(id, name);
        self.containers.push(Some(Box::new(container)));
        Ok(id)
    }
    
    fn validate_spec(&self, _spec: &[u8]) -> Result<(), ContainerError> {
        Ok(())
    }
}

pub trait Sandbox {
    fn set_namespace(&mut self, container_id: ContainerID, ns_type: Namespace) -> Result<(), ContainerError>;
    fn set_cgroup(&mut self, container_id: ContainerID, cpu_limit: usize, mem_limit: usize) -> Result<(), ContainerError>;
    fn set_seccomp(&mut self, container_id: ContainerID, profile: &[u8]) -> Result<(), ContainerError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Namespace { PID = 0, Network = 1, Mount = 2, IPC = 3, UTS = 4, User = 5 }

#[repr(C)]
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

impl Sandbox for SimpleSandbox {
    fn set_namespace(&mut self, container_id: ContainerID, ns_type: Namespace) -> Result<(), ContainerError> {
        self.namespaces.push((container_id, ns_type));
        Ok(())
    }
    
    fn set_cgroup(&mut self, container_id: ContainerID, cpu_limit: usize, mem_limit: usize) -> Result<(), ContainerError> {
        self.cgroups.push((container_id, (cpu_limit, mem_limit)));
        Ok(())
    }
    
    fn set_seccomp(&mut self, container_id: ContainerID, profile: &[u8]) -> Result<(), ContainerError> {
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

#[repr(C)]
pub struct SimpleImageManager {
    pub images: Vec<([u8; 128], [u8; 32])>,
}

impl SimpleImageManager {
    pub fn new() -> Self {
        SimpleImageManager {
            images: Vec::new(),
        }
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

pub trait ContainerRuntime {
    fn create_container(&mut self, name: &[u8], image: &[u8]) -> Result<ContainerID, ContainerError>;
    fn start_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn stop_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn remove_container(&mut self, id: ContainerID) -> Result<(), ContainerError>;
    fn list_containers(&self) -> Vec<ContainerID>;
}

#[repr(C)]
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

impl ContainerRuntime for SimpleContainerRuntime {
    fn create_container(&mut self, name: &[u8], image: &[u8]) -> Result<ContainerID, ContainerError> {
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clone(&self) -> Vec<T> {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                let item = core::ptr::read(self.data.add(i));
                new_vec.push(item);
            }
        }
        new_vec
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
