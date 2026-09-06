//! # SigmaOS Sovereign Message Queue
//!
//! POSIX-style priority-ordered message queue for inter-process communication.
//!
//! ## Overview
//!
//! Messages are stored in priority order (highest-priority first). When two
//! messages share the same priority, they are served in FIFO order (by
//! arrival timestamp).
//!
//! ```text
//! send(msg)
//!   ──► [prio=255] ──► [prio=128] ──► [prio=64] ──► [prio=0]
//!                                                    receive()
//! ```
//!
//! ## Comparison with POSIX `mq_open`
//!
//! | POSIX                  | SigmaOS                         |
//! |------------------------|---------------------------------|
//! | `mq_open()`            | `SigmaMessageQueue::open()`     |
//! | `mq_send()`            | `SigmaMessageQueue::send()`     |
//! | `mq_receive()`         | `SigmaMessageQueue::receive()`  |
//! | `mq_getattr()`         | `SigmaMessageQueue::attrs()`    |
//! | `struct mq_attr`       | `MessageQueueAttributes`        |

#![allow(dead_code)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors that can occur during message queue operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageQueueError {
    /// The queue has reached its maximum message count.
    QueueFull,
    /// The queue is empty and no messages are available.
    QueueEmpty,
    /// The message payload exceeds `max_msg_size`.
    MessageTooLarge,
    /// The supplied queue name is invalid.
    InvalidName(String),
    /// The queue has been closed / destroyed.
    Closed,
}

impl core::fmt::Display for MessageQueueError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MessageQueueError::QueueFull      => write!(f, "mq: queue full"),
            MessageQueueError::QueueEmpty     => write!(f, "mq: queue empty"),
            MessageQueueError::MessageTooLarge => write!(f, "mq: message too large"),
            MessageQueueError::InvalidName(n)  => write!(f, "mq: invalid name '{}'", n),
            MessageQueueError::Closed          => write!(f, "mq: queue closed"),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Message
// ─────────────────────────────────────────────────────────────────────────────

/// A single message in the queue.
///
/// Messages with higher `priority` values are received first.
#[derive(Debug, Clone)]
pub struct SigmaMessage {
    /// Message priority (0 = lowest, 255 = highest).
    pub priority: u8,
    /// Raw message payload.
    pub data: Vec<u8>,
    /// Monotonic timestamp (nanoseconds since boot, or similar counter).
    pub timestamp: u64,
}

impl SigmaMessage {
    /// Construct a new message.
    pub fn new(priority: u8, data: Vec<u8>, timestamp: u64) -> Self {
        SigmaMessage { priority, data, timestamp }
    }

    /// Create a message with priority 0 (lowest).
    pub fn low(data: Vec<u8>, timestamp: u64) -> Self {
        Self::new(0, data, timestamp)
    }

    /// Create a message with priority 128 (normal).
    pub fn normal(data: Vec<u8>, timestamp: u64) -> Self {
        Self::new(128, data, timestamp)
    }

