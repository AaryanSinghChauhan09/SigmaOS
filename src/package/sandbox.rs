#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

use core::mem;
/// OOP-based Package Sandboxing for SigmaOS
/// Based on Ideas-999-Structured: Package, Build & Reproducibility Item 28
/// Implements isolated environments for package builds
use core::sync::atomic::{AtomicUsize, Ordering};

pub type SandboxID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum SandboxState {
    Created = 0,
    Running = 1,
    Stopped = 2,
    Failed = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SandboxError {
    Success = 0,
    CreateFailed = 1,
    StartFailed = 2,
    ResourceLimit = 3,
}

pub trait BuildSandbox {
    fn id(&self) -> SandboxID;
    fn state(&self) -> SandboxState;
    fn create(&mut self) -> Result<(), SandboxError>;
    fn start(&mut self) -> Result<(), SandboxError>;
    fn stop(&mut self) -> Result<(), SandboxError>;
    fn execute_command(&mut self, command: &[u8]) -> Result<(), SandboxError>;
}

#[repr(C)]
pub struct SimpleBuildSandbox {
    pub id: SandboxID,
    pub state: AtomicUsize,
    pub rootfs: [u8; 256],
    pub memory_limit: AtomicUsize,
    pub cpu_limit: AtomicUsize,
}

impl SimpleBuildSandbox {
    pub fn new(id: SandboxID, rootfs: &[u8]) -> Self {
        let mut rootfs_array = [0u8; 256];
        let rootfs_len = rootfs.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(rootfs.as_ptr(), rootfs_array.as_mut_ptr(), rootfs_len);
        }
        SimpleBuildSandbox {
            id,
            state: AtomicUsize::new(SandboxState::Created as usize),
            rootfs: rootfs_array,
            memory_limit: AtomicUsize::new(1024 * 1024 * 1024),
            cpu_limit: AtomicUsize::new(2),
        }
    }
}

impl BuildSandbox for SimpleBuildSandbox {
    fn id(&self) -> SandboxID {
        self.id
    }
    fn state(&self) -> SandboxState {
        match self.state.load(Ordering::SeqCst) {
            0 => SandboxState::Created,
            1 => SandboxState::Running,
            2 => SandboxState::Stopped,
            _ => SandboxState::Failed,
        }
    }

    fn create(&mut self) -> Result<(), SandboxError> {
        self.state
            .store(SandboxState::Created as usize, Ordering::SeqCst);
        Ok(())
    }

    fn start(&mut self) -> Result<(), SandboxError> {
        self.state
            .store(SandboxState::Running as usize, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), SandboxError> {
        self.state
            .store(SandboxState::Stopped as usize, Ordering::SeqCst);
        Ok(())
    }

    fn execute_command(&mut self, _command: &[u8]) -> Result<(), SandboxError> {
        if self.state.load(Ordering::SeqCst) != SandboxState::Running as usize {
            return Err(SandboxError::StartFailed);
        }
        Ok(())
    }
}

pub trait NetworkIsolation {
    fn enable_network(&mut self, enabled: bool);
    fn set_allowed_hosts(&mut self, hosts: Vec<[u8; 128]>);
    fn is_network_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNetworkIsolation {
    pub network_enabled: AtomicUsize,
    pub allowed_hosts: Vec<[u8; 128]>,
}

impl SimpleNetworkIsolation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleNetworkIsolation {
            network_enabled: AtomicUsize::new(0),
            allowed_hosts: Vec::new(),
        }
    }
}

impl NetworkIsolation for SimpleNetworkIsolation {
    fn enable_network(&mut self, enabled: bool) {
        self.network_enabled
            .store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
    }

    fn set_allowed_hosts(&mut self, hosts: Vec<[u8; 128]>) {
        self.allowed_hosts = hosts;
    }

    fn is_network_enabled(&self) -> bool {
        self.network_enabled.load(Ordering::SeqCst) == 1
    }
}

pub trait FilesystemIsolation {
    fn bind_mount(&mut self, source: &[u8], target: &[u8]) -> Result<(), SandboxError>;
    fn set_readonly(&mut self, path: &[u8], readonly: bool) -> Result<(), SandboxError>;
    fn create_tmpfs(&mut self, path: &[u8], size_mb: usize) -> Result<(), SandboxError>;
}

#[repr(C)]
pub struct SimpleFilesystemIsolation {
    pub mounts: Vec<([u8; 256], [u8; 256], AtomicUsize)>,
}

impl SimpleFilesystemIsolation {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFilesystemIsolation { mounts: Vec::new() }
    }
}

