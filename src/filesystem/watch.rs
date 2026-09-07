// File Watch Event Queue System
// Implements ring buffer event storage, coalescing, and delivery for file monitoring

use crate::filesystem::file_monitor::{FileEvent, FileEventType};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Ring buffer size for events
pub const RING_BUFFER_SIZE: usize = 4096;

/// Event coalescing window (milliseconds)
pub const COALESCE_WINDOW_MS: u64 = 100;

/// Event queue with ring buffer and coalescing
pub struct EventQueue {
    /// Ring buffer of events
    buffer: VecDeque<FileEvent>,
    /// Maximum buffer size
    max_size: usize,
    /// Last event timestamp for coalescing
    last_event_time: u64,
    /// Last event path for coalescing
    last_event_path: Option<String>,
    /// Last event type for coalescing
    last_event_type: Option<FileEventType>,
    /// Coalescing enabled
    coalesce_enabled: bool,
}

impl EventQueue {
    /// Create new event queue
    pub fn new(max_size: usize) -> Self {
        EventQueue {
            buffer: VecDeque::with_capacity(max_size),
            max_size,
            last_event_time: 0,
            last_event_path: None,
            last_event_type: None,
            coalesce_enabled: true,
        }
    }

    /// Create with default size
    pub fn with_default_size() -> Self {
        Self::new(RING_BUFFER_SIZE)
    }

    /// Enable/disable coalescing
    pub fn set_coalescing(&mut self, enabled: bool) {
        self.coalesce_enabled = enabled;
    }

    /// Check if event should be coalesced
    fn should_coalesce(&self, event: &FileEvent) -> bool {
        if !self.coalesce_enabled {
            return false;
        }

        let time_delta = event.timestamp.saturating_sub(self.last_event_time);

        // Coalesce if:
        // 1. Same file
        // 2. Same event type (or both are modify-like events)
        // 3. Within coalesce window
        if time_delta > COALESCE_WINDOW_MS {
            return false;
        }

        if let Some(ref last_path) = self.last_event_path {
            if event.path.to_string_lossy() != *last_path {
                return false;
            }
        } else {
            return false;
        }

        if let Some(last_type) = self.last_event_type {
            // Allow coalescing of similar events
            match (last_type, event.event_type) {
                (FileEventType::Modify, FileEventType::Modify) => true,
                (FileEventType::Open, FileEventType::Open) => true,
                (FileEventType::Close, FileEventType::Close) => true,
                _ => false,
            }
        } else {
            false
        }
    }

    /// Add event to queue with coalescing
    pub fn add_event(&mut self, event: FileEvent) {
        // Check if we should coalesce
        if self.should_coalesce(&event) {
            // Update last event time but don't add to queue
            self.last_event_time = event.timestamp;
            return;
        }

        // Add event to buffer
        if self.buffer.len() >= self.max_size {
            // Remove oldest event (ring buffer behavior)
            self.buffer.pop_front();
        }

        // Update last event tracking
        self.last_event_time = event.timestamp;
        self.last_event_path = Some(event.path.to_string_lossy().to_string());
        self.last_event_type = Some(event.event_type);

        self.buffer.push_back(event);
    }

    /// Get next event without removing
    pub fn peek(&self) -> Option<&FileEvent> {
        self.buffer.front()
    }

    /// Remove and return next event
    pub fn pop(&mut self) -> Option<FileEvent> {
        self.buffer.pop_front()
    }

    /// Get all events without removing
    pub fn peek_all(&self) -> Vec<&FileEvent> {
        self.buffer.iter().collect()
    }

    /// Get and remove all events
    pub fn drain_all(&mut self) -> Vec<FileEvent> {
        self.buffer.drain(..).collect()
    }

    /// Get event count
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Clear all events
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.last_event_time = 0;
        self.last_event_path = None;
        self.last_event_type = None;
    }

    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// Get load factor (0.0 to 1.0)
    pub fn load_factor(&self) -> f64 {
        self.len() as f64 / self.max_size as f64
    }

    /// Check if buffer is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.max_size
    }

    /// Resize buffer
    pub fn resize(&mut self, new_size: usize) {
        self.max_size = new_size;
        while self.buffer.len() > new_size {
            self.buffer.pop_front();
        }
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::with_default_size()
    }
}

/// Thread-safe event queue with delivery
pub struct ThreadSafeEventQueue {
    queue: Arc<Mutex<EventQueue>>,
}

impl ThreadSafeEventQueue {
    /// Create new thread-safe queue
    pub fn new(max_size: usize) -> Self {
        ThreadSafeEventQueue {
            queue: Arc::new(Mutex::new(EventQueue::new(max_size))),
        }
    }

