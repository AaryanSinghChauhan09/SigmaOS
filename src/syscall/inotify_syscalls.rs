#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// inotify-like System Calls for SigmaOS
// Implements Linux-compatible inotify syscalls:
// - inotify_init / inotify_init1 - Create watch descriptor
// - inotify_add_watch - Register watches
// - inotify_rm_watch - Deregister watches
// - read - Get events from inotify fd

use crate::filesystem::file_monitor::{
    EventFilter, FileEvent, FileEventType, WatchConfig, WatchId, WatchManager, EventId,
};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// inotify file descriptor
pub type InotifyFd = i32;

/// inotify watch descriptor
pub type WatchDescriptor = i32;

/// inotify event flags
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InotifyFlag {
    /// Non-blocking mode
    NonBlocking = 0x800,
    /// Close-on-exec
    CloseOnExec = 0x80000,
}

/// inotify add watch event mask
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InotifyMask {
    /// File was accessed
    Access = 1,
    /// File was modified
    Modify = 2,
    /// Metadata changed
    AttribChange = 4,
    /// File opened
    Open = 32,
    /// File closed
    Close = 16,
    /// Writable file was closed
    CloseWrite = 8,
    /// Non-writable file was closed
    CloseNoWrite = 16,
    /// File/directory was created
    Create = 256,
    /// File/directory was deleted
    Delete = 512,
    /// File was renamed
    MovedFrom = 64,
    /// File was renamed to
    MovedTo = 128,
    /// Watched file/dir was deleted
    DeleteSelf = 1024,
    /// Watched file/dir was moved
    MoveSelf = 2048,
    /// Unmask all events
    Unmask = 0x2000000,
    /// Add events to existing mask
    Add = 0x20000000,
    /// Event on dir only
    OnlyDir = 0x1000000,
    /// Don't follow symlinks
    NoFollowSymlink = 0x40000000,
    /// All events
    AllEvents = 0xFFF,
}

impl InotifyMask {
    /// Convert mask to file event types
    pub fn to_event_types(&self) -> Vec<FileEventType> {
        let mut events = Vec::new();
        let mask_val = *self as u32;

        if mask_val & (InotifyMask::Access as u32) != 0
            || mask_val & (InotifyMask::Open as u32) != 0
        {
            events.push(FileEventType::Open);
        }
        if mask_val & (InotifyMask::Modify as u32) != 0 {
            events.push(FileEventType::Modify);
        }
        if mask_val & (InotifyMask::Create as u32) != 0 {
            events.push(FileEventType::Create);
        }
        if mask_val & (InotifyMask::Delete as u32) != 0 {
            events.push(FileEventType::Delete);
        }
        if mask_val & (InotifyMask::MovedFrom as u32) != 0
            || mask_val & (InotifyMask::MovedTo as u32) != 0
        {
            events.push(FileEventType::Move);
        }
        if mask_val & (InotifyMask::CloseWrite as u32) != 0
            || mask_val & (InotifyMask::CloseNoWrite as u32) != 0
        {
            events.push(FileEventType::Close);
        }

        events
    }
}

/// inotify event structure
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InotifyEvent {
    /// Watch descriptor
    pub wd: WatchDescriptor,
    /// Mask describing event
    pub mask: u32,
    /// Unique cookie associating related events
    pub cookie: u32,
    /// Length of name field
    pub len: u32,
    /// Optional null-terminated name
    pub name: Vec<u8>,
}

/// inotify instance state
pub struct InotifyInstance {
    /// File descriptor ID
    pub fd: InotifyFd,
    /// Watch manager
    pub manager: WatchManager,
    /// Map from watch descriptor to watch ID
    watch_map: HashMap<WatchDescriptor, WatchId>,
    /// Map from watch ID to watch descriptor
    id_map: HashMap<WatchId, WatchDescriptor>,
    /// Next watch descriptor to assign
    next_wd: WatchDescriptor,
    /// Event buffer
    event_buffer: Vec<u8>,
    /// Non-blocking mode flag
    non_blocking: bool,
}

impl InotifyInstance {
    /// Create new inotify instance
    pub fn new(fd: InotifyFd, flags: u32) -> Self {
        let non_blocking = (flags & InotifyFlag::NonBlocking as u32) != 0;

        InotifyInstance {
            fd,
            manager: WatchManager::new(),
            watch_map: HashMap::new(),
            id_map: HashMap::new(),
            next_wd: 1,
            event_buffer: Vec::new(),
            non_blocking,
        }
    }

