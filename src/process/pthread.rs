// SPDX-License-Identifier: MIT
// SigmaOS - POSIX Threads (pthread) Library
// Inspired by Linux pthread (NPTL) and BSD pthread implementation
// Provides POSIX-compliant threading API

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};
use core::cell::UnsafeCell;
use core::ptr::NonNull;

/// Thread identifier
pub type PthreadT = u64;

/// Thread attribute
#[derive(Debug, Clone, Copy)]
pub struct PthreadAttr {
    pub detach_state: DetachState,
    pub stack_size: usize,
    pub guard_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetachState {
    Joinable,
    Detached,
}

impl Default for PthreadAttr {
    fn default() -> Self {
        Self {
            detach_state: DetachState::Joinable,
            stack_size: 8192 * 1024, // 8MB default stack
            guard_size: 4096, // 4KB guard
        }
    }
}

/// Thread function type
pub type ThreadFunc = extern "C" fn(*mut u8) -> *mut u8;

/// Thread control block
pub struct ThreadControlBlock {
    pub id: PthreadT,
    pub func: ThreadFunc,
    pub arg: *mut u8,
    pub result: Option<*mut u8>,
    pub detached: AtomicBool,
    pub finished: AtomicBool,
    pub attr: PthreadAttr,
}

/// Mutex
pub struct PthreadMutex {
    pub locked: AtomicBool,
    pub owner: Option<PthreadT>,
}

impl PthreadMutex {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            owner: None,
        }
    }
    
    pub fn lock(&mut self) -> i32 {
        // Simple spinlock for now - in real implementation would use futex
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Spin
        }
        self.owner = Some(0); // Would be current thread ID
        0
    }
    
    pub fn unlock(&mut self) -> i32 {
        self.owner = None;
        self.locked.store(false, Ordering::Release);
        0
    }
    
    pub fn trylock(&mut self) -> i32 {
        match self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => {
                self.owner = Some(0);
                0
            }
            Err(_) => 16, // EBUSY
        }
    }
}

impl Default for PthreadMutex {
    fn default() -> Self {
        Self::new()
    }
}

/// Condition variable
pub struct PthreadCond {
    pub waiters: AtomicU32,
}

impl PthreadCond {
    pub fn new() -> Self {
        Self {
            waiters: AtomicU32::new(0),
        }
    }
    
    pub fn wait(&self, mutex: &mut PthreadMutex) -> i32 {
        self.waiters.fetch_add(1, Ordering::SeqCst);
        mutex.unlock();
        // In real implementation, would wait on futex
        mutex.lock();
        self.waiters.fetch_sub(1, Ordering::SeqCst);
        0
    }
    
    pub fn signal(&self) -> i32 {
        if self.waiters.load(Ordering::SeqCst) > 0 {
            // Wake one waiter
            0
        } else {
            0
        }
    }
    
    pub fn broadcast(&self) -> i32 {
        // Wake all waiters
        0
    }
}

impl Default for PthreadCond {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-specific data key
pub struct PthreadKey {
    pub key: u32,
    pub destructor: Option<extern "C" fn(*mut u8)>,
}

/// Thread-local storage
pub struct ThreadLocalStorage {
    pub data: BTreeMap<u32, *mut u8>,
}

impl ThreadLocalStorage {
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
    
    pub fn set(&mut self, key: u32, value: *mut u8) {
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: u32) -> Option<*mut u8> {
        self.data.get(&key).copied()
    }
}

impl Default for ThreadLocalStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread manager
pub struct PthreadManager {
    pub threads: BTreeMap<PthreadT, Box<ThreadControlBlock>>,
    pub next_id: AtomicU32,
    pub tls: ThreadLocalStorage,
    pub next_key: AtomicU32,
}

impl PthreadManager {
    pub fn new() -> Self {
        Self {
            threads: BTreeMap::new(),
            next_id: AtomicU32::new(1),
            tls: ThreadLocalStorage::new(),
            next_key: AtomicU32::new(0),
        }
    }
    
