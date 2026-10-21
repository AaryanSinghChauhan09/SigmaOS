#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// File system watcher for monitoring file and directory changes
/// Inspired by Linux inotify and Omarchy's refresh- commands
#[derive(Debug, Clone)]
pub struct FileWatcher {
    watchers: Vec<Watch>,
}

/// A watch on a file or directory
#[derive(Debug, Clone)]
pub struct Watch {
    id: String,
    path: String,
    event_mask: EventMask,
    state: WatchState,
}

/// Types of file system events to watch for
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventMask {
    Access,
    Modify,
    Attrib,
    CloseWrite,
    CloseNowrite,
    Open,
    MoveFrom,
    MoveTo,
    Create,
    Delete,
    DeleteSelf,
    MoveSelf,
    All,
}

/// State of a watch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchState {
    Active,
    Paused,
    Disabled,
}

/// File system event
#[derive(Debug, Clone)]
pub struct FileEvent {
    pub watch_id: String,
    pub event_type: EventMask,
    pub path: String,
    pub timestamp: u64,
}

impl FileWatcher {
    pub fn new() -> Self {
        Self {
            watchers: Vec::new(),
        }
    }

    /// Add a new watch on a file or directory
    pub fn add_watch(&mut self, id: String, path: String, event_mask: EventMask) -> Result<String, String> {
        if self.watchers.iter().any(|w| w.id == id) {
            return Err(format!("Watch with id {} already exists", id));
        }
        
        let watch = Watch {
            id: id.clone(),
            path,
            event_mask,
            state: WatchState::Active,
        };
        self.watchers.push(watch);
        Ok(id)
    }

    /// Remove a watch by ID
    pub fn remove_watch(&mut self, id: &str) -> Result<(), String> {
        let pos = self.watchers.iter().position(|w| w.id == id)
            .ok_or_else(|| format!("Watch with id {} not found", id))?;
        self.watchers.remove(pos);
        Ok(())
    }

    /// Pause a watch
    pub fn pause_watch(&mut self, id: &str) -> Result<(), String> {
        let watch = self.watchers.iter_mut().find(|w| w.id == id)
            .ok_or_else(|| format!("Watch with id {} not found", id))?;
        watch.state = WatchState::Paused;
        Ok(())
    }

    /// Resume a paused watch
    pub fn resume_watch(&mut self, id: &str) -> Result<(), String> {
        let watch = self.watchers.iter_mut().find(|w| w.id == id)
            .ok_or_else(|| format!("Watch with id {} not found", id))?;
        watch.state = WatchState::Active;
        Ok(())
    }

    /// Disable a watch
    pub fn disable_watch(&mut self, id: &str) -> Result<(), String> {
        let watch = self.watchers.iter_mut().find(|w| w.id == id)
            .ok_or_else(|| format!("Watch with id {} not found", id))?;
        watch.state = WatchState::Disabled;
        Ok(())
    }

    /// Get all watches
    pub fn get_watches(&self) -> &[Watch] {
        &self.watchers
    }

    /// Get active watches
    pub fn get_active_watches(&self) -> Vec<&Watch> {
        self.watchers.iter().filter(|w| w.state == WatchState::Active).collect()
    }

    /// Get watch by ID
    pub fn get_watch(&self, id: &str) -> Option<&Watch> {
        self.watchers.iter().find(|w| w.id == id)
    }

    /// Get watches by path
    pub fn get_watches_by_path(&self, path: &str) -> Vec<&Watch> {
        self.watchers.iter().filter(|w| w.path == path).collect()
    }
}

impl Default for FileWatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_watch() {
        let mut watcher = FileWatcher::new();
        let result = watcher.add_watch("watch1".to_string(), "/tmp/test".to_string(), EventMask::Modify);
        assert!(result.is_ok());
        assert_eq!(watcher.get_watches().len(), 1);
    }

    #[test]
    fn test_duplicate_watch() {
        let mut watcher = FileWatcher::new();
        watcher.add_watch("watch1".to_string(), "/tmp/test".to_string(), EventMask::Modify).unwrap();
        let result = watcher.add_watch("watch1".to_string(), "/tmp/other".to_string(), EventMask::Create);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_watch() {
        let mut watcher = FileWatcher::new();
        watcher.add_watch("watch1".to_string(), "/tmp/test".to_string(), EventMask::Modify).unwrap();
        let result = watcher.remove_watch("watch1");
        assert!(result.is_ok());
        assert_eq!(watcher.get_watches().len(), 0);
    }

    #[test]
    fn test_pause_resume_watch() {
        let mut watcher = FileWatcher::new();
        watcher.add_watch("watch1".to_string(), "/tmp/test".to_string(), EventMask::Modify).unwrap();
        
        watcher.pause_watch("watch1").unwrap();
        assert_eq!(watcher.get_watch("watch1").unwrap().state, WatchState::Paused);
        
        watcher.resume_watch("watch1").unwrap();
        assert_eq!(watcher.get_watch("watch1").unwrap().state, WatchState::Active);
    }

    #[test]
    fn test_get_active_watches() {
        let mut watcher = FileWatcher::new();
        watcher.add_watch("watch1".to_string(), "/tmp/test1".to_string(), EventMask::Modify).unwrap();
        watcher.add_watch("watch2".to_string(), "/tmp/test2".to_string(), EventMask::Create).unwrap();
        watcher.pause_watch("watch1").unwrap();
        
        let active = watcher.get_active_watches();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id, "watch2");
    }
}
