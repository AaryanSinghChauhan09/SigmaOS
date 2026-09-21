// Linux-inspired signalfd for signal-based file descriptor notifications
// Provides signal events through file descriptors

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Signal flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SignalFdFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl SignalFdFlags {
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

impl Default for SignalFdFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal mask
#[derive(Debug, Clone, Copy)]
pub struct SignalMask {
    pub bits: u64,
}

impl SignalMask {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    /// Add a signal to the mask
    pub fn add(&mut self, signal: u32) {
        self.bits |= 1 << signal;
    }

    /// Remove a signal from the mask
    pub fn remove(&mut self, signal: u32) {
        self.bits &= !(1 << signal);
    }

    /// Check if signal is in mask
    pub fn contains(&self, signal: u32) -> bool {
        (self.bits & (1 << signal)) != 0
    }

    /// Clear all signals
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}

impl Default for SignalMask {
    fn default() -> Self {
        Self::new()
    }
}

/// Signal information
#[derive(Debug, Clone, Copy)]
pub struct SignalInfo {
    pub signo: u32,
    pub errno: i32,
    pub code: u32,
    pub pid: u32,
    pub uid: u32,
    pub value: i32,
}

impl SignalInfo {
    pub fn new(signo: u32) -> Self {
        Self {
            signo,
            errno: 0,
            code: 0,
            pid: 0,
            uid: 0,
            value: 0,
        }
    }

    pub fn with_errno(mut self, errno: i32) -> Self {
        self.errno = errno;
        self
    }

    pub fn with_code(mut self, code: u32) -> Self {
        self.code = code;
        self
    }

    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = pid;
        self
    }

    pub fn with_uid(mut self, uid: u32) -> Self {
        self.uid = uid;
        self
    }

    pub fn with_value(mut self, value: i32) -> Self {
        self.value = value;
        self
    }
}

/// Signal file descriptor
#[derive(Debug, Clone)]
pub struct SignalFd {
    pub id: u64,
    pub flags: SignalFdFlags,
    pub mask: SignalMask,
    pub signals: Vec<SignalInfo>,
}

impl SignalFd {
    pub fn new(id: u64, flags: SignalFdFlags, mask: SignalMask) -> Self {
        Self {
            id,
            flags,
            mask,
            signals: Vec::new(),
        }
    }

    /// Set signal mask
    pub fn set_mask(&mut self, mask: SignalMask) {
        self.mask = mask;
    }

    /// Get signal mask
    pub fn get_mask(&self) -> SignalMask {
        self.mask
    }

    /// Add a signal (simulates signal delivery)
    pub fn add_signal(&mut self, info: SignalInfo) {
        if self.mask.contains(info.signo) {
            self.signals.push(info);
        }
    }

    /// Read signals (clears the queue)
    pub fn read(&mut self) -> Vec<SignalInfo> {
        let signals = self.signals.clone();
        self.signals.clear();
        signals
    }

    /// Peek at signals without clearing
    pub fn peek(&self) -> &[SignalInfo] {
        &self.signals
    }

    /// Get signal count
    pub fn signal_count(&self) -> usize {
        self.signals.len()
    }
}

/// Signalfd manager for system-wide signalfd management
pub struct SignalFdManager {
    signal_fds: Arc<Mutex<HashMap<u64, SignalFd>>>,
    next_fd_id: Arc<Mutex<u64>>,
}

