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

/// Represents a high-performance structured sovereign pipe (defeating legacy Linux pipes)
pub struct SovereignPipe {
    pub id: u64,
    pub reader_pid: u64,
    pub writer_pid: u64,
    pub ring_buffer: Vec<Vec<u8>>, // Zero-copy circular structured chunks
    pub max_capacity: usize,
    pub bytes_transferred: u64,
    pub non_blocking: bool,
}

impl SovereignPipe {
    pub fn new(id: u64, reader_pid: u64, writer_pid: u64, capacity: usize) -> Self {
        Self {
            id,
            reader_pid,
            writer_pid,
            ring_buffer: Vec::new(),
            max_capacity: capacity,
            bytes_transferred: 0,
            non_blocking: false,
        }
    }

    /// Set non-blocking mode (mimics O_NONBLOCK flag on Linux pipes)
    pub fn set_non_blocking(&mut self, non_blocking: bool) {
        self.non_blocking = non_blocking;
    }

    /// Resize capacity dynamically (mimics F_SETPIPE_SZ fcntl command)
    pub fn resize_capacity(&mut self, new_capacity: usize) -> Result<(), IpcError> {
        if new_capacity < self.ring_buffer.len() {
            return Err(IpcError::InvalidMessage);
        }
        self.max_capacity = new_capacity;
        Ok(())
    }

    /// Checks if pipe has data to read (mimics POLLIN)
    pub fn can_read(&self) -> bool {
        !self.ring_buffer.is_empty()
    }

    /// Checks if pipe has space to write (mimics POLLOUT)
    pub fn can_write(&self) -> bool {
        self.ring_buffer.len() < self.max_capacity
    }

    /// Structured write operation with dynamic backpressure (returns Err if capacity reached)
    pub fn write_structure(&mut self, payload: Vec<u8>) -> Result<(), IpcError> {
        if self.ring_buffer.len() >= self.max_capacity {
            if self.non_blocking {
                return Err(IpcError::WouldBlock);
            }
            return Err(IpcError::ChannelFull);
        }
        self.bytes_transferred += payload.len() as u64;
        self.ring_buffer.push(payload);
        Ok(())
    }

    /// Structured read operation (returns None or WouldBlock error)
    pub fn read_structure(&mut self) -> Result<Option<Vec<u8>>, IpcError> {
        if self.ring_buffer.is_empty() {
            if self.non_blocking {
                return Err(IpcError::WouldBlock);
            }
            Ok(None)
        } else {
            // Read in FIFO order (circular ring style)
            Ok(Some(self.ring_buffer.remove(0)))
        }
    }

    /// User-defined stream processing: Filters or transforms pipe payloads in-place
    pub fn filter_stream<F>(&mut self, filter_func: F)
    where
        F: Fn(&Vec<u8>) -> bool,
    {
        self.ring_buffer.retain(|item| filter_func(item));
    }
}

/// IPC manager
pub struct IpcManager {
    channels: Vec<Channel>,
    next_id: u64,
    pub pipes: Vec<SovereignPipe>,
}

impl IpcManager {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            next_id: 0,
            pipes: Vec::new(),
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
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or(IpcError::ChannelNotFound)?;

        if channel.sender != sender {
            return Err(IpcError::PermissionDenied);
        }

        channel.send(message)
    }

    pub fn receive(&mut self, channel_id: u64, receiver: u64) -> Result<Option<Message>, IpcError> {
        let channel = self
            .channels
            .iter_mut()
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
    WouldBlock,
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

    #[test]
    fn test_sovereign_pipes_vs_linux_pipes() {
        let mut pipe = SovereignPipe::new(1, 101, 102, 10);

        assert!(pipe.can_write());
        assert!(!pipe.can_read());

        // Write structured frames
        pipe.write_structure(vec![1, 2, 3]).unwrap();
        pipe.write_structure(vec![10, 20, 30]).unwrap();
        pipe.write_structure(vec![4, 5, 6]).unwrap();

        assert_eq!(pipe.ring_buffer.len(), 3);
        assert!(pipe.can_read());

        // Run user-defined filter on structured stream
        pipe.filter_stream(|payload| payload[0] < 10);

        // Payloads starting with 10 or greater are filtered out (i.e. vec![10, 20, 30] removed)
        assert_eq!(pipe.ring_buffer.len(), 2);

        // Read structures back in FIFO order
        let r1 = pipe.read_structure().unwrap().unwrap();
        assert_eq!(r1, vec![1, 2, 3]);

        let r2 = pipe.read_structure().unwrap().unwrap();
        assert_eq!(r2, vec![4, 5, 6]);

        let r3 = pipe.read_structure().unwrap();
        assert!(r3.is_none());

        // Test non-blocking mode & dynamic resizing
        pipe.set_non_blocking(true);
        assert_eq!(pipe.read_structure().unwrap_err(), IpcError::WouldBlock);

        pipe.resize_capacity(20).unwrap();
        assert_eq!(pipe.max_capacity, 20);
    }
}
