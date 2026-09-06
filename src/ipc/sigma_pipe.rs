//! # SigmaOS Sovereign IPC Pipe
//!
//! Unix-style inter-process communication pipes with ring-buffer semantics.
//! Provides both anonymous pipes (`SigmaPipe`) and named pipes (`SigmaFifo`).
//!
//! ## Overview
//!
//! ```text
//! Writer ──[write()]──► PipeBuffer ──[read()]──► Reader
//!                       (ring buf)
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! let (mut writer, mut reader) = SigmaPipe::new(4096);
//! writer.write(b"hello, sigma!").unwrap();
//! let mut buf = [0u8; 64];
//! let n = reader.read(&mut buf).unwrap();
//! assert_eq!(&buf[..n], b"hello, sigma!");
//! ```

#![allow(dead_code)]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors that can occur during pipe operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipeError {
    /// The pipe buffer is full; the write would block.
    WouldBlock,
    /// The pipe buffer is empty; the read would block.
    Empty,
    /// The write end has been closed; further reads return 0.
    BrokenPipe,
    /// The read end has been closed; further writes fail.
    ReadEndClosed,
    /// The supplied buffer slice has zero length.
    ZeroLengthBuffer,
    /// An invalid argument was supplied.
    InvalidArgument(String),
}

impl core::fmt::Display for PipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PipeError::WouldBlock     => write!(f, "pipe: would block"),
            PipeError::Empty          => write!(f, "pipe: empty"),
            PipeError::BrokenPipe     => write!(f, "pipe: broken pipe (write end closed)"),
            PipeError::ReadEndClosed  => write!(f, "pipe: read end closed"),
            PipeError::ZeroLengthBuffer => write!(f, "pipe: zero-length buffer"),
            PipeError::InvalidArgument(s) => write!(f, "pipe: invalid argument: {}", s),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Ring buffer
// ─────────────────────────────────────────────────────────────────────────────

/// A fixed-capacity ring (circular) buffer used as the backing store for a pipe.
///
/// Bytes are written at `write_pos` and consumed from `read_pos`. Both
/// positions wrap around modulo `capacity`.
pub struct PipeBuffer {
    /// Raw storage for buffered bytes.
    data: Vec<u8>,
    /// Maximum number of bytes the buffer can hold.
    capacity: usize,
    /// Index of the next byte to be read.
    read_pos: usize,
    /// Index where the next byte will be written.
    write_pos: usize,
    /// Number of bytes currently in the buffer.
    len: usize,
}

impl PipeBuffer {
    /// Create a new `PipeBuffer` with the given capacity.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "PipeBuffer capacity must be > 0");
        let mut data = Vec::with_capacity(capacity);
        data.resize(capacity, 0u8);
        PipeBuffer {
            data,
            capacity,
            read_pos: 0,
            write_pos: 0,
            len: 0,
        }
    }

    /// Returns the total capacity of the buffer in bytes.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the number of bytes currently stored.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the buffer contains no bytes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of bytes that can be written before the buffer is full.
    #[inline]
    pub fn available(&self) -> usize {
        self.capacity - self.len
    }

    /// Write bytes from `data` into the buffer.
    ///
    /// Returns the number of bytes actually written, which may be less than
    /// `data.len()` if the buffer is nearly full. Returns `PipeError::WouldBlock`
    /// if there is no space at all.
    pub fn write(&mut self, data: &[u8]) -> Result<usize, PipeError> {
        if data.is_empty() {
            return Ok(0);
        }
        if self.available() == 0 {
            return Err(PipeError::WouldBlock);
        }

        let to_write = data.len().min(self.available());
        for i in 0..to_write {
            self.data[self.write_pos] = data[i];
            self.write_pos = (self.write_pos + 1) % self.capacity;
        }
        self.len += to_write;
        Ok(to_write)
    }

    /// Read bytes from the buffer into `buf`.
    ///
    /// Returns the number of bytes placed into `buf`. Returns
    /// `PipeError::Empty` if there is nothing to read.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PipeError> {
        if buf.is_empty() {
            return Err(PipeError::ZeroLengthBuffer);
        }
        if self.is_empty() {
            return Err(PipeError::Empty);
        }

        let to_read = buf.len().min(self.len);
        for item in buf.iter_mut().take(to_read) {
            *item = self.data[self.read_pos];
            self.read_pos = (self.read_pos + 1) % self.capacity;
        }
        self.len -= to_read;
        Ok(to_read)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Pipe ends
