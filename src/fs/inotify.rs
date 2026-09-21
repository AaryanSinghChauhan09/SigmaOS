// Linux inotify-inspired Filesystem Event Notification
// Real-time filesystem monitoring for SigmaOS

use std::collections::HashMap;
use std::path::PathBuf;

/// inotify event types (Linux kernel inotify.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InotifyEventType {
    Access = 0x00000001,
    Modify = 0x00000002,
    Attrib = 0x00000004,
    CloseWrite = 0x00000008,
    CloseNowrite = 0x00000010,
    Open = 0x00000020,
    MovedFrom = 0x00000040,
    MovedTo = 0x00000080,
    Create = 0x00000100,
    Delete = 0x00000200,
    DeleteSelf = 0x00000400,
    Unmount = 0x00000800,
    QOverflow = 0x00001000,
    Ignored = 0x00002000,
    Isdir = 0x40000000,
    Oneshot = 0x80000000,
}

impl InotifyEventType {
    pub fn from_bits(bits: u32) -> Vec<Self> {
        let mut events = Vec::new();
        
        if bits & (InotifyEventType::Access as u32) != 0 {
            events.push(InotifyEventType::Access);
        }
        if bits & (InotifyEventType::Modify as u32) != 0 {
            events.push(InotifyEventType::Modify);
        }
        if bits & (InotifyEventType::Attrib as u32) != 0 {
            events.push(InotifyEventType::Attrib);
        }
        if bits & (InotifyEventType::CloseWrite as u32) != 0 {
            events.push(InotifyEventType::CloseWrite);
        }
        if bits & (InotifyEventType::CloseNowrite as u32) != 0 {
            events.push(InotifyEventType::CloseNowrite);
        }
        if bits & (InotifyEventType::Open as u32) != 0 {
            events.push(InotifyEventType::Open);
        }
        if bits & (InotifyEventType::MovedFrom as u32) != 0 {
            events.push(InotifyEventType::MovedFrom);
        }
        if bits & (InotifyEventType::MovedTo as u32) != 0 {
            events.push(InotifyEventType::MovedTo);
        }
        if bits & (InotifyEventType::Create as u32) != 0 {
            events.push(InotifyEventType::Create);
        }
        if bits & (InotifyEventType::Delete as u32) != 0 {
            events.push(InotifyEventType::Delete);
        }
        if bits & (InotifyEventType::DeleteSelf as u32) != 0 {
            events.push(InotifyEventType::DeleteSelf);
        }
        if bits & (InotifyEventType::Unmount as u32) != 0 {
            events.push(InotifyEventType::Unmount);
        }
        if bits & (InotifyEventType::QOverflow as u32) != 0 {
            events.push(InotifyEventType::QOverflow);
        }
        if bits & (InotifyEventType::Ignored as u32) != 0 {
            events.push(InotifyEventType::Ignored);
        }
        if bits & (InotifyEventType::Isdir as u32) != 0 {
            events.push(InotifyEventType::Isdir);
        }
        if bits & (InotifyEventType::Oneshot as u32) != 0 {
            events.push(InotifyEventType::Oneshot);
        }
        
        events
    }
}

/// inotify watch descriptor
#[derive(Debug, Clone)]
pub struct WatchDescriptor {
    pub wd: i32,
    pub path: PathBuf,
    pub mask: u32,
    pub cookie: u32,
}

impl WatchDescriptor {
    pub fn new(wd: i32, path: PathBuf, mask: u32) -> Self {
        WatchDescriptor {
            wd,
            path,
            mask,
            cookie: 0,
        }
    }
}

/// inotify event
#[derive(Debug, Clone)]
pub struct InotifyEvent {
    pub wd: i32,
    pub mask: u32,
    pub cookie: u32,
    pub name: String,
    pub path: PathBuf,
}

impl InotifyEvent {
    pub fn new(wd: i32, mask: u32, cookie: u32, name: String, path: PathBuf) -> Self {
        InotifyEvent {
            wd,
            mask,
            cookie,
            name,
            path,
        }
    }