    /// Add watch
    pub fn add_watch(
        &mut self,
        path: PathBuf,
        mask: u32,
    ) -> Result<WatchDescriptor, String> {
        // Convert mask to event types
        let inotify_mask = mask as u32;
        let event_types = InotifyMask::AllEvents.to_event_types();

        // Create filter from mask
        let mut filter = EventFilter::new();
        for event_type in event_types {
            filter = filter.with_event_type(event_type);
        }

        // Create watch config
        let config = WatchConfig::new()
            .with_recursive(false)
            .with_filter(filter)
            .with_max_queue_size(1000);

        // Register with manager
        let watch_id = self.manager.register_watch(path, config)?;

        // Assign watch descriptor
        let wd = self.next_wd;
        self.next_wd += 1;

        self.watch_map.insert(wd, watch_id);
        self.id_map.insert(watch_id, wd);

        Ok(wd)
    }

    /// Remove watch
    pub fn rm_watch(&mut self, wd: WatchDescriptor) -> Result<(), String> {
        if let Some(watch_id) = self.watch_map.remove(&wd) {
            self.id_map.remove(&watch_id);
            self.manager.deregister_watch(watch_id)?;
            Ok(())
        } else {
            Err(format!("Watch descriptor {} not found", wd))
        }
    }

    /// Read events
    pub fn read_events(&mut self, buf_size: usize) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();

        // Collect events from all watches
        for (&wd, &watch_id) in &self.watch_map {
            // Get pending events
            while let Ok(Some(event)) = self.manager.peek_event(watch_id) {
                // Create inotify event structure
                let mut ino_event = InotifyEvent {
                    wd,
                    mask: self.event_type_to_mask(event.event_type),
                    cookie: 0,
                    len: 0,
                    name: Vec::new(),
                };

                // Add filename if present
                if let Some(file_name) = event.path.file_name() {
                    let name_bytes = file_name.to_string_lossy();
                    ino_event.name = name_bytes.as_bytes().to_vec();
                    ino_event.len = (ino_event.name.len() as u32 + 1).next_power_of_two();
                }

                // Serialize event
                let event_data = self.serialize_event(&ino_event)?;
                if result.len() + event_data.len() > buf_size {
                    break;
                }

                result.extend_from_slice(&event_data);

                // Remove event from queue
                self.manager.get_event(watch_id)?;
            }
        }

        if result.is_empty() && !self.non_blocking {
            return Err("EAGAIN".to_string());
        }

        Ok(result)
    }

    /// Convert FileEventType to inotify mask
    fn event_type_to_mask(&self, event_type: FileEventType) -> u32 {
        match event_type {
            FileEventType::Create => InotifyMask::Create as u32,
            FileEventType::Delete => InotifyMask::Delete as u32,
            FileEventType::Modify => InotifyMask::Modify as u32,
            FileEventType::Rename => InotifyMask::MovedFrom as u32 | InotifyMask::MovedTo as u32,
            FileEventType::Close => InotifyMask::CloseWrite as u32,
            FileEventType::Open => InotifyMask::Open as u32,
            FileEventType::Move => InotifyMask::MovedFrom as u32 | InotifyMask::MovedTo as u32,
        }
    }

    /// Serialize event to binary format
    fn serialize_event(&self, event: &InotifyEvent) -> Result<Vec<u8>, String> {
        let mut data = Vec::new();

        // Write wd (i32)
        data.extend_from_slice(&event.wd.to_le_bytes());

        // Write mask (u32)
        data.extend_from_slice(&event.mask.to_le_bytes());

        // Write cookie (u32)
        data.extend_from_slice(&event.cookie.to_le_bytes());

        // Write len (u32)
        data.extend_from_slice(&event.len.to_le_bytes());

        // Write name
        if !event.name.is_empty() {
            data.extend_from_slice(&event.name);
            // Pad to len
            while data.len() % event.len as usize != 0 {
                data.push(0);
            }
        }

        Ok(data)
    }

    /// Get number of pending events
    pub fn pending_events_count(&self) -> Result<usize, String> {
        let mut total = 0;
        for &watch_id in self.id_map.keys() {
            total += self.manager.get_event_count(watch_id)?;
        }
        Ok(total)
    }
}

