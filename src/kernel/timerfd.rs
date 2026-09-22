// Linux-inspired timerfd for timer-based file descriptor notifications
// Provides timer events through file descriptors

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Timer clock types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerClock {
    Realtime,
    Monotonic,
    Boottime,
}

/// Timer flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl TimerFlags {
    pub fn new() -> Self {
        Self {
            non_blocking: false,
            close_on_exec: false,
        }
    }

    pub fn with_non_blocking(mut self, value: bool) -> Self {
        self.non_blocking = value;
        self
    }

    pub fn with_close_on_exec(mut self, value: bool) -> Self {
        self.close_on_exec = value;
        self
    }
}

impl Default for TimerFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer specification
#[derive(Debug, Clone, Copy)]
pub struct TimerSpec {
    pub interval_sec: u64,
    pub interval_nsec: u32,
    pub value_sec: u64,
    pub value_nsec: u32,
}

impl TimerSpec {
    pub fn new() -> Self {
        Self {
            interval_sec: 0,
            interval_nsec: 0,
            value_sec: 0,
            value_nsec: 0,
        }
    }

    /// Check if timer is periodic
    pub fn is_periodic(&self) -> bool {
        self.interval_sec > 0 || self.interval_nsec > 0
    }

    /// Get interval in nanoseconds
    pub fn interval_nanos(&self) -> u128 {
        (self.interval_sec as u128) * 1_000_000_000 + (self.interval_nsec as u128)
    }

    /// Get initial value in nanoseconds
    pub fn value_nanos(&self) -> u128 {
        (self.value_sec as u128) * 1_000_000_000 + (self.value_nsec as u128)
    }
}

impl Default for TimerSpec {
    fn default() -> Self {
        Self::new()
    }
}

/// Timer expiration count
#[derive(Debug, Clone, Copy)]
pub struct TimerExpirations {
    pub count: u64,
}

impl TimerExpirations {
    pub fn new(count: u64) -> Self {
        Self { count }
    }

    pub fn as_u64(&self) -> u64 {
        self.count
    }
}

/// Timer file descriptor
#[derive(Debug, Clone)]
pub struct TimerFd {
    pub id: u64,
    pub clock: TimerClock,
    pub flags: TimerFlags,
    pub spec: TimerSpec,
    pub expirations: u64,
    pub armed: bool,
}

impl TimerFd {
    pub fn new(id: u64, clock: TimerClock, flags: TimerFlags) -> Self {
        Self {
            id,
            clock,
            flags,
            spec: TimerSpec::new(),
            expirations: 0,
            armed: false,
        }
    }

    /// Set timer specification
    pub fn set_time(&mut self, spec: TimerSpec) {
        self.spec = spec;
        self.armed = true;
    }

    /// Get timer specification
    pub fn get_time(&self) -> TimerSpec {
        self.spec
    }

    /// Disarm timer
    pub fn disarm(&mut self) {
        self.armed = false;
        self.spec = TimerSpec::new();
    }

    /// Check if armed
    pub fn is_armed(&self) -> bool {
        self.armed
    }

    /// Read expirations (resets count)
    pub fn read_expirations(&mut self) -> TimerExpirations {
        let expirations = self.expirations;
        self.expirations = 0;
        TimerExpirations::new(expirations)
    }

    /// Simulate timer expiration
    pub fn expire(&mut self) {
        if self.armed {
            self.expirations += 1;
        }
    }

    /// Get expiration count without resetting
    pub fn peek_expirations(&self) -> u64 {
        self.expirations
    }
}

/// Timerfd manager for system-wide timerfd management
pub struct TimerFdManager {
    timers: Arc<Mutex<HashMap<u64, TimerFd>>>,
    next_timer_id: Arc<Mutex<u64>>,
}