    /// Create a message with priority 255 (highest).
    pub fn high(data: Vec<u8>, timestamp: u64) -> Self {
        Self::new(255, data, timestamp)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Attributes
// ─────────────────────────────────────────────────────────────────────────────

/// Attributes of a message queue, analogous to POSIX `struct mq_attr`.
#[derive(Debug, Clone, Copy)]
pub struct MessageQueueAttributes {
    /// Maximum number of messages the queue can hold.
    pub max_msgs: usize,
    /// Maximum size (bytes) of a single message payload.
    pub max_msg_size: usize,
    /// Current number of messages in the queue.
    pub cur_msgs: usize,
}

impl MessageQueueAttributes {
    /// Create default attributes: 32 messages, max 4 KiB each.
    pub fn default_attrs() -> Self {
        MessageQueueAttributes {
            max_msgs: 32,
            max_msg_size: 4096,
            cur_msgs: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Queue
// ─────────────────────────────────────────────────────────────────────────────

/// A POSIX-compatible, priority-ordered message queue.
///
/// # Priority ordering
///
/// Messages are delivered in **descending priority** order. Within the same
/// priority, older messages (lower `timestamp`) are delivered first.
///
/// # Capacity enforcement
///
/// [`send`](SigmaMessageQueue::send) returns [`MessageQueueError::QueueFull`]
/// when `cur_msgs == max_msgs`, and [`MessageQueueError::MessageTooLarge`]
/// when `data.len() > max_msg_size`.
pub struct SigmaMessageQueue {
    /// Optional human-readable name (analogous to POSIX `/name`).
    pub name: String,
    attrs: MessageQueueAttributes,
    messages: Vec<SigmaMessage>,
    closed: bool,
}

impl SigmaMessageQueue {
    /// Create a new queue with the given `name` and `attributes`.
    ///
    /// # Errors
    ///
    /// Returns [`MessageQueueError::InvalidName`] if `name` is empty.
    pub fn open(name: String, attrs: MessageQueueAttributes) -> Result<Self, MessageQueueError> {
        if name.is_empty() {
            return Err(MessageQueueError::InvalidName("(empty)".to_string()));
        }
        Ok(SigmaMessageQueue {
            name,
            attrs,
            messages: Vec::new(),
            closed: false,
        })
    }

    /// Create a queue with default attributes.
    pub fn open_default(name: &str) -> Result<Self, MessageQueueError> {
        Self::open(name.to_string(), MessageQueueAttributes::default_attrs())
    }

    /// Send a message to the queue.
    ///
    /// The queue maintains messages in sorted order after each insert, ensuring
    /// O(n) send with O(1) receive.
    ///
    /// # Errors
    ///
    /// - [`MessageQueueError::Closed`] if the queue has been closed.
    /// - [`MessageQueueError::QueueFull`] if `cur_msgs == max_msgs`.
    /// - [`MessageQueueError::MessageTooLarge`] if `msg.data.len() > max_msg_size`.
    pub fn send(&mut self, msg: SigmaMessage) -> Result<(), MessageQueueError> {
        if self.closed {
            return Err(MessageQueueError::Closed);
        }
        if self.attrs.cur_msgs >= self.attrs.max_msgs {
            return Err(MessageQueueError::QueueFull);
        }
        if msg.data.len() > self.attrs.max_msg_size {
            return Err(MessageQueueError::MessageTooLarge);
        }

        // Insert in sorted position (highest priority first, then oldest first)
        let pos = self.messages.partition_point(|m| {
            m.priority > msg.priority
                || (m.priority == msg.priority && m.timestamp <= msg.timestamp)
        });
        self.messages.insert(pos, msg);
        self.attrs.cur_msgs += 1;
        Ok(())
    }

    /// Receive the highest-priority message from the queue.
    ///
    /// # Errors
    ///
    /// - [`MessageQueueError::Closed`] if the queue has been closed.
    /// - [`MessageQueueError::QueueEmpty`] if no messages are available.
    pub fn receive(&mut self) -> Result<SigmaMessage, MessageQueueError> {
        if self.closed {
            return Err(MessageQueueError::Closed);
        }
        if self.messages.is_empty() {
            return Err(MessageQueueError::QueueEmpty);
        }
        let msg = self.messages.remove(0);
        self.attrs.cur_msgs -= 1;
        Ok(msg)
    }

    /// Peek at the next message without removing it from the queue.
    pub fn peek(&self) -> Option<&SigmaMessage> {
        self.messages.first()
    }

    /// Return current queue attributes (including live `cur_msgs`).
    pub fn attrs(&self) -> MessageQueueAttributes {
        self.attrs
    }

    /// Return the number of messages currently in the queue.
    pub fn len(&self) -> usize {
        self.attrs.cur_msgs
    }

    /// Return `true` if the queue contains no messages.
    pub fn is_empty(&self) -> bool {
        self.attrs.cur_msgs == 0
    }

    /// Close the queue.  Further operations will return
    /// [`MessageQueueError::Closed`].
    pub fn close(&mut self) {
        self.closed = true;
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_priority_ordering() {
        let mut mq = SigmaMessageQueue::open_default("/test").unwrap();
        mq.send(SigmaMessage::low(vec![1], 1)).unwrap();
        mq.send(SigmaMessage::high(vec![3], 3)).unwrap();
        mq.send(SigmaMessage::normal(vec![2], 2)).unwrap();

        assert_eq!(mq.receive().unwrap().data, vec![3]); // high
        assert_eq!(mq.receive().unwrap().data, vec![2]); // normal
        assert_eq!(mq.receive().unwrap().data, vec![1]); // low
    }

    #[test]
    fn test_queue_full() {
        let attrs = MessageQueueAttributes { max_msgs: 2, max_msg_size: 128, cur_msgs: 0 };
        let mut mq = SigmaMessageQueue::open("test".to_string(), attrs).unwrap();
        mq.send(SigmaMessage::normal(vec![0], 0)).unwrap();
        mq.send(SigmaMessage::normal(vec![0], 1)).unwrap();
        assert_eq!(mq.send(SigmaMessage::normal(vec![0], 2)), Err(MessageQueueError::QueueFull));
    }

    #[test]
    fn test_message_too_large() {
        let attrs = MessageQueueAttributes { max_msgs: 10, max_msg_size: 4, cur_msgs: 0 };
        let mut mq = SigmaMessageQueue::open("tiny".to_string(), attrs).unwrap();
        let big = vec![0u8; 5];
        assert_eq!(mq.send(SigmaMessage::normal(big, 0)), Err(MessageQueueError::MessageTooLarge));
    }

    #[test]
    fn test_empty_receive() {
        let mut mq = SigmaMessageQueue::open_default("/empty").unwrap();
        assert_eq!(mq.receive(), Err(MessageQueueError::QueueEmpty));
    }
}