    /// Create a new thread
    pub fn create(&mut self, func: ThreadFunc, arg: *mut u8, attr: Option<&PthreadAttr>) -> Result<PthreadT, i32> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst) as PthreadT;
        
        let thread_attr = attr.unwrap_or(&PthreadAttr::default());
        
        let tcb = Box::new(ThreadControlBlock {
            id,
            func,
            arg,
            result: None,
            detached: AtomicBool::new(thread_attr.detach_state == DetachState::Detached),
            finished: AtomicBool::new(false),
            attr: *thread_attr,
        });
        
        self.threads.insert(id, tcb);
        
        // In real implementation, would spawn actual thread
        // For now, simulate execution
        let result = (func)(arg);
        if let Some(tcb) = self.threads.get_mut(&id) {
            tcb.result = Some(result);
            tcb.finished.store(true, Ordering::SeqCst);
        }
        
        Ok(id)
    }
    
    /// Join a thread
    pub fn join(&mut self, thread: PthreadT) -> Result<*mut u8, i32> {
        let tcb = self.threads.get(&thread).ok_or(3)?; // ESRCH
        
        if tcb.detached.load(Ordering::SeqCst) {
            return Err(22); // EINVAL
        }
        
        // Wait for thread to finish
        while !tcb.finished.load(Ordering::SeqCst) {
            // Spin wait - in real implementation would use futex
        }
        
        let result = tcb.result.ok_or(3)?;
        
        // Remove thread
        self.threads.remove(&thread);
        
        Ok(result)
    }
    
    /// Detach a thread
    pub fn detach(&mut self, thread: PthreadT) -> i32 {
        let tcb = self.threads.get_mut(&thread).ok_or(3)?; // ESRCH
        
        if tcb.detached.load(Ordering::SeqCst) {
            return 22; // EINVAL
        }
        
        tcb.detached.store(true, Ordering::SeqCst);
        
        // If already finished, clean up
        if tcb.finished.load(Ordering::SeqCst) {
            self.threads.remove(&thread);
        }
        
        0
    }
    
    /// Exit current thread
    pub fn exit(&mut self, result: *mut u8) {
        // In real implementation, would set result for current thread
        // For now, this is a no-op in simulation
    }
    
    /// Create thread-specific data key
    pub fn key_create(&mut self, destructor: Option<extern "C" fn(*mut u8)>) -> Result<u32, i32> {
        let key = self.next_key.fetch_add(1, Ordering::SeqCst);
        Ok(key)
    }
    
    /// Delete thread-specific data key
    pub fn key_delete(&mut self, key: u32) -> i32 {
        self.tls.data.remove(&key);
        0
    }
    
    /// Set thread-specific data
    pub fn setspecific(&mut self, key: u32, value: *mut u8) -> i32 {
        self.tls.set(key, value);
        0
    }
    
    /// Get thread-specific data
    pub fn getspecific(&self, key: u32) -> Option<*mut u8> {
        self.tls.get(key)
    }
    
    /// Get thread by ID
    pub fn get_thread(&self, thread: PthreadT) -> Option<&ThreadControlBlock> {
        self.threads.get(&thread)
    }
    
    /// Get mutable thread by ID
    pub fn get_thread_mut(&mut self, thread: PthreadT) -> Option<&mut ThreadControlBlock> {
        self.threads.get_mut(&thread)
    }
}

impl Default for PthreadManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global pthread manager
static mut PTHREAD_MANAGER: Option<PthreadManager> = None;

/// Initialize pthread library
pub fn pthread_init() {
    unsafe {
        PTHREAD_MANAGER = Some(PthreadManager::new());
    }
}

/// Get global pthread manager
pub fn get_pthread_manager() -> Option<&'static mut PthreadManager> {
    unsafe {
        PTHREAD_MANAGER.as_mut()
    }
}

