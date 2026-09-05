// File Monitoring Infrastructure for SigmaOS
// Provides foundational file watch system with event types, watch management,
// and thread-safe event queue operations.
//
// This module implements:
// - Watch registration and deregistration
// - File event types (CREATE, DELETE, MODIFY, RENAME, CLOSE, OPEN, MOVE)
// - Watch event queue management
// - Event filtering by type and path patterns
// - Thread-safe operations using Arc/Mutex

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

/// Unique identifier for a watch
pub type WatchId = u64;

/// Unique identifier for an event
pub type EventId = u64;

/// File event types that can be monitored
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileEventType {
    /// File or directory created
    Create = 1,
    /// File or directory deleted
    Delete = 2,
    /// File content modified
    Modify = 3,
    /// File or directory renamed
    Rename = 4,
    /// File descriptor closed
    Close = 5,
    /// File descriptor opened
    Open = 6,
    /// File moved to different directory
    Move = 7,
}

impl FileEventType {
    /// Convert event type to readable string
    pub fn as_str(&self) -> &'static str {
        match self {
            FileEventType::Create => "CREATE",
            FileEventType::Delete => "DELETE",
            FileEventType::Modify => "MODIFY",
            FileEventType::Rename => "RENAME",
            FileEventType::Close => "CLOSE",
            FileEventType::Open => "OPEN",
            FileEventType::Move => "MOVE",
        }
    }
}

/// File system event with metadata
#[derive(Debug, Clone)]
pub struct FileEvent {
    /// Unique event identifier
    pub id: EventId,
    /// Type of file event
    pub event_type: FileEventType,
    /// Path where event occurred
    pub path: PathBuf,
    /// Optional additional path (for renames/moves)
    pub related_path: Option<PathBuf>,
    /// Timestamp when event occurred (seconds since epoch)
    pub timestamp: u64,
    /// Watch ID this event belongs to
    pub watch_id: WatchId,
}

impl FileEvent {
    /// Create a new file event
    pub fn new(
        id: EventId,
        event_type: FileEventType,
        path: PathBuf,
        watch_id: WatchId,
        timestamp: u64,
    ) -> Self {
        FileEvent {
            id,
            event_type,
            path,
            related_path: None,
            timestamp,
            watch_id,
        }
    }

    /// Create a new file event with related path (for renames/moves)
    pub fn with_related_path(
        id: EventId,
        event_type: FileEventType,
        path: PathBuf,
        related_path: PathBuf,
        watch_id: WatchId,
        timestamp: u64,
    ) -> Self {
        FileEvent {
            id,
            event_type,
            path,
            related_path: Some(related_path),
            timestamp,
            watch_id,
        }
    }
}

/// Filter criteria for events
#[derive(Debug, Clone)]
pub struct EventFilter {
    /// Event types to include (if empty, all types included)
    pub event_types: Vec<FileEventType>,
    /// Path patterns to match (if empty, all paths included)
    pub path_patterns: Vec<String>,
}

impl EventFilter {
    /// Create a new empty filter (accepts all events)
    pub fn new() -> Self {
        EventFilter {
            event_types: Vec::new(),
            path_patterns: Vec::new(),
        }
    }

    /// Add event type to filter
    pub fn with_event_type(mut self, event_type: FileEventType) -> Self {
        self.event_types.push(event_type);
        self
    }

    /// Add path pattern to filter
    pub fn with_path_pattern(mut self, pattern: String) -> Self {
        self.path_patterns.push(pattern);
        self
    }

    /// Check if event matches this filter
    pub fn matches(&self, event: &FileEvent) -> bool {
        // Check event type
        if !self.event_types.is_empty() && !self.event_types.contains(&event.event_type) {
            return false;
        }

        // Check path pattern
        if !self.path_patterns.is_empty() {
            let path_str = event.path.to_string_lossy();
            let matches_pattern = self.path_patterns.iter().any(|pattern| {
                // Simple pattern matching: check if path contains pattern
                path_str.contains(pattern)
            });
            if !matches_pattern {
                return false;
            }
        }

        true
    }
}

