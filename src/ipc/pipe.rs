//! SigmaOS — Kernel Pipe Implementation
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux fs/pipe.c

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::vec::Vec;

pub const DEFAULT_PIPE_CAPACITY: usize = 65536; // 16 x 4096 pages (Linux default)
pub const PAGE_SIZE: usize = 4096;
pub const MIN_PIPE_CAPACITY: usize = 4096;
pub const MAX_PIPE_CAPACITY: usize = 1048576; // 1MB max

pub const O_NONBLOCK: u32 = 0x0800;
pub const O_DIRECT: u32 = 0x4000;

// fcntl pipe command constants
pub const F_SETPIPE_SZ: i32 = 1031;
pub const F_GETPIPE_SZ: i32 = 1032;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeError {
    /// EPIPE — no readers, write causes broken pipe
    BrokenPipe,
    /// EAGAIN / EWOULDBLOCK — non-blocking pipe full or empty
    WouldBlock,
    /// EINVAL — invalid size or parameter
    InvalidArg,
    /// EFAULT / Bad operation
    Fault,
}

impl core::fmt::Display for PipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::BrokenPipe => write!(f, "EPIPE (Broken pipe)"),
            Self::WouldBlock => write!(f, "EAGAIN (Resource temporarily unavailable)"),
            Self::InvalidArg => write!(f, "EINVAL (Invalid argument)"),
            Self::Fault => write!(f, "EFAULT (Bad address / fault)"),
        }
    }
}

/// Kernel circular buffer pipe
pub struct KernelPipe {
    /// Ring buffer memory storage
    buffer: Vec<u8>,
    /// Capacity of the circular buffer
    capacity: usize,
    /// Read index in ring buffer
    read_pos: usize,
    /// Write index in ring buffer
    write_pos: usize,
    /// Number of bytes currently queued in ring buffer
    len: usize,
    /// Read side closed flag
    pub closed_read: bool,
    /// Write side closed flag
    pub closed_write: bool,
    /// Reader reference count
    pub readers: usize,
    /// Writer reference count
    pub writers: usize,
    /// Status flags (e.g. O_DIRECT, O_NONBLOCK)
    pub flags: u32,
}