impl SignalFdManager {
    pub fn new() -> Self {
        Self {
            signal_fds: Arc::new(Mutex::new(HashMap::new())),
            next_fd_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new signalfd
    pub fn create_signal_fd(&self, flags: SignalFdFlags, mask: SignalMask) -> u64 {
        let mut next_id = self.next_fd_id.lock().unwrap();
        let fd_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let signal_fd = SignalFd::new(fd_id, flags, mask);
        let mut signal_fds = self.signal_fds.lock().unwrap();
        signal_fds.insert(fd_id, signal_fd);

        fd_id
    }

    /// Get a signalfd by ID
    pub fn get_signal_fd(&self, fd_id: u64) -> Option<SignalFd> {
        let signal_fds = self.signal_fds.lock().unwrap();
        signal_fds.get(&fd_id).cloned()
    }

    /// Remove a signalfd
    pub fn remove_signal_fd(&self, fd_id: u64) -> Result<(), String> {
        let mut signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.remove(&fd_id) {
            Some(_) => Ok(()),
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Set signal mask
    pub fn set_mask(&self, fd_id: u64, mask: SignalMask) -> Result<(), String> {
        let mut signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.get_mut(&fd_id) {
            Some(signal_fd) => {
                signal_fd.set_mask(mask);
                Ok(())
            }
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Get signal mask
    pub fn get_mask(&self, fd_id: u64) -> Result<SignalMask, String> {
        let signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.get(&fd_id) {
            Some(signal_fd) => Ok(signal_fd.get_mask()),
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Read signals
    pub fn read(&self, fd_id: u64) -> Result<Vec<SignalInfo>, String> {
        let mut signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.get_mut(&fd_id) {
            Some(signal_fd) => Ok(signal_fd.read()),
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Add a signal (simulates signal delivery)
    pub fn add_signal(&self, fd_id: u64, info: SignalInfo) -> Result<(), String> {
        let mut signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.get_mut(&fd_id) {
            Some(signal_fd) => {
                signal_fd.add_signal(info);
                Ok(())
            }
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Get signal count
    pub fn signal_count(&self, fd_id: u64) -> Result<usize, String> {
        let signal_fds = self.signal_fds.lock().unwrap();
        match signal_fds.get(&fd_id) {
            Some(signal_fd) => Ok(signal_fd.signal_count()),
            None => Err(format!("Signal fd {} not found", fd_id)),
        }
    }

    /// Get fd count
    pub fn fd_count(&self) -> usize {
        let signal_fds = self.signal_fds.lock().unwrap();
        signal_fds.len()
    }
}

impl Default for SignalFdManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_mask() {
        let mut mask = SignalMask::new();
        assert!(!mask.contains(1));

        mask.add(1);
        assert!(mask.contains(1));

        mask.remove(1);
        assert!(!mask.contains(1));
    }

    #[test]
    fn test_signal_mask_multiple() {
        let mut mask = SignalMask::new();
        mask.add(1);
        mask.add(2);
        mask.add(3);

        assert!(mask.contains(1));
        assert!(mask.contains(2));
        assert!(mask.contains(3));
    }

    #[test]
    fn test_signal_info() {
        let info = SignalInfo::new(9)
            .with_errno(1)
            .with_code(2)
            .with_pid(100)
            .with_uid(1000)
            .with_value(42);

        assert_eq!(info.signo, 9);
        assert_eq!(info.errno, 1);
        assert_eq!(info.code, 2);
        assert_eq!(info.pid, 100);
        assert_eq!(info.uid, 1000);
        assert_eq!(info.value, 42);
    }

    #[test]
    fn test_signal_fd() {
        let flags = SignalFdFlags::new();
        let mask = SignalMask::new();
        let signal_fd = SignalFd::new(1, flags, mask);

        assert_eq!(signal_fd.id, 1);
        assert_eq!(signal_fd.signal_count(), 0);
    }

    #[test]
    fn test_signal_fd_add_signal() {
        let flags = SignalFdFlags::new();
        let mut mask = SignalMask::new();
        mask.add(9);
        let mut signal_fd = SignalFd::new(1, flags, mask);

        let info = SignalInfo::new(9);
        signal_fd.add_signal(info);

        assert_eq!(signal_fd.signal_count(), 1);
    }

    #[test]
    fn test_signal_fd_add_signal_not_in_mask() {
        let flags = SignalFdFlags::new();
        let mask = SignalMask::new();
        let mut signal_fd = SignalFd::new(1, flags, mask);

        let info = SignalInfo::new(9);
        signal_fd.add_signal(info);

        assert_eq!(signal_fd.signal_count(), 0);
    }

    #[test]
    fn test_signal_fd_read() {
        let flags = SignalFdFlags::new();
        let mut mask = SignalMask::new();
        mask.add(9);
        let mut signal_fd = SignalFd::new(1, flags, mask);

        let info = SignalInfo::new(9);
        signal_fd.add_signal(info);
        signal_fd.add_signal(info);

        let signals = signal_fd.read();
        assert_eq!(signals.len(), 2);
        assert_eq!(signal_fd.signal_count(), 0);
    }

    #[test]
    fn test_signal_fd_manager() {
        let manager = SignalFdManager::new();

        let flags = SignalFdFlags::new();
        let mask = SignalMask::new();
        let fd_id = manager.create_signal_fd(flags, mask);

        assert_eq!(fd_id, 1);
        assert_eq!(manager.fd_count(), 1);
    }

    #[test]
    fn test_signal_fd_manager_add_signal() {
        let manager = SignalFdManager::new();

        let flags = SignalFdFlags::new();
        let mut mask = SignalMask::new();
        mask.add(9);
        let fd_id = manager.create_signal_fd(flags, mask);

        let info = SignalInfo::new(9);
        manager.add_signal(fd_id, info).unwrap();

        assert_eq!(manager.signal_count(fd_id).unwrap(), 1);
    }

    #[test]
    fn test_signal_fd_manager_set_mask() {
        let manager = SignalFdManager::new();

        let flags = SignalFdFlags::new();
        let mask = SignalMask::new();
        let fd_id = manager.create_signal_fd(flags, mask);

        let mut new_mask = SignalMask::new();
        new_mask.add(10);
        manager.set_mask(fd_id, new_mask).unwrap();

        let retrieved = manager.get_mask(fd_id).unwrap();
        assert!(retrieved.contains(10));
    }

    #[test]
    fn test_signal_fd_manager_remove() {
        let manager = SignalFdManager::new();

        let flags = SignalFdFlags::new();
        let mask = SignalMask::new();
        let fd_id = manager.create_signal_fd(flags, mask);

        manager.remove_signal_fd(fd_id).unwrap();
        assert_eq!(manager.fd_count(), 0);
    }

    #[test]
    fn test_signal_fd_manager_invalid() {
        let manager = SignalFdManager::new();
        assert!(manager.set_mask(999, SignalMask::new()).is_err());
        assert!(manager.read(999).is_err());
    }
}
