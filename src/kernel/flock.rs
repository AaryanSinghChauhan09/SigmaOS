// Linux-inspired file locking (flock)
// Provides advisory file locking

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Lock type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    Shared = 1,    // Read lock
    Exclusive = 2, // Write lock
}

/// Lock operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockOp {
    LockShared = 1,
    LockExclusive = 2,
    Unlock = 8,
}

/// File lock
#[derive(Debug, Clone)]
pub struct FileLock {
    pub fd: i32,
    pub lock_type: LockType,
    pub owner: u32,
    pub acquired: bool,
}

impl FileLock {
    pub fn new(fd: i32, lock_type: LockType, owner: u32) -> Self {
        Self {
            fd,
            lock_type,
            owner,
            acquired: false,
        }
    }

    /// Acquire lock
    pub fn acquire(&mut self) -> Result<(), String> {
        if self.acquired {
            return Err("Lock already acquired".to_string());
        }
        self.acquired = true;
        Ok(())
    }

    /// Release lock
    pub fn release(&mut self) {
        self.acquired = false;
    }

    /// Check if locked
    pub fn is_locked(&self) -> bool {
        self.acquired
    }
}

/// File lock manager for system-wide file lock management
pub struct FileLockManager {
    pub locks: Arc<Mutex<HashMap<i32, FileLock>>>,
    pub next_owner: Arc<Mutex<u32>>,
}

impl FileLockManager {
    pub fn new() -> Self {
        Self {
            locks: Arc::new(Mutex::new(HashMap::new())),
            next_owner: Arc::new(Mutex::new(1)),
        }
    }

    /// Create lock
    pub fn create_lock(&self, fd: i32, lock_type: LockType) -> u32 {
        let mut next_owner = self.next_owner.lock().unwrap();
        let owner = *next_owner;
        *next_owner += 1;
        drop(next_owner);

        let lock = FileLock::new(fd, lock_type, owner);
        let mut locks = self.locks.lock().unwrap();
        locks.insert(fd, lock);

        owner
    }

    /// Get lock
    pub fn get_lock(&self, fd: i32) -> Option<FileLock> {
        let locks = self.locks.lock().unwrap();
        locks.get(&fd).cloned()
    }

    /// Remove lock
    pub fn remove_lock(&self, fd: i32) -> Result<(), String> {
        let mut locks = self.locks.lock().unwrap();
        match locks.remove(&fd) {
            Some(_) => Ok(()),
            None => Err(format!("Lock for fd {} not found", fd)),
        }
    }

    /// Acquire lock
    pub fn acquire(&self, fd: i32) -> Result<(), String> {
        let mut locks = self.locks.lock().unwrap();
        match locks.get_mut(&fd) {
            Some(lock) => lock.acquire(),
            None => Err(format!("Lock for fd {} not found", fd)),
        }
    }

    /// Release lock
    pub fn release(&self, fd: i32) -> Result<(), String> {
        let mut locks = self.locks.lock().unwrap();
        match locks.get_mut(&fd) {
            Some(lock) => {
                lock.release();
                Ok(())
            }
            None => Err(format!("Lock for fd {} not found", fd)),
        }
    }

    /// Perform lock operation
    pub fn flock(&self, fd: i32, op: LockOp) -> Result<(), String> {
        match op {
            LockOp::LockShared => {
                let mut locks = self.locks.lock().unwrap();
                match locks.get_mut(&fd) {
                    Some(lock) => {
                        lock.lock_type = LockType::Shared;
                        lock.acquire()
                    }
                    None => Err(format!("Lock for fd {} not found", fd)),
                }
            }
            LockOp::LockExclusive => {
                let mut locks = self.locks.lock().unwrap();
                match locks.get_mut(&fd) {
                    Some(lock) => {
                        lock.lock_type = LockType::Exclusive;
                        lock.acquire()
                    }
                    None => Err(format!("Lock for fd {} not found", fd)),
                }
            }
            LockOp::Unlock => self.release(fd),
        }
    }

    /// Get lock count
    pub fn lock_count(&self) -> usize {
        let locks = self.locks.lock().unwrap();
        locks.len()
    }
}

impl Default for FileLockManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_lock() {
        let lock = FileLock::new(3, LockType::Shared, 1);
        assert_eq!(lock.fd, 3);
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_file_lock_acquire() {
        let mut lock = FileLock::new(3, LockType::Shared, 1);
        lock.acquire().unwrap();
        assert!(lock.is_locked());
    }

    #[test]
    fn test_file_lock_acquire_twice() {
        let mut lock = FileLock::new(3, LockType::Shared, 1);
        lock.acquire().unwrap();
        assert!(lock.acquire().is_err());
    }

    #[test]
    fn test_file_lock_release() {
        let mut lock = FileLock::new(3, LockType::Shared, 1);
        lock.acquire().unwrap();
        lock.release();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_file_lock_manager() {
        let manager = FileLockManager::new();

        let owner = manager.create_lock(3, LockType::Shared);
        assert_eq!(owner, 1);
        assert_eq!(manager.lock_count(), 1);
    }

    #[test]
    fn test_file_lock_manager_acquire() {
        let manager = FileLockManager::new();

        manager.create_lock(3, LockType::Shared);
        manager.acquire(3).unwrap();

        let lock = manager.get_lock(3).unwrap();
        assert!(lock.is_locked());
    }

    #[test]
    fn test_file_lock_manager_release() {
        let manager = FileLockManager::new();

        manager.create_lock(3, LockType::Shared);
        manager.acquire(3).unwrap();
        manager.release(3).unwrap();

        let lock = manager.get_lock(3).unwrap();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_file_lock_manager_flock_shared() {
        let manager = FileLockManager::new();

        manager.create_lock(3, LockType::Exclusive);
        manager.flock(3, LockOp::LockShared).unwrap();

        let lock = manager.get_lock(3).unwrap();
        assert_eq!(lock.lock_type, LockType::Shared);
    }

    #[test]
    fn test_file_lock_manager_flock_exclusive() {
        let manager = FileLockManager::new();

        manager.create_lock(3, LockType::Shared);
        manager.flock(3, LockOp::LockExclusive).unwrap();

        let lock = manager.get_lock(3).unwrap();
        assert_eq!(lock.lock_type, LockType::Exclusive);
    }

    #[test]
    fn test_file_lock_manager_flock_unlock() {
        let manager = FileLockManager::new();

        manager.create_lock(3, LockType::Shared);
        manager.acquire(3).unwrap();
        manager.flock(3, LockOp::Unlock).unwrap();

        let lock = manager.get_lock(3).unwrap();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_file_lock_manager_invalid() {
        let manager = FileLockManager::new();
        assert!(manager.get_lock(999).is_none());
        assert!(manager.acquire(999).is_err());
    }
}
