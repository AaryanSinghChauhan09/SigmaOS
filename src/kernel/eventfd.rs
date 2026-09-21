// Linux-inspired eventfd for event notification via file descriptor
// Inter-process and inter-thread communication for SigmaOS

use std::sync::atomic::{AtomicU64, Ordering};

/// eventfd configuration flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventFdFlags {
    pub non_blocking: bool,
    pub semaphore_semantics: bool,
    pub close_on_exec: bool,
}

impl EventFdFlags {
    pub fn new() -> Self {
        EventFdFlags {
            non_blocking: false,
            semaphore_semantics: false,
            close_on_exec: false,
        }
    }
}

impl Default for EventFdFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// eventfd instance
pub struct EventFd {
    counter: AtomicU64,
    flags: EventFdFlags,
}

impl EventFd {
    pub fn new(initval: u64, flags: EventFdFlags) -> Self {
        EventFd {
            counter: AtomicU64::new(initval),
            flags,
        }
    }

    /// Write to eventfd (adds to counter)
    pub fn write(&self, value: u64) -> Result<(), String> {
        if value == 0 {
            return Err("Cannot write zero to eventfd".to_string());
        }

        // Add value to counter
        let old_value = self.counter.fetch_add(value, Ordering::SeqCst);
        
        // Check for overflow
        if old_value > u64::MAX - value {
            self.counter.fetch_sub(value, Ordering::SeqCst);
            return Err("Counter overflow".to_string());
        }

        Ok(())
    }

    /// Read from eventfd (returns and resets counter)
    pub fn read(&self) -> Result<u64, String> {
        if self.flags.semaphore_semantics {
            // Semaphore semantics: decrement by 1
            let old_value = self.counter.fetch_sub(1, Ordering::SeqCst);
            if old_value == 0 {
                self.counter.fetch_add(1, Ordering::SeqCst);
                return Err("Counter is zero".to_string());
            }
            Ok(1)
        } else {
            // Default semantics: return and reset counter
            let value = self.counter.swap(0, Ordering::SeqCst);
            if value == 0 {
                return Err("Counter is zero".to_string());
            }
            Ok(value)
        }
    }

    /// Get current counter value without modifying it
    pub fn peek(&self) -> u64 {
        self.counter.load(Ordering::SeqCst)
    }

    /// Check if counter is non-zero (readable)
    pub fn is_readable(&self) -> bool {
        self.counter.load(Ordering::SeqCst) > 0
    }

    /// Reset counter to zero
    pub fn reset(&self) {
        self.counter.store(0, Ordering::SeqCst);
    }
}

impl Default for EventFd {
    fn default() -> Self {
        Self::new(0, EventFdFlags::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_fd_creation() {
        let efd = EventFd::new(0, EventFdFlags::new());
        assert_eq!(efd.peek(), 0);
    }

    #[test]
    fn test_event_fd_creation_with_initval() {
        let efd = EventFd::new(10, EventFdFlags::new());
        assert_eq!(efd.peek(), 10);
    }

    #[test]
    fn test_event_fd_write() {
        let efd = EventFd::new(0, EventFdFlags::new());
        efd.write(5).unwrap();
        assert_eq!(efd.peek(), 5);
    }

    #[test]
    fn test_event_fd_write_zero() {
        let efd = EventFd::new(0, EventFdFlags::new());
        let result = efd.write(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_fd_write_multiple() {
        let efd = EventFd::new(0, EventFdFlags::new());
        efd.write(5).unwrap();
        efd.write(3).unwrap();
        assert_eq!(efd.peek(), 8);
    }

    #[test]
    fn test_event_fd_read() {
        let efd = EventFd::new(0, EventFdFlags::new());
        efd.write(5).unwrap();
        
        let value = efd.read().unwrap();
        assert_eq!(value, 5);
        assert_eq!(efd.peek(), 0);
    }

    #[test]
    fn test_event_fd_read_zero() {
        let efd = EventFd::new(0, EventFdFlags::new());
        let result = efd.read();
        assert!(result.is_err());
    }

    #[test]
    fn test_event_fd_overflow() {
        let efd = EventFd::new(u64::MAX - 5, EventFdFlags::new());
        let result = efd.write(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_fd_semaphore_semantics() {
        let mut flags = EventFdFlags::new();
        flags.semaphore_semantics = true;
        
        let efd = EventFd::new(5, flags);
        
        // Each read decrements by 1
        assert_eq!(efd.read().unwrap(), 1);
        assert_eq!(efd.read().unwrap(), 1);
        assert_eq!(efd.read().unwrap(), 1);
        assert_eq!(efd.read().unwrap(), 1);
        assert_eq!(efd.read().unwrap(), 1);
        
        // Sixth read should fail
        let result = efd.read();
        assert!(result.is_err());
    }

    #[test]
    fn test_event_fd_is_readable() {
        let efd = EventFd::new(0, EventFdFlags::new());
        assert!(!efd.is_readable());
        
        efd.write(1).unwrap();
        assert!(efd.is_readable());
    }

    #[test]
    fn test_event_fd_reset() {
        let efd = EventFd::new(0, EventFdFlags::new());
        efd.write(10).unwrap();
        
        efd.reset();
        assert_eq!(efd.peek(), 0);
    }
}
