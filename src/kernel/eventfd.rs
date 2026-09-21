// Linux-inspired eventfd for event notification
// Provides file descriptor-based event counting

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Eventfd flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventFdFlags {
    pub non_blocking: bool,
    pub semaphore: bool,
}

impl EventFdFlags {
    pub fn new() -> Self {
        Self {
            non_blocking: false,
            semaphore: false,
        }
    }

    pub fn with_non_blocking(mut self, value: bool) -> Self {
        self.non_blocking = value;
        self
    }

    pub fn with_semaphore(mut self, value: bool) -> Self {
        self.semaphore = value;
        self
    }
}

impl Default for EventFdFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Eventfd instance
#[derive(Debug, Clone)]
pub struct EventFd {
    pub id: u64,
    pub flags: EventFdFlags,
    pub counter: u64,
}

impl EventFd {
    pub fn new(id: u64, flags: EventFdFlags, initval: u64) -> Self {
        Self {
            id,
            flags,
            counter: initval,
        }
    }

    /// Read from eventfd (decrements counter)
    pub fn read(&mut self) -> Result<u64, String> {
        if self.counter == 0 {
            if self.flags.non_blocking {
                return Err("Would block".to_string());
            }
            return Err("Counter is zero".to_string());
        }

        if self.flags.semaphore {
            // Semaphore mode: read 1
            self.counter -= 1;
            Ok(1)
        } else {
            // Counter mode: read full counter
            let value = self.counter;
            self.counter = 0;
            Ok(value)
        }
    }

    /// Write to eventfd (adds to counter)
    pub fn write(&mut self, value: u64) -> Result<(), String> {
        let max = u64::MAX - self.counter;
        if value > max {
            return Err("Counter overflow".to_string());
        }

        self.counter += value;
        Ok(())
    }

    /// Get current counter value
    pub fn get_counter(&self) -> u64 {
        self.counter
    }

    /// Check if readable
    pub fn is_readable(&self) -> bool {
        self.counter > 0
    }
}

/// Eventfd manager for system-wide eventfd management
pub struct EventFdManager {
    event_fds: Arc<Mutex<HashMap<u64, EventFd>>>,
    next_fd_id: Arc<Mutex<u64>>,
}

