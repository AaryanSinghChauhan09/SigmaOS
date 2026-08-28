// SPDX-License-Identifier: MIT
// Sovereign Standard Streams Controller
// Linux & BSD inspired standard input, output, and error stream management.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Standard File Descriptors
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;
pub const STDERR_FILENO: i32 = 2;

/// Stream Buffering Mode (FreeBSD setvbuf inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamBufferMode {
    Unbuffered,
    LineBuffered,
    BlockBuffered(usize),
}

/// File Descriptor Entry for standard streams
#[derive(Debug, Clone)]
pub struct StandardStreamHandle {
    pub fd: i32,
    pub name: String,
    pub buffer_mode: StreamBufferMode,
    pub internal_buffer: Vec<u8>,
    pub redirected_fd: Option<i32>,
    pub is_closed: bool,
    pub is_tty: bool,                // Linux isatty(3) / BSD tty capability
    pub broken_stream_sigpipe: bool, // Linux EPIPE / SIGPIPE signal trigger
}

impl StandardStreamHandle {
    pub fn new(fd: i32, name: &str, buffer_mode: StreamBufferMode) -> Self {
        let is_tty = fd >= 0 && fd <= 2; // Default 0, 1, 2 attached to interactive TTY
        Self {
            fd,
            name: name.to_string(),
            buffer_mode,
            internal_buffer: Vec::new(),
            redirected_fd: None,
            is_closed: false,
            is_tty,
            broken_stream_sigpipe: false,
        }
    }

    /// Auto-detects optimal stream buffering mode (isatty ? LineBuffered : BlockBuffered)
    pub fn auto_detect_buffering(&mut self) {
        if self.is_tty {
            self.buffer_mode = StreamBufferMode::LineBuffered;
        } else {
            self.buffer_mode = StreamBufferMode::BlockBuffered(4096);
        }
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Vec<u8> {
        if self.is_closed {
            self.broken_stream_sigpipe = true; // Trigger SIGPIPE signal state on write to closed stream
            return Vec::new();
        }
        let mut flushed = Vec::new();

        match self.buffer_mode {
            StreamBufferMode::Unbuffered => {
                flushed.extend_from_slice(data);
            }
            StreamBufferMode::LineBuffered => {
                for &b in data {
                    self.internal_buffer.push(b);
                    if b == b'\n' {
                        flushed.extend(self.internal_buffer.drain(..));
                    }
                }
            }
            StreamBufferMode::BlockBuffered(limit) => {
                for &b in data {
                    self.internal_buffer.push(b);
                    if self.internal_buffer.len() >= limit {
                        flushed.extend(self.internal_buffer.drain(..));
                    }
                }
            }
        }
        flushed
    }

    pub fn flush(&mut self) -> Vec<u8> {
        self.internal_buffer.drain(..).collect()
    }
}

/// Pipe Splice & Tee Router for Standard Stream Multiplexing
#[derive(Debug, Clone, Default)]
pub struct StreamTeeSpliceRouter;

impl StreamTeeSpliceRouter {
    pub fn new() -> Self {
        Self
    }

    /// Linux-style zero-copy splice between standard stream buffers
    pub fn splice_streams(&self, src: &mut StandardStreamHandle, dest: &mut StandardStreamHandle, max_bytes: usize) -> usize {
        let available = src.internal_buffer.len().min(max_bytes);
        if available == 0 {
            return 0;
        }
        let chunk: Vec<u8> = src.internal_buffer.drain(..available).collect();
        let written = dest.write_bytes(&chunk);
        written.len()
    }

    /// Linux-style tee duplication from src stream to target stream without consuming src buffer
    pub fn tee_stream(&self, src: &StandardStreamHandle, target: &mut StandardStreamHandle, max_bytes: usize) -> usize {
        let available = src.internal_buffer.len().min(max_bytes);
        if available == 0 {
            return 0;
        }
        let chunk = &src.internal_buffer[..available];
        let written = target.write_bytes(chunk);
        written.len()
    }
}

/// Sovereign Standard Stream Controller
#[derive(Debug, Clone)]
pub struct StandardStreamController {
    pub handles: BTreeMap<i32, StandardStreamHandle>,
    pub active_pledges: Vec<String>,
}

impl StandardStreamController {
    pub fn new() -> Self {
        let mut handles = BTreeMap::new();
        handles.insert(
            STDIN_FILENO,
            StandardStreamHandle::new(STDIN_FILENO, "stdin", StreamBufferMode::LineBuffered),
        );
        handles.insert(
            STDOUT_FILENO,
            StandardStreamHandle::new(STDOUT_FILENO, "stdout", StreamBufferMode::LineBuffered),
        );
        handles.insert(
            STDERR_FILENO,
            StandardStreamHandle::new(STDERR_FILENO, "stderr", StreamBufferMode::Unbuffered),
        );

        Self {
            handles,
            active_pledges: vec!["stdio".to_string()],
        }
    }

    /// OpenBSD-style stream pledge check (validates `stdio` promise)
    pub fn validate_pledge_stdio(&self) -> Result<(), &'static str> {
        if self.active_pledges.iter().any(|p| p == "stdio") {
            Ok(())
        } else {
            Err("SIGABRT: pledge violation - operation requires stdio promise")
        }
    }

    /// POSIX dup2 file descriptor redirection
    pub fn dup2(&mut self, oldfd: i32, newfd: i32) -> Result<i32, &'static str> {
        self.validate_pledge_stdio()?;

        if !self.handles.contains_key(&oldfd) {
            return Err("EBADF: Bad file descriptor oldfd");
        }

