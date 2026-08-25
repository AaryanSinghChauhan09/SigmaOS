/// OOP-based Security Audit for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 542
/// Implements security event logging and audit trails
extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type EventID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    PlainText,
    Json,
    Binary,
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Authentication = 0,
    Authorization = 1,
    FileAccess = 2,
    SystemChange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            timestamp: AtomicUsize::new(0),
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
        unsafe { core::mem::transmute(self.event_type.load(Ordering::SeqCst)) }
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
}

impl SimpleAuditLogger {
    pub fn new() -> Self {
        SimpleAuditLogger { events: Vec::new() }
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
        for i in 0..self.events.len() {
            if let Some(ref event) = self.events[i] {
                let event: &Box<dyn AuditEvent> = event;
                if event.id() == id {
                    return Some(event.as_ref());
                }
            }
        }
        None
    }

    fn query_events(&self, event_type: EventType, user_id: usize) -> Vec<EventID> {
        let mut ids = Vec::new();
        for i in 0..self.events.len() {
            if let Some(ref event) = self.events[i] {
                let event: &Box<dyn AuditEvent> = event;
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
            let mut remove = false;
            if let Some(ref event) = self.events[i] {
                let event: &Box<dyn AuditEvent> = event;
                if event.timestamp() < older_than {
                    remove = true;
                }
            }
            if remove {
                self.events.remove(i);
            } else {
                i += 1;
            }
        }
        Ok(())
    }
}

pub trait AuditPolicy {
    fn check_compliance(&self, event: &dyn AuditEvent) -> Result<bool, AuditError>;
    fn enforce_policy(&mut self, event: &dyn AuditEvent) -> Result<(), AuditError>;
}

#[repr(C)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_event_logging() {
        let mut logger = SimpleAuditLogger::new();
        let event = SimpleAuditEvent::new(
            1,
            EventType::Authentication,
            1001,
            b"User logged in successfully",
        );
        logger.log_event(Box::new(event)).unwrap();

        let found = logger.get_event(1).unwrap();
        assert_eq!(found.user_id(), 1001);
        assert_eq!(found.event_type(), EventType::Authentication);
    }
}
