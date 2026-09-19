// SPDX-License-Identifier: MIT
// SigmaOS Lock Dependency Detection (Lockdep)
// Deadlock detection and lock order validation inspired by Linux lockdep

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Lock class ID
pub type LockClassId = u64;

/// Lock instance ID
pub type LockInstanceId = u64;

/// Lock dependency graph node
#[derive(Debug, Clone)]
pub struct LockClass {
    pub id: LockClassId,
    pub name: String,
    pub dependencies: BTreeSet<LockClassId>,
}

impl LockClass {
    pub fn new(id: LockClassId, name: String) -> Self {
        LockClass {
            id,
            name,
            dependencies: BTreeSet::new(),
        }
    }

    pub fn add_dependency(&mut self, dep_id: LockClassId) {
        self.dependencies.insert(dep_id);
    }

    pub fn has_dependency(&self, dep_id: LockClassId) -> bool {
        self.dependencies.contains(&dep_id)
    }
}

/// Lockdep error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockdepError {
    InvalidLockClass,
    CircularDependency,
    AlreadyHeld,
    NotHeld,
}

/// Lockdep subsystem
#[derive(Debug)]
pub struct LockdepSubsystem {
    lock_classes: BTreeMap<LockClassId, LockClass>,
    next_class_id: AtomicU64,
    held_locks: BTreeMap<LockInstanceId, LockClassId>,
    next_instance_id: AtomicU64,
    lock_depth: AtomicU32,
}

impl LockdepSubsystem {
    pub fn new() -> Self {
        LockdepSubsystem {
            lock_classes: BTreeMap::new(),
            next_class_id: AtomicU64::new(1),
            held_locks: BTreeMap::new(),
            next_instance_id: AtomicU64::new(1),
            lock_depth: AtomicU32::new(0),
        }
    }

    /// Register a lock class
    pub fn register_lock_class(&mut self, name: String) -> LockClassId {
        let id = self.next_class_id.fetch_add(1, Ordering::SeqCst);
        let lock_class = LockClass::new(id, name);
        self.lock_classes.insert(id, lock_class);
        id
    }

    /// Acquire a lock
    pub fn acquire_lock(&mut self, class_id: LockClassId) -> Result<LockInstanceId, LockdepError> {
        if !self.lock_classes.contains_key(&class_id) {
            return Err(LockdepError::InvalidLockClass);
        }

        // Check for circular dependency
        for (&held_class_id, &held_class) in &self.held_locks {
            if self.would_circular(held_class_id, class_id) {
                return Err(LockdepError::CircularDependency);
            }
        }

        let instance_id = self.next_instance_id.fetch_add(1, Ordering::SeqCst);
        self.held_locks.insert(instance_id, class_id);
        self.lock_depth.fetch_add(1, Ordering::SeqCst);
        
        Ok(instance_id)
    }

    /// Release a lock
    pub fn release_lock(&mut self, instance_id: LockInstanceId) -> Result<(), LockdepError> {
        let class_id = self.held_locks.remove(&instance_id).ok_or(LockdepError::NotHeld)?;
        self.lock_depth.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }

    /// Add lock dependency (lock ordering rule)
    pub fn add_dependency(&mut self, from: LockClassId, to: LockClassId) -> Result<(), LockdepError> {
        if !self.lock_classes.contains_key(&from) || !self.lock_classes.contains_key(&to) {
            return Err(LockdepError::InvalidLockClass);
        }

        if from == to {
            return Err(LockdepError::CircularDependency);
        }

        if let Some(lock_class) = self.lock_classes.get_mut(&from) {
            lock_class.add_dependency(to);
            Ok(())
        } else {
            Err(LockdepError::InvalidLockClass)
        }
    }

    /// Check if acquiring `to` after `from` would cause circular dependency
    fn would_circular(&self, from: LockClassId, to: LockClassId) -> bool {
        let mut visited = BTreeSet::new();
        self.check_circular_helper(to, from, &mut visited)
    }

    fn check_circular_helper(&self, current: LockClassId, target: LockClassId, visited: &mut BTreeSet<LockClassId>) -> bool {
        if current == target {
            return true;
        }

        if visited.contains(&current) {
            return false;
        }

        visited.insert(current);

        if let Some(lock_class) = self.lock_classes.get(&current) {
            for &dep_id in &lock_class.dependencies {
                if self.check_circular_helper(dep_id, target, visited) {
                    return true;
                }
            }
        }

        false
    }

    /// Get current lock depth
    pub fn lock_depth(&self) -> u32 {
        self.lock_depth.load(Ordering::SeqCst)
    }

    /// Get lock class count
    pub fn lock_class_count(&self) -> usize {
        self.lock_classes.len()
    }

    /// Get held lock count
    pub fn held_lock_count(&self) -> usize {
        self.held_locks.len()
    }
}

impl Default for LockdepSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_class_registration() {
        let mut lockdep = LockdepSubsystem::new();
        
        let id1 = lockdep.register_lock_class("mutex".to_string());
        let id2 = lockdep.register_lock_class("rwlock".to_string());
        
        assert!(id1 > 0);
        assert!(id2 > id1);
        assert_eq!(lockdep.lock_class_count(), 2);
    }

    #[test]
    fn test_lock_acquire_release() {
        let mut lockdep = LockdepSubsystem::new();
        
        let class_id = lockdep.register_lock_class("mutex".to_string());
        
        let instance_id = lockdep.acquire_lock(class_id).unwrap();
        assert_eq!(lockdep.lock_depth(), 1);
        assert_eq!(lockdep.held_lock_count(), 1);
        
        lockdep.release_lock(instance_id).unwrap();
        assert_eq!(lockdep.lock_depth(), 0);
        assert_eq!(lockdep.held_lock_count(), 0);
    }

    #[test]
    fn test_lock_dependency() {
        let mut lockdep = LockdepSubsystem::new();
        
        let id1 = lockdep.register_lock_class("mutex".to_string());
        let id2 = lockdep.register_lock("rwlock".to_string());
        
        assert!(lockdep.add_dependency(id1, id2).is_ok());
    }

    #[test]
    fn test_circular_detection() {
        let mut lockdep = lockdep = LockdepSubsystem::new();
        
        let id1 = lockdep.register_lock_class("mutex".to_string());
        let id2 = lockdep.register_lock("rwlock".to_string());
        
        lockdep.add_dependency(id1, id2).unwrap();
        lockdep.add_dependency(id2, id1).unwrap(); // Circular
        
        lockdep.acquire_lock(id1).unwrap();
        let result = lockdep.acquire_lock(id2);
        
        assert!(result.is_err());
    }
}
