//! # POSIX Message Queues
//!
//! POSIX message queue implementation (mq_open, mq_send, mq_receive).
//! Inspired by Linux ipc/mqueue.c and FreeBSD kern/uipc_mqueue.c.

#![no_std]

extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// POSIX message queue attributes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MqAttr {
    pub mq_flags: i64,      // Message queue flags (O_NONBLOCK)
    pub mq_maxmsg: i64,     // Maximum number of messages
    pub mq_msgsize: i64,    // Maximum message size
    pub mq_curmsgs: i64,    // Number of messages currently queued
}

impl Default for MqAttr {
    fn default() -> Self {
        Self {
            mq_flags: 0,
            mq_maxmsg: 10,      // Default max messages
            mq_msgsize: 8192,   // Default max message size (8KB)
            mq_curmsgs: 0,
        }
    }
}

/// Message with priority
#[derive(Debug, Clone)]
pub struct MqMessage {
    pub data: Vec<u8>,
    pub priority: u32,
    pub timestamp: u64,
}

/// POSIX message queue
pub struct MessageQueue {
    name: String,
    attr: MqAttr,
    messages: VecDeque<MqMessage>,
    next_msg_id: AtomicU64,
    open_count: AtomicU32,
    unlink_pending: bool,
}

impl MessageQueue {
    /// Create new message queue
    pub fn new(name: String, attr: MqAttr) -> Self {
        Self {
            name,
            attr,
            messages: VecDeque::new(),
            next_msg_id: AtomicU64::new(0),
            open_count: AtomicU32::new(1),
            unlink_pending: false,
        }
    }
    
    /// Send message with priority
    pub fn send(&mut self, data: &[u8], priority: u32) -> Result<(), MqError> {
        // Check message size
        if data.len() > self.attr.mq_msgsize as usize {
            return Err(MqError::MessageTooLarge);
        }
        
        // Check if queue is full
        if self.messages.len() >= self.attr.mq_maxmsg as usize {
            if self.attr.mq_flags & 0x4000 != 0 { // O_NONBLOCK
                return Err(MqError::WouldBlock);
            } else {
                // In production: block on wait queue
                return Err(MqError::WouldBlock);
            }
        }
        
        // Create message
        let msg = MqMessage {
            data: data.to_vec(),
            priority,
            timestamp: self.next_msg_id.fetch_add(1, Ordering::SeqCst),
        };
        
        // Insert in priority order (higher priority first, FIFO within same priority)
        let insert_pos = self.messages.iter().position(|m| {
            m.priority < priority || (m.priority == priority && m.timestamp > msg.timestamp)
        }).unwrap_or(self.messages.len());
        
        self.messages.insert(insert_pos, msg);
        self.attr.mq_curmsgs += 1;
        
        // In production: wake waiting receivers
        Ok(())
    }
    
    /// Receive highest priority message
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<(usize, u32), MqError> {
        // Check if queue is empty
        if self.messages.is_empty() {
            if self.attr.mq_flags & 0x4000 != 0 { // O_NONBLOCK
                return Err(MqError::WouldBlock);
            } else {
                // In production: block on wait queue
                return Err(MqError::WouldBlock);
            }
        }
        
        // Get highest priority message (front of queue)
        let msg = self.messages.pop_front().unwrap();
        self.attr.mq_curmsgs -= 1;
        
        // Check buffer size
        if buf.len() < msg.data.len() {
            return Err(MqError::BufferTooSmall);
        }
        
        // Copy message data
        let len = msg.data.len();
        buf[..len].copy_from_slice(&msg.data);
        
        // In production: wake waiting senders
        Ok((len, msg.priority))
    }
    
    /// Timed receive with timeout
    pub fn timedreceive(&mut self, buf: &mut [u8], _timeout_ns: u64) -> Result<(usize, u32), MqError> {
        // Simplified: just call regular receive
        // In production: use timeout for blocking
        self.receive(buf)
    }
    
    /// Get queue attributes
    pub fn getattr(&self) -> MqAttr {
        self.attr
    }
    
    /// Set queue attributes (only mq_flags can be changed)
    pub fn setattr(&mut self, new_attr: &MqAttr) -> MqAttr {
        let old_attr = self.attr;
        self.attr.mq_flags = new_attr.mq_flags;
        old_attr
    }
    
    /// Notify process when message arrives (not implemented)
    pub fn notify(&mut self, _notification: MqNotification) -> Result<(), MqError> {
        // In production: register signal/thread notification
        Err(MqError::NotImplemented)
    }
    
    /// Increment open count
    pub fn open(&self) {
        self.open_count.fetch_add(1, Ordering::SeqCst);
    }
    
    /// Decrement open count
    pub fn close(&self) -> u32 {
        self.open_count.fetch_sub(1, Ordering::SeqCst).saturating_sub(1)
    }
    
    /// Mark for deletion
    pub fn unlink(&mut self) {
        self.unlink_pending = true;
    }
    