impl Default for EventFilter {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for a file watch
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Maximum events to queue before discarding old events
    pub max_queue_size: usize,
    /// Whether to recursively watch subdirectories
    pub recursive: bool,
    /// Filter for events
    pub filter: EventFilter,
}

impl WatchConfig {
    /// Create default watch configuration
    pub fn new() -> Self {
        WatchConfig {
            max_queue_size: 1000,
            recursive: false,
            filter: EventFilter::new(),
        }
    }

    /// Set maximum queue size
    pub fn with_max_queue_size(mut self, size: usize) -> Self {
        self.max_queue_size = size;
        self
    }

    /// Set recursive watching
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }

    /// Set event filter
    pub fn with_filter(mut self, filter: EventFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual file watch with event queue
#[derive(Debug)]
pub struct Watch {
    /// Unique identifier for this watch
    pub id: WatchId,
    /// Path being watched
    pub path: PathBuf,
    /// Configuration for this watch
    pub config: WatchConfig,
    /// Event queue
    event_queue: Vec<FileEvent>,
    /// Next event ID to assign
    next_event_id: EventId,
}

impl Watch {
    /// Create a new watch
    pub fn new(id: WatchId, path: PathBuf, config: WatchConfig) -> Self {
        Watch {
            id,
            path,
            config,
            event_queue: Vec::new(),
            next_event_id: 0,
        }
    }

    /// Add event to queue, respecting max size
    pub fn add_event(&mut self, event: FileEvent) {
        // Check filter
        if !self.config.filter.matches(&event) {
            return;
        }

        // Remove oldest event if queue is full
        if self.event_queue.len() >= self.config.max_queue_size {
            self.event_queue.remove(0);
        }

        self.event_queue.push(event);
    }

    /// Get next event without removing from queue
    pub fn peek_event(&self) -> Option<&FileEvent> {
        self.event_queue.first()
    }

    /// Remove and return next event from queue
    pub fn pop_event(&mut self) -> Option<FileEvent> {
        if self.event_queue.is_empty() {
            None
        } else {
            Some(self.event_queue.remove(0))
        }
    }

    /// Get number of events in queue
    pub fn event_count(&self) -> usize {
        self.event_queue.len()
    }

    /// Clear all events from queue
    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }

    /// Get all events matching filter
    pub fn get_events(&self) -> Vec<FileEvent> {
        self.event_queue.clone()
    }

    /// Generate next event ID
    pub fn next_event_id(&mut self) -> EventId {
        let id = self.next_event_id;
        self.next_event_id = self.next_event_id.wrapping_add(1);
        id
    }
}

/// Thread-safe file watch manager
pub struct WatchManager {
    /// Map of watch ID to Watch
    watches: Arc<Mutex<HashMap<WatchId, Watch>>>,
    /// Next watch ID to assign
    next_watch_id: Arc<Mutex<WatchId>>,
    /// Global event counter
    event_counter: Arc<Mutex<EventId>>,
}

