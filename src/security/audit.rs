#![no_std]

use core::mem;
/// OOP-based Security Audit for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 542
/// Implements security event logging and audit trails
use core::sync::atomic::{AtomicUsize, Ordering};

pub type EventID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat { Json, Text, Binary }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Authentication = 0,
    Authorization = 1,
    FileAccess = 2,
    SystemChange = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AuditError {
    Success = 0,
    LogFull = 1,
    InvalidEvent = 2,
}

pub trait AuditEvent {
    fn id(&self) -> EventID;
    fn event_type(&self) -> EventType;
    fn timestamp(&self) -> u64;
    fn user_id(&self) -> usize;
    fn description(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleAuditEvent {
    pub id: EventID,
    pub event_type: AtomicUsize,
    pub timestamp: AtomicUsize,
    pub user_id: AtomicUsize,
    pub description: [u8; 256],
}

impl SimpleAuditEvent {
    pub fn new(id: EventID, event_type: EventType, user_id: usize, description: &[u8]) -> Self {
        let mut desc_array = [0u8; 256];
        let desc_len = description.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(description.as_ptr(), desc_array.as_mut_ptr(), desc_len);
        }
        SimpleAuditEvent {
            id,
            event_type: AtomicUsize::new(event_type as usize),
            timestamp: AtomicUsize::new(1000000),
            user_id: AtomicUsize::new(user_id),
            description: desc_array,
        }
    }
}

impl AuditEvent for SimpleAuditEvent {
    fn id(&self) -> EventID {
        self.id
    }
    fn event_type(&self) -> EventType {
        match self.event_type.load(Ordering::SeqCst) as u32 {
            0 => EventType::Login,
            1 => EventType::Logout,
            2 => EventType::FileAccess,
            3 => EventType::ProcessExec,
            4 => EventType::NetworkAccess,
            _ => EventType::Other,
        }
    }
    fn timestamp(&self) -> u64 {
        self.timestamp.load(Ordering::SeqCst) as u64
    }
    fn user_id(&self) -> usize {
        self.user_id.load(Ordering::SeqCst)
    }
    fn description(&self) -> &[u8] {
        let len = self.description.iter().position(|&b| b == 0).unwrap_or(256);
        &self.description[..len]
    }
}

pub trait AuditLogger {
    fn log_event(&mut self, event: Box<dyn AuditEvent>) -> Result<EventID, AuditError>;
    fn get_event(&self, id: EventID) -> Option<&dyn AuditEvent>;
    fn query_events(&self, event_type: EventType, user_id: usize) -> Vec<EventID>;
    fn clear_events(&mut self, older_than: u64) -> Result<(), AuditError>;
}

pub struct SimpleAuditLogger {
    pub events: Vec<Option<Box<dyn AuditEvent>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAuditLogger {
    pub fn new() -> Self {
        SimpleAuditLogger {
            events: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleAuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditLogger for SimpleAuditLogger {
    fn log_event(&mut self, event: Box<dyn AuditEvent>) -> Result<EventID, AuditError> {
        let id = event.id();
        self.events.push(Some(event));
        Ok(id)
    }

    fn get_event(&self, id: EventID) -> Option<&dyn AuditEvent> {
        for event_option in &self.events {
            if let Some(ref event) = *event_option {
                if event.id() == id {
                    return Some(event.as_ref());
                }
            }
        }
        None
    }

    fn query_events(&self, event_type: EventType, user_id: usize) -> Vec<EventID> {
        let mut ids = Vec::new();
        for event_option in &self.events {
            if let Some(ref event) = *event_option {
                if event.event_type() == event_type && event.user_id() == user_id {
                    ids.push(event.id());
                }
            }
        }
        ids
    }

    fn clear_events(&mut self, older_than: u64) -> Result<(), AuditError> {
        let mut i = 0;
        while i < self.events.len() {
            if let Some(ref event) = self.events[i] {
                if event.timestamp() < older_than {
                    self.events.remove(i);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }
        Ok(())
    }
}

pub trait AuditPolicy {
    fn check_compliance(&self, event: &dyn AuditEvent) -> bool;
    fn enforce_policy(&mut self, event: &dyn AuditEvent) -> Result<(), AuditError>;
}

pub struct SimpleAuditPolicy {
    pub require_authentication: AtomicUsize,
}

impl SimpleAuditPolicy {
    pub fn new() -> Self {
        SimpleAuditPolicy {
            require_authentication: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleAuditPolicy {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditPolicy for SimpleAuditPolicy {
    fn check_compliance(&self, event: &dyn AuditEvent) -> bool {
        if self.require_authentication.load(Ordering::SeqCst) == 1 {
            event.event_type() == EventType::Authentication
        } else {
            true
        }
    }

    fn enforce_policy(&mut self, event: &dyn AuditEvent) -> Result<(), AuditError> {
        if self.check_compliance(event) {
            Ok(())
        } else {
            Err(AuditError::InvalidEvent)
        }
    }
}

struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_audit_logger() {
        let mut logger = SimpleAuditLogger::new();
        let event = Box::new(SimpleAuditEvent::new(
            101,
            EventType::Authentication,
            42,
            b"User logged in",
        ));
        let id = logger.log_event(event).unwrap();
        assert_eq!(id, 101);

        let retrieved = logger.get_event(101).unwrap();
        assert_eq!(retrieved.id(), 101);

        let queried = logger.query_events(EventType::Authentication, 42);
        assert_eq!(queried.len(), 1);
        assert_eq!(queried[0], 101);

        logger.clear_events(2000000).unwrap();
        assert!(logger.get_event(101).is_none());
    }
}
