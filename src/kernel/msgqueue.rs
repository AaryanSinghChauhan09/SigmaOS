// Linux-inspired message queue (msgqueue) for IPC
// System V message queue interface for SigmaOS

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// message type
#[derive(Debug, Clone)]
pub struct Message {
    pub mtype: i32,
    pub data: Vec<u8>,
}

impl Message {
    pub fn new(mtype: i32, data: Vec<u8>) -> Self {
        Message { mtype, data }
    }
}

/// message queue
pub struct MessageQueue {
    messages: VecDeque<Message>,
    capacity: usize,
    key: i32,
}

impl MessageQueue {
    pub fn new(key: i32, capacity: usize) -> Self {
        MessageQueue {
            messages: VecDeque::with_capacity(capacity),
            capacity,
            key,
        }
    }

    /// Send a message
    pub fn send(&mut self, message: Message) -> Result<(), String> {
        if self.messages.len() >= self.capacity {
            return Err("Queue full".to_string());
        }

        self.messages.push_back(message);
        Ok(())
    }

    /// Receive a message
    pub fn receive(&mut self) -> Result<Message, String> {
        if self.messages.is_empty() {
            return Err("Queue empty".to_string());
        }

        Ok(self.messages.pop_front().unwrap())
    }

    /// Get message count
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Get queue capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get queue key
    pub fn key(&self) -> i32 {
        self.key
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        self.messages.len() >= self.capacity
    }
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new(0, 1024)
    }
}

/// message queue manager
pub struct MessageQueueManager {
    queues: HashMap<i32, Arc<Mutex<MessageQueue>>>,
    next_key: i32,
}

impl MessageQueueManager {
    pub fn new() -> Self {
        MessageQueueManager {
            queues: HashMap::new(),
            next_key: 1,
        }
    }

    /// Create a new message queue
    pub fn create(&mut self, capacity: usize) -> Result<i32, String> {
        let key = self.next_key;
        self.next_key += 1;

        let queue = Arc::new(Mutex::new(MessageQueue::new(key, capacity)));
        self.queues.insert(key, queue);

        Ok(key)
    }

    /// Get a message queue by key
    pub fn get(&self, key: i32) -> Option<Arc<Mutex<MessageQueue>>> {
        self.queues.get(&key).cloned()
    }

    /// Remove a message queue
    pub fn remove(&mut self, key: i32) -> Result<(), String> {
        self.queues.remove(&key)
            .ok_or_else(|| format!("Queue not found: {}", key))?;
        Ok(())
    }

    /// Get queue count
    pub fn queue_count(&self) -> usize {
        self.queues.len()
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
    fn test_message_creation() {
        let message = Message::new(1, b"hello".to_vec());
        assert_eq!(message.mtype, 1);
        assert_eq!(message.data, b"hello");
    }

    #[test]
    fn test_message_queue_creation() {
        let queue = MessageQueue::new(12345, 1024);
        assert_eq!(queue.key(), 12345);
        assert_eq!(queue.capacity(), 1024);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_message_queue_send() {
        let mut queue = MessageQueue::new(12345, 1024);
        let message = Message::new(1, b"hello".to_vec());
        
        queue.send(message).unwrap();
        assert_eq!(queue.message_count(), 1);
    }

    #[test]
    fn test_message_queue_send_full() {
        let mut queue = MessageQueue::new(12345, 1);
        let message = Message::new(1, b"hello".to_vec());
        
        queue.send(message).unwrap();
        
        let result = queue.send(Message::new(2, b"world".to_vec()));
        assert!(result.is_err());
    }

    #[test]
    fn test_message_queue_receive() {
        let mut queue = MessageQueue::new(12345, 1024);
        let message = Message::new(1, b"hello".to_vec());
        
        queue.send(message).unwrap();
        
        let received = queue.receive().unwrap();
        assert_eq!(received.mtype, 1);
        assert_eq!(received.data, b"hello");
    }

    #[test]
    fn test_message_queue_receive_empty() {
        let mut queue = MessageQueue::new(12345, 1024);
        
        let result = queue.receive();
        assert!(result.is_err());
    }

    #[test]
    fn test_message_queue_manager_creation() {
        let manager = MessageQueueManager::new();
        assert_eq!(manager.queue_count(), 0);
    }

    #[test]
    fn test_message_queue_manager_create() {
        let mut manager = MessageQueueManager::new();
        
        let key = manager.create(1024).unwrap();
        assert_eq!(key, 1);
        assert_eq!(manager.queue_count(), 1);
    }

    #[test]
    fn test_message_queue_manager_get() {
        let mut manager = MessageQueueManager::new();
        
        let key = manager.create(1024).unwrap();
        let queue = manager.get(key);
        
        assert!(queue.is_some());
    }

    #[test]
    fn test_message_queue_manager_remove() {
        let mut manager = MessageQueueManager::new();
        
        let key = manager.create(1024).unwrap();
        manager.remove(key).unwrap();
        
        assert_eq!(manager.queue_count(), 0);
    }
}
