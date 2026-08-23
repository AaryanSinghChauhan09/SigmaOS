/// OOP-based Security Audit for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 542
/// Implements security event logging and audit trails
extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type EventID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Authentication = 0,
    Authorization = 1,
    FileAccess = 2,
    SystemChange = 3,
}

#[repr(C)]
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
        match self.event_type.load(Ordering::SeqCst) {
            0 => EventType::Authentication,
            1 => EventType::Authorization,
            2 => EventType::FileAccess,
            _ => EventType::SystemChange,
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
            if let Some(ref event) = event_option {
                let event_ref: &dyn AuditEvent = &**event;
                if event_ref.id() == id {
                    return Some(event_ref);
                }
            }
        }
        None
    }

    fn query_events(&self, event_type: EventType, user_id: usize) -> Vec<EventID> {
        let mut ids = Vec::new();
        for event_option in &self.events {
            if let Some(ref event) = event_option {
                let event_ref: &dyn AuditEvent = &**event;
                if event_ref.event_type() == event_type && event_ref.user_id() == user_id {
                    ids.push(event_ref.id());
                }
            }
        }
        ids
    }

    fn clear_events(&mut self, older_than: u64) -> Result<(), AuditError> {
        self.events.retain(|event_opt| {
            if let Some(ref event) = *event_opt {
                event.timestamp() >= older_than
            } else {
                false
            }
        });
        Ok(())
    }
}

pub trait AuditPolicy {
    fn check_compliance(&self, event: &dyn AuditEvent) -> Result<bool, AuditError>;
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
