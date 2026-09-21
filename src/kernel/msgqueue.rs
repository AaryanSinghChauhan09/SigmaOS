// Linux-inspired message queue for IPC
// Provides System V style message queue

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Message queue permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsgQPerm {
    pub read: bool,
    pub write: bool,
}

impl MsgQPerm {
    pub fn new() -> Self {
        Self {
            read: false,
            write: false,
        }
    }

    pub fn with_read(mut self) -> Self {
        self.read = true;
        self
    }

    pub fn with_write(mut self) -> Self {
        self.write = true;
        self
    }
}

impl Default for MsgQPerm {
    fn default() -> Self {
        Self::new()
    }
}

/// Message
#[derive(Debug, Clone)]
pub struct Message {
    pub mtype: i64,
    pub data: Vec<u8>,
}

impl Message {
    pub fn new(mtype: i64, data: Vec<u8>) -> Self {
        Self { mtype, data }
    }
}

/// Message queue
#[derive(Debug, Clone)]
pub struct MessageQueue {
    pub id: u64,
    pub key: i32,
    pub messages: VecDeque<Message>,
    pub perm: MsgQPerm,
    pub max_size: usize,
}

impl MessageQueue {
    pub fn new(id: u64, key: i32, perm: MsgQPerm, max_size: usize) -> Self {
        Self {
            id,
            key,
            messages: VecDeque::new(),
            perm,
            max_size,
        }
    }

    /// Send a message
    pub fn send(&mut self, message: Message) -> Result<(), String> {
        if !self.perm.write {
            return Err("No write permission".to_string());
        }

        if self.messages.len() >= self.max_size {
            return Err("Queue full".to_string());
        }

        self.messages.push_back(message);
        Ok(())
    }

