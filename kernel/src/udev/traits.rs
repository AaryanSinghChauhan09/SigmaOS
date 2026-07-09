// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired device hotplug (udev equivalent) for SigmaOS
// Zero-allocation, performance-optimized device management

/// Device event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceEvent {
    Add,
    Remove,
    Change,
    Move,
    Online,
    Offline,
    Bind,
    Unbind,
}

/// Device action
pub struct DeviceAction {
    pub event: DeviceEvent,
    pub devpath: String,
    pub subsystem: String,
    pub devtype: Option<String>,
    pub kernel: String,
    pub devname: Option<String>,
    pub devlinks: Vec<String>,
    pub properties: Vec<(String, String)>,
}

impl DeviceAction {
    pub fn new(event: DeviceEvent, devpath: &str) -> Self {
        Self {
            event,
            devpath: devpath.to_string(),
            subsystem: String::new(),
            devtype: None,
            kernel: String::new(),
            devname: None,
            devlinks: Vec::new(),
            properties: Vec::new(),
        }
    }
    
    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.push((key.to_string(), value.to_string()));
    }
    
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

/// Device rule
pub struct DeviceRule {
    pub match_conditions: Vec<MatchCondition>,
    pub actions: Vec<RuleAction>,
    pub priority: u32,
}

#[derive(Debug, Clone)]
pub enum MatchCondition {
    Kernel(String),
    Subsystem(String),
    Driver(String),
    Attr(String, String),
    Kernel(String),
    Parent(String),
    Tag(String),
    Environment(String, String),
}

#[derive(Debug, Clone)]
pub enum RuleAction {
    Add(String, String),
    Remove(String),
    Link(String),
    Owner(String),
    Group(String),
    Mode(String),
    Run(String),
    Import(String),
    Goto(String),
    Label(String),
    SetEnv(String, String),
}

/// Device database
pub struct DeviceDatabase {
    pub devices: Vec<DeviceInfo>,
}

pub struct DeviceInfo {
    pub devpath: String,
    pub subsystem: String,
    pub devtype: Option<String>,
    pub devname: Option<String>,
    pub devlinks: Vec<String>,
    pub properties: Vec<(String, String)>,
    pub tags: Vec<String>,
}

impl DeviceDatabase {
    pub const fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
    
    pub fn add_device(&mut self, device: DeviceInfo) {
        self.devices.push(device);
    }
    
    pub fn remove_device(&mut self, devpath: &str) -> bool {
        if let Some(pos) = self.devices.iter().position(|d| d.devpath == devpath) {
            self.devices.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn get_device(&self, devpath: &str) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.devpath == devpath)
    }
    
    pub fn find_by_name(&self, devname: &str) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.devname.as_ref().map(|n| n == devname).unwrap_or(false))
    }
}

/// Device manager
pub trait DeviceManager {
    /// Initialize device manager
    fn init(&mut self) -> Result<(), DeviceError>;
    
    /// Process device event
    fn process_event(&mut self, event: DeviceAction) -> Result<(), DeviceError>;
    
    /// Add device rule
    fn add_rule(&mut self, rule: DeviceRule) -> Result<(), DeviceError>;
    
    /// Remove device rule
    fn remove_rule(&mut self, id: u32) -> Result<(), DeviceError>;
    
    /// List devices
    fn list_devices(&self) -> Vec<&DeviceInfo>;
    
    /// Get device by path
    fn get_device(&self, devpath: &str) -> Option<&DeviceInfo>;
}

/// Device error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    InvalidDevice,
    InvalidRule,
    PermissionDenied,
    DeviceNotFound,
    RuleNotFound,
    DatabaseError,
    Other,
}

/// Device monitor
pub struct DeviceMonitor {
    pub event_queue: Vec<DeviceAction>,
    pub max_events: usize,
}

impl DeviceMonitor {
    pub const fn new(max_events: usize) -> Self {
        Self {
            event_queue: Vec::new(),
            max_events,
        }
    }
    
    pub fn add_event(&mut self, event: DeviceAction) -> Result<(), DeviceError> {
        if self.event_queue.len() >= self.max_events {
            return Err(DeviceError::DatabaseError);
        }
        self.event_queue.push(event);
        Ok(())
    }
    