impl WatchManager {
    /// Create a new watch manager
    pub fn new() -> Self {
        WatchManager {
            watches: Arc::new(Mutex::new(HashMap::new())),
            next_watch_id: Arc::new(Mutex::new(1)),
            event_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Register a new watch
    pub fn register_watch(&self, path: PathBuf, config: WatchConfig) -> Result<WatchId, String> {
        let mut watch_id_guard = self
            .next_watch_id
            .lock()
            .map_err(|_| "Failed to acquire watch ID lock".to_string())?;
        let watch_id = *watch_id_guard;
        *watch_id_guard = watch_id.wrapping_add(1);

        let watch = Watch::new(watch_id, path.clone(), config);

        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        watches.insert(watch_id, watch);

        Ok(watch_id)
    }

    /// Deregister a watch by ID
    pub fn deregister_watch(&self, watch_id: WatchId) -> Result<bool, String> {
        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        Ok(watches.remove(&watch_id).is_some())
    }

    /// Add event to a watch
    pub fn add_event(
        &self,
        watch_id: WatchId,
        event_type: FileEventType,
        path: PathBuf,
    ) -> Result<EventId, String> {
        let mut event_counter = self
            .event_counter
            .lock()
            .map_err(|_| "Failed to acquire event counter lock".to_string())?;
        let event_id = *event_counter;
        *event_counter = event_counter.wrapping_add(1);
        drop(event_counter);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Failed to get timestamp".to_string())?
            .as_secs();

        let event = FileEvent::new(event_id, event_type, path, watch_id, timestamp);

        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get_mut(&watch_id) {
            watch.add_event(event);
            Ok(event_id)
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Add event with related path (for renames/moves)
    pub fn add_event_with_related(
        &self,
        watch_id: WatchId,
        event_type: FileEventType,
        path: PathBuf,
        related_path: PathBuf,
    ) -> Result<EventId, String> {
        let mut event_counter = self
            .event_counter
            .lock()
            .map_err(|_| "Failed to acquire event counter lock".to_string())?;
        let event_id = *event_counter;
        *event_counter = event_counter.wrapping_add(1);
        drop(event_counter);

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| "Failed to get timestamp".to_string())?
            .as_secs();

        let event = FileEvent::with_related_path(event_id, event_type, path, related_path, watch_id, timestamp);

        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get_mut(&watch_id) {
            watch.add_event(event);
            Ok(event_id)
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Get next event from a watch
    pub fn get_event(&self, watch_id: WatchId) -> Result<Option<FileEvent>, String> {
        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get_mut(&watch_id) {
            Ok(watch.pop_event())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Peek next event without removing
    pub fn peek_event(&self, watch_id: WatchId) -> Result<Option<FileEvent>, String> {
        let watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get(&watch_id) {
            Ok(watch.peek_event().cloned())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Get event count for a watch
    pub fn get_event_count(&self, watch_id: WatchId) -> Result<usize, String> {
        let watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get(&watch_id) {
            Ok(watch.event_count())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Get all events for a watch
    pub fn get_all_events(&self, watch_id: WatchId) -> Result<Vec<FileEvent>, String> {
        let watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get(&watch_id) {
            Ok(watch.get_events())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Clear events for a watch
    pub fn clear_events(&self, watch_id: WatchId) -> Result<(), String> {
        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get_mut(&watch_id) {
            watch.clear_events();
            Ok(())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Get total number of registered watches
    pub fn watch_count(&self) -> Result<usize, String> {
        let watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        Ok(watches.len())
    }

    /// Update watch configuration
    pub fn update_watch_config(
        &self,
        watch_id: WatchId,
        config: WatchConfig,
    ) -> Result<(), String> {
        let mut watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get_mut(&watch_id) {
            watch.config = config;
            Ok(())
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }

    /// Get watch info
    pub fn get_watch_info(&self, watch_id: WatchId) -> Result<(PathBuf, usize, bool), String> {
        let watches = self
            .watches
            .lock()
            .map_err(|_| "Failed to acquire watches lock".to_string())?;
        if let Some(watch) = watches.get(&watch_id) {
            Ok((
                watch.path.clone(),
                watch.event_count(),
                watch.config.recursive,
            ))
        } else {
            Err(format!("Watch {} not found", watch_id))
        }
    }
}

impl Default for WatchManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for WatchManager {
    fn clone(&self) -> Self {
        WatchManager {
            watches: Arc::clone(&self.watches),
            next_watch_id: Arc::clone(&self.next_watch_id),
            event_counter: Arc::clone(&self.event_counter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_event_type_strings() {
        assert_eq!(FileEventType::Create.as_str(), "CREATE");
        assert_eq!(FileEventType::Delete.as_str(), "DELETE");
        assert_eq!(FileEventType::Modify.as_str(), "MODIFY");
        assert_eq!(FileEventType::Rename.as_str(), "RENAME");
        assert_eq!(FileEventType::Close.as_str(), "CLOSE");
        assert_eq!(FileEventType::Open.as_str(), "OPEN");
        assert_eq!(FileEventType::Move.as_str(), "MOVE");
    }

    #[test]
    fn test_file_event_creation() {
        let path = PathBuf::from("/test/file.txt");
        let event = FileEvent::new(1, FileEventType::Create, path.clone(), 1, 1000);
        assert_eq!(event.id, 1);
        assert_eq!(event.event_type, FileEventType::Create);
        assert_eq!(event.path, path);
        assert_eq!(event.timestamp, 1000);
        assert_eq!(event.watch_id, 1);
        assert!(event.related_path.is_none());
    }

    #[test]
    fn test_file_event_with_related_path() {
        let path = PathBuf::from("/test/old_name.txt");
        let related_path = PathBuf::from("/test/new_name.txt");
        let event = FileEvent::with_related_path(
            1,
            FileEventType::Rename,
            path.clone(),
            related_path.clone(),
            1,
            1000,
        );
        assert_eq!(event.related_path, Some(related_path));
    }

    #[test]
    fn test_event_filter_empty() {
        let filter = EventFilter::new();
        let event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test"), 1, 1000);
        assert!(filter.matches(&event));
    }

    #[test]
    fn test_event_filter_by_type() {
        let filter = EventFilter::new().with_event_type(FileEventType::Create);
        let create_event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test"), 1, 1000);
        let delete_event = FileEvent::new(2, FileEventType::Delete, PathBuf::from("/test"), 1, 1000);

        assert!(filter.matches(&create_event));
        assert!(!filter.matches(&delete_event));
    }

    #[test]
    fn test_event_filter_by_pattern() {
        let filter = EventFilter::new().with_path_pattern("/var/log".to_string());
        let matching_event =
            FileEvent::new(1, FileEventType::Create, PathBuf::from("/var/log/app.log"), 1, 1000);
        let non_matching_event =
            FileEvent::new(2, FileEventType::Create, PathBuf::from("/tmp/test"), 1, 1000);

        assert!(filter.matches(&matching_event));
        assert!(!filter.matches(&non_matching_event));
    }

    #[test]
    fn test_event_filter_combined() {
        let filter = EventFilter::new()
            .with_event_type(FileEventType::Create)
            .with_path_pattern("/var".to_string());

        let matching_event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/var/test"), 1, 1000);
        let wrong_type = FileEvent::new(2, FileEventType::Delete, PathBuf::from("/var/test"), 1, 1000);
        let wrong_path = FileEvent::new(3, FileEventType::Create, PathBuf::from("/tmp/test"), 1, 1000);

        assert!(filter.matches(&matching_event));
        assert!(!filter.matches(&wrong_type));
        assert!(!filter.matches(&wrong_path));
    }

    #[test]
    fn test_watch_config_builder() {
        let config = WatchConfig::new()
            .with_max_queue_size(500)
            .with_recursive(true)
            .with_filter(EventFilter::new().with_event_type(FileEventType::Modify));

        assert_eq!(config.max_queue_size, 500);
        assert!(config.recursive);
        assert_eq!(config.filter.event_types.len(), 1);
    }

    #[test]
    fn test_watch_creation() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();
        let watch = Watch::new(1, path.clone(), config);

        assert_eq!(watch.id, 1);
        assert_eq!(watch.path, path);
        assert_eq!(watch.event_count(), 0);
    }

    #[test]
    fn test_watch_add_event() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();
        let mut watch = Watch::new(1, path, config);

        let event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test/file.txt"), 1, 1000);
        watch.add_event(event.clone());

        assert_eq!(watch.event_count(), 1);
        assert_eq!(watch.peek_event().unwrap().id, 1);
    }

    #[test]
    fn test_watch_pop_event() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();
        let mut watch = Watch::new(1, path, config);

        let event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test/file.txt"), 1, 1000);
        watch.add_event(event.clone());

        assert_eq!(watch.event_count(), 1);
        let popped = watch.pop_event().unwrap();
        assert_eq!(popped.id, 1);
        assert_eq!(watch.event_count(), 0);
    }

    #[test]
    fn test_watch_queue_respects_max_size() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new().with_max_queue_size(3);
        let mut watch = Watch::new(1, path, config);

        for i in 0..5 {
            let event =
                FileEvent::new(i, FileEventType::Create, PathBuf::from("/test/file.txt"), 1, 1000 + i);
            watch.add_event(event);
        }

        assert_eq!(watch.event_count(), 3);
        // First two events should be removed
        let first = watch.pop_event().unwrap();
        assert_eq!(first.id, 2);
    }

    #[test]
    fn test_watch_filter_applied() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new().with_filter(
            EventFilter::new().with_event_type(FileEventType::Create)
        );
        let mut watch = Watch::new(1, path, config);

        let create_event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test/file.txt"), 1, 1000);
        let delete_event = FileEvent::new(2, FileEventType::Delete, PathBuf::from("/test/file.txt"), 1, 1001);

        watch.add_event(create_event);
        watch.add_event(delete_event);

        // Only create event should be in queue
        assert_eq!(watch.event_count(), 1);
        assert_eq!(watch.pop_event().unwrap().event_type, FileEventType::Create);
    }

    #[test]
    fn test_watch_clear_events() {
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();
        let mut watch = Watch::new(1, path, config);

        for i in 0..3 {
            let event =
                FileEvent::new(i, FileEventType::Create, PathBuf::from("/test/file.txt"), 1, 1000 + i);
            watch.add_event(event);
        }

        assert_eq!(watch.event_count(), 3);
        watch.clear_events();
        assert_eq!(watch.event_count(), 0);
    }

    #[test]
    fn test_watch_manager_registration() {
        let manager = WatchManager::new();
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();

        let watch_id = manager.register_watch(path, config).unwrap();
        assert_eq!(watch_id, 1);

        let watch_count = manager.watch_count().unwrap();
        assert_eq!(watch_count, 1);
    }

    #[test]
    fn test_watch_manager_multiple_watches() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();

        let id1 = manager.register_watch(PathBuf::from("/test1"), config.clone()).unwrap();
        let id2 = manager.register_watch(PathBuf::from("/test2"), config).unwrap();

        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(manager.watch_count().unwrap(), 2);
    }

    #[test]
    fn test_watch_manager_deregister() {
        let manager = WatchManager::new();
        let path = PathBuf::from("/test");
        let config = WatchConfig::new();

        let watch_id = manager.register_watch(path, config).unwrap();
        assert_eq!(manager.watch_count().unwrap(), 1);

        let removed = manager.deregister_watch(watch_id).unwrap();
        assert!(removed);
        assert_eq!(manager.watch_count().unwrap(), 0);
    }

    #[test]
    fn test_watch_manager_deregister_nonexistent() {
        let manager = WatchManager::new();
        let removed = manager.deregister_watch(999).unwrap();
        assert!(!removed);
    }

    #[test]
    fn test_watch_manager_add_event() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let event_id = manager
            .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
            .unwrap();
        assert_eq!(event_id, 0);

        let event_count = manager.get_event_count(watch_id).unwrap();
        assert_eq!(event_count, 1);
    }

    #[test]
    fn test_watch_manager_get_event() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let _ = manager
            .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
            .unwrap();

        let event = manager.get_event(watch_id).unwrap();
        assert!(event.is_some());
        assert_eq!(event.unwrap().event_type, FileEventType::Create);
    }

    #[test]
    fn test_watch_manager_peek_event() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let _ = manager
            .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
            .unwrap();

        // Peek should not remove event
        let event1 = manager.peek_event(watch_id).unwrap();
        let event2 = manager.peek_event(watch_id).unwrap();
        assert_eq!(event1.unwrap().id, event2.unwrap().id);
        assert_eq!(manager.get_event_count(watch_id).unwrap(), 1);
    }

    #[test]
    fn test_watch_manager_add_event_with_related() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let _ = manager
            .add_event_with_related(
                watch_id,
                FileEventType::Rename,
                PathBuf::from("/test/old.txt"),
                PathBuf::from("/test/new.txt"),
            )
            .unwrap();

        let event = manager.get_event(watch_id).unwrap().unwrap();
        assert_eq!(event.event_type, FileEventType::Rename);
        assert_eq!(
            event.related_path.unwrap(),
            PathBuf::from("/test/new.txt")
        );
    }

    #[test]
    fn test_watch_manager_get_all_events() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        for i in 0..3 {
            let _ = manager
                .add_event(watch_id, FileEventType::Create, PathBuf::from(&format!("/test/file{}.txt", i)))
                .unwrap();
        }

        let events = manager.get_all_events(watch_id).unwrap();
        assert_eq!(events.len(), 3);
    }

    #[test]
    fn test_watch_manager_clear_events() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        for _ in 0..3 {
            let _ = manager
                .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
                .unwrap();
        }

        assert_eq!(manager.get_event_count(watch_id).unwrap(), 3);
        manager.clear_events(watch_id).unwrap();
        assert_eq!(manager.get_event_count(watch_id).unwrap(), 0);
    }

    #[test]
    fn test_watch_manager_update_config() {
        let manager = WatchManager::new();
        let config = WatchConfig::new().with_recursive(false);
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let new_config = WatchConfig::new().with_recursive(true);
        manager.update_watch_config(watch_id, new_config).unwrap();

        let (_, _, recursive) = manager.get_watch_info(watch_id).unwrap();
        assert!(recursive);
    }

    #[test]
    fn test_watch_manager_get_watch_info() {
        let manager = WatchManager::new();
        let path = PathBuf::from("/test");
        let config = WatchConfig::new().with_recursive(true);
        let watch_id = manager.register_watch(path.clone(), config).unwrap();

        let _ = manager
            .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
            .unwrap();

        let (info_path, count, recursive) = manager.get_watch_info(watch_id).unwrap();
        assert_eq!(info_path, path);
        assert_eq!(count, 1);
        assert!(recursive);
    }

    #[test]
    fn test_watch_manager_clone() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();
        let watch_id = manager.register_watch(PathBuf::from("/test"), config).unwrap();

        let _ = manager
            .add_event(watch_id, FileEventType::Create, PathBuf::from("/test/file.txt"))
            .unwrap();

        // Clone manager
        let cloned_manager = manager.clone();

        // Both should see the same data
        assert_eq!(manager.watch_count().unwrap(), 1);
        assert_eq!(cloned_manager.watch_count().unwrap(), 1);
        assert_eq!(manager.get_event_count(watch_id).unwrap(), 1);
        assert_eq!(cloned_manager.get_event_count(watch_id).unwrap(), 1);
    }

    #[test]
    fn test_watch_manager_concurrent_watches() {
        let manager = WatchManager::new();
        let config = WatchConfig::new();

        let watch_id1 = manager.register_watch(PathBuf::from("/test1"), config.clone()).unwrap();
        let watch_id2 = manager.register_watch(PathBuf::from("/test2"), config).unwrap();

        let _ = manager
            .add_event(watch_id1, FileEventType::Create, PathBuf::from("/test1/file1.txt"))
            .unwrap();
        let _ = manager
            .add_event(watch_id2, FileEventType::Modify, PathBuf::from("/test2/file2.txt"))
            .unwrap();

        let event1 = manager.get_event(watch_id1).unwrap().unwrap();
        let event2 = manager.get_event(watch_id2).unwrap().unwrap();

        assert_eq!(event1.event_type, FileEventType::Create);
        assert_eq!(event2.event_type, FileEventType::Modify);
        assert_eq!(event1.path, PathBuf::from("/test1/file1.txt"));
        assert_eq!(event2.path, PathBuf::from("/test2/file2.txt"));
    }

    #[test]
    fn test_watch_manager_nonexistent_watch() {
        let manager = WatchManager::new();
        let result = manager.get_event(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_filter_multiple_types() {
        let filter = EventFilter::new()
            .with_event_type(FileEventType::Create)
            .with_event_type(FileEventType::Delete);

        let create_event = FileEvent::new(1, FileEventType::Create, PathBuf::from("/test"), 1, 1000);
        let delete_event = FileEvent::new(2, FileEventType::Delete, PathBuf::from("/test"), 1, 1000);
        let modify_event = FileEvent::new(3, FileEventType::Modify, PathBuf::from("/test"), 1, 1000);

        assert!(filter.matches(&create_event));
        assert!(filter.matches(&delete_event));
        assert!(!filter.matches(&modify_event));
    }
}
