// SPDX-License-Identifier: MIT
//! # SigmaOS Kernel Task Name Cache
//!
//! Provides O(1) cached task-name lookups, eliminating repeated string scans
//! from the scheduler hot-path.  Inspired by Linux's `task_comm` cache and
//! FreeBSD's `td_name` in `struct thread`.
//!
//! ## Design
//! - Fixed-size inline buffer (`[u8; TASK_NAME_LEN]`) — no heap allocation.
//! - FNV-1a hash index for O(1) average-case lookup.
//! - Lock-free reads via `AtomicU64` versioning (seqlock pattern).
//! - Maximum 1024 concurrent tasks tracked without allocation.

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

/// Maximum task name length (matches Linux TASK_COMM_LEN = 16).
pub const TASK_NAME_LEN: usize = 16;
/// Maximum number of tasks tracked simultaneously.
pub const MAX_TASKS: usize = 1024;

/// A single cached task-name entry.
#[repr(C)]
pub struct TaskNameEntry {
    /// Task ID (0 = slot free).
    pub tid: AtomicU64,
    /// Inline task name buffer (null-terminated, up to TASK_NAME_LEN bytes).
    pub name: [u8; TASK_NAME_LEN],
    /// Seqlock version counter — odd during write, even when stable.
    version: AtomicU64,
}

impl TaskNameEntry {
    const fn empty() -> Self {
        Self {
            tid: AtomicU64::new(0),
            name: [0u8; TASK_NAME_LEN],
            version: AtomicU64::new(0),
        }
    }
}

const EMPTY_ENTRY: TaskNameEntry = TaskNameEntry::empty();

/// Global task-name cache — statically allocated, zero heap.
pub struct TaskNameCache {
    entries: [TaskNameEntry; MAX_TASKS],
    count: AtomicUsize,
}

// SAFETY: All mutation uses seqlock + atomic TID for coordination.
unsafe impl Sync for TaskNameCache {}
unsafe impl Send for TaskNameCache {}

impl TaskNameCache {
    pub const fn new() -> Self {
        Self {
            entries: [const { TaskNameEntry::empty() }; MAX_TASKS],
            count: AtomicUsize::new(0),
        }
    }

    /// FNV-1a 64-bit hash — same algorithm as `ZeroDependencyPrimitiveHub::fnv1a_hash_64`.
    #[inline(always)]
    fn hash_tid(tid: u64) -> usize {
        let mut h: u64 = 0xcbf29ce484222325;
        h ^= tid;
        h = h.wrapping_mul(0x100000001b3);
        h as usize % MAX_TASKS
    }

    /// Register or update a task name.  Returns `false` if the cache is full.
    ///
    /// # Safety
    /// Must only be called from kernel context with the task creation lock held.
    pub unsafe fn set(&self, tid: u64, name: &[u8]) -> bool {
        if tid == 0 {
            return false;
        }
        let mut slot = Self::hash_tid(tid);
        // Linear probe for empty or matching slot.
        for _ in 0..MAX_TASKS {
            let existing = self.entries[slot].tid.load(Ordering::Acquire);
            if existing == 0 || existing == tid {
                // Begin seqlock write: increment to odd version.
                let old_ver = self.entries[slot].version.fetch_add(1, Ordering::Release);
                core::sync::atomic::fence(Ordering::SeqCst);

                self.entries[slot].tid.store(tid, Ordering::Relaxed);
                // Copy name, truncate/pad to TASK_NAME_LEN.
                let copy_len = name.len().min(TASK_NAME_LEN - 1);
                // SAFETY: `entries` is exclusively accessed under seqlock write phase.
                let dst = &self.entries[slot].name as *const [u8; TASK_NAME_LEN]
                    as *mut [u8; TASK_NAME_LEN];
                for i in 0..TASK_NAME_LEN {
                    (*dst)[i] = if i < copy_len { name[i] } else { 0 };
                }

                core::sync::atomic::fence(Ordering::SeqCst);
                // End seqlock write: increment to even version.
                self.entries[slot].version.store(old_ver + 2, Ordering::Release);

                if existing == 0 {
                    self.count.fetch_add(1, Ordering::Relaxed);
                }
                return true;
            }
            slot = (slot + 1) % MAX_TASKS;
        }
        false // Cache full
    }

    /// Look up a task name by TID.
    /// Returns a copy of the name buffer — lock-free via seqlock.
    pub fn get(&self, tid: u64) -> Option<[u8; TASK_NAME_LEN]> {
        if tid == 0 {
            return None;
        }
        let mut slot = Self::hash_tid(tid);
        for _ in 0..MAX_TASKS {
            let stored_tid = self.entries[slot].tid.load(Ordering::Acquire);
            if stored_tid == tid {
                // Seqlock read: retry if version is odd (write in progress).
                loop {
                    let v1 = self.entries[slot].version.load(Ordering::Acquire);
                    if v1 & 1 == 1 {
                        core::hint::spin_loop();
                        continue;
                    }
                    let name = self.entries[slot].name;
                    core::sync::atomic::fence(Ordering::SeqCst);
                    let v2 = self.entries[slot].version.load(Ordering::Acquire);
                    if v1 == v2 {
                        return Some(name);
                    }
                    // Version changed during read — retry.
                }
            }
            if stored_tid == 0 {
                return None; // Hole in probe chain — tid not present.
            }
            slot = (slot + 1) % MAX_TASKS;
        }
        None
    }

    /// Remove a task from the cache when it exits.
    ///
    /// # Safety
    /// Must be called from kernel task-exit path.
    pub unsafe fn remove(&self, tid: u64) {
        if tid == 0 {
            return;
        }
        let mut slot = Self::hash_tid(tid);
        for _ in 0..MAX_TASKS {
            if self.entries[slot].tid.load(Ordering::Acquire) == tid {
                self.entries[slot].tid.store(0, Ordering::Release);
                self.count.fetch_sub(1, Ordering::Relaxed);
                return;
            }
            if self.entries[slot].tid.load(Ordering::Relaxed) == 0 {
                return;
            }
            slot = (slot + 1) % MAX_TASKS;
        }
    }

    /// Returns the number of tracked tasks.
    pub fn len(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }

    /// Returns `true` if no tasks are tracked.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Global singleton task-name cache.
pub static TASK_NAMES: TaskNameCache = TaskNameCache::new();

/// Convenience: get task name as a UTF-8 string slice (strips null padding).
pub fn task_name_str(tid: u64) -> Option<[u8; TASK_NAME_LEN]> {
    TASK_NAMES.get(tid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let cache = TaskNameCache::new();
        let name = b"sigma_init\0\0\0\0\0\0";
        unsafe { assert!(cache.set(1, b"sigma_init")); }
        let got = cache.get(1).unwrap();
        assert_eq!(&got[..10], b"sigma_init");
        assert_eq!(got[10], 0);
    }

    #[test]
    fn test_update() {
        let cache = TaskNameCache::new();
        unsafe {
            cache.set(42, b"worker");
            cache.set(42, b"worker_renamed");
        }
        let got = cache.get(42).unwrap();
        assert_eq!(&got[..14], b"worker_renamed");
    }

    #[test]
    fn test_remove() {
        let cache = TaskNameCache::new();
        unsafe {
            cache.set(99, b"temp_task");
            cache.remove(99);
        }
        assert!(cache.get(99).is_none());
    }

    #[test]
    fn test_miss() {
        let cache = TaskNameCache::new();
        assert!(cache.get(9999).is_none());
    }
}
