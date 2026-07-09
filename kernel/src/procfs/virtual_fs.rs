// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired procfs/sysfs virtual filesystems for SigmaOS
// Zero-allocation, performance-optimized virtual filesystem operations

/// Virtual filesystem entry
pub struct VfsEntry {
    pub name: String,
    pub entry_type: VfsEntryType,
    pub permissions: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub read_fn: Option<ReadFn>,
    pub write_fn: Option<WriteFn>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsEntryType {
    RegularFile,
    Directory,
    SymbolicLink,
}

pub type ReadFn = fn() -> Vec<u8>;
pub type WriteFn = fn(&[u8]) -> Result<(), VfsError>;

impl VfsEntry {
    pub fn new(name: &str, entry_type: VfsEntryType) -> Self {
        Self {
            name: name.to_string(),
            entry_type,
            permissions: 0o644,
            uid: 0,
            gid: 0,
            size: 0,
            read_fn: None,
            write_fn: None,
        }
    }
}

/// Virtual filesystem error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VfsError {
    EntryNotFound,
    PermissionDenied,
    InvalidOperation,
    ReadOnly,
    WriteOnly,
    Other,
}

/// Procfs entry
pub struct ProcfsEntry {
    pub pid: u32,
    pub name: String,
    pub entry_type: ProcfsEntryType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcfsEntryType {
    Status,
    Cmdline,
    Environ,
    Maps,
    Fd,
    Stat,
    Statm,
    Meminfo,
    Cpuinfo,
    Loadavg,
    Uptime,
    Version,
    Mounts,
    Filesystems,
    Devices,
    Interrupts,
    Softirqs,
    Self,
    ThreadSelf,
    Net,
    Sys,
}

impl ProcfsEntry {
    pub fn new(pid: u32, name: &str, entry_type: ProcfsEntryType) -> Self {
        Self {
            pid,
            name: name.to_string(),
            entry_type,
            data: Vec::new(),
        }
    }
}

/// Procfs directory structure
pub struct ProcfsDirectory {
    pub pid: u32,
    pub entries: Vec<ProcfsEntry>,
}

impl ProcfsDirectory {
    pub const fn new(pid: u32) -> Self {
        Self {
            pid,
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: ProcfsEntry) {
        self.entries.push(entry);
    }
    
    pub fn get_entry(&self, name: &str) -> Option<&ProcfsEntry> {
        self.entries.iter().find(|e| e.name == name)
    }
}

/// Sysfs entry
pub struct SysfsEntry {
    pub path: String,
    pub entry_type: SysfsEntryType,
    pub data: Vec<u8>,
    pub attributes: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsEntryType {
    Device,
    Driver,
    Module,
    Class,
    Bus,
    Block,
    Net,
    Power,
    Firmware,
}

impl SysfsEntry {
    pub fn new(path: &str, entry_type: SysfsEntryType) -> Self {
        Self {
            path: path.to_string(),
            entry_type,
            data: Vec::new(),
            attributes: Vec::new(),
        }
    }
    
    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.push((key.to_string(), value.to_string()));
    }
}

/// Sysfs directory structure
pub struct SysfsDirectory {
    pub path: String,
    pub entries: Vec<SysfsEntry>,
}

impl SysfsDirectory {
    pub const fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: SysfsEntry) {
        self.entries.push(entry);
    }
    
    pub fn get_entry(&self, path: &str) -> Option<&SysfsEntry> {
        self.entries.iter().find(|e| e.path == path)
    }
}

/// Virtual filesystem manager
pub trait VirtualFsManager {
    /// Initialize virtual filesystem
    fn init(&mut self) -> Result<(), VfsError>;
    
    /// Mount virtual filesystem
    fn mount(&mut self, fs_type: VirtualFsType, mount_point: &str) -> Result<(), VfsError>;
    
    /// Unmount virtual filesystem
    fn unmount(&mut self, mount_point: &str) -> Result<(), VfsError>;
    
    /// Read entry data
    fn read(&self, path: &str) -> Result<Vec<u8>, VfsError>;
    
    /// Write entry data
    fn write(&mut self, path: &str, data: &[u8]) -> Result<(), VfsError>;
    
    /// List directory entries
    fn list(&self, path: &str) -> Result<Vec<String>, VfsError>;
}

/// Virtual filesystem types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualFsType {
    Procfs,
    Sysfs,
    Debugfs,
    Configfs,
    Securityfs,
    Tracefs,
    Bpf,
}

/// Procfs process information
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub state: ProcessState,
    pub comm: String,
    pub utime: u64,
    pub stime: u64,
    pub vsize: u64,
    pub rss: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    TracingStop,
    Zombie,
    Dead,
}

/// Sysfs device information
pub struct DeviceInfo {
    pub dev_name: String,
    pub dev_type: DeviceType,
    pub major: u32,
    pub minor: u32,
    pub driver: Option<String>,
    pub subsystem: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Block,
    Character,
    Network,
}

/// Standard procfs paths
pub mod procfs_paths {
    pub const PROC: &str = "/proc";
    pub const PROC_SELF: &str = "/proc/self";
    pub const PROC_THREAD_SELF: &str = "/proc/thread-self";
    pub const PROC_STATUS: &str = "status";
    pub const PROC_CMDLINE: &str = "cmdline";
    pub const PROC_ENVIRON: &str = "environ";
    pub const PROC_MAPS: &str = "maps";
    pub const PROC_FD: &str = "fd";
    pub const PROC_STAT: &str = "stat";
    pub const PROC_STATM: &str = "statm";
    pub const PROC_MEMINFO: &str = "/proc/meminfo";
    pub const PROC_CPUINFO: &str = "/proc/cpuinfo";
    pub const PROC_LOADAVG: &str = "/proc/loadavg";
    pub const PROC_UPTIME: &str = "/proc/uptime";
    pub const PROC_VERSION: &str = "/proc/version";
    pub const PROC_MOUNTS: &str = "/proc/mounts";
    pub const PROC_FILESYSTEMS: &str = "/proc/filesystems";
    pub const PROC_DEVICES: &str = "/proc/devices";
    pub const PROC_INTERRUPTS: &str = "/proc/interrupts";
    pub const PROC_SOFTIRQS: &str = "/proc/softirqs";
    pub const PROC_NET: &str = "/proc/net";
    pub const PROC_SYS: &str = "/proc/sys";
}

/// Standard sysfs paths
pub mod sysfs_paths {
    pub const SYS: &str = "/sys";
    pub const SYS_BLOCK: &str = "/sys/block";
    pub const SYS_BUS: &str = "/sys/bus";
    pub const SYS_CLASS: &str = "/sys/class";
    pub const SYS_DEVICES: &str = "/sys/devices";
    pub const SYS_FIRMWARE: &str = "/sys/firmware";
    pub const SYS_MODULE: &str = "/sys/module";
    pub const SYS_POWER: &str = "/sys/power";
    pub const SYS_KERNEL: &str = "/sys/kernel";
    pub const SYS_FS: &str = "/sys/fs";
    pub const SYS_DEBUG: &str = "/sys/kernel/debug";
    pub const SYS_SECURITY: &str = "/sys/kernel/security";
}

/// Virtual filesystem mount options
pub mod mount_options {
    pub const RW: &str = "rw";
    pub const RO: &str = "ro";
    pub const NOSUID: &str = "nosuid";
    pub const NOEXEC: &str = "noexec";
    pub const NODEV: &str = "nodev";
    pub const NOATIME: &str = "noatime";
    pub const NODIRATIME: &str = "nodiratime";
    pub const RELATIME: &str = "relatime";
}
