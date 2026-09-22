// Linux-inspired fanotify (file access notifications)
// Directory and file access monitoring for SigmaOS

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

/// Fanotify event mask (Linux fanotify.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FanotifyEventMask {
    pub access: bool,
    pub modify: bool,
    pub close_write: bool,
    pub close_nowrite: bool,
    pub open: bool,
    pub events_child: bool,
    pub on_dir: bool,
    pub event_on_child: bool,
}

impl FanotifyEventMask {
    pub fn new() -> Self {
        FanotifyEventMask {
            access: false,
            modify: false,
            close_write: false,
            close_nowrite: false,
            open: false,
            events_child: false,
            on_dir: false,
            event_on_child: false,
        }
    }

    pub fn with_access(mut self) -> Self {
        self.access = true;
        self
    }

    pub fn with_modify(mut self) -> Self {
        self.modify = true;
        self
    }

    pub fn with_open(mut self) -> Self {
        self.open = true;
        self
    }

    pub fn with_on_dir(mut self) -> Self {
        self.on_dir = true;
        self
    }
}

impl Default for FanotifyEventMask {
    fn default() -> Self {
        Self::new()
    }
}

/// Fanotify event
#[derive(Debug, Clone)]
pub struct FanotifyEvent {
    pub mask: FanotifyEventMask,
    pub pid: u32,
    pub fd: i32,
    pub path: String,
}

impl FanotifyEvent {
    pub fn new(mask: FanotifyEventMask, pid: u32, fd: i32, path: String) -> Self {
        FanotifyEvent {
            mask,
            pid,
            fd,
            path,
        }
    }
}

/// Fanotify watch (monitoring a directory or file)
#[derive(Debug, Clone)]
pub struct FanotifyWatch {
    id: u64,
    path: String,
    mask: FanotifyEventMask,
    events: Vec<FanotifyEvent>,
}

impl FanotifyWatch {
    pub fn new(id: u64, path: String, mask: FanotifyEventMask) -> Self {
        FanotifyWatch {
            id,
            path,
            mask,
            events: Vec::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn mask(&self) -> FanotifyEventMask {
        self.mask
    }

    pub fn add_event(&mut self, event: FanotifyEvent) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> &[FanotifyEvent] {
        &self.events
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn clear_events(&mut self) {
        self.events.clear();
    }
}

/// Fanotify manager for the system
pub struct FanotifyManager {
    watches: BTreeMap<u64, Arc<Mutex<FanotifyWatch>>>,
    next_watch_id: u64,
}

impl FanotifyManager {
    pub fn new() -> Self {
        FanotifyManager {
            watches: BTreeMap::new(),
            next_watch_id: 1,
        }
    }

    /// Create a new watch
    pub fn create_watch(&mut self, path: String, mask: FanotifyEventMask) -> u64 {
        let id = self.next_watch_id;
        self.next_watch_id += 1;

        let watch = Arc::new(Mutex::new(FanotifyWatch::new(id, path, mask)));
        self.watches.insert(id, watch);

        id
    }

    /// Get watch
    pub fn get_watch(&self, watch_id: u64) -> Option<Arc<Mutex<FanotifyWatch>>> {
        self.watches.get(&watch_id).cloned()
    }

    /// Report an event to a watch
    pub fn report_event(&self, watch_id: u64, event: FanotifyEvent) -> Result<(), String> {
        let watch = self.watches.get(&watch_id)
            .ok_or_else(|| format!("Watch not found: {}", watch_id))?;

        let mut watch_guard = watch.lock().unwrap();
        watch_guard.add_event(event);

        Ok(())
    }

    /// Get events from watch
    pub fn get_events(&self, watch_id: u64) -> Result<Vec<FanotifyEvent>, String> {
        let watch = self.watches.get(&watch_id)
            .ok_or_else(|| format!("Watch not found: {}", watch_id))?;

        let watch_guard = watch.lock().unwrap();
        Ok(watch_guard.get_events().to_vec())
    }

    /// Clear events from watch
    pub fn clear_events(&self, watch_id: u64) -> Result<(), String> {
        let watch = self.watches.get(&watch_id)
            .ok_or_else(|| format!("Watch not found: {}", watch_id))?;

        let mut watch_guard = watch.lock().unwrap();
        watch_guard.clear_events();

        Ok(())
    }

    /// Remove watch
    pub fn remove_watch(&mut self, watch_id: u64) -> Result<(), String> {
        self.watches.remove(&watch_id)
            .ok_or_else(|| format!("Watch not found: {}", watch_id))?;
        Ok(())
    }

    /// Get watch count
    pub fn watch_count(&self) -> usize {
        self.watches.len()
    }
}

impl Default for FanotifyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fanotify_event_mask_creation() {
        let mask = FanotifyEventMask::new();
        assert!(!mask.access);
        assert!(!mask.modify);
    }

    #[test]
    fn test_fanotify_event_mask_with_access() {
        let mask = FanotifyEventMask::new().with_access();
        assert!(mask.access);
    }

    #[test]
    fn test_fanotify_event_mask_with_modify() {
        let mask = FanotifyEventMask::new().with_modify();
        assert!(mask.modify);
    }

    #[test]
    fn test_fanotify_event_creation() {
        let mask = FanotifyEventMask::new().with_access();
        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());

        assert_eq!(event.pid, 100);
        assert_eq!(event.path, "/test/path");
    }

    #[test]
    fn test_fanotify_watch_creation() {
        let mask = FanotifyEventMask::new().with_access();
        let watch = FanotifyWatch::new(1, "/test/path".to_string(), mask);

        assert_eq!(watch.id(), 1);
        assert_eq!(watch.path(), "/test/path");
        assert_eq!(watch.event_count(), 0);
    }

    #[test]
    fn test_fanotify_watch_add_event() {
        let mask = FanotifyEventMask::new().with_access();
        let mut watch = FanotifyWatch::new(1, "/test/path".to_string(), mask);
        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());

        watch.add_event(event);
        assert_eq!(watch.event_count(), 1);
    }

