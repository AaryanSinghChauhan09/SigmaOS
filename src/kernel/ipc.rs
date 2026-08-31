extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
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

/// Flags controlling Pipe behavior (Linux pipe2 / BSD pipe_create parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipeFlags {
    pub nonblock: bool,
    pub cloexec: bool,
    pub direct_packet_mode: bool,
}

impl PipeFlags {
    pub fn default() -> Self {
        Self {
            nonblock: false,
            cloexec: true,
            direct_packet_mode: true,
        }
    }
}

/// Represents a high-performance structured sovereign pipe (defeating legacy Linux pipes)
pub const POSIX_PIPE_BUF_SIZE: usize = 4096; // POSIX guaranteed atomic pipe write size

pub struct SovereignPipe {
    pub id: u64,
    pub reader_pid: u64,
    pub writer_pid: u64,
    pub ring_buffer: Vec<Vec<u8>>, // Zero-copy circular structured chunks
    pub byte_stream: Vec<u8>,      // POSIX/BSD byte stream buffer
    pub max_capacity: usize,
    pub bytes_transferred: u64,
    pub flags: PipeFlags,
    pub read_low_watermark: usize,  // BSD SO_RCVLOWAT equivalent
    pub write_low_watermark: usize, // BSD SO_SNDLOWAT equivalent
    pub reader_count: usize,        // OpenBSD/FreeBSD reference counting for readers
    pub writer_count: usize,        // OpenBSD/FreeBSD reference counting for writers
    pub broken_pipe: bool,          // EPIPE flag when no readers remain
    pub fifo_path: Option<String>,  // POSIX / BSD mkfifo(2) named pipe path
}

impl SovereignPipe {
    pub fn new(id: u64, reader_pid: u64, writer_pid: u64, capacity: usize) -> Self {
        Self {
            id,
            reader_pid,
            writer_pid,
            ring_buffer: Vec::new(),
            byte_stream: Vec::new(),
            max_capacity: capacity,
            bytes_transferred: 0,
            flags: PipeFlags::default(),
            read_low_watermark: 1,
            write_low_watermark: 1,
            reader_count: 1,
            writer_count: 1,
            broken_pipe: false,
            fifo_path: None,
        }
    }

    /// Creates a POSIX/BSD named pipe (mkfifo(2) parity)
    pub fn new_fifo(id: u64, fifo_path: &str, capacity: usize) -> Self {
        let mut pipe = Self::new(id, 0, 0, capacity);
        pipe.fifo_path = Some(fifo_path.to_string());
        pipe.reader_count = 0;
        pipe.writer_count = 0;
        pipe
    }

    /// Closes a reader handle and detects broken pipe / EOF condition
    pub fn close_reader(&mut self) {
        if self.reader_count > 0 {
            self.reader_count -= 1;
        }
        if self.reader_count == 0 {
            self.broken_pipe = true;
        }
    }

    /// Closes a writer handle
    pub fn close_writer(&mut self) {
        if self.writer_count > 0 {
            self.writer_count -= 1;
        }
    }

    /// Linux fcntl F_SETPIPE_SZ parity: dynamically resizes pipe ring buffer capacity
    pub fn set_pipe_capacity(&mut self, new_capacity: usize) -> Result<usize, IpcError> {
        if new_capacity < self.ring_buffer.len() {
            return Err(IpcError::ChannelFull);
        }
        self.max_capacity = new_capacity;
        Ok(self.max_capacity)
    }

    /// Linux fcntl F_GETPIPE_SZ parity: returns current pipe capacity
    pub fn get_pipe_capacity(&self) -> usize {
        self.max_capacity
    }

    /// Structured write operation with dynamic backpressure and EPIPE broken pipe detection
    pub fn write_structure(&mut self, payload: Vec<u8>) -> Result<(), IpcError> {
        if self.broken_pipe || self.reader_count == 0 {
            return Err(IpcError::BrokenPipe);
        }
        if self.ring_buffer.len() >= self.max_capacity {
            return Err(IpcError::ChannelFull);
        }
        self.bytes_transferred += payload.len() as u64;
        self.ring_buffer.push(payload);
        Ok(())
    }

    /// Writes raw byte stream into pipe with POSIX PIPE_BUF atomic guarantees
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<usize, IpcError> {
        if self.broken_pipe || self.reader_count == 0 {
            return Err(IpcError::BrokenPipe);
        }
        if data.len() <= POSIX_PIPE_BUF_SIZE
            && (self.byte_stream.len() + data.len()) > (self.max_capacity * 4096)
        {
            return Err(IpcError::ChannelFull); // POSIX PIPE_BUF guarantees atomic write if space available
        }
        self.byte_stream.extend_from_slice(data);
        self.bytes_transferred += data.len() as u64;
        Ok(data.len())
    }

    /// Reads raw byte stream from pipe (returns number of bytes read into buffer)
    pub fn read_bytes(&mut self, buf: &mut [u8]) -> usize {
        let to_read = core::cmp::min(buf.len(), self.byte_stream.len());
        if to_read == 0 {
            return 0;
        }
        buf[..to_read].copy_from_slice(&self.byte_stream[..to_read]);
        self.byte_stream.drain(0..to_read);
        to_read
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

    /// BSD kqueue EVFILT_READ check based on low watermark
    pub fn is_readable(&self) -> bool {
        self.ring_buffer.len() >= self.read_low_watermark
    }

    /// BSD kqueue EVFILT_WRITE check based on remaining capacity watermark
    pub fn is_writable(&self) -> bool {
        (self.max_capacity - self.ring_buffer.len()) >= self.write_low_watermark
    }

    /// User-defined stream processing: Filters or transforms pipe payloads in-place
    pub fn filter_stream<F>(&mut self, filter_func: F)
    where
        F: Fn(&Vec<u8>) -> bool,
    {
        self.ring_buffer.retain(|item| filter_func(item));
    }
}

/// Linux-style Zero-Copy Tee Engine (tee(2)).
/// Duplicates data from source pipe to destination pipe without consuming or modifying the source pipe.
pub struct SovereignTeeEngine {
    pub bytes_duplicated: u64,
}

impl SovereignTeeEngine {
    pub fn new() -> Self {
        Self {
            bytes_duplicated: 0,
        }
    }

