#![no_std]
#![no_main]

/// OOP-based MQTT Client for SigmaOS
/// Based on Ideas-999-Structured: IoT & Smart Home Item 986
/// Implements MQTT messaging for IoT

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TopicID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum QoS { AtMostOnce = 0, AtLeastOnce = 1, ExactlyOnce = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MQTTError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait MQTTTopic {
    fn id(&self) -> TopicID;
    fn topic(&self) -> &[u8];
    fn qos(&self) -> QoS;
}

#[repr(C)]
pub struct SimpleMQTTTopic {
    pub id: TopicID,
    pub topic: [u8; 128],
    pub qos: AtomicUsize,
}

impl SimpleMQTTTopic {
    pub fn new(id: TopicID, topic: &[u8], qos: QoS) -> Self {
        let mut topic_array = [0u8; 128];
        let topic_len = topic.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(topic.as_ptr(), topic_array.as_mut_ptr(), topic_len);
        }
        SimpleMQTTTopic {
            id,
            topic: topic_array,
            qos: AtomicUsize::new(qos as usize),
        }
    }
}

impl MQTTTopic for SimpleMQTTTopic {
    fn id(&self) -> TopicID { self.id }
    fn topic(&self) -> &[u8] {
        let len = self.topic.iter().position(|&b| b == 0).unwrap_or(128);
        &self.topic[..len]
    }
    fn qos(&self) -> QoS { unsafe { core::mem::transmute(self.qos.load(Ordering::SeqCst)) } }
}

pub trait MQTTClient {
    fn connect(&mut self, broker: &[u8], port: u16) -> Result<(), MQTTError>;
    fn disconnect(&mut self) -> Result<(), MQTTError>;
    fn subscribe(&mut self, topic: &[u8], qos: QoS) -> Result<TopicID, MQTTError>;
    fn publish(&self, topic: &[u8], message: &[u8], qos: QoS) -> Result<(), MQTTError>;
}

#[repr(C)]
pub struct SimpleMQTTClient {
    pub connected: AtomicUsize,
    pub subscriptions: Vec<Option<Box<dyn MQTTTopic>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMQTTClient {
    pub fn new() -> Self {
        SimpleMQTTClient {
            connected: AtomicUsize::new(0),
            subscriptions: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MQTTClient for SimpleMQTTClient {
    fn connect(&mut self, _broker: &[u8], _port: u16) -> Result<(), MQTTError> {
        self.connected.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn disconnect(&mut self) -> Result<(), MQTTError> {
        self.connected.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn subscribe(&mut self, topic: &[u8], qos: QoS) -> Result<TopicID, MQTTError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mqtt_topic = SimpleMQTTTopic::new(id, topic, qos);
        self.subscriptions.push(Some(Box::new(mqtt_topic)));
        Ok(id)
    }
    
    fn publish(&self, _topic: &[u8], _message: &[u8], _qos: QoS) -> Result<(), MQTTError> {
        if self.connected.load(Ordering::SeqCst) == 1 {
            Ok(())
        } else {
            Err(MQTTError::ConnectionFailed)
        }
    }
}

pub trait MessageHandler {
    def on_message(&self, topic: &[u8], payload: &[u8]);
}

#[repr(C)]
pub struct SimpleMessageHandler {
    pub messages: Vec<([u8; 128], Vec<u8>)>,
}

impl SimpleMessageHandler {
    pub fn new() -> Self {
        SimpleMessageHandler {
            messages: Vec::new(),
        }
    }
}

impl MessageHandler for SimpleMessageHandler {
    fn on_message(&self, topic: &[u8], payload: &[u8]) {
        let mut topic_array = [0u8; 128];
        let topic_len = topic.len().min(127);
        for i in 0..topic_len {
            topic_array[i] = topic[i];
        }
        let mut payload_vec = Vec::new();
        for &byte in payload {
            payload_vec.push(byte);
        }
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