        let mut duplicated = self.handles.get(&oldfd).unwrap().clone();
        duplicated.fd = newfd;
        duplicated.redirected_fd = Some(oldfd);

        self.handles.insert(newfd, duplicated);
        Ok(newfd)
    }

    pub fn write_to_fd(&mut self, fd: i32, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        self.validate_pledge_stdio()?;

        let handle = self.handles.get_mut(&fd).ok_or("EBADF: Invalid file descriptor")?;
        Ok(handle.write_bytes(data))
    }

    pub fn set_buffering(&mut self, fd: i32, mode: StreamBufferMode) -> Result<(), &'static str> {
        let handle = self.handles.get_mut(&fd).ok_or("EBADF: Invalid file descriptor")?;
        handle.buffer_mode = mode;
        Ok(())
    }

    /// Linux stdbuf / setvbuf override parity
    pub fn apply_stdbuf_override(&mut self, stdout_mode: StreamBufferMode, stderr_mode: StreamBufferMode) {
        if let Some(stdout) = self.handles.get_mut(&STDOUT_FILENO) {
            stdout.buffer_mode = stdout_mode;
        }
        if let Some(stderr) = self.handles.get_mut(&STDERR_FILENO) {
            stderr.buffer_mode = stderr_mode;
        }
    }

    /// Synchronizes and flushes all open standard stream buffers (fflush(NULL) parity)
    pub fn flush_all(&mut self) -> BTreeMap<i32, Vec<u8>> {
        let mut flushed_streams = BTreeMap::new();
        for (fd, handle) in self.handles.iter_mut() {
            let data = handle.flush();
            if !data.is_empty() {
                flushed_streams.insert(*fd, data);
            }
        }
        flushed_streams
    }

    /// Linux isatty(3) / BSD tty query
    pub fn isatty(&self, fd: i32) -> bool {
        self.handles.get(&fd).map(|h| h.is_tty && !h.is_closed).unwrap_or(false)
    }
}

impl Default for StandardStreamController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_stream_descriptors_and_buffering() {
        let mut controller = StandardStreamController::new();

        // Stderr is unbuffered
        let stderr_out = controller.write_to_fd(STDERR_FILENO, b"error message\n").unwrap();
        assert_eq!(stderr_out, b"error message\n");

        // Stdout is line buffered
        let stdout_partial = controller.write_to_fd(STDOUT_FILENO, b"hello ").unwrap();
        assert_eq!(stdout_partial.len(), 0); // Not flushed yet

        let stdout_newline = controller.write_to_fd(STDOUT_FILENO, b"world\n").unwrap();
        assert_eq!(stdout_newline, b"hello world\n");
    }

    #[test]
    fn test_posix_dup2_stream_redirection() {
        let mut controller = StandardStreamController::new();

        // Redirect stdout (1) to fd 10
        let new_fd = controller.dup2(STDOUT_FILENO, 10).unwrap();
        assert_eq!(new_fd, 10);

        let out = controller.write_to_fd(10, b"redirected output\n").unwrap();
        assert_eq!(out, b"redirected output\n");
    }

    #[test]
    fn test_openbsd_stream_pledge_verification() {
        let mut controller = StandardStreamController::new();
        assert!(controller.validate_pledge_stdio().is_ok());

        controller.active_pledges.clear();
        assert!(controller.validate_pledge_stdio().is_err());
        assert!(controller.write_to_fd(STDOUT_FILENO, b"data").is_err());
    }

    #[test]
    fn test_stream_splice_and_tee_routing() {
        let mut src = StandardStreamHandle::new(3, "pipe_in", StreamBufferMode::BlockBuffered(1024));
        let mut dest = StandardStreamHandle::new(4, "pipe_out", StreamBufferMode::Unbuffered);

        src.write_bytes(b"spliced stream chunk");

        let router = StreamTeeSpliceRouter::new();
        let teed = router.tee_stream(&src, &mut dest, 7);
        assert_eq!(teed, 7);

        let spliced = router.splice_streams(&mut src, &mut dest, 100);
        assert_eq!(spliced, 20);
    }

    #[test]
    fn test_stdbuf_isatty_flush_all_and_sigpipe() {
        let mut controller = StandardStreamController::new();

        // TTY query check
        assert!(controller.isatty(STDOUT_FILENO));
        assert!(controller.isatty(STDERR_FILENO));

        // Auto-detect buffering
        let mut pipe_handle = StandardStreamHandle::new(5, "pipe_out", StreamBufferMode::Unbuffered);
        pipe_handle.is_tty = false;
        pipe_handle.auto_detect_buffering();
        assert_eq!(pipe_handle.buffer_mode, StreamBufferMode::BlockBuffered(4096));

        // Linux stdbuf override
        controller.apply_stdbuf_override(StreamBufferMode::Unbuffered, StreamBufferMode::LineBuffered);
        assert_eq!(controller.handles.get(&STDOUT_FILENO).unwrap().buffer_mode, StreamBufferMode::Unbuffered);

        // SIGPIPE signal trigger on closed stream write
        pipe_handle.is_closed = true;
        pipe_handle.write_bytes(b"doomed write");
        assert!(pipe_handle.broken_stream_sigpipe);

        // Multi-stream flush_all
        let mut ctrl2 = StandardStreamController::new();
        ctrl2.set_buffering(STDOUT_FILENO, StreamBufferMode::BlockBuffered(1024)).unwrap();
        ctrl2.write_to_fd(STDOUT_FILENO, b"buffered_stdout").unwrap();
        let flushed = ctrl2.flush_all();
        assert!(flushed.contains_key(&STDOUT_FILENO));
        assert_eq!(flushed.get(&STDOUT_FILENO).unwrap(), b"buffered_stdout");
    }
}