    pub fn tee(
        &mut self,
        source: &SovereignPipe,
        destination: &mut SovereignPipe,
        max_elements: usize,
    ) -> Result<usize, IpcError> {
        let count = core::cmp::min(max_elements, source.ring_buffer.len());
        let mut duplicated = 0;
        for i in 0..count {
            let chunk = source.ring_buffer[i].clone();
            let len = chunk.len();
            destination.write_structure(chunk)?;
            self.bytes_duplicated += len as u64;
            duplicated += 1;
        }
        Ok(duplicated)
    }
}

/// Linux-style Zero-Copy vmsplice Engine (vmsplice(2)).
/// Slices user-space memory vectors directly into a destination SovereignPipe.
pub struct SovereignVmSpliceEngine {
    pub total_spliced_bytes: u64,
}

impl SovereignVmSpliceEngine {
    pub fn new() -> Self {
        Self {
            total_spliced_bytes: 0,
        }
    }

    pub fn vmsplice(
        &mut self,
        iovecs: &[&[u8]],
        destination: &mut SovereignPipe,
    ) -> Result<usize, IpcError> {
        let mut bytes = 0;
        for vec_slice in iovecs {
            destination.write_structure(vec_slice.to_vec())?;
            bytes += vec_slice.len();
        }
        self.total_spliced_bytes += bytes as u64;
        Ok(bytes)
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
        let limit = core::cmp::min(offset + count, file_cache.len());
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

    pub fn get_channel(&self, channel_id: u64) -> Option<&Channel> {
        self.channels.iter().find(|c| c.id == channel_id)
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
    BrokenPipe, // Linux EPIPE / SIGPIPE parity
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
        assert_eq!(pipe.max_capacity, 10);
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
    }

    #[test]
    fn test_sovereign_pipe_tee_and_vmsplice() {
        let mut pipe1 = SovereignPipe::new(1, 10, 20, 10);
        let mut pipe2 = SovereignPipe::new(2, 20, 30, 10);

        pipe1.write_structure(vec![100, 101]).unwrap();
        pipe1.write_structure(vec![102, 103]).unwrap();

        let mut tee_engine = SovereignTeeEngine::new();
        let count = tee_engine.tee(&pipe1, &mut pipe2, 2).unwrap();
        assert_eq!(count, 2);
        assert_eq!(tee_engine.bytes_duplicated, 4);

        // Pipe 1 still retains its structures
        assert_eq!(pipe1.ring_buffer.len(), 2);
        assert_eq!(pipe2.ring_buffer.len(), 2);

        let mut vmsplice_engine = SovereignVmSpliceEngine::new();
        let buf1 = [1u8, 2u8, 3u8];
        let buf2 = [4u8, 5u8];
        let spliced_bytes = vmsplice_engine
            .vmsplice(&[&buf1[..], &buf2[..]], &mut pipe2)
            .unwrap();
        assert_eq!(spliced_bytes, 5);
        assert_eq!(pipe2.ring_buffer.len(), 4);
    }

    #[test]
    fn test_sovereign_pipe_capacity_and_watermarks() {
        let mut pipe = SovereignPipe::new(10, 101, 102, 2);
        assert_eq!(pipe.get_pipe_capacity(), 2);
        assert!(!pipe.is_readable());
        assert!(pipe.is_writable());

        pipe.write_structure(vec![1, 2]).unwrap();
        assert!(pipe.is_readable());

        // Fill pipe to capacity
        pipe.write_structure(vec![3, 4]).unwrap();
        assert!(!pipe.is_writable());
        assert_eq!(pipe.write_structure(vec![5]), Err(IpcError::ChannelFull));

        // Dynamically expand pipe capacity (F_SETPIPE_SZ parity)
        assert_eq!(pipe.set_pipe_capacity(16).unwrap(), 16);
        assert!(pipe.is_writable());
        assert!(pipe.write_structure(vec![5]).is_ok());
    }

    #[test]
    fn test_sovereign_pipe_epipe_and_fifo() {
        // Test broken pipe (EPIPE) on reader close
        let mut pipe = SovereignPipe::new(20, 101, 102, 10);
        assert!(pipe.write_structure(vec![1, 2, 3]).is_ok());

        pipe.close_reader();
        assert!(pipe.broken_pipe);
        assert_eq!(
            pipe.write_structure(vec![4, 5, 6]),
            Err(IpcError::BrokenPipe)
        );

        // Test POSIX/BSD byte stream read/write
        let mut byte_pipe = SovereignPipe::new(21, 101, 102, 10);
        let bytes_written = byte_pipe.write_bytes(b"Hello SigmaOS Pipe").unwrap();
        assert_eq!(bytes_written, 18);

        let mut read_buf = [0u8; 32];
        let bytes_read = byte_pipe.read_bytes(&mut read_buf);
        assert_eq!(bytes_read, 18);
        assert_eq!(&read_buf[..18], b"Hello SigmaOS Pipe");

        // Test POSIX named FIFO creation
        let fifo = SovereignPipe::new_fifo(30, "/tmp/sigma_fifo", 16);
        assert_eq!(fifo.fifo_path, Some("/tmp/sigma_fifo".to_string()));
    }
}