    /// Create with default size
    pub fn with_default_size() -> Self {
        Self::new(RING_BUFFER_SIZE)
    }

    /// Add event
    pub fn add_event(&self, event: FileEvent) -> Result<(), String> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        queue.add_event(event);
        Ok(())
    }

    /// Peek next event
    pub fn peek(&self) -> Result<Option<FileEvent>, String> {
        let queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.peek().cloned())
    }

    /// Get next event
    pub fn pop(&self) -> Result<Option<FileEvent>, String> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.pop())
    }

    /// Get all events
    pub fn peek_all(&self) -> Result<Vec<FileEvent>, String> {
        let queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.peek_all().into_iter().cloned().collect())
    }

    /// Drain all events
    pub fn drain_all(&self) -> Result<Vec<FileEvent>, String> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.drain_all())
    }

    /// Get event count
    pub fn len(&self) -> Result<usize, String> {
        let queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.len())
    }

    /// Check if empty
    pub fn is_empty(&self) -> Result<bool, String> {
        let queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.is_empty())
    }

    /// Clear queue
    pub fn clear(&self) -> Result<(), String> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        queue.clear();
        Ok(())
    }

    /// Get load factor
    pub fn load_factor(&self) -> Result<f64, String> {
        let queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        Ok(queue.load_factor())
    }

    /// Enable/disable coalescing
    pub fn set_coalescing(&self, enabled: bool) -> Result<(), String> {
        let mut queue = self
            .queue
            .lock()
            .map_err(|_| "Failed to acquire queue lock".to_string())?;
        queue.set_coalescing(enabled);
        Ok(())
    }
}

impl Default for ThreadSafeEventQueue {
    fn default() -> Self {
        Self::with_default_size()
    }
}

impl Clone for ThreadSafeEventQueue {
    fn clone(&self) -> Self {
        ThreadSafeEventQueue {
            queue: Arc::clone(&self.queue),
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_event(id: u64, event_type: FileEventType, timestamp: u64) -> FileEvent {
        FileEvent::new(
            id,
            event_type,
            PathBuf::from("/test/file.txt"),
            1,
            timestamp,
        )
    }

    #[test]
    fn test_event_queue_creation() {
        let queue = EventQueue::new(100);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.capacity(), 100);
    }

    #[test]
    fn test_event_queue_add_event() {
        let mut queue = EventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);

        queue.add_event(event);
        assert_eq!(queue.len(), 1);
        assert!(!queue.is_empty());
    }

    #[test]
    fn test_event_queue_peek() {
        let mut queue = EventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);
        queue.add_event(event.clone());

        let peeked = queue.peek();
        assert!(peeked.is_some());
        assert_eq!(peeked.unwrap().id, 1);
        // Event should still be in queue
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_event_queue_pop() {
        let mut queue = EventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);
        queue.add_event(event);