    /// Check if should be deleted
    pub fn should_delete(&self) -> bool {
        self.unlink_pending && self.open_count.load(Ordering::SeqCst) == 0
    }
}

/// Notification types
#[derive(Debug, Clone, Copy)]
pub enum MqNotification {
    None,
    Signal(i32),     // Signal number
    Thread,          // Spawn thread
}

/// Global message queue registry
pub struct MqRegistry {
    queues: BTreeMap<String, MessageQueue>,
}

impl MqRegistry {
    pub fn new() -> Self {
        Self {
            queues: BTreeMap::new(),
        }
    }
    
    /// Open or create message queue
    pub fn open(&mut self, name: &str, flags: i32, attr: Option<MqAttr>) -> Result<&mut MessageQueue, MqError> {
        let o_creat = flags & 0x40;   // O_CREAT
        let o_excl = flags & 0x80;    // O_EXCL
        
        if o_creat != 0 {
            if o_excl != 0 && self.queues.contains_key(name) {
                return Err(MqError::AlreadyExists);
            }
            
            if !self.queues.contains_key(name) {
                let queue = MessageQueue::new(
                    name.into(),
                    attr.unwrap_or_default()
                );
                self.queues.insert(name.into(), queue);
            }
        } else if !self.queues.contains_key(name) {
            return Err(MqError::NotFound);
        }
        
        let queue = self.queues.get_mut(name).unwrap();
        queue.open();
        Ok(queue)
    }
    
    /// Close message queue
    pub fn close(&mut self, name: &str) -> Result<(), MqError> {
        let queue = self.queues.get(name).ok_or(MqError::NotFound)?;
        let count = queue.close();
        
        // Delete if unlinked and no more references
        if count == 0 && queue.should_delete() {
            self.queues.remove(name);
        }
        
        Ok(())
    }
    
    /// Unlink (delete) message queue
    pub fn unlink(&mut self, name: &str) -> Result<(), MqError> {
        let queue = self.queues.get_mut(name).ok_or(MqError::NotFound)?;
        queue.unlink();
        
        // Delete immediately if no open references
        if queue.should_delete() {
            self.queues.remove(name);
        }
        
        Ok(())
    }
}

/// Message queue errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MqError {
    NotFound,
    AlreadyExists,
    MessageTooLarge,
    BufferTooSmall,
    WouldBlock,
    Timeout,
    InvalidName,
    NotImplemented,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mq_creation() {
        let attr = MqAttr::default();
        let mq = MessageQueue::new("/test".into(), attr);
        assert_eq!(mq.name, "/test");
        assert_eq!(mq.messages.len(), 0);
    }
    
    #[test]
    fn test_mq_send_receive() {
        let attr = MqAttr::default();
        let mut mq = MessageQueue::new("/test".into(), attr);
        
        let data = b"Hello, World!";
        assert!(mq.send(data, 0).is_ok());
        
        let mut buf = [0u8; 1024];
        let result = mq.receive(&mut buf);
        assert!(result.is_ok());
        
        let (len, priority) = result.unwrap();
        assert_eq!(len, data.len());
        assert_eq!(priority, 0);
        assert_eq!(&buf[..len], data);
    }
    
    #[test]
    fn test_mq_priority_ordering() {
        let attr = MqAttr::default();
        let mut mq = MessageQueue::new("/test".into(), attr);
        
        mq.send(b"Low", 0).unwrap();
        mq.send(b"High", 10).unwrap();
        mq.send(b"Medium", 5).unwrap();
        
        let mut buf = [0u8; 1024];
        
        let (len, pri) = mq.receive(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"High");
        assert_eq!(pri, 10);
        
        let (len, pri) = mq.receive(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"Medium");
        assert_eq!(pri, 5);
        
        let (len, pri) = mq.receive(&mut buf).unwrap();
        assert_eq!(&buf[..len], b"Low");
        assert_eq!(pri, 0);
    }
    
    #[test]
    fn test_mq_registry() {
        let mut registry = MqRegistry::new();
        
        // Create queue
        let attr = MqAttr::default();
        assert!(registry.open("/test", 0x40, Some(attr)).is_ok());
        
        // Try to create again with O_EXCL
        assert_eq!(
            registry.open("/test", 0x40 | 0x80, Some(attr)),
            Err(MqError::AlreadyExists)
        );
        
        // Open existing
        assert!(registry.open("/test", 0, None).is_ok());
        
        // Unlink
        assert!(registry.unlink("/test").is_ok());
    }
    
    #[test]
    fn test_mq_buffer_too_small() {
        let attr = MqAttr::default();
        let mut mq = MessageQueue::new("/test".into(), attr);
        
        mq.send(b"Hello, World!", 0).unwrap();
        
        let mut buf = [0u8; 5];
        assert_eq!(mq.receive(&mut buf), Err(MqError::BufferTooSmall));
    }
}