impl EventFdManager {
    pub fn new() -> Self {
        Self {
            event_fds: Arc::new(Mutex::new(HashMap::new())),
            next_fd_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new eventfd
    pub fn create_event_fd(&self, flags: EventFdFlags, initval: u64) -> u64 {
        let mut next_id = self.next_fd_id.lock().unwrap();
        let fd_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let event_fd = EventFd::new(fd_id, flags, initval);
        let mut event_fds = self.event_fds.lock().unwrap();
        event_fds.insert(fd_id, event_fd);

        fd_id
    }

    /// Get an eventfd by ID
    pub fn get_event_fd(&self, fd_id: u64) -> Option<EventFd> {
        let event_fds = self.event_fds.lock().unwrap();
        event_fds.get(&fd_id).cloned()
    }

    /// Remove an eventfd
    pub fn remove_event_fd(&self, fd_id: u64) -> Result<(), String> {
        let mut event_fds = self.event_fds.lock().unwrap();
        match event_fds.remove(&fd_id) {
            Some(_) => Ok(()),
            None => Err(format!("Event fd {} not found", fd_id)),
        }
    }

    /// Read from eventfd
    pub fn read(&self, fd_id: u64) -> Result<u64, String> {
        let mut event_fds = self.event_fds.lock().unwrap();
        match event_fds.get_mut(&fd_id) {
            Some(event_fd) => event_fd.read(),
            None => Err(format!("Event fd {} not found", fd_id)),
        }
    }

    /// Write to eventfd
    pub fn write(&self, fd_id: u64, value: u64) -> Result<(), String> {
        let mut event_fds = self.event_fds.lock().unwrap();
        match event_fds.get_mut(&fd_id) {
            Some(event_fd) => event_fd.write(value),
            None => Err(format!("Event fd {} not found", fd_id)),
        }
    }

    /// Get counter value
    pub fn get_counter(&self, fd_id: u64) -> Result<u64, String> {
        let event_fds = self.event_fds.lock().unwrap();
        match event_fds.get(&fd_id) {
            Some(event_fd) => Ok(event_fd.get_counter()),
            None => Err(format!("Event fd {} not found", fd_id)),
        }
    }

    /// Check if readable
    pub fn is_readable(&self, fd_id: u64) -> Result<bool, String> {
        let event_fds = self.event_fds.lock().unwrap();
        match event_fds.get(&fd_id) {
            Some(event_fd) => Ok(event_fd.is_readable()),
            None => Err(format!("Event fd {} not found", fd_id)),
        }
    }

    /// Get fd count
    pub fn fd_count(&self) -> usize {
        let event_fds = self.event_fds.lock().unwrap();
        event_fds.len()
    }
}

impl Default for EventFdManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_fd_flags() {
        let flags = EventFdFlags::new().with_non_blocking(true).with_semaphore(true);
        assert!(flags.non_blocking);
        assert!(flags.semaphore);
    }

    #[test]
    fn test_event_fd() {
        let flags = EventFdFlags::new();
        let event_fd = EventFd::new(1, flags, 0);

        assert_eq!(event_fd.id, 1);
        assert_eq!(event_fd.counter, 0);
    }

    #[test]
    fn test_event_fd_read_counter_mode() {
        let flags = EventFdFlags::new();
        let mut event_fd = EventFd::new(1, flags, 5);

        let value = event_fd.read().unwrap();
        assert_eq!(value, 5);
        assert_eq!(event_fd.counter, 0);
    }

    #[test]
    fn test_event_fd_read_semaphore_mode() {
        let flags = EventFdFlags::new().with_semaphore(true);
        let mut event_fd = EventFd::new(1, flags, 5);

        let value = event_fd.read().unwrap();
        assert_eq!(value, 1);
        assert_eq!(event_fd.counter, 4);
    }

    #[test]
    fn test_event_fd_read_zero() {
        let flags = EventFdFlags::new();
        let mut event_fd = EventFd::new(1, flags, 0);

        assert!(event_fd.read().is_err());
    }

    #[test]
    fn test_event_fd_read_zero_non_blocking() {
        let flags = EventFdFlags::new().with_non_blocking(true);
        let mut event_fd = EventFd::new(1, flags, 0);

        assert!(event_fd.read().is_err());
    }

    #[test]
    fn test_event_fd_write() {
        let flags = EventFdFlags::new();
        let mut event_fd = EventFd::new(1, flags, 0);

        event_fd.write(10).unwrap();
        assert_eq!(event_fd.counter, 10);
    }

    #[test]
    fn test_event_fd_write_overflow() {
        let flags = EventFdFlags::new();
        let mut event_fd = EventFd::new(1, flags, u64::MAX - 5);

        assert!(event_fd.write(10).is_err());
    }

    #[test]
    fn test_event_fd_is_readable() {
        let flags = EventFdFlags::new();
        let event_fd = EventFd::new(1, flags, 5);

        assert!(event_fd.is_readable());
    }

    #[test]
    fn test_event_fd_is_readable_zero() {
        let flags = EventFdFlags::new();
        let event_fd = EventFd::new(1, flags, 0);

        assert!(!event_fd.is_readable());
    }

    #[test]
    fn test_event_fd_manager() {
        let manager = EventFdManager::new();

        let flags = EventFdFlags::new();
        let fd_id = manager.create_event_fd(flags, 0);

        assert_eq!(fd_id, 1);
        assert_eq!(manager.fd_count(), 1);
    }

    #[test]
    fn test_event_fd_manager_write_read() {
        let manager = EventFdManager::new();

        let flags = EventFdFlags::new();
        let fd_id = manager.create_event_fd(flags, 0);

        manager.write(fd_id, 42).unwrap();
        let value = manager.read(fd_id).unwrap();

        assert_eq!(value, 42);
    }

    #[test]
    fn test_event_fd_manager_semaphore_mode() {
        let manager = EventFdManager::new();

        let flags = EventFdFlags::new().with_semaphore(true);
        let fd_id = manager.create_event_fd(flags, 5);

        let value1 = manager.read(fd_id).unwrap();
        let value2 = manager.read(fd_id).unwrap();

        assert_eq!(value1, 1);
        assert_eq!(value2, 1);
        assert_eq!(manager.get_counter(fd_id).unwrap(), 3);
    }

    #[test]
    fn test_event_fd_manager_remove() {
        let manager = EventFdManager::new();

        let flags = EventFdFlags::new();
        let fd_id = manager.create_event_fd(flags, 0);

        manager.remove_event_fd(fd_id).unwrap();
        assert_eq!(manager.fd_count(), 0);
    }

    #[test]
    fn test_event_fd_manager_invalid() {
        let manager = EventFdManager::new();
        assert!(manager.read(999).is_err());
        assert!(manager.write(999, 10).is_err());
    }
}