        let popped = queue.pop();
        assert!(popped.is_some());
        assert_eq!(popped.unwrap().id, 1);
        // Event should be removed
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_event_queue_fifo_order() {
        let mut queue = EventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop().unwrap().id, 1);
        assert_eq!(queue.pop().unwrap().id, 2);
        assert_eq!(queue.pop().unwrap().id, 3);
    }

    #[test]
    fn test_event_queue_ring_buffer_overflow() {
        let mut queue = EventQueue::new(3);

        for i in 1..=5 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        // Should only have last 3 events
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop().unwrap().id, 3);
        assert_eq!(queue.pop().unwrap().id, 4);
        assert_eq!(queue.pop().unwrap().id, 5);
    }

    #[test]
    fn test_event_queue_peek_all() {
        let mut queue = EventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        let all = queue.peek_all();
        assert_eq!(all.len(), 3);
        assert_eq!(all[0].id, 1);
        assert_eq!(all[1].id, 2);
        assert_eq!(all[2].id, 3);
        // Queue should still have all events
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn test_event_queue_drain_all() {
        let mut queue = EventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        let all = queue.drain_all();
        assert_eq!(all.len(), 3);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_event_queue_clear() {
        let mut queue = EventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        queue.clear();
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_event_queue_coalescing_disabled() {
        let mut queue = EventQueue::with_default_size();
        queue.set_coalescing(false);

        let event1 = create_test_event(1, FileEventType::Modify, 1000);
        let event2 = create_test_event(2, FileEventType::Modify, 1050);

        queue.add_event(event1);
        queue.add_event(event2);

        // Both events should be in queue
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_event_queue_coalescing_same_type() {
        let mut queue = EventQueue::with_default_size();
        queue.set_coalescing(true);

        let event1 = create_test_event(1, FileEventType::Modify, 1000);
        let event2 = create_test_event(2, FileEventType::Modify, 1050);

        queue.add_event(event1);
        queue.add_event(event2);

        // Second event should coalesce (not added)
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_event_queue_coalescing_different_type() {
        let mut queue = EventQueue::with_default_size();
        queue.set_coalescing(true);

        let event1 = create_test_event(1, FileEventType::Modify, 1000);
        let event2 = create_test_event(2, FileEventType::Create, 1050);

        queue.add_event(event1);
        queue.add_event(event2);

        // Different event types - no coalescing
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_event_queue_coalescing_outside_window() {
        let mut queue = EventQueue::with_default_size();
        queue.set_coalescing(true);

        let event1 = create_test_event(1, FileEventType::Modify, 1000);
        let event2 = create_test_event(2, FileEventType::Modify, 2000);

        queue.add_event(event1);
        queue.add_event(event2);

        // Outside coalesce window - both events
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn test_event_queue_load_factor() {
        let mut queue = EventQueue::new(100);

        for i in 1..=50 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        let factor = queue.load_factor();
        assert!(factor > 0.4 && factor < 0.6);
    }

    #[test]
    fn test_event_queue_resize() {
        let mut queue = EventQueue::new(100);

        for i in 1..=50 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event);
        }

        queue.resize(30);
        assert_eq!(queue.len(), 30);
        assert_eq!(queue.capacity(), 30);
    }

    #[test]
    fn test_thread_safe_queue_creation() {
        let queue = ThreadSafeEventQueue::with_default_size();
        assert_eq!(queue.len().unwrap(), 0);
    }

    #[test]
    fn test_thread_safe_queue_add_and_pop() {
        let queue = ThreadSafeEventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);

        queue.add_event(event).unwrap();
        assert_eq!(queue.len().unwrap(), 1);

        let popped = queue.pop().unwrap();
        assert_eq!(popped.unwrap().id, 1);
        assert_eq!(queue.len().unwrap(), 0);
    }

    #[test]
    fn test_thread_safe_queue_peek() {
        let queue = ThreadSafeEventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);

        queue.add_event(event).unwrap();
        let peeked = queue.peek().unwrap();
        assert_eq!(peeked.unwrap().id, 1);
        assert_eq!(queue.len().unwrap(), 1);
    }

    #[test]
    fn test_thread_safe_queue_drain_all() {
        let queue = ThreadSafeEventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event).unwrap();
        }

        let all = queue.drain_all().unwrap();
        assert_eq!(all.len(), 3);
        assert_eq!(queue.len().unwrap(), 0);
    }

    #[test]
    fn test_thread_safe_queue_clone() {
        let queue = ThreadSafeEventQueue::with_default_size();
        let event = create_test_event(1, FileEventType::Create, 1000);

        queue.add_event(event).unwrap();

        let cloned = queue.clone();
        assert_eq!(cloned.len().unwrap(), 1);
        assert_eq!(cloned.pop().unwrap().unwrap().id, 1);
    }

    #[test]
    fn test_thread_safe_queue_is_empty() {
        let queue = ThreadSafeEventQueue::with_default_size();
        assert!(queue.is_empty().unwrap());

        let event = create_test_event(1, FileEventType::Create, 1000);
        queue.add_event(event).unwrap();
        assert!(!queue.is_empty().unwrap());
    }

    #[test]
    fn test_thread_safe_queue_clear() {
        let queue = ThreadSafeEventQueue::with_default_size();

        for i in 1..=3 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event).unwrap();
        }

        queue.clear().unwrap();
        assert!(queue.is_empty().unwrap());
    }

    #[test]
    fn test_thread_safe_queue_coalescing() {
        let queue = ThreadSafeEventQueue::with_default_size();
        queue.set_coalescing(true).unwrap();

        let event1 = create_test_event(1, FileEventType::Modify, 1000);
        let event2 = create_test_event(2, FileEventType::Modify, 1050);

        queue.add_event(event1).unwrap();
        queue.add_event(event2).unwrap();

        // Second event coalesced
        assert_eq!(queue.len().unwrap(), 1);
    }

    #[test]
    fn test_thread_safe_queue_load_factor() {
        let queue = ThreadSafeEventQueue::new(100);

        for i in 1..=50 {
            let event = create_test_event(i as u64, FileEventType::Create, 1000 + i);
            queue.add_event(event).unwrap();
        }

        let factor = queue.load_factor().unwrap();
        assert!(factor > 0.4 && factor < 0.6);
    }
}
