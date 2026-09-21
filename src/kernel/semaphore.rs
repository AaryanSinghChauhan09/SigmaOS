// Linux-inspired IPC Semaphores
// Provides counting semaphores for inter-process synchronization

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Semaphore operation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemaphoreResult {
    Success,
    WouldBlock,
    InvalidValue,
    Timeout,
}

/// Semaphore
#[derive(Debug, Clone)]
pub struct Semaphore {
    pub id: u64,
    pub value: i32,
    pub max_value: i32,
}

impl Semaphore {
    pub fn new(id: u64, initial_value: i32, max_value: i32) -> Self {
        Self {
            id,
            value: initial_value,
            max_value,
        }
    }

    /// Wait (decrement) operation
    pub fn wait(&mut self) -> SemaphoreResult {
        if self.value > 0 {
            self.value -= 1;
            SemaphoreResult::Success
        } else {
            SemaphoreResult::WouldBlock
        }
    }

    /// Try wait (non-blocking decrement)
    pub fn try_wait(&mut self) -> SemaphoreResult {
        if self.value > 0 {
            self.value -= 1;
            SemaphoreResult::Success
        } else {
            SemaphoreResult::WouldBlock
        }
    }

    /// Post (increment) operation
    pub fn post(&mut self) -> SemaphoreResult {
        if self.value < self.max_value {
            self.value += 1;
            SemaphoreResult::Success
        } else {
            SemaphoreResult::InvalidValue
        }
    }

    /// Get current value
    pub fn get_value(&self) -> i32 {
        self.value
    }

    /// Check if resource is available
    pub fn is_available(&self) -> bool {
        self.value > 0
    }
}

/// Semaphore set (System V style)
#[derive(Debug, Clone)]
pub struct SemaphoreSet {
    pub id: u64,
    pub key: i32,
    pub semaphores: Arc<Mutex<HashMap<u64, Semaphore>>>,
    pub next_sem_id: Arc<Mutex<u64>>,
}

