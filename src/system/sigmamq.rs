// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaMQ - System Message Queue
//!
//! SigmaMQ provides system message queue functionality for SigmaOS, inspired by RabbitMQ/Kafka from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Message state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageState {
    /// Queued
    Queued,
    /// Processing
    Processing,
    /// Acknowledged
    Acknowledged,
    /// Failed
    Failed,
}

impl fmt::Display for MessageState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageState::Queued => write!(f, "queued"),
            MessageState::Processing => write!(f, "processing"),
            MessageState::Acknowledged => write!(f, "acknowledged"),
            MessageState::Failed => write!(f, "failed"),
        }
    }
}

/// Message
#[derive(Debug, Clone)]
pub struct Message {
    /// Message ID
    pub message_id: String,
    /// Queue name
    pub queue: String,
    /// Payload
    pub payload: String,
    /// State
    pub state: MessageState,
}

impl Message {
    /// Create a new message
    pub fn new(message_id: String, queue: String, payload: String) -> Self {
        Self {
            message_id,
            queue,
            payload,
            state: MessageState::Queued,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: MessageState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaMQ - System Message Queue
pub struct SigmaMQ {
    /// Messages (message_id -> message)
    messages: BTreeMap<String, Message>,
    /// Queue messages (queue -> message_ids)
    queue_messages: BTreeMap<String, Vec<String>>,
    /// Running state
    running: bool,
    /// Message counter
    message_counter: u32,
}

impl SigmaMQ {
    /// Create a new SigmaMQ instance
    pub fn new() -> Self {
        Self {
            messages: BTreeMap::new(),
            queue_messages: BTreeMap::new(),
            running: false,
            message_counter: 0,
        }
    }

    /// Start the message queue
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the message queue
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the queue is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Publish a message
    pub fn publish(&mut self, message: Message) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMQ not running".to_string());
        }

        if self.messages.contains_key(&message.message_id) {
            return Err(format!("Message '{}' already exists", message.message_id));
        }

        let queue = message.queue.clone();
        let msg_id = message.message_id.clone();
        self.messages.insert(msg_id.clone(), message);

        self.queue_messages
            .entry(queue)
            .or_insert_with(Vec::new)
            .push(msg_id);

        self.message_counter += 1;
        Ok(())
    }

    /// Consume a message from a queue
    pub fn consume(&mut self, queue: &str) -> Result<Option<String>, String> {
        if !self.running {
            return Err("SigmaMQ not running".to_string());
        }

        if let Some(message_ids) = self.queue_messages.get_mut(queue) {
            if let Some(message_id) = message_ids.first() {
                let id = message_id.clone();
                if let Some(message) = self.messages.get_mut(&id) {
                    message.state = MessageState::Processing;
                }
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    /// Acknowledge a message
    pub fn acknowledge(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMQ not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MessageState::Acknowledged;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Mark a message as failed
    pub fn mark_failed(&mut self, message_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaMQ not running".to_string());
        }

        match self.messages.get_mut(message_id) {
            Some(message) => {
                message.state = MessageState::Failed;
                Ok(())
            }
            None => Err(format!("Message '{}' not found", message_id)),
        }
    }

    /// Remove a message
    pub fn remove_message(&mut self, message_id: &str) -> Result<(), String> {
        if let Some(message) = self.messages.remove(message_id) {
            if let Some(message_ids) = self.queue_messages.get_mut(&message.queue) {
                message_ids.retain(|id| id != message_id);
            }
            Ok(())
        } else {
            Err(format!("Message '{}' not found", message_id))
        }
    }

    /// Get a message
    pub fn get_message(&self, message_id: &str) -> Option<&Message> {
        self.messages.get(message_id)
    }

    /// List all messages
    pub fn list_messages(&self) -> Vec<&Message> {
        self.messages.values().collect()
    }

    /// List messages by queue
    pub fn list_by_queue(&self, queue: &str) -> Vec<&Message> {
        if let Some(message_ids) = self.queue_messages.get(queue) {
            message_ids
                .iter()
                .filter_map(|id| self.messages.get(id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// List messages by state
    pub fn list_by_state(&self, state: MessageState) -> Vec<&Message> {
        self.messages
            .values()
            .filter(|m| m.state == state)
            .collect()
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get queue statistics
    pub fn queue_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert(
            "total_messages".to_string(),
            self.message_count().to_string(),
        );
        stats.insert(
            "queued_messages".to_string(),
            self.list_by_state(MessageState::Queued).len().to_string(),
        );
        stats.insert(
            "processing_messages".to_string(),
            self.list_by_state(MessageState::Processing)
                .len()
                .to_string(),
        );
        stats.insert(
            "acknowledged_messages".to_string(),
            self.list_by_state(MessageState::Acknowledged)
                .len()
                .to_string(),
        );
        stats.insert(
            "failed_messages".to_string(),
            self.list_by_state(MessageState::Failed).len().to_string(),
        );
        stats.insert(
            "total_queues".to_string(),
            self.queue_messages.len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaMQ {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaMQ instance (placeholder for actual global state)
static mut SIGMAMQ_INSTANCE: Option<SigmaMQ> = None;

/// Initialize the global SigmaMQ instance
pub fn init_sigmamq() {
    unsafe {
        SIGMAMQ_INSTANCE = Some(SigmaMQ::new());
    }
}

/// Get the global SigmaMQ instance
pub fn get_sigmamq() -> Option<&'static mut SigmaMQ> {
    unsafe { SIGMAMQ_INSTANCE.as_mut() }
}

/// Convenience function to publish a message
pub fn publish_message(message: Message) -> Result<(), String> {
    if let Some(mq) = get_sigmamq() {
        mq.publish(message)
    } else {
        Err("SigmaMQ not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_state_display() {
        assert_eq!(MessageState::Queued.to_string(), "queued");
        assert_eq!(MessageState::Processing.to_string(), "processing");
    }

    #[test]
    fn test_message() {
        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        )
        .with_state(MessageState::Acknowledged);

        assert_eq!(message.message_id, "msg1");
        assert_eq!(message.queue, "test_queue");
        assert_eq!(message.state, MessageState::Acknowledged);
    }

    #[test]
    fn test_sigmamq() {
        let mut mq = SigmaMQ::new();
        assert!(!mq.is_running());
        mq.start();
        assert!(mq.is_running());
        mq.stop();
        assert!(!mq.is_running());
    }

    #[test]
    fn test_sigmamq_publish() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        );

        assert!(mq.publish(message).is_ok());
        assert_eq!(mq.message_count(), 1);
    }

    #[test]
    fn test_sigmamq_consume() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        );

        mq.publish(message).unwrap();
        let result = mq.consume("test_queue");

        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_sigmamq_acknowledge() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        )
        .with_state(MessageState::Processing);

        mq.publish(message).unwrap();
        assert!(mq.acknowledge("msg1").is_ok());

        let msg = mq.get_message("msg1").unwrap();
        assert_eq!(msg.state, MessageState::Acknowledged);
    }

    #[test]
    fn test_sigmamq_list_by_queue() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        );

        mq.publish(message).unwrap();

        let messages = mq.list_by_queue("test_queue");
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_sigmamq_list_by_state() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        )
        .with_state(MessageState::Acknowledged);

        mq.publish(message).unwrap();

        let messages = mq.list_by_state(MessageState::Acknowledged);
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn test_sigmamq_stats() {
        let mut mq = SigmaMQ::new();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        );

        mq.publish(message).unwrap();

        let stats = mq.queue_stats();
        assert_eq!(stats.get("total_messages"), Some(&"1".to_string()));
        assert_eq!(stats.get("total_queues"), Some(&"1".to_string()));
    }

    #[test]
    fn test_publish_message_convenience() {
        init_sigmamq();

        let mut mq = get_sigmamq().unwrap();
        mq.start();

        let message = Message::new(
            "msg1".to_string(),
            "test_queue".to_string(),
            "payload".to_string(),
        );

        let result = publish_message(message);
        assert!(result.is_ok());
    }
}