    /// Get event types from mask
    pub fn event_types(&self) -> Vec<InotifyEventType> {
        InotifyEventType::from_bits(self.mask)
    }

    /// Check if event contains a specific type
    pub fn has_event_type(&self, event_type: InotifyEventType) -> bool {
        self.mask & (event_type as u32) != 0
    }
}

/// inotify instance
pub struct Inotify {
    watches: HashMap<i32, WatchDescriptor>,
    next_wd: i32,
    events: Vec<InotifyEvent>,
}

impl Inotify {
    pub fn new() -> Self {
        Inotify {
            watches: HashMap::new(),
            next_wd: 1,
            events: Vec::new(),
        }
    }

    /// Add a watch
    pub fn add_watch(&mut self, path: PathBuf, mask: u32) -> Result<i32, String> {
        let wd = self.next_wd;
        self.next_wd += 1;

        let watch = WatchDescriptor::new(wd, path.clone(), mask);
        self.watches.insert(wd, watch);

        Ok(wd)
    }

    /// Remove a watch
    pub fn remove_watch(&mut self, wd: i32) -> Result<(), String> {
        self.watches.remove(&wd)
            .ok_or_else(|| format!("Watch descriptor not found: {}", wd))?;
        Ok(())
    }

    /// Get a watch descriptor
    pub fn get_watch(&self, wd: i32) -> Option<&WatchDescriptor> {
        self.watches.get(&wd)
    }

    /// Add an event (simulated)
    pub fn add_event(&mut self, event: InotifyEvent) {
        self.events.push(event);
    }

    /// Read events
    pub fn read_events(&mut self) -> Vec<InotifyEvent> {
        let events = self.events.clone();
        self.events.clear();
        events
    }

    /// Get all watches
    pub fn get_watches(&self) -> Vec<&WatchDescriptor> {
        self.watches.values().collect()
    }

    /// Get watch count
    pub fn watch_count(&self) -> usize {
        self.watches.len()
    }
}

impl Default for Inotify {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inotify_event_type_from_bits() {
        let bits = (InotifyEventType::Modify as u32) | (InotifyEventType::Create as u32);
        let events = InotifyEventType::from_bits(bits);
        
        assert_eq!(events.len(), 2);
        assert!(events.contains(&InotifyEventType::Modify));
        assert!(events.contains(&InotifyEventType::Create));
    }

    #[test]
    fn test_watch_descriptor() {
        let wd = WatchDescriptor::new(1, PathBuf::from("/tmp"), 0xffffffff);
        
        assert_eq!(wd.wd, 1);
        assert_eq!(wd.path, PathBuf::from("/tmp"));
    }

    #[test]
    fn test_inotify_add_watch() {
        let mut inotify = Inotify::new();
        
        let wd = inotify.add_watch(PathBuf::from("/tmp"), 0xffffffff).unwrap();
        assert_eq!(wd, 1);
        assert_eq!(inotify.watch_count(), 1);
    }

    #[test]
    fn test_inotify_remove_watch() {
        let mut inotify = Inotify::new();
        
        let wd = inotify.add_watch(PathBuf::from("/tmp"), 0xffffffff).unwrap();
        inotify.remove_watch(wd).unwrap();
        
        assert_eq!(inotify.watch_count(), 0);
    }

    #[test]
    fn test_inotify_event() {
        let event = InotifyEvent::new(
            1,
            InotifyEventType::Modify as u32,
            0,
            "test.txt".to_string(),
            PathBuf::from("/tmp"),
        );
        
        assert!(event.has_event_type(InotifyEventType::Modify));
        assert!(!event.has_event_type(InotifyEventType::Delete));
    }

    #[test]
    fn test_inotify_read_events() {
        let mut inotify = Inotify::new();
        
        let event = InotifyEvent::new(
            1,
            InotifyEventType::Create as u32,
            0,
            "test.txt".to_string(),
            PathBuf::from("/tmp"),
        );
        inotify.add_event(event);
        
        let events = inotify.read_events();
        assert_eq!(events.len(), 1);
        assert!(events[0].has_event_type(InotifyEventType::Create));
    }
}
