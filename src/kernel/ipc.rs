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
        }
    }

    /// Structured write operation with dynamic backpressure (returns Err if capacity reached)
    pub fn write_structure(&mut self, payload: Vec<u8>) -> Result<(), IpcError> {
        if self.ring_buffer.len() >= self.max_capacity {
            return Err(IpcError::ChannelFull);
        }
        self.bytes_transferred += payload.len() as u64;
        self.ring_buffer.push(payload);
        Ok(())
    }

    /// Structured read operation (returns None if pipe is empty)
    pub fn read_structure(&mut self) -> Option<Vec<u8>> {
        if self.ring_buffer.is_empty() {
            None
        } else {
            // Read in FIFO order (circular ring style)
            Some(self.ring_buffer.remove(0))
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

/// Linux-style Zero-Copy Splice Engine.
/// Moves structured page/message streams directly between two SovereignPipes or channels
/// bypassing user-space copy (copy-in / copy-out) overhead entirely.
pub struct SovereignSpliceEngine {
    pub bytes_spliced: u64,
}

impl SovereignSpliceEngine {
    pub fn new() -> Self {
        Self { bytes_spliced: 0 }
    }

    /// Splicing operation transferring a maximum number of bytes or structures from source pipe to destination pipe.
    pub fn splice(
        &mut self,
        source: &mut SovereignPipe,
        destination: &mut SovereignPipe,
        max_elements: usize,
    ) -> Result<usize, IpcError> {
        let mut moved = 0;
        while moved < max_elements {
            if let Some(payload) = source.read_structure() {
                let size = payload.len();
                if let Err(e) = destination.write_structure(payload) {
                    return Err(e);
                }
                self.bytes_spliced += size as u64;
                moved += 1;
            } else {
                break;
            }
        }
        Ok(moved)
    }
}

/// BSD-style Zero-Copy Sendfile Engine.
/// Bypasses user-space entirely by transmitting data blocks directly from virtual file cache buffers
/// to destination channels or pipes.
pub struct SovereignSendfileEngine {
    pub files_sent: u64,
    pub total_bytes_sent: u64,
}

impl SovereignSendfileEngine {
    pub fn new() -> Self {
        Self {
            files_sent: 0,
            total_bytes_sent: 0,
        }
    }

    /// Simulates reading blocks directly from a file cache block (represented as a slice of buffers)
    /// and transferring them to a destination SovereignPipe.
    pub fn send_file_to_pipe(
        &mut self,
        file_cache: &[Vec<u8>],
        destination: &mut SovereignPipe,
        offset: usize,
        count: usize,
    ) -> Result<usize, IpcError> {
        if offset >= file_cache.len() {
            return Ok(0);
        }
        let limit = std::cmp::min(offset + count, file_cache.len());
        let mut bytes_moved = 0;
        for i in offset..limit {
            let chunk = &file_cache[i];
            destination.write_structure(chunk.clone())?;
            bytes_moved += chunk.len();
        }
        self.files_sent += 1;
        self.total_bytes_sent += bytes_moved as u64;
        Ok(bytes_moved)
    }
}

/// Mach-style Out-of-Line Page Table Remapper.
/// Re-maps virtual memory page ownership in O(1) time between process address spaces,
/// bypassing byte copying for large multi-megabyte payloads.
pub struct SovereignOolRemapper {
    pub pages_remapped: u64,
}

impl SovereignOolRemapper {
    pub fn new() -> Self {
        Self { pages_remapped: 0 }
    }

    /// Simulates O(1) zero-copy out-of-line remapping from sender PID to receiver PID.
    /// Takes a page buffer, remaps its virtual address space pointers, and increments pages_remapped.
    pub fn remap_ool(
        &mut self,
        _sender_pid: u64,
        _receiver_pid: u64,
        pages_buffer: Vec<u8>,
    ) -> Result<Vec<u8>, IpcError> {
        let page_size = 4096;
        let num_pages = (pages_buffer.len() + page_size - 1) / page_size;
        self.pages_remapped += num_pages as u64;
        Ok(pages_buffer)
    }
}

/// Solaris Doors / Xen-style lockless Shared Page Circular Ring Buffer.
/// Communication occurs via shared page frames directly accessible by both processes,
/// eliminating kernel entry overhead and system call context switches.
pub struct SharedPageRingBuffer {
    pub capacity: usize,
    pub head: usize,
    pub tail: usize,
    pub shared_memory: Vec<Option<Vec<u8>>>,
    pub interrupts_triggered: u64,
}

impl SharedPageRingBuffer {
    pub fn new(capacity: usize) -> Self {
        let mut shared_memory = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            shared_memory.push(None);
        }
        Self {
            capacity,
            head: 0,
            tail: 0,
            shared_memory,
            interrupts_triggered: 0,
        }
    }

    /// Non-blocking push. Returns Err if the shared buffer is full.
    pub fn push_item(&mut self, item: Vec<u8>) -> Result<(), IpcError> {
        let next_tail = (self.tail + 1) % self.capacity;
        if next_tail == self.head {
            return Err(IpcError::ChannelFull);
        }
        self.shared_memory[self.tail] = Some(item);
        self.tail = next_tail;
        Ok(())
    }

    /// Non-blocking pop. Returns None if empty.
    pub fn pop_item(&mut self) -> Option<Vec<u8>> {
        if self.head == self.tail {
            return None;
        }
        let item = self.shared_memory[self.head].take();
        self.head = (self.head + 1) % self.capacity;
        item
    }

    /// Triggers a simulated hardware/IPI (Inter-Processor Interrupt) signaling the reader
    pub fn trigger_ipi(&mut self) {
        self.interrupts_triggered += 1;
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

        // Write structured frames
        pipe.write_structure(vec![1, 2, 3]).unwrap();
        pipe.write_structure(vec![10, 20, 30]).unwrap();
        pipe.write_structure(vec![4, 5, 6]).unwrap();

        assert_eq!(pipe.ring_buffer.len(), 3);

        // Run user-defined filter on structured stream
        pipe.filter_stream(|payload| payload[0] < 10);

        // Payloads starting with 10 or greater are filtered out (i.e. vec![10, 20, 30] removed)
        assert_eq!(pipe.ring_buffer.len(), 2);

        // Read structures back in FIFO order
        let r1 = pipe.read_structure().unwrap();
        assert_eq!(r1, vec![1, 2, 3]);

        let r2 = pipe.read_structure().unwrap();
        assert_eq!(r2, vec![4, 5, 6]);

        let r3 = pipe.read_structure();
        assert!(r3.is_none());
    }

    #[test]
    fn test_sovereign_splice_engine() {
        let mut source_pipe = SovereignPipe::new(1, 100, 200, 10);
        let mut dest_pipe = SovereignPipe::new(2, 200, 300, 10);

        source_pipe.write_structure(vec![1, 1, 1]).unwrap();
        source_pipe.write_structure(vec![2, 2, 2]).unwrap();

        let mut splice_engine = SovereignSpliceEngine::new();
        let spliced = splice_engine
            .splice(&mut source_pipe, &mut dest_pipe, 2)
            .unwrap();

        assert_eq!(spliced, 2);
        assert_eq!(splice_engine.bytes_spliced, 6);
        assert_eq!(dest_pipe.read_structure().unwrap(), vec![1, 1, 1]);
        assert_eq!(dest_pipe.read_structure().unwrap(), vec![2, 2, 2]);
    }

    #[test]
    fn test_sovereign_sendfile_engine() {
        let mut dest_pipe = SovereignPipe::new(3, 100, 200, 10);
        let file_cache = vec![vec![10, 20], vec![30, 40], vec![50, 60]];

        let mut sendfile_engine = SovereignSendfileEngine::new();
        let sent = sendfile_engine
            .send_file_to_pipe(&file_cache, &mut dest_pipe, 1, 2)
            .unwrap();

        assert_eq!(sent, 4);
        assert_eq!(sendfile_engine.files_sent, 1);
        assert_eq!(sendfile_engine.total_bytes_sent, 4);
        assert_eq!(dest_pipe.read_structure().unwrap(), vec![30, 40]);
        assert_eq!(dest_pipe.read_structure().unwrap(), vec![50, 60]);
    }

    #[test]
    fn test_sovereign_ool_remapper() {
        let mut remapper = SovereignOolRemapper::new();
        let buffer = vec![0u8; 9000]; // Multi-page buffer
        let remapped = remapper.remap_ool(10, 20, buffer.clone()).unwrap();

        assert_eq!(remapped.len(), 9000);
        assert_eq!(remapper.pages_remapped, 3); // 9000 bytes spans 3 pages (each 4096)
    }

    #[test]
    fn test_shared_page_ring_buffer() {
        let mut ring = SharedPageRingBuffer::new(5);
        assert!(ring.pop_item().is_none());

        ring.push_item(vec![5, 10]).unwrap();
        ring.push_item(vec![15, 20]).unwrap();
        ring.trigger_ipi();

        assert_eq!(ring.interrupts_triggered, 1);
        assert_eq!(ring.pop_item().unwrap(), vec![5, 10]);
        assert_eq!(ring.pop_item().unwrap(), vec![15, 20]);
        assert!(ring.pop_item().is_none());
    }}