impl SemaphoreSet {
    pub fn new(id: u64, key: i32) -> Self {
        Self {
            id,
            key,
            semaphores: Arc::new(Mutex::new(HashMap::new())),
            next_sem_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a semaphore in the set
    pub fn create_semaphore(&self, initial_value: i32, max_value: i32) -> u64 {
        let mut next_id = self.next_sem_id.lock().unwrap();
        let sem_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let semaphore = Semaphore::new(sem_id, initial_value, max_value);
        let mut semaphores = self.semaphores.lock().unwrap();
        semaphores.insert(sem_id, semaphore);

        sem_id
    }

    /// Get a semaphore by ID
    pub fn get_semaphore(&self, sem_id: u64) -> Option<Semaphore> {
        let semaphores = self.semaphores.lock().unwrap();
        semaphores.get(&sem_id).cloned()
    }

    /// Remove a semaphore
    pub fn remove_semaphore(&self, sem_id: u64) -> Result<(), String> {
        let mut semaphores = self.semaphores.lock().unwrap();
        match semaphores.remove(&sem_id) {
            Some(_) => Ok(()),
            None => Err(format!("Semaphore {} not found", sem_id)),
        }
    }

    /// Get semaphore count
    pub fn semaphore_count(&self) -> usize {
        let semaphores = self.semaphores.lock().unwrap();
        semaphores.len()
    }
}

/// Semaphore manager for system-wide semaphore management
pub struct SemaphoreManager {
    semaphore_sets: Arc<Mutex<HashMap<u64, SemaphoreSet>>>,
    next_set_id: Arc<Mutex<u64>>,
}

impl SemaphoreManager {
    pub fn new() -> Self {
        Self {
            semaphore_sets: Arc::new(Mutex::new(HashMap::new())),
            next_set_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a semaphore set
    pub fn create_semaphore_set(&self, key: i32) -> u64 {
        let mut next_id = self.next_set_id.lock().unwrap();
        let set_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let set = SemaphoreSet::new(set_id, key);
        let mut sets = self.semaphore_sets.lock().unwrap();
        sets.insert(set_id, set);

        set_id
    }

    /// Get a semaphore set by ID
    pub fn get_semaphore_set(&self, set_id: u64) -> Option<SemaphoreSet> {
        let sets = self.semaphore_sets.lock().unwrap();
        sets.get(&set_id).cloned()
    }

    /// Remove a semaphore set
    pub fn remove_semaphore_set(&self, set_id: u64) -> Result<(), String> {
        let mut sets = self.semaphore_sets.lock().unwrap();
        match sets.remove(&set_id) {
            Some(_) => Ok(()),
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Create semaphore in a set
    pub fn create_semaphore(&self, set_id: u64, initial_value: i32, max_value: i32) -> Result<u64, String> {
        let sets = self.semaphore_sets.lock().unwrap();
        match sets.get(&set_id) {
            Some(set) => Ok(set.create_semaphore(initial_value, max_value)),
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Wait on a semaphore
    pub fn wait(&self, set_id: u64, sem_id: u64) -> Result<SemaphoreResult, String> {
        let sets = self.semaphore_sets.lock().unwrap();
        match sets.get(&set_id) {
            Some(set) => {
                let mut semaphores = set.semaphores.lock().unwrap();
                match semaphores.get_mut(&sem_id) {
                    Some(sem) => Ok(sem.wait()),
                    None => Err(format!("Semaphore {} not found", sem_id)),
                }
            }
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Try wait on a semaphore
    pub fn try_wait(&self, set_id: u64, sem_id: u64) -> Result<SemaphoreResult, String> {
        let sets = self.semaphore_sets.lock().unwrap();
        match sets.get(&set_id) {
            Some(set) => {
                let mut semaphores = set.semaphores.lock().unwrap();
                match semaphores.get_mut(&sem_id) {
                    Some(sem) => Ok(sem.try_wait()),
                    None => Err(format!("Semaphore {} not found", sem_id)),
                }
            }
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Post to a semaphore
    pub fn post(&self, set_id: u64, sem_id: u64) -> Result<SemaphoreResult, String> {
        let sets = self.semaphore_sets.lock().unwrap();
        match sets.get(&set_id) {
            Some(set) => {
                let mut semaphores = set.semaphores.lock().unwrap();
                match semaphores.get_mut(&sem_id) {
                    Some(sem) => Ok(sem.post()),
                    None => Err(format!("Semaphore {} not found", sem_id)),
                }
            }
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Get semaphore value
    pub fn get_value(&self, set_id: u64, sem_id: u64) -> Result<i32, String> {
        let sets = self.semaphore_sets.lock().unwrap();
        match sets.get(&set_id) {
            Some(set) => {
                let semaphores = set.semaphores.lock().unwrap();
                match semaphores.get(&sem_id) {
                    Some(sem) => Ok(sem.get_value()),
                    None => Err(format!("Semaphore {} not found", sem_id)),
                }
            }
            None => Err(format!("Semaphore set {} not found", set_id)),
        }
    }

    /// Get semaphore set count
    pub fn set_count(&self) -> usize {
        let sets = self.semaphore_sets.lock().unwrap();
        sets.len()
    }
}

impl Default for SemaphoreManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semaphore() {
        let mut semaphore = Semaphore::new(1, 5, 10);
        assert_eq!(semaphore.get_value(), 5);
        assert!(semaphore.is_available());

        assert_eq!(semaphore.wait(), SemaphoreResult::Success);
        assert_eq!(semaphore.get_value(), 4);

        assert_eq!(semaphore.post(), SemaphoreResult::Success);
        assert_eq!(semaphore.get_value(), 5);
    }

    #[test]
    fn test_semaphore_would_block() {
        let mut semaphore = Semaphore::new(1, 0, 10);
        assert_eq!(semaphore.wait(), SemaphoreResult::WouldBlock);
        assert!(!semaphore.is_available());
    }

    #[test]
    fn test_semaphore_post_overflow() {
        let mut semaphore = Semaphore::new(1, 10, 10);
        assert_eq!(semaphore.post(), SemaphoreResult::InvalidValue);
    }

    #[test]
    fn test_semaphore_set() {
        let set = SemaphoreSet::new(1, 12345);
        assert_eq!(set.id, 1);
        assert_eq!(set.key, 12345);

        let sem_id = set.create_semaphore(5, 10);
        assert_eq!(sem_id, 1);
        assert_eq!(set.semaphore_count(), 1);
    }

    #[test]
    fn test_semaphore_set_operations() {
        let set = SemaphoreSet::new(1, 12345);
        let sem_id = set.create_semaphore(5, 10);

        let mut semaphores = set.semaphores.lock().unwrap();
        let sem = semaphores.get_mut(&sem_id).unwrap();
        sem.wait();
        assert_eq!(sem.get_value(), 4);

        let sem = semaphores.get_mut(&sem_id).unwrap();
        sem.post();
        assert_eq!(sem.get_value(), 5);
    }

    #[test]
    fn test_semaphore_manager() {
        let manager = SemaphoreManager::new();

        let set_id = manager.create_semaphore_set(12345);
        assert_eq!(set_id, 1);

        let sem_id = manager.create_semaphore(set_id, 5, 10).unwrap();
        assert_eq!(sem_id, 1);

        assert_eq!(manager.wait(set_id, sem_id).unwrap(), SemaphoreResult::Success);
        assert_eq!(manager.get_value(set_id, sem_id).unwrap(), 4);

        assert_eq!(manager.post(set_id, sem_id).unwrap(), SemaphoreResult::Success);
        assert_eq!(manager.get_value(set_id, sem_id).unwrap(), 5);
    }

    #[test]
    fn test_semaphore_manager_invalid() {
        let manager = SemaphoreManager::new();
        assert!(manager.wait(999, 1).is_err());
        assert!(manager.create_semaphore(999, 5, 10).is_err());
    }

    #[test]
    fn test_semaphore_manager_multiple_sets() {
        let manager = SemaphoreManager::new();

        let set_id1 = manager.create_semaphore_set(11111);
        let set_id2 = manager.create_semaphore_set(22222);

        let sem_id1 = manager.create_semaphore(set_id1, 5, 10).unwrap();
        let sem_id2 = manager.create_semaphore(set_id2, 3, 10).unwrap();

        manager.wait(set_id1, sem_id1).unwrap();
        manager.wait(set_id2, sem_id2).unwrap();

        assert_eq!(manager.get_value(set_id1, sem_id1).unwrap(), 4);
        assert_eq!(manager.get_value(set_id2, sem_id2).unwrap(), 2);

        assert_eq!(manager.set_count(), 2);
    }
}
