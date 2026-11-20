//! IPC channel for SigmaOS-app-framework.
//!
//! Provides a typed in-process message bus. The real implementation
//! uses zbus D-Bus; this module provides the API surface and a stub
//! implementation for testing.

use std::collections::VecDeque;
use std::any::Any;

/// An IPC message: a topic string plus a boxed payload.
pub struct IpcMessage {
    pub topic: String,
    pub payload: Box<dyn Any + Send>,
}

impl std::fmt::Debug for IpcMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IpcMessage")
            .field("topic", &self.topic)
            .finish()
    }
}

/// A simple in-process IPC channel (stub for zbus D-Bus).
#[derive(Debug, Default)]
pub struct IpcChannel {
    pub name: String,
    queue: VecDeque<String>, // simplified: just topic strings for testing
}

impl IpcChannel {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), queue: VecDeque::new() }
    }

    /// Send a message to the channel.
    pub fn send(&mut self, topic: impl Into<String>) {
        self.queue.push_back(topic.into());
    }

    /// Receive the next message from the channel.
    pub fn receive(&mut self) -> Option<String> {
        self.queue.pop_front()
    }

    pub fn pending(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/// IPC bus that routes messages between named channels.
#[derive(Debug, Default)]
pub struct IpcBus {
    channels: std::collections::HashMap<String, IpcChannel>,
}

impl IpcBus {
    pub fn new() -> Self { Self::default() }

    pub fn create_channel(&mut self, name: impl Into<String>) -> &mut IpcChannel {
        let name = name.into();
        self.channels.entry(name.clone()).or_insert_with(|| IpcChannel::new(name.clone()));
        self.channels.get_mut(&name).unwrap()
    }

    pub fn channel(&self, name: &str) -> Option<&IpcChannel> {
        self.channels.get(name)
    }

    pub fn channel_mut(&mut self, name: &str) -> Option<&mut IpcChannel> {
        self.channels.get_mut(name)
    }

    pub fn broadcast(&mut self, topic: impl Into<String>) {
        let topic = topic.into();
        for ch in self.channels.values_mut() {
            ch.send(topic.clone());
        }
    }
}