    /// Receive a message
    pub fn receive(&mut self, mtype: i64) -> Result<Message, String> {
        if !self.perm.read {
            return Err("No read permission".to_string());
        }

        if self.messages.is_empty() {
            return Err("Queue empty".to_string());
        }

        if mtype == 0 {
            // Receive first message
            match self.messages.pop_front() {
                Some(msg) => Ok(msg),
                None => Err("Queue empty".to_string()),
            }
        } else {
            // Receive first message with matching type
            let pos = self.messages.iter().position(|m| m.mtype == mtype);
            match pos {
                Some(idx) => {
                    let msg = self.messages.remove(idx).unwrap();
                    Ok(msg)
                }
                None => Err("No message with matching type".to_string()),
            }
        }
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Check if full
    pub fn is_full(&self) -> bool {
        self.messages.len() >= self.max_size
    }
}

/// Message queue manager for system-wide message queue management
pub struct MessageQueueManager {
    queues: Arc<Mutex<HashMap<u64, MessageQueue>>>,
    next_queue_id: Arc<Mutex<u64>>,
}

impl MessageQueueManager {
    pub fn new() -> Self {
        Self {
            queues: Arc::new(Mutex::new(HashMap::new())),
            next_queue_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new message queue
    pub fn create_queue(&self, key: i32, perm: MsgQPerm, max_size: usize) -> u64 {
        let mut next_id = self.next_queue_id.lock().unwrap();
        let queue_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let queue = MessageQueue::new(queue_id, key, perm, max_size);
        let mut queues = self.queues.lock().unwrap();
        queues.insert(queue_id, queue);

        queue_id
    }

    /// Get a queue by ID
    pub fn get_queue(&self, queue_id: u64) -> Option<MessageQueue> {
        let queues = self.queues.lock().unwrap();
        queues.get(&queue_id).cloned()
    }

    /// Get a queue by key
    pub fn get_queue_by_key(&self, key: i32) -> Option<MessageQueue> {
        let queues = self.queues.lock().unwrap();
        queues.values().find(|q| q.key == key).cloned()
    }

    /// Remove a queue
    pub fn remove_queue(&self, queue_id: u64) -> Result<(), String> {
        let mut queues = self.queues.lock().unwrap();
        match queues.remove(&queue_id) {
            Some(_) => Ok(()),
            None => Err(format!("Queue {} not found", queue_id)),
        }
    }

    /// Send a message
    pub fn send(&self, queue_id: u64, message: Message) -> Result<(), String> {
        let mut queues = self.queues.lock().unwrap();
        match queues.get_mut(&queue_id) {
            Some(queue) => queue.send(message),
            None => Err(format!("Queue {} not found", queue_id)),
        }
    }

    /// Receive a message
    pub fn receive(&self, queue_id: u64, mtype: i64) -> Result<Message, String> {
        let mut queues = self.queues.lock().unwrap();
        match queues.get_mut(&queue_id) {
            Some(queue) => queue.receive(mtype),
            None => Err(format!("Queue {} not found", queue_id)),
        }
    }

    /// Get queue count
    pub fn queue_count(&self) -> usize {
        let queues = self.queues.lock().unwrap();
        queues.len()
    }
}

impl Default for MessageQueueManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_q_perm() {
        let perm = MsgQPerm::new().with_read().with_write();
        assert!(perm.read);
        assert!(perm.write);
    }

    #[test]
    fn test_message() {
        let message = Message::new(1, b"Hello".to_vec());
        assert_eq!(message.mtype, 1);
        assert_eq!(message.data, b"Hello");
    }

    #[test]
    fn test_message_queue() {
        let perm = MsgQPerm::new().with_read().with_write();
        let queue = MessageQueue::new(1, 12345, perm, 10);

        assert_eq!(queue.id, 1);
        assert_eq!(queue.key, 12345);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_message_queue_send() {
        let perm = MsgQPerm::new().with_read().with_write();
        let mut queue = MessageQueue::new(1, 12345, perm, 10);

        let message = Message::new(1, b"Hello".to_vec());
        queue.send(message).unwrap();

        assert_eq!(queue.message_count(), 1);
    }

    #[test]
    fn test_message_queue_send_no_perm() {
        let perm = MsgQPerm::new().with_read();
        let mut queue = MessageQueue::new(1, 12345, perm, 10);

        let message = Message::new(1, b"Hello".to_vec());
        assert!(queue.send(message).is_err());
    }

    #[test]
    fn test_message_queue_send_full() {
        let perm = MsgQPerm::new().with_read().with_write();
        let mut queue = MessageQueue::new(1, 12345, perm, 1);

        let message = Message::new(1, b"Hello".to_vec());
        queue.send(message.clone()).unwrap();
        assert!(queue.send(message).is_err());
    }

    #[test]
    fn test_message_queue_receive() {
        let perm = MsgQPerm::new().with_read().with_write();
        let mut queue = MessageQueue::new(1, 12345, perm, 10);

        let message = Message::new(1, b"Hello".to_vec());
        queue.send(message).unwrap();

        let received = queue.receive(0).unwrap();
        assert_eq!(received.mtype, 1);
        assert_eq!(received.data, b"Hello");
    }

    #[test]
    fn test_message_queue_receive_no_perm() {
        let perm = MsgQPerm::new().with_write();
        let mut queue = MessageQueue::new(1, 12345, perm, 10);

        assert!(queue.receive(0).is_err());
    }

    #[test]
    fn test_message_queue_receive_by_type() {
        let perm = MsgQPerm::new().with_read().with_write();
        let mut queue = MessageQueue::new(1, 12345, perm, 10);

        queue.send(Message::new(1, b"Type1".to_vec())).unwrap();
        queue.send(Message::new(2, b"Type2".to_vec())).unwrap();

        let received = queue.receive(2).unwrap();
        assert_eq!(received.mtype, 2);
        assert_eq!(received.data, b"Type2");
    }

    #[test]
    fn test_message_queue_manager() {
        let manager = MessageQueueManager::new();

        let perm = MsgQPerm::new().with_read().with_write();
        let queue_id = manager.create_queue(12345, perm, 10);

        assert_eq!(queue_id, 1);
        assert_eq!(manager.queue_count(), 1);
    }

    #[test]
    fn test_message_queue_manager_send_receive() {
        let manager = MessageQueueManager::new();

        let perm = MsgQPerm::new().with_read().with_write();
        let queue_id = manager.create_queue(12345, perm, 10);

        let message = Message::new(1, b"Hello".to_vec());
        manager.send(queue_id, message).unwrap();
        let received = manager.receive(queue_id, 0).unwrap();

        assert_eq!(received.data, b"Hello");
    }

    #[test]
    fn test_message_queue_manager_get_by_key() {
        let manager = MessageQueueManager::new();

        let perm = MsgQPerm::new().with_read().with_write();
        manager.create_queue(12345, perm, 10);

        let queue = manager.get_queue_by_key(12345);
        assert!(queue.is_some());
    }

    #[test]
    fn test_message_queue_manager_remove() {
        let manager = MessageQueueManager::new();

        let perm = MsgQPerm::new().with_read().with_write();
        let queue_id = manager.create_queue(12345, perm, 10);

        manager.remove_queue(queue_id).unwrap();
        assert_eq!(manager.queue_count(), 0);
    }

    #[test]
    fn test_message_queue_manager_invalid() {
        let manager = MessageQueueManager::new();
        assert!(manager.send(999, Message::new(1, vec![])).is_err());
        assert!(manager.receive(999, 0).is_err());
    }
}