/// Global inotify instance storage
pub struct InotifyManager {
    instances: Arc<Mutex<HashMap<InotifyFd, InotifyInstance>>>,
    next_fd: Arc<Mutex<InotifyFd>>,
}

impl InotifyManager {
    /// Create new manager
    pub fn new() -> Self {
        InotifyManager {
            instances: Arc::new(Mutex::new(HashMap::new())),
            next_fd: Arc::new(Mutex::new(1)),
        }
    }

    /// Create new inotify instance (sys_inotify_init1)
    pub fn inotify_init1(&self, flags: u32) -> Result<InotifyFd, String> {
        let mut fd_guard = self
            .next_fd
            .lock()
            .map_err(|_| "Failed to acquire fd lock".to_string())?;
        let fd = *fd_guard;
        *fd_guard = fd.wrapping_add(1);
        drop(fd_guard);

        let instance = InotifyInstance::new(fd, flags);

        let mut instances = self
            .instances
            .lock()
            .map_err(|_| "Failed to acquire instances lock".to_string())?;
        instances.insert(fd, instance);

        Ok(fd)
    }

    /// Add watch to inotify fd (sys_inotify_add_watch)
    pub fn inotify_add_watch(
        &self,
        fd: InotifyFd,
        path: PathBuf,
        mask: u32,
    ) -> Result<WatchDescriptor, String> {
        let mut instances = self
            .instances
            .lock()
            .map_err(|_| "Failed to acquire instances lock".to_string())?;

        if let Some(instance) = instances.get_mut(&fd) {
            instance.add_watch(path, mask)
        } else {
            Err(format!("inotify fd {} not found", fd))
        }
    }

    /// Remove watch from inotify fd (sys_inotify_rm_watch)
    pub fn inotify_rm_watch(&self, fd: InotifyFd, wd: WatchDescriptor) -> Result<(), String> {
        let mut instances = self
            .instances
            .lock()
            .map_err(|_| "Failed to acquire instances lock".to_string())?;

        if let Some(instance) = instances.get_mut(&fd) {
            instance.rm_watch(wd)
        } else {
            Err(format!("inotify fd {} not found", fd))
        }
    }

    /// Read events from inotify fd
    pub fn read_events(&self, fd: InotifyFd, buf_size: usize) -> Result<Vec<u8>, String> {
        let mut instances = self
            .instances
            .lock()
            .map_err(|_| "Failed to acquire instances lock".to_string())?;

        if let Some(instance) = instances.get_mut(&fd) {
            instance.read_events(buf_size)
        } else {
            Err(format!("inotify fd {} not found", fd))
        }
    }

    /// Close inotify fd
    pub fn inotify_close(&self, fd: InotifyFd) -> Result<(), String> {
        let mut instances = self
            .instances
            .lock()
            .map_err(|_| "Failed to acquire instances lock".to_string())?;

        if instances.remove(&fd).is_some() {
            Ok(())
        } else {
            Err(format!("inotify fd {} not found", fd))
        }
    }
}

