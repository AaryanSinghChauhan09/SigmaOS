// Linux-inspired pipe for inter-process communication
// Anonymous and named pipes for SigmaOS

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// pipe configuration flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipeFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl PipeFlags {
    pub fn new() -> Self {
        PipeFlags {
            non_blocking: false,
            close_on_exec: false,
        }
    }
}

impl Default for PipeFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// pipe instance
pub struct Pipe {
    buffer: VecDeque<u8>,
    capacity: usize,
    flags: PipeFlags,
    read_closed: bool,
    write_closed: bool,
}

impl Pipe {
    pub fn new(capacity: usize, flags: PipeFlags) -> Self {
        Pipe {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            flags,
            read_closed: false,
            write_closed: false,
        }
    }

    /// Write to pipe
    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        if self.write_closed {
            return Err("Write end closed".to_string());
        }

        if self.read_closed {
            return Err("Broken pipe".to_string());
        }

        let available = self.capacity - self.buffer.len();
        let to_write = data.len().min(available);

        if to_write == 0 && !self.flags.non_blocking {
            return Err("Would block".to_string());
        }

        for &byte in data.iter().take(to_write) {
            self.buffer.push_back(byte);
        }

        Ok(to_write)
    }

    /// Read from pipe
    pub fn read(&mut self, count: usize) -> Result<Vec<u8>, String> {
        if self.read_closed {
            return Err("Read end closed".to_string());
        }

        if self.buffer.is_empty() {
            if self.write_closed {
                return Ok(Vec::new()); // EOF
            }
            if self.flags.non_blocking {
                return Err("Would block".to_string());
            }
            return Err("Would block".to_string());
        }

        let to_read = count.min(self.buffer.len());
        let mut result = Vec::with_capacity(to_read);

        for _ in 0..to_read {
            if let Some(byte) = self.buffer.pop_front() {
                result.push(byte);
            }
        }

        Ok(result)
    }

    /// Close read end
    pub fn close_read(&mut self) {
        self.read_closed = true;
    }

    /// Close write end
    pub fn close_write(&mut self) {
        self.write_closed = true;
    }

    /// Check if read end is closed
    pub fn is_read_closed(&self) -> bool {
        self.read_closed
    }

    /// Check if write end is closed
    pub fn is_write_closed(&self) -> bool {
        self.write_closed
    }

    /// Get available bytes to read
    pub fn available(&self) -> usize {
        self.buffer.len()
    }

    /// Get available space to write
    pub fn available_write(&self) -> usize {
        self.capacity - self.buffer.len()
    }

    /// Check if pipe is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Check if pipe is full
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::new(65536, PipeFlags::new()) // 64 KB default
    }
}

/// pipe file descriptor pair
pub struct PipePair {
    pub read_fd: Arc<Mutex<Pipe>>,
    pub write_fd: Arc<Mutex<Pipe>>,
}

impl PipePair {
    pub fn new(capacity: usize, flags: PipeFlags) -> Self {
        let pipe = Arc::new(Mutex::new(Pipe::new(capacity, flags)));
        PipePair {
            read_fd: pipe.clone(),
            write_fd: pipe,
        }
    }
}

impl Default for PipePair {
    fn default() -> Self {
        Self::new(65536, PipeFlags::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_creation() {
        let pipe = Pipe::new(1024, PipeFlags::new());
        assert_eq!(pipe.capacity, 1024);
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipe_write() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        let result = pipe.write(b"hello").unwrap();
        assert_eq!(result, 5);
        assert_eq!(pipe.available(), 5);
    }

    #[test]
    fn test_pipe_read() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.write(b"hello").unwrap();
        
        let data = pipe.read(10).unwrap();
        assert_eq!(data, b"hello");
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipe_write_full() {
        let mut pipe = Pipe::new(10, PipeFlags::new());
        pipe.write(b"0123456789").unwrap();
        
        let result = pipe.write(b"x");
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_read_empty_blocking() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        let result = pipe.read(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_read_empty_nonblocking() {
        let mut flags = PipeFlags::new();
        flags.non_blocking = true;
        let mut pipe = Pipe::new(1024, flags);
        
        let result = pipe.read(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_close_read() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.write(b"hello").unwrap();
        pipe.close_read();
        
        let result = pipe.read(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_close_write() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.close_write();
        
        let result = pipe.write(b"hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_broken_pipe() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.close_read();
        
        let result = pipe.write(b"hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe_read_eof() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.close_write();
        
        let data = pipe.read(10).unwrap();
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_pipe_pair() {
        let pair = PipePair::new(1024, PipeFlags::new());
        
        {
            let mut write_guard = pair.write_fd.lock().unwrap();
            write_guard.write(b"hello").unwrap();
        }
        
        {
            let mut read_guard = pair.read_fd.lock().unwrap();
            let data = read_guard.read(10).unwrap();
            assert_eq!(data, b"hello");
        }
    }

    #[test]
    fn test_pipe_available_write() {
        let mut pipe = Pipe::new(1024, PipeFlags::new());
        pipe.write(b"hello").unwrap();
        
        assert_eq!(pipe.available_write(), 1024 - 5);
    }
}