// ─────────────────────────────────────────────────────────────────────────────

/// Shared inner state for a pipe.
///
/// Both the read and write halves hold a reference to this via index in the
/// kernel's pipe table in a real OS. Here, for a sovereign library model, we
/// use a simple heap-allocated `PipeInner` wrapped in an `alloc::rc::Rc<core::cell::RefCell<…>>`.
struct PipeInner {
    buffer: PipeBuffer,
    write_closed: bool,
    read_closed: bool,
}

impl PipeInner {
    fn new(capacity: usize) -> Self {
        PipeInner {
            buffer: PipeBuffer::new(capacity),
            write_closed: false,
            read_closed: false,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Writer half
// ─────────────────────────────────────────────────────────────────────────────

/// The write end of a [`SigmaPipe`].
///
/// Dropping this type is equivalent to calling [`close_write`](PipeWriter::close_write).
pub struct PipeWriter {
    inner: alloc::rc::Rc<core::cell::RefCell<PipeInner>>,
}

impl PipeWriter {
    /// Write `data` into the pipe.
    ///
    /// Returns the number of bytes written, which may be less than
    /// `data.len()` if the ring buffer is nearly full.
    ///
    /// # Errors
    ///
    /// - [`PipeError::ReadEndClosed`] if the read end has already been closed.
    /// - [`PipeError::WouldBlock`] if the buffer is completely full.
    pub fn write(&mut self, data: &[u8]) -> Result<usize, PipeError> {
        let mut inner = self.inner.borrow_mut();
        if inner.read_closed {
            return Err(PipeError::ReadEndClosed);
        }
        inner.buffer.write(data)
    }

    /// Signal EOF on the write side.  Subsequent reads will drain remaining
    /// buffered data and then return 0 bytes.
    pub fn close_write(&mut self) {
        self.inner.borrow_mut().write_closed = true;
    }

    /// Returns `true` if the pipe's read end is still open.
    pub fn is_read_open(&self) -> bool {
        !self.inner.borrow().read_closed
    }

    /// Returns the number of bytes currently buffered.
    pub fn buffered(&self) -> usize {
        self.inner.borrow().buffer.len()
    }
}

impl Drop for PipeWriter {
    fn drop(&mut self) {
        self.close_write();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Reader half
// ─────────────────────────────────────────────────────────────────────────────

/// The read end of a [`SigmaPipe`].
///
/// Dropping this type is equivalent to calling [`close_read`](PipeReader::close_read).
pub struct PipeReader {
    inner: alloc::rc::Rc<core::cell::RefCell<PipeInner>>,
}

impl PipeReader {
    /// Read bytes from the pipe into `buf`.
    ///
    /// Returns the number of bytes read (may be less than `buf.len()`).
    /// Returns `Ok(0)` when the write end has been closed and the buffer is
    /// drained (EOF).
    ///
    /// # Errors
    ///
    /// - [`PipeError::Empty`] if the buffer is empty but the write end is
    ///   still open (would block in blocking mode).
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, PipeError> {
        let mut inner = self.inner.borrow_mut();
        match inner.buffer.read(buf) {
            Ok(n) => Ok(n),
            Err(PipeError::Empty) => {
                if inner.write_closed {
                    Ok(0) // EOF
                } else {
                    Err(PipeError::Empty)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Signal that the reader is no longer consuming data.
    pub fn close_read(&mut self) {
        self.inner.borrow_mut().read_closed = true;
    }

    /// Returns `true` if the write end is still open.
    pub fn is_write_open(&self) -> bool {
        !self.inner.borrow().write_closed
    }

    /// Returns the number of bytes currently buffered.
    pub fn buffered(&self) -> usize {
        self.inner.borrow().buffer.len()
    }
}

impl Drop for PipeReader {
    fn drop(&mut self) {
        self.close_read();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SigmaPipe — anonymous pipe
// ─────────────────────────────────────────────────────────────────────────────

/// An anonymous, unidirectional IPC pipe.
///
/// Internally backed by a [`PipeBuffer`] ring buffer.  Call [`SigmaPipe::new`]
/// to obtain a `(PipeWriter, PipeReader)` pair.
///
/// # Design notes
///
/// * No OS threads are involved; blocking is modelled as `WouldBlock` /
///   `Empty` errors that callers must retry.
/// * The ring buffer is heap-allocated via `alloc`.
pub struct SigmaPipe;

impl SigmaPipe {
    /// Create a new pipe with the given buffer `capacity` in bytes.
    ///
    /// Returns `(writer, reader)`.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> (PipeWriter, PipeReader) {
        let inner = alloc::rc::Rc::new(core::cell::RefCell::new(PipeInner::new(capacity)));
        (
            PipeWriter { inner: alloc::rc::Rc::clone(&inner) },
            PipeReader { inner },
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SigmaFifo — named pipe
// ─────────────────────────────────────────────────────────────────────────────

/// A named pipe (FIFO) with an associated filesystem path.
///
/// Named pipes allow unrelated processes to communicate via a well-known
/// path, analogous to POSIX `mkfifo(3)`.
///
/// # Example
///
/// ```rust,ignore
/// let mut fifo = SigmaFifo::create("/tmp/my.fifo".to_string(), 8192);
/// fifo.writer().write(b"data").unwrap();
/// let mut buf = [0u8; 32];
/// fifo.reader().read(&mut buf).unwrap();
/// ```
pub struct SigmaFifo {
    /// The filesystem path this FIFO is associated with.
    pub path: String,
    writer: PipeWriter,
    reader: PipeReader,
}

impl SigmaFifo {
    /// Create a new named pipe bound to `path` with the given `capacity`.
    pub fn create(path: String, capacity: usize) -> Self {
        let (writer, reader) = SigmaPipe::new(capacity);
        SigmaFifo { path, writer, reader }
    }

    /// Returns a mutable reference to the write end.
    pub fn writer(&mut self) -> &mut PipeWriter {
        &mut self.writer
    }

    /// Returns a mutable reference to the read end.
    pub fn reader(&mut self) -> &mut PipeReader {
        &mut self.reader
    }

    /// The filesystem path of this FIFO.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Unlink (remove) this named pipe from the virtual filesystem.
    ///
    /// In a real kernel this would call `vfs_unlink`; here it is a no-op
    /// placeholder that closes both ends.
    pub fn unlink(mut self) {
        self.writer.close_write();
        self.reader.close_read();
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_pipe_write_read() {
        let (mut w, mut r) = SigmaPipe::new(256);
        let written = w.write(b"hello").unwrap();
        assert_eq!(written, 5);
        let mut buf = [0u8; 16];
        let read = r.read(&mut buf).unwrap();
        assert_eq!(read, 5);
        assert_eq!(&buf[..5], b"hello");
    }

    #[test]
    fn test_pipe_eof_on_writer_close() {
        let (mut w, mut r) = SigmaPipe::new(64);
        w.write(b"eof").unwrap();
        w.close_write();
        let mut buf = [0u8; 8];
        assert_eq!(r.read(&mut buf).unwrap(), 3);
        // After drain + write closed → EOF = Ok(0)
        assert_eq!(r.read(&mut buf).unwrap(), 0);
    }

    #[test]
    fn test_pipe_full_returns_would_block() {
        let (mut w, _r) = SigmaPipe::new(4);
        w.write(b"1234").unwrap(); // fill
        let result = w.write(b"X");
        assert_eq!(result, Err(PipeError::WouldBlock));
    }

    #[test]
    fn test_named_fifo() {
        let mut fifo = SigmaFifo::create("/tmp/sigma_test.fifo".to_string(), 128);
        fifo.writer().write(b"fifo data").unwrap();
        let mut buf = [0u8; 32];
        let n = fifo.reader().read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"fifo data");
    }
}