    #[test]
    fn test_fanotify_watch_clear_events() {
        let mask = FanotifyEventMask::new().with_access();
        let mut watch = FanotifyWatch::new(1, "/test/path".to_string(), mask);
        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());

        watch.add_event(event);
        watch.clear_events();
        assert_eq!(watch.event_count(), 0);
    }

    #[test]
    fn test_fanotify_manager_creation() {
        let manager = FanotifyManager::new();
        assert_eq!(manager.watch_count(), 0);
    }

    #[test]
    fn test_fanotify_manager_create_watch() {
        let mut manager = FanotifyManager::new();
        let mask = FanotifyEventMask::new().with_access();
        let id = manager.create_watch("/test/path".to_string(), mask);

        assert_eq!(id, 1);
        assert_eq!(manager.watch_count(), 1);
    }

    #[test]
    fn test_fanotify_manager_report_event() {
        let mut manager = FanotifyManager::new();
        let mask = FanotifyEventMask::new().with_access();
        let watch_id = manager.create_watch("/test/path".to_string(), mask);

        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());
        assert!(manager.report_event(watch_id, event).is_ok());
    }

    #[test]
    fn test_fanotify_manager_get_events() {
        let mut manager = FanotifyManager::new();
        let mask = FanotifyEventMask::new().with_access();
        let watch_id = manager.create_watch("/test/path".to_string(), mask);

        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());
        manager.report_event(watch_id, event).unwrap();

        let events = manager.get_events(watch_id).unwrap();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_fanotify_manager_clear_events() {
        let mut manager = FanotifyManager::new();
        let mask = FanotifyEventMask::new().with_access();
        let watch_id = manager.create_watch("/test/path".to_string(), mask);

        let event = FanotifyEvent::new(mask, 100, 3, "/test/path".to_string());
        manager.report_event(watch_id, event).unwrap();

        assert!(manager.clear_events(watch_id).is_ok());
    }

    #[test]
    fn test_fanotify_manager_remove_watch() {
        let mut manager = FanotifyManager::new();
        let mask = FanotifyEventMask::new().with_access();
        let watch_id = manager.create_watch("/test/path".to_string(), mask);

        assert!(manager.remove_watch(watch_id).is_ok());
        assert_eq!(manager.watch_count(), 0);
    }
}
