//! # Pipe IPC Mechanism
//!
//! Unix pipe implementation inspired by Linux pipe buffer and BSD pipe architecture.
//! Provides anonymous pipes, named pipes (FIFOs), and splice operations.

#![no_std]

extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Pipe buffer size (64KB - Linux default)
pub const PIPE_BUF_SIZE: usize = 65536;

/// Maximum atomic write size (4KB - POSIX.1 minimum)
pub const PIPE_ATOMIC_SIZE: usize = 4096;

/// Pipe error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeError {
    /// Pipe is full
    WouldBlock,
    /// Pipe is broken (no writers or readers)
    BrokenPipe,
    /// Invalid operation
    InvalidOperation,
    /// Buffer too large for atomic write
    TooBig,
}

/// Pipe end type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeEnd {
    Read,
    Write,
}

/// Pipe buffer implementation (inspired by Linux pipe_buffer)
pub struct PipeBuffer {
    /// Ring buffer for data
    data: VecDeque<u8>,
    /// Maximum capacity
    capacity: usize,
    /// Number of readers
    reader_count: AtomicUsize,
    /// Number of writers
    writer_count: AtomicUsize,
    /// Non-blocking mode
    nonblocking: AtomicBool,
}

impl PipeBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
            capacity,
            reader_count: AtomicUsize::new(0),
            writer_count: AtomicUsize::new(0),
            nonblocking: AtomicBool::new(false),
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(PIPE_BUF_SIZE)
    }

    /// Register a pipe end (reader or writer)
    pub fn register(&self, end: PipeEnd) {
        match end {
            PipeEnd::Read => {
                self.reader_count.fetch_add(1, Ordering::SeqCst);
            }
            PipeEnd::Write => {
                self.writer_count.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    /// Unregister a pipe end
    pub fn unregister(&self, end: PipeEnd) {
        match end {
            PipeEnd::Read => {
                self.reader_count.fetch_sub(1, Ordering::SeqCst);
            }
            PipeEnd::Write => {
                self.writer_count.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }

    pub fn has_readers(&self) -> bool {
        self.reader_count.load(Ordering::SeqCst) > 0
    }

    pub fn has_writers(&self) -> bool {
        self.writer_count.load(Ordering::SeqCst) > 0
    }

    pub fn set_nonblocking(&self, nonblocking: bool) {
        self.nonblocking.store(nonblocking, Ordering::SeqCst);
    }

    pub fn is_nonblocking(&self) -> bool {
        self.nonblocking.load(Ordering::SeqCst)
    }

    /// Write data to pipe
    pub fn write(&mut self, data: &[u8]) -> Result<usize, PipeError> {
        if !self.has_readers() {
            return Err(PipeError::BrokenPipe);
        }

        // Check for atomic write guarantee
        if data.len() <= PIPE_ATOMIC_SIZE {
            // Atomic write - must write all or none
            if self.available_space() < data.len() {
                if self.is_nonblocking() {
                    return Err(PipeError::WouldBlock);
                }
                // In blocking mode, would wait here
                return Err(PipeError::WouldBlock);
            }
            self.data.extend(data.iter().copied());
            Ok(data.len())
        } else {
            // Non-atomic write - write as much as possible
            let available = self.available_space();
            if available == 0 {
                if self.is_nonblocking() {
                    return Err(PipeError::WouldBlock);
                }
                return Err(PipeError::WouldBlock);
            }
            let to_write = available.min(data.len());
            self.data.extend(data[..to_write].iter().copied());
            Ok(to_write)
        }
    }

    /// Read data from pipe
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, PipeError> {
        if self.data.is_empty() {
            if !self.has_writers() {
                // EOF - no writers left
                return Ok(0);
            }
            if self.is_nonblocking() {
                return Err(PipeError::WouldBlock);
            }
            // In blocking mode, would wait here
            return Err(PipeError::WouldBlock);
        }

        let to_read = buffer.len().min(self.data.len());
        for i in 0..to_read {
            buffer[i] = self.data.pop_front().unwrap();
        }
        Ok(to_read)
    }

    /// Available space in pipe
    pub fn available_space(&self) -> usize {
        self.capacity - self.data.len()
    }

    /// Available data to read
    pub fn available_data(&self) -> usize {
        self.data.len()
    }

    /// Check if pipe is full
    pub fn is_full(&self) -> bool {
        self.data.len() >= self.capacity
    }

    /// Check if pipe is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clear all data from pipe
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

/// Pipe file descriptor
pub struct PipeFd {
    /// Shared pipe buffer
    buffer: *mut PipeBuffer,
    /// Which end this FD represents
    end: PipeEnd,
}

impl PipeFd {
    pub fn new(buffer: *mut PipeBuffer, end: PipeEnd) -> Self {
        unsafe {
            (*buffer).register(end);
        }
        Self { buffer, end }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize, PipeError> {
        if self.end != PipeEnd::Write {
            return Err(PipeError::InvalidOperation);
        }
        unsafe { (*self.buffer).write(data) }
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, PipeError> {
        if self.end != PipeEnd::Read {
            return Err(PipeError::InvalidOperation);
        }
        unsafe { (*self.buffer).read(buffer) }
    }

    pub fn set_nonblocking(&self, nonblocking: bool) {
        unsafe {
            (*self.buffer).set_nonblocking(nonblocking);
        }
    }

    pub fn available(&self) -> usize {
        unsafe {
            match self.end {
                PipeEnd::Read => (*self.buffer).available_data(),
                PipeEnd::Write => (*self.buffer).available_space(),
            }
        }
    }
}

impl Drop for PipeFd {
    fn drop(&mut self) {
        unsafe {
            (*self.buffer).unregister(self.end);
        }
    }
}

/// Pipe creation result
pub struct Pipe {
    pub read_fd: PipeFd,
    pub write_fd: PipeFd,
}

/// Create a new anonymous pipe
pub fn create_pipe() -> Pipe {
    let buffer = Box::into_raw(Box::new(PipeBuffer::with_default_capacity()));
    Pipe {
        read_fd: PipeFd::new(buffer, PipeEnd::Read),
        write_fd: PipeFd::new(buffer, PipeEnd::Write),
    }
}

/// Splice operation (inspired by Linux splice syscall)
pub struct SpliceOperation {
    /// Source pipe
    src: *mut PipeBuffer,
    /// Destination pipe
    dst: *mut PipeBuffer,
    /// Maximum bytes to transfer
    max_bytes: usize,
}

impl SpliceOperation {
    pub fn new(src: *mut PipeBuffer, dst: *mut PipeBuffer, max_bytes: usize) -> Self {
        Self {
            src,
            dst,
            max_bytes,
        }
    }

    /// Execute splice (zero-copy transfer between pipes)
    pub fn execute(&mut self) -> Result<usize, PipeError> {
        unsafe {
            let available = (*self.src).available_data().min(self.max_bytes);
            let space = (*self.dst).available_space();
            let to_transfer = available.min(space);

            if to_transfer == 0 {
                return Err(PipeError::WouldBlock);
            }

            // In a real implementation, this would be zero-copy
            // For now, we copy through a temporary buffer
            let mut temp = Vec::with_capacity(to_transfer);
            temp.resize(to_transfer, 0);

            let read = (*self.src).read(&mut temp)?;
            let written = (*self.dst).write(&temp[..read])?;

            Ok(written)
        }
    }
}

/// Named pipe (FIFO) implementation
pub struct NamedPipe {
    /// Path in filesystem
    pub path: *const u8,
    /// Shared buffer
    buffer: PipeBuffer,
}

impl NamedPipe {
    pub fn new(path: *const u8) -> Self {
        Self {
            path,
            buffer: PipeBuffer::with_default_capacity(),
        }
    }

    pub fn open_read(&mut self) -> PipeFd {
        let buffer_ptr = &mut self.buffer as *mut PipeBuffer;
        PipeFd::new(buffer_ptr, PipeEnd::Read)
    }

    pub fn open_write(&mut self) -> PipeFd {
        let buffer_ptr = &mut self.buffer as *mut PipeBuffer;
        PipeFd::new(buffer_ptr, PipeEnd::Write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_creation() {
        let pipe = create_pipe();
        assert_eq!(pipe.read_fd.available(), 0);
        assert!(pipe.write_fd.available() > 0);
    }

    #[test]
    fn test_pipe_write_read() {
        let mut pipe = create_pipe();
        let data = b"Hello, pipe!";
        let written = pipe.write_fd.write(data).unwrap();
        assert_eq!(written, data.len());

        let mut buffer = [0u8; 64];
        let read = pipe.read_fd.read(&mut buffer).unwrap();
        assert_eq!(read, data.len());
        assert_eq!(&buffer[..read], data);
    }

    #[test]
    fn test_broken_pipe() {
        let mut buffer = PipeBuffer::with_default_capacity();
        buffer.register(PipeEnd::Write);
        // No readers - should get BrokenPipe
        let result = buffer.write(b"test");
        assert_eq!(result, Err(PipeError::BrokenPipe));
    }

    #[test]
    fn test_atomic_write() {
        let mut buffer = PipeBuffer::new(PIPE_ATOMIC_SIZE);
        buffer.register(PipeEnd::Read);
        buffer.register(PipeEnd::Write);

        let data = vec![0u8; PIPE_ATOMIC_SIZE + 1];
        // This should fail because buffer is too small for atomic write
        buffer.set_nonblocking(true);
        let result = buffer.write(&data);
        assert_eq!(result, Err(PipeError::WouldBlock));
    }

    #[test]
    fn test_pipe_capacity() {
        let buffer = PipeBuffer::new(1024);
        assert_eq!(buffer.capacity, 1024);
        assert_eq!(buffer.available_space(), 1024);
        assert_eq!(buffer.available_data(), 0);
    }
}
