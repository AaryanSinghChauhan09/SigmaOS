#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::kernel::vfs::inode::FsError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    NotFound,
    AlreadyExists,
    InvalidConfig,
    ResourceLimit,
    CapabilityDenied,
}

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

pub struct NamespaceSet {
    pub pidns: Option<usize>,
    pub mntns: Option<usize>,
    pub netns: Option<usize>,
    pub utsns: Option<usize>,
    pub ipcns: Option<usize>,
    pub userns: Option<usize>,
    pub cgroupns: Option<usize>,
}

impl NamespaceSet {
    pub fn new() -> Self {
        NamespaceSet {
            pidns: None,
            mntns: None,
            netns: None,
            utsns: None,
            ipcns: None,
            userns: None,
            cgroupns: None,
        }
    }

    pub fn clone(&self) -> Self {
        NamespaceSet {
            pidns: self.pidns,
            mntns: self.mntns,
            netns: self.netns,
            utsns: self.utsns,
            ipcns: self.ipcns,
            userns: self.userns,
            cgroupns: self.cgroupns,
        }
    }
}

pub struct OciSpec {
    pub version: String,
    pub platform: String,
    pub process: OciProcess,
    pub mounts: Vec<OciMount>,
}

pub struct OciProcess {
    pub args: Vec<String>,
    pub env: Vec<String>,
    pub cwd: String,
    pub user: OciUser,
    pub capabilities: Vec<String>,
    pub rlimits: Vec<OciRlimit>,
    pub no_new_privileges: bool,
}

pub struct OciUser {
    pub uid: u32,
    pub gid: u32,
    pub additional_gids: Vec<u32>,
}

pub struct OciRlimit {
    pub rlimit_type: String,
    pub soft: u64,
    pub hard: u64,
}

pub struct OciMount {
    pub destination: String,
    pub r#type: String,
    pub source: String,
    pub options: Vec<String>,
}

pub enum ContainerState {
    Created,
    Running,
    Paused,
    Stopped,
    Deleted,
}

pub struct Container {
    pub id: String,
    pub bundle: String,
    pub config: OciSpec,
    pub image: String,
    pub state: ContainerState,
    pub pid: Option<u64>,
    pub rootfs: String,
    pub layers: Vec<String>,
    pub namespaces: NamespaceConfig,
}

impl Container {
    pub fn new(id: &str, bundle: &str) -> Self {
        Container {
            id: id.to_string(),
            bundle: bundle.to_string(),
            config: OciSpec {
                version: String::new(),
                platform: String::new(),
                process: OciProcess {
                    args: Vec::new(),
                    env: Vec::new(),
                    cwd: String::from("/"),
                    user: OciUser { uid: 0, gid: 0, additional_gids: Vec::new() },
                    capabilities: Vec::new(),
                    rlimits: Vec::new(),
                    no_new_privileges: false,
                },
                mounts: Vec::new(),
            },
            image: String::new(),
            state: ContainerState::Created,
            pid: None,
            rootfs: String::new(),
            layers: Vec::new(),
            namespaces: NamespaceConfig::new(),
        }
    }
}

pub trait Runtime: Send + Sync {
    fn create(&mut self, container: &mut Container) -> Result<(), ContainerError>;
    fn start(&mut self, container: &mut Container) -> Result<(), ContainerError>;
    fn kill(&mut self, container: &mut Container, signal: i32) -> Result<(), ContainerError>;
    fn delete(&mut self, container: &mut Container) -> Result<(), ContainerError>;
    fn pause(&mut self, container: &mut Container) -> Result<(), ContainerError>;
    fn resume(&mut self, container: &mut Container) -> Result<(), ContainerError>;
    fn exec(&mut self, container: &mut Container, args: &[String]) -> Result<(), ContainerError>;
    fn state(&self, container: &Container) -> Result<ContainerState, ContainerError>;
    fn update(&mut self, container: &mut Container, resources: &ResourceConfig) -> Result<(), ContainerError>;
}

pub struct ResourceConfig {
    pub cpu_quota: i64,
    pub cpu_period: u64,
    pub cpu_shares: u64,
    pub memory_limit: u64,
    pub memory_swap: u64,
    pub pids_limit: i64,
    pub blkio_weight: u16,
    pub io_max_bytes: u64,
    pub io_read_bps: u64,
    pub io_write_bps: u64,
}

pub struct ContainerManager {
    pub containers: Vec<Container>,
    pub runtime: Box<dyn Runtime>,
}

impl ContainerManager {
    pub fn new(runtime: Box<dyn Runtime>) -> Self {
        ContainerManager {
            containers: Vec::new(),
            runtime,
        }
    }

    pub fn create(&mut self, container: &mut Container) -> Result<(), ContainerError> {
        self.runtime.create(container)?;
        self.containers.push(container.clone());
        Ok(())
    }

    pub fn start(&mut self, id: &str) -> Result<(), ContainerError> {
        if let Some(container) = self.containers.iter_mut().find(|c| c.id == id) {
            self.runtime.start(container)?;
            Ok(())
        } else {
            Err(ContainerError::NotFound)
        }
    }

    pub fn stop(&mut self, id: &str) -> Result<(), ContainerError> {
        if let Some(container) = self.containers.iter_mut().find(|c| c.id == id) {
            self.runtime.kill(container, 15)?;
            Ok(())
        } else {
            Err(ContainerError::NotFound)
        }
    }

    pub fn remove(&mut self, id: &str) -> Result<(), ContainerError> {
        if let Some(container) = self.containers.iter_mut().find(|c| c.id == id) {
            self.runtime.delete(container)?;
            self.containers.retain(|c| c.id != id);
            Ok(())
        } else {
            Err(ContainerError::NotFound)
        }
    }

    pub fn list(&self) -> &[Container] {
        &self.containers
    }
}