    pub fn get_event(&mut self) -> Option<DeviceAction> {
        self.event_queue.pop()
    }
    
    pub fn event_count(&self) -> usize {
        self.event_queue.len()
    }
}

/// Device node creation
pub struct DeviceNode {
    pub major: u32,
    pub minor: u32,
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub name: String,
}

impl DeviceNode {
    pub const fn new(major: u32, minor: u32, mode: u32, name: String) -> Self {
        Self {
            major,
            minor,
            mode,
            uid: 0,
            gid: 0,
            name,
        }
    }
    
    pub fn set_owner(&mut self, uid: u32, gid: u32) {
        self.uid = uid;
        self.gid = gid;
    }
}

/// Device attributes
pub struct DeviceAttributes {
    pub attributes: Vec<(String, String)>,
}

impl DeviceAttributes {
    pub const fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }
    
    pub fn add(&mut self, key: &str, value: &str) {
        self.attributes.push((key.to_string(), value.to_string()));
    }
    
    pub fn get(&self, key: &str) -> Option<&str> {
        self.attributes.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }
}

/// Device tags
pub struct DeviceTags {
    pub tags: Vec<String>,
}

impl DeviceTags {
    pub const fn new() -> Self {
        Self {
            tags: Vec::new(),
        }
    }
    
    pub fn add(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }
    
    pub fn has(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

/// Device symlink
pub struct DeviceSymlink {
    pub target: String,
    pub link_name: String,
}

impl DeviceSymlink {
    pub const fn new(target: String, link_name: String) -> Self {
        Self {
            target,
            link_name,
        }
    }
}

/// Standard device paths
pub mod device_paths {
    pub const DEV: &str = "/dev";
    pub const DEV_BLOCK: &str = "/dev/block";
    pub const DEV_CHAR: &str = "/dev/char";
    pub const DEV_DISK: &str = "/dev/disk";
    pub const DEV_DISK_BY_ID: &str = "/dev/disk/by-id";
    pub const DEV_DISK_BY_PATH: &str = "/dev/disk/by-path";
    pub const DEV_DISK_BY_UUID: &str = "/dev/disk/by-uuid";
    pub const DEV_DISK_BY_LABEL: &str = "/dev/disk/by-label";
    pub const DEV_INPUT: &str = "/dev/input";
    pub const DEV_NET: &str = "/dev/net";
    pub const DEV_SOUND: &str = "/dev/snd";
    pub const DEV_VIDEO: &str = "/dev/video";
    pub const DEV_PTY: &str = "/dev/pty";
    pub const DEV_TTY: &str = "/dev/tty";
    pub const DEV_PTS: &str = "/dev/pts";
    pub const DEV_SHM: &str = "/dev/shm";
    pub const DEV_MQUEUE: &str = "/dev/mqueue";
    pub const DEV_HUGE_PAGES: &str = "/dev/hugepages";
    pub const RUN_UDEV: &str = "/run/udev";
    pub const RUN_UDEV_DATA: &str = "/run/udev/data";
    pub const ETC_UDEV: &str = "/etc/udev";
    pub const ETC_UDEV_RULES_D: &str = "/etc/udev/rules.d";
    pub const LIB_UDEV: &str = "/lib/udev";
    pub const LIB_UDEV_RULES_D: &str = "/lib/udev/rules.d";
}

/// Device subsystems
pub mod subsystems {
    pub const BLOCK: &str = "block";
    pub const CHAR: &str = "char";
    pub const NET: &str = "net";
    pub const INPUT: &str = "input";
    pub const SOUND: &str = "sound";
    pub const VIDEO: &str = "video";
    pub const TTY: &str = "tty";
    pub const USB: &str = "usb";
    pub const PCI: &str = "pci";
    pub const PLATFORM: &str = "platform";
    pub const SERIO: &str = "serio";
    pub const SCSI: &str = "scsi";
    pub const SCSI_DISK: &str = "scsi_disk";
    pub const SCSI_TAPE: &str = "scsi_tape";
    pub const SCSI_GENERIC: &str = "scsi_generic";
}