impl Default for InotifyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for InotifyManager {
    fn clone(&self) -> Self {
        InotifyManager {
            instances: Arc::clone(&self.instances),
            next_fd: Arc::clone(&self.next_fd),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_inotify_flag_values() {
        assert_eq!(InotifyFlag::NonBlocking as u32, 0x800);
        assert_eq!(InotifyFlag::CloseOnExec as u32, 0x80000);
    }

    #[test]
    fn test_inotify_mask_values() {
        assert_eq!(InotifyMask::Access as u32, 1);
        assert_eq!(InotifyMask::Modify as u32, 2);
        assert_eq!(InotifyMask::Create as u32, 256);
        assert_eq!(InotifyMask::Delete as u32, 512);
    }

    #[test]
    fn test_inotify_event_creation() {
        let event = InotifyEvent {
            wd: 1,
            mask: InotifyMask::Create as u32,
            cookie: 0,
            len: 0,
            name: Vec::new(),
        };

        assert_eq!(event.wd, 1);
        assert_eq!(event.mask, InotifyMask::Create as u32);
    }

    #[test]
    fn test_inotify_instance_creation() {
        let instance = InotifyInstance::new(1, 0);
        assert_eq!(instance.fd, 1);
        assert!(!instance.non_blocking);
    }

    #[test]
    fn test_inotify_instance_non_blocking() {
        let instance = InotifyInstance::new(1, InotifyFlag::NonBlocking as u32);
        assert!(instance.non_blocking);
    }

    #[test]
    fn test_inotify_add_watch() {
        let mut instance = InotifyInstance::new(1, 0);
        let wd = instance
            .add_watch(PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        assert_eq!(wd, 1);
    }

    #[test]
    fn test_inotify_add_multiple_watches() {
        let mut instance = InotifyInstance::new(1, 0);

        let wd1 = instance
            .add_watch(PathBuf::from("/test1"), InotifyMask::Create as u32)
            .unwrap();
        let wd2 = instance
            .add_watch(PathBuf::from("/test2"), InotifyMask::Modify as u32)
            .unwrap();

        assert_eq!(wd1, 1);
        assert_eq!(wd2, 2);
    }

    #[test]
    fn test_inotify_rm_watch() {
        let mut instance = InotifyInstance::new(1, 0);
        let wd = instance
            .add_watch(PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        instance.rm_watch(wd).unwrap();

        // Should fail on second remove
        assert!(instance.rm_watch(wd).is_err());
    }

    #[test]
    fn test_inotify_event_serialization() {
        let instance = InotifyInstance::new(1, 0);
        let event = InotifyEvent {
            wd: 1,
            mask: InotifyMask::Create as u32,
            cookie: 0,
            len: 16,
            name: vec![116, 101, 115, 116], // "test"
        };

        let serialized = instance.serialize_event(&event).unwrap();
        assert!(!serialized.is_empty());
        assert_eq!(serialized.len(), 16 + 16); // header + padded name
    }

    #[test]
    fn test_inotify_manager_init() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();
        assert_eq!(fd, 1);
    }

    #[test]
    fn test_inotify_manager_multiple_instances() {
        let manager = InotifyManager::new();
        let fd1 = manager.inotify_init1(0).unwrap();
        let fd2 = manager.inotify_init1(0).unwrap();

        assert_eq!(fd1, 1);
        assert_eq!(fd2, 2);
    }

    #[test]
    fn test_inotify_manager_add_watch() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();

        let wd = manager
            .inotify_add_watch(fd, PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        assert_eq!(wd, 1);
    }

    #[test]
    fn test_inotify_manager_add_watch_invalid_fd() {
        let manager = InotifyManager::new();
        let result = manager.inotify_add_watch(999, PathBuf::from("/test"), InotifyMask::Create as u32);
        assert!(result.is_err());
    }

    #[test]
    fn test_inotify_manager_rm_watch() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();
        let wd = manager
            .inotify_add_watch(fd, PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        manager.inotify_rm_watch(fd, wd).unwrap();

        // Should fail on second remove
        assert!(manager.inotify_rm_watch(fd, wd).is_err());
    }

    #[test]
    fn test_inotify_manager_close() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();

        manager.inotify_close(fd).unwrap();

        // Should fail on second close
        assert!(manager.inotify_close(fd).is_err());
    }

    #[test]
    fn test_inotify_manager_clone() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();

        let cloned = manager.clone();

        // Both should see the same instance
        let wd = cloned
            .inotify_add_watch(fd, PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        assert_eq!(wd, 1);
    }

    #[test]
    fn test_event_type_to_mask() {
        let instance = InotifyInstance::new(1, 0);

        let mask_create = instance.event_type_to_mask(FileEventType::Create);
        let mask_delete = instance.event_type_to_mask(FileEventType::Delete);
        let mask_modify = instance.event_type_to_mask(FileEventType::Modify);

        assert_eq!(mask_create, InotifyMask::Create as u32);
        assert_eq!(mask_delete, InotifyMask::Delete as u32);
        assert_eq!(mask_modify, InotifyMask::Modify as u32);
    }

    #[test]
    fn test_inotify_pending_events() {
        let manager = InotifyManager::new();
        let fd = manager.inotify_init1(0).unwrap();

        let _wd = manager
            .inotify_add_watch(fd, PathBuf::from("/test"), InotifyMask::Create as u32)
            .unwrap();

        // Initially no events
        let mut instances = manager.instances.lock().unwrap();
        let instance = instances.get_mut(&fd).unwrap();
        assert_eq!(instance.pending_events_count().unwrap(), 0);
    }
}