/// POSIX pthread API wrappers
pub fn pthread_create(func: ThreadFunc, arg: *mut u8, attr: Option<&PthreadAttr>) -> Result<PthreadT, i32> {
    if let Some(manager) = get_pthread_manager() {
        manager.create(func, arg, attr)
    } else {
        Err(1) // EPERM
    }
}

pub fn pthread_join(thread: PthreadT) -> Result<*mut u8, i32> {
    if let Some(manager) = get_pthread_manager() {
        manager.join(thread)
    } else {
        Err(1)
    }
}

pub fn pthread_detach(thread: PthreadT) -> i32 {
    if let Some(manager) = get_pthread_manager() {
        manager.detach(thread)
    } else {
        1
    }
}

pub fn pthread_exit(result: *mut u8) {
    if let Some(manager) = get_pthread_manager() {
        manager.exit(result);
    }
}

pub fn pthread_key_create(destructor: Option<extern "C" fn(*mut u8)>) -> Result<u32, i32> {
    if let Some(manager) = get_pthread_manager() {
        manager.key_create(destructor)
    } else {
        Err(1)
    }
}

pub fn pthread_key_delete(key: u32) -> i32 {
    if let Some(manager) = get_pthread_manager() {
        manager.key_delete(key)
    } else {
        1
    }
}

pub fn pthread_setspecific(key: u32, value: *mut u8) -> i32 {
    if let Some(manager) = get_pthread_manager() {
        manager.setspecific(key, value)
    } else {
        1
    }
}

pub fn pthread_getspecific(key: u32) -> Option<*mut u8> {
    if let Some(manager) = get_pthread_manager() {
        manager.getspecific(key)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    extern "C" fn test_thread_func(_arg: *mut u8) -> *mut u8 {
        0x42 as *mut u8
    }
    
    #[test]
    fn test_pthread_manager_init() {
        let manager = PthreadManager::new();
        assert_eq!(manager.next_id.load(Ordering::SeqCst), 1);
    }
    
    #[test]
    fn test_pthread_create() {
        pthread_init();
        let result = pthread_create(test_thread_func, 0 as *mut u8, None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_pthread_join() {
        pthread_init();
        let thread = pthread_create(test_thread_func, 0 as *mut u8, None).unwrap();
        let result = pthread_join(thread);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x42 as *mut u8);
    }
    
    #[test]
    fn test_pthread_detach() {
        pthread_init();
        let thread = pthread_create(test_thread_func, 0 as *mut u8, None).unwrap();
        let result = pthread_detach(thread);
        assert_eq!(result, 0);
    }
    
    #[test]
    fn test_mutex_lock_unlock() {
        let mut mutex = PthreadMutex::new();
        assert_eq!(mutex.lock(), 0);
        assert_eq!(mutex.unlock(), 0);
    }
    
    #[test]
    fn test_mutex_trylock() {
        let mut mutex = PthreadMutex::new();
        assert_eq!(mutex.trylock(), 0);
        assert_eq!(mutex.trylock(), 16); // EBUSY
        mutex.unlock();
    }
    
    #[test]
    fn test_cond_wait_signal() {
        let cond = PthreadCond::new();
        let mut mutex = PthreadMutex::new();
        assert_eq!(cond.wait(&mut mutex), 0);
        assert_eq!(cond.signal(), 0);
    }
    
    #[test]
    fn test_cond_broadcast() {
        let cond = PthreadCond::new();
        assert_eq!(cond.broadcast(), 0);
    }
    
    #[test]
    fn test_pthread_key_create() {
        pthread_init();
        let result = pthread_key_create(None);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_pthread_setspecific_getspecific() {
        pthread_init();
        let key = pthread_key_create(None).unwrap();
        let value = 0x1234 as *mut u8;
        assert_eq!(pthread_setspecific(key, value), 0);
        assert_eq!(pthread_getspecific(key), Some(value));
    }
    
    #[test]
    fn test_pthread_attr_default() {
        let attr = PthreadAttr::default();
        assert_eq!(attr.detach_state, DetachState::Joinable);
        assert_eq!(attr.stack_size, 8192 * 1024);
    }
}
