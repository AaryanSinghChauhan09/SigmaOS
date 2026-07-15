// SigmaOS Kernel IPC (Inter-Process Communication)
// Zero-latency capability-based IPC

use crate::security::CapabilityToken;

/// IPC message type
#[derive(Debug, Clone)]
pub enum Message {
    Data(Vec<u8>),
    FileDescriptor(u64),
    Capability(CapabilityToken),
    Signal(u32),
}

/// IPC channel
#[derive(Debug)]
pub struct Channel {
    pub id: u64,
    pub sender: u64,
    pub receiver: u64,
    pub messages: Vec<Message>,
    pub capacity: usize,
}

impl Channel {
    pub fn new(id: u64, sender: u64, receiver: u64) -> Self {
        Self {
            id,
            sender,
            receiver,
            messages: Vec::new(),
            capacity: 256,
        }
    }

    pub fn send(&mut self, message: Message) -> Result<(), IpcError> {
        if self.messages.len() >= self.capacity {
            return Err(IpcError::ChannelFull);
        }
        self.messages.push(message);
        Ok(())
    }

    pub fn receive(&mut self) -> Option<Message> {
        self.messages.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

/// IPC manager
pub struct IpcManager {
    channels: Vec<Channel>,
    next_id: u64,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            next_id: 0,
        }
    }

    pub fn create_channel(&mut self, sender: u64, receiver: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let channel = Channel::new(id, sender, receiver);
        self.channels.push(channel);
        
        id
    }

    pub fn send(&mut self, channel_id: u64, message: Message, sender: u64) -> Result<(), IpcError> {
        let channel = self.channels.iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or(IpcError::ChannelNotFound)?;
        
        if channel.sender != sender {
            return Err(IpcError::PermissionDenied);
        }
        
        channel.send(message)
    }

    pub fn receive(&mut self, channel_id: u64, receiver: u64) -> Result<Option<Message>, IpcError> {
        let channel = self.channels.iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or(IpcError::ChannelNotFound)?;
        
        if channel.receiver != receiver {
            return Err(IpcError::PermissionDenied);
        }
        
        Ok(channel.receive())
    }

    pub fn remove_channel(&mut self, channel_id: u64) {
        self.channels.retain(|c| c.id != channel_id);
    }
}

impl Default for IpcManager {
    fn default() -> Self {
        Self::new()
    }
}

/// IPC errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    ChannelNotFound,
    ChannelFull,
    PermissionDenied,
    InvalidMessage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let channel = Channel::new(1, 100, 200);
        assert_eq!(channel.id, 1);
        assert_eq!(channel.sender, 100);
        assert_eq!(channel.receiver, 200);
    }

    #[test]
    fn test_send_receive() {
        let mut channel = Channel::new(1, 100, 200);
        let message = Message::Data(vec![1, 2, 3]);
        
        assert!(channel.send(message.clone()).is_ok());
        assert_eq!(channel.len(), 1);
        
        let received = channel.receive();
        assert!(received.is_some());
    }

    #[test]
    fn test_ipc_manager() {
        let mut manager = IpcManager::new();
        let channel_id = manager.create_channel(100, 200);
        
        let message = Message::Data(vec![1, 2, 3]);
        assert!(manager.send(channel_id, message, 100).is_ok());
        
        let received = manager.receive(channel_id, 200);
        assert!(received.is_ok());
    }
}