impl TimerFdManager {
    pub fn new() -> Self {
        Self {
            timers: Arc::new(Mutex::new(HashMap::new())),
            next_timer_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new timerfd
    pub fn create_timer(&self, clock: TimerClock, flags: TimerFlags) -> u64 {
        let mut next_id = self.next_timer_id.lock().unwrap();
        let timer_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let timer = TimerFd::new(timer_id, clock, flags);
        let mut timers = self.timers.lock().unwrap();
        timers.insert(timer_id, timer);

        timer_id
    }

    /// Get a timer by ID
    pub fn get_timer(&self, timer_id: u64) -> Option<TimerFd> {
        let timers = self.timers.lock().unwrap();
        timers.get(&timer_id).cloned()
    }

    /// Remove a timer
    pub fn remove_timer(&self, timer_id: u64) -> Result<(), String> {
        let mut timers = self.timers.lock().unwrap();
        match timers.remove(&timer_id) {
            Some(_) => Ok(()),
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Set timer time
    pub fn set_time(&self, timer_id: u64, spec: TimerSpec) -> Result<(), String> {
        let mut timers = self.timers.lock().unwrap();
        match timers.get_mut(&timer_id) {
            Some(timer) => {
                timer.set_time(spec);
                Ok(())
            }
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Get timer time
    pub fn get_time(&self, timer_id: u64) -> Result<TimerSpec, String> {
        let timers = self.timers.lock().unwrap();
        match timers.get(&timer_id) {
            Some(timer) => Ok(timer.get_time()),
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Disarm timer
    pub fn disarm(&self, timer_id: u64) -> Result<(), String> {
        let mut timers = self.timers.lock().unwrap();
        match timers.get_mut(&timer_id) {
            Some(timer) => {
                timer.disarm();
                Ok(())
            }
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Read expirations
    pub fn read(&self, timer_id: u64) -> Result<TimerExpirations, String> {
        let mut timers = self.timers.lock().unwrap();
        match timers.get_mut(&timer_id) {
            Some(timer) => Ok(timer.read_expirations()),
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Simulate timer expiration (for testing)
    pub fn expire(&self, timer_id: u64) -> Result<(), String> {
        let mut timers = self.timers.lock().unwrap();
        match timers.get_mut(&timer_id) {
            Some(timer) => {
                timer.expire();
                Ok(())
            }
            None => Err(format!("Timer {} not found", timer_id)),
        }
    }

    /// Get timer count
    pub fn timer_count(&self) -> usize {
        let timers = self.timers.lock().unwrap();
        timers.len()
    }
}

impl Default for TimerFdManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_spec() {
        let spec = TimerSpec::new();
        assert!(!spec.is_periodic());
        assert_eq!(spec.interval_nanos(), 0);
    }

    #[test]
    fn test_timer_spec_periodic() {
        let mut spec = TimerSpec::new();
        spec.interval_sec = 1;
        assert!(spec.is_periodic());
        assert_eq!(spec.interval_nanos(), 1_000_000_000);
    }

    #[test]
    fn test_timer_flags() {
        let flags = TimerFlags::new()
            .with_non_blocking(true)
            .with_close_on_exec(true);

        assert!(flags.non_blocking);
        assert!(flags.close_on_exec);
    }

    #[test]
    fn test_timer_fd() {
        let timer = TimerFd::new(1, TimerClock::Monotonic, TimerFlags::new());
        assert_eq!(timer.id, 1);
        assert!(!timer.is_armed());
        assert_eq!(timer.peek_expirations(), 0);
    }

    #[test]
    fn test_timer_fd_set_time() {
        let mut timer = TimerFd::new(1, TimerClock::Monotonic, TimerFlags::new());
        let spec = TimerSpec::new();
        timer.set_time(spec);
        assert!(timer.is_armed());
    }

    #[test]
    fn test_timer_fd_disarm() {
        let mut timer = TimerFd::new(1, TimerClock::Monotonic, TimerFlags::new());
        let spec = TimerSpec::new();
        timer.set_time(spec);
        timer.disarm();
        assert!(!timer.is_armed());
    }

    #[test]
    fn test_timer_fd_expirations() {
        let mut timer = TimerFd::new(1, TimerClock::Monotonic, TimerFlags::new());
        timer.expire();
        timer.expire();

        assert_eq!(timer.peek_expirations(), 2);

        let expirations = timer.read_expirations();
        assert_eq!(expirations.as_u64(), 2);
        assert_eq!(timer.peek_expirations(), 0);
    }

    #[test]
    fn test_timer_fd_manager() {
        let manager = TimerFdManager::new();

        let timer_id = manager.create_timer(TimerClock::Monotonic, TimerFlags::new());
        assert_eq!(timer_id, 1);

        let spec = TimerSpec::new();
        manager.set_time(timer_id, spec).unwrap();

        let retrieved = manager.get_timer(timer_id).unwrap();
        assert!(retrieved.is_armed());
    }

    #[test]
    fn test_timer_fd_manager_expire() {
        let manager = TimerFdManager::new();

        let timer_id = manager.create_timer(TimerClock::Monotonic, TimerFlags::new());
        let spec = TimerSpec::new();
        manager.set_time(timer_id, spec).unwrap();

        manager.expire(timer_id).unwrap();
        manager.expire(timer_id).unwrap();

        let expirations = manager.read(timer_id).unwrap();
        assert_eq!(expirations.as_u64(), 2);
    }

    #[test]
    fn test_timer_fd_manager_disarm() {
        let manager = TimerFdManager::new();

        let timer_id = manager.create_timer(TimerClock::Monotonic, TimerFlags::new());
        let spec = TimerSpec::new();
        manager.set_time(timer_id, spec).unwrap();

        manager.disarm(timer_id).unwrap();

        let timer = manager.get_timer(timer_id).unwrap();
        assert!(!timer.is_armed());
    }

    #[test]
    fn test_timer_fd_manager_remove() {
        let manager = TimerFdManager::new();

        let timer_id = manager.create_timer(TimerClock::Monotonic, TimerFlags::new());
        manager.remove_timer(timer_id).unwrap();

        assert_eq!(manager.timer_count(), 0);
    }

    #[test]
    fn test_timer_fd_manager_invalid() {
        let manager = TimerFdManager::new();
        assert!(manager.set_time(999, TimerSpec::new()).is_err());
        assert!(manager.read(999).is_err());
    }
}