impl FilesystemIsolation for SimpleFilesystemIsolation {
    fn bind_mount(&mut self, source: &[u8], target: &[u8]) -> Result<(), SandboxError> {
        let mut source_array = [0u8; 256];
        let mut target_array = [0u8; 256];
        let source_len = source.len().min(255);
        let target_len = target.len().min(255);

        for i in 0..source_len {
            source_array[i] = source[i];
        }
        for i in 0..target_len {
            target_array[i] = target[i];
        }

        self.mounts
            .push((source_array, target_array, AtomicUsize::new(0)));
        Ok(())
    }

    fn set_readonly(&mut self, path: &[u8], readonly: bool) -> Result<(), SandboxError> {
        for mount in &mut self.mounts {
            let target = &mount.1;
            let len = target.iter().position(|&b| b == 0).unwrap_or(256);
            if &target[..len] == path {
                mount
                    .2
                    .store(if readonly { 1 } else { 0 }, Ordering::SeqCst);
                return Ok(());
            }
        }
        Err(SandboxError::CreateFailed)
    }

    fn create_tmpfs(&mut self, path: &[u8], _size_mb: usize) -> Result<(), SandboxError> {
        let mut path_array = [0u8; 256];
        let path_len = path.len().min(255);
        for i in 0..path_len {
            path_array[i] = path[i];
        }
        self.mounts
            .push((path_array, [0u8; 256], AtomicUsize::new(1)));
        Ok(())
    }
}

pub trait SandboxManager {
    fn create_sandbox(&mut self, rootfs: &[u8]) -> Result<SandboxID, SandboxError>;
    fn destroy_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError>;
    fn get_sandbox(&self, id: SandboxID) -> Option<&dyn BuildSandbox>;
    fn list_sandboxes(&self) -> Vec<SandboxID>;
}

#[repr(C)]
pub struct SimpleSandboxManager {
    pub sandboxes: Vec<Option<SimpleBuildSandbox>>,
    pub next_id: AtomicUsize,
}

impl SimpleSandboxManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleSandboxManager {
            sandboxes: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SandboxManager for SimpleSandboxManager {
    fn create_sandbox(&mut self, rootfs: &[u8]) -> Result<SandboxID, SandboxError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let sandbox = SimpleBuildSandbox::new(id, rootfs);
        self.sandboxes.push(Some(sandbox));
        Ok(id)
    }

    fn destroy_sandbox(&mut self, id: SandboxID) -> Result<(), SandboxError> {
        for sandbox_option in &mut self.sandboxes {
            if let Some(ref sandbox) = *sandbox_option {
                if sandbox.id == id {
                    *sandbox_option = None;
                    return Ok(());
                }
            }
        }
        Err(SandboxError::CreateFailed)
    }

    fn get_sandbox(&self, id: SandboxID) -> Option<&dyn BuildSandbox> {
        for sandbox_option in &self.sandboxes {
            if let Some(ref sandbox) = *sandbox_option {
                if sandbox.id == id {
                    return Some(sandbox as &dyn BuildSandbox);
                }
            }
        }
        None
    }

    fn list_sandboxes(&self) -> Vec<SandboxID> {
        let mut ids = Vec::new();
        for sandbox_option in &self.sandboxes {
            if let Some(ref sandbox) = *sandbox_option {
                ids.push(sandbox.id);
            }
        }
        ids
    }
}

pub trait ResourceQuota {
    fn set_memory_quota(&mut self, sandbox_id: SandboxID, bytes: usize)
        -> Result<(), SandboxError>;
    fn set_cpu_quota(&mut self, sandbox_id: SandboxID, cores: usize) -> Result<(), SandboxError>;
    fn set_disk_quota(&mut self, sandbox_id: SandboxID, bytes: usize) -> Result<(), SandboxError>;
}

#[repr(C)]
pub struct SimpleResourceQuota {
    pub manager: SimpleSandboxManager,
}

impl SimpleResourceQuota {
    pub fn new(manager: SimpleSandboxManager) -> Self {
        SimpleResourceQuota { manager }
    }
}

impl ResourceQuota for SimpleResourceQuota {
    fn set_memory_quota(
        &mut self,
        sandbox_id: SandboxID,
        bytes: usize,
    ) -> Result<(), SandboxError> {
        for sandbox_option in &mut self.manager.sandboxes {
            if let Some(ref mut sandbox) = *sandbox_option {
                if sandbox.id == sandbox_id {
                    sandbox.memory_limit.store(bytes, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SandboxError::CreateFailed)
    }

    fn set_cpu_quota(&mut self, sandbox_id: SandboxID, cores: usize) -> Result<(), SandboxError> {
        for sandbox_option in &mut self.manager.sandboxes {
            if let Some(ref mut sandbox) = *sandbox_option {
                if sandbox.id == sandbox_id {
                    sandbox.cpu_limit.store(cores, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SandboxError::CreateFailed)
    }

    fn set_disk_quota(
        &mut self,
        _sandbox_id: SandboxID,
        _bytes: usize,
    ) -> Result<(), SandboxError> {
        Ok(())
    }
}