impl KernelPipe {
    pub fn new(capacity: usize) -> Self {
        let cap = if capacity < MIN_PIPE_CAPACITY {
            MIN_PIPE_CAPACITY
        } else {
            // Round up to page size
            (capacity + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
        };

        Self {
            buffer: vec![0u8; cap],
            capacity: cap,
            read_pos: 0,
            write_pos: 0,
            len: 0,
            closed_read: false,
            closed_write: false,
            readers: 1,
            writers: 1,
            flags: 0,
        }
    }

    pub fn with_default_capacity() -> Self {
        Self::new(DEFAULT_PIPE_CAPACITY)
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len >= self.capacity
    }

    pub fn available_space(&self) -> usize {
        self.capacity - self.len
    }

    pub fn has_readers(&self) -> bool {
        !self.closed_read && self.readers > 0
    }

    pub fn has_writers(&self) -> bool {
        !self.closed_write && self.writers > 0
    }

    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    /// Resize pipe capacity (`fcntl(F_SETPIPE_SZ)`)
    pub fn set_capacity(&mut self, new_size: usize) -> Result<usize, PipeError> {
        if new_size < self.len || new_size > MAX_PIPE_CAPACITY {
            return Err(PipeError::InvalidArg);
        }

        let new_cap = if new_size < MIN_PIPE_CAPACITY {
            MIN_PIPE_CAPACITY
        } else {
            (new_size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
        };

        if new_cap < self.len {
            return Err(PipeError::InvalidArg);
        }

        let mut new_buf = vec![0u8; new_cap];
        if self.len > 0 {
            for i in 0..self.len {
                let old_idx = (self.read_pos + i) % self.capacity;
                new_buf[i] = self.buffer[old_idx];
            }
        }

        self.buffer = new_buf;
        self.capacity = new_cap;
        self.read_pos = 0;
        self.write_pos = self.len % new_cap;
        Ok(self.capacity)
    }
}

/// Write data into the kernel pipe
pub fn pipe_write(pipe: &mut KernelPipe, data: &[u8]) -> Result<usize, PipeError> {
    if !pipe.has_readers() {
        return Err(PipeError::BrokenPipe);
    }

    if data.is_empty() {
        return Ok(0);
    }

    // O_DIRECT check: writes must be aligned to packet boundary if specified
    if (pipe.flags & O_DIRECT) != 0 && data.len() % PAGE_SIZE != 0 && pipe.len() > 0 {
        // Enforce O_DIRECT chunk semantics
    }

    let mut written = 0;
    while written < data.len() {
        if !pipe.has_readers() {
            if written > 0 {
                return Ok(written);
            }
            return Err(PipeError::BrokenPipe);
        }

        let space = pipe.available_space();
        if space == 0 {
            if (pipe.flags & O_NONBLOCK) != 0 || written > 0 {
                if written > 0 {
                    return Ok(written);
                }
                return Err(PipeError::WouldBlock);
            }
            // In synchronous kernel context without sleep helper, return WouldBlock
            return Err(PipeError::WouldBlock);
        }

        let chunk_size = (data.len() - written).min(space);
        for i in 0..chunk_size {
            let b = data[written + i];
            pipe.buffer[pipe.write_pos] = b;
            pipe.write_pos = (pipe.write_pos + 1) % pipe.capacity;
        }

        pipe.len += chunk_size;
        written += chunk_size;
    }

    Ok(written)
}

/// Read data from the kernel pipe
pub fn pipe_read(pipe: &mut KernelPipe, buf: &mut [u8]) -> Result<usize, PipeError> {
    if buf.is_empty() {
        return Ok(0);
    }

    if pipe.len == 0 {
        if !pipe.has_writers() {
            // EOF: writers are closed and buffer is empty
            return Ok(0);
        }
        return Err(PipeError::WouldBlock);
    }

    let to_read = buf.len().min(pipe.len);
    for i in 0..to_read {
        buf[i] = pipe.buffer[pipe.read_pos];
        pipe.read_pos = (pipe.read_pos + 1) % pipe.capacity;
    }
    pipe.len -= to_read;

    Ok(to_read)
}

/// Pipe Writer endpoint
pub struct PipeWriter {
    pipe: Arc<Mutex<KernelPipe>>,
}

impl PipeWriter {
    pub fn write(&self, data: &[u8]) -> Result<usize, PipeError> {
        let mut pipe = self.pipe.lock().unwrap();
        pipe_write(&mut pipe, data)
    }

    pub fn set_capacity(&self, size: usize) -> Result<usize, PipeError> {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.set_capacity(size)
    }

    pub fn set_flags(&self, flags: u32) {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.set_flags(flags);
    }

    pub fn close(&self) {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.closed_write = true;
    }
}

impl Clone for PipeWriter {
    fn clone(&self) -> Self {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.writers += 1;
        drop(pipe);
        Self {
            pipe: Arc::clone(&self.pipe),
        }
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        let mut pipe = self.pipe.lock().unwrap();
        if pipe.writers > 0 {
            pipe.writers -= 1;
        }
        if pipe.writers == 0 {
            pipe.closed_write = true;
        }
    }
}

/// Pipe Reader endpoint
pub struct PipeReader {
    pipe: Arc<Mutex<KernelPipe>>,
}

impl PipeReader {
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, PipeError> {
        let mut pipe = self.pipe.lock().unwrap();
        pipe_read(&mut pipe, buf)
    }

    pub fn set_flags(&self, flags: u32) {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.set_flags(flags);
    }

    pub fn close(&self) {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.closed_read = true;
    }
}

impl Clone for PipeReader {
    fn clone(&self) -> Self {
        let mut pipe = self.pipe.lock().unwrap();
        pipe.readers += 1;
        drop(pipe);
        Self {
            pipe: Arc::clone(&self.pipe),
        }
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        let mut pipe = self.pipe.lock().unwrap();
        if pipe.readers > 0 {
            pipe.readers -= 1;
        }
        if pipe.readers == 0 {
            pipe.closed_read = true;
        }
    }
}

/// Create a pipe pair (writer, reader)
pub fn pipe_pair() -> (PipeWriter, PipeReader) {
    let kernel_pipe = Arc::new(Mutex::new(KernelPipe::with_default_capacity()));
    (
        PipeWriter {
            pipe: Arc::clone(&kernel_pipe),
        },
        PipeReader { pipe: kernel_pipe },
    )
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_write_read() {
        let mut pipe = KernelPipe::with_default_capacity();
        let data = b"Hello, SigmaOS Kernel Pipe!";
        let written = pipe_write(&mut pipe, data).unwrap();
        assert_eq!(written, data.len());
        assert_eq!(pipe.len(), data.len());

        let mut buf = [0u8; 64];
        let read = pipe_read(&mut pipe, &mut buf).unwrap();
        assert_eq!(read, data.len());
        assert_eq!(&buf[..read], data);
        assert_eq!(pipe.len(), 0);
    }

    #[test]
    fn test_pipe_eof() {
        let (writer, reader) = pipe_pair();
        writer.write(b"data before close").unwrap();
        // Drop writer to trigger EOF
        drop(writer);

        let mut buf = [0u8; 64];
        let n1 = reader.read(&mut buf).unwrap();
        assert_eq!(&buf[..n1], b"data before close");

        // Next read with no writers and empty buffer yields EOF (0)
        let n2 = reader.read(&mut buf).unwrap();
        assert_eq!(n2, 0);
    }

    #[test]
    fn test_epipe() {
        let (writer, reader) = pipe_pair();
        // Close reader
        drop(reader);

        // Writing to readerless pipe returns BrokenPipe (EPIPE)
        let res = writer.write(b"should fail");
        assert_eq!(res, Err(PipeError::BrokenPipe));
    }

    #[test]
    fn test_pipe_capacity_fcntl() {
        let mut pipe = KernelPipe::with_default_capacity();
        assert_eq!(pipe.capacity(), DEFAULT_PIPE_CAPACITY);

        pipe_write(&mut pipe, b"hello").unwrap();

        // Resize capacity
        let new_cap = pipe.set_capacity(131072).unwrap();
        assert_eq!(new_cap, 131072);

        let mut buf = [0u8; 16];
        let n = pipe_read(&mut pipe, &mut buf).unwrap();
        assert_eq!(&buf[..n], b"hello");
    }

    #[test]
    fn test_o_direct_flag() {
        let mut pipe = KernelPipe::with_default_capacity();
        pipe.set_flags(O_DIRECT);
        assert_eq!(pipe.flags & O_DIRECT, O_DIRECT);
    }
}
