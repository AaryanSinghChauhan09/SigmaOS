// Linux-inspired pipe for inter-process communication
// Provides anonymous pipe implementation

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Pipe flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipeFlags {
    pub non_blocking: bool,
    pub close_on_exec: bool,
}

impl PipeFlags {
    pub fn new() -> Self {
        Self {
            non_blocking: false,
            close_on_exec: false,
        }
    }

    pub fn with_non_blocking(mut self, value: bool) -> Self {
        self.non_blocking = value;
        self
    }

    pub fn with_close_on_exec(mut self, value: bool) -> Self {
        self.close_on_exec = value;
        self
    }
}

impl Default for PipeFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Pipe instance
#[derive(Debug, Clone)]
pub struct Pipe {
    pub id: u64,
    pub read_end: u64,
    pub write_end: u64,
    pub buffer: VecDeque<u8>,
    pub capacity: usize,
    pub read_closed: bool,
    pub write_closed: bool,
}

impl Pipe {
    pub fn new(id: u64, capacity: usize) -> Self {
        Self {
            id,
            read_end: id * 2,
            write_end: id * 2 + 1,
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            read_closed: false,
            write_closed: false,
        }
    }

    /// Read from pipe
    pub fn read(&mut self, count: usize) -> Result<Vec<u8>, String> {
        if self.read_closed && self.buffer.is_empty() {
            return Err("Pipe read end closed".to_string());
        }

        if self.buffer.is_empty() {
            if self.write_closed {
                return Err("Pipe write end closed".to_string());
            }
            return Err("Would block".to_string());
        }

        let mut result = Vec::new();
        for _ in 0..count {
            if let Some(byte) = self.buffer.pop_front() {
                result.push(byte);
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Write to pipe
    pub fn write(&mut self, data: &[u8]) -> Result<usize, String> {
        if self.write_closed {
            return Err("Pipe write end closed".to_string());
        }

        if self.read_closed {
            return Err("Pipe read end closed".to_string());
        }

        let available = self.capacity - self.buffer.len();
        let to_write = data.len().min(available);

        for &byte in data.iter().take(to_write) {
            self.buffer.push_back(byte);
        }

        Ok(to_write)
    }

    /// Close read end
    pub fn close_read(&mut self) {
        self.read_closed = true;
    }

    /// Close write end
    pub fn close_write(&mut self) {
        self.write_closed = true;
    }

    /// Get available bytes for reading
    pub fn available_read(&self) -> usize {
        self.buffer.len()
    }

    /// Get available space for writing
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

/// Pipe manager for system-wide pipe management
pub struct PipeManager {
    pipes: Arc<Mutex<HashMap<u64, Pipe>>>,
    next_pipe_id: Arc<Mutex<u64>>,
}

impl PipeManager {
    pub fn new() -> Self {
        Self {
            pipes: Arc::new(Mutex::new(HashMap::new())),
            next_pipe_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new pipe
    pub fn create_pipe(&self, capacity: usize) -> Result<(u64, u64), String> {
        let mut next_id = self.next_pipe_id.lock().unwrap();
        let pipe_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let pipe = Pipe::new(pipe_id, capacity);
        let read_end = pipe.read_end;
        let write_end = pipe.write_end;

        let mut pipes = self.pipes.lock().unwrap();
        pipes.insert(pipe_id, pipe);

        Ok((read_end, write_end))
    }

    /// Get a pipe by ID
    pub fn get_pipe(&self, pipe_id: u64) -> Option<Pipe> {
        let pipes = self.pipes.lock().unwrap();
        pipes.get(&pipe_id).cloned()
    }

    /// Remove a pipe
    pub fn remove_pipe(&self, pipe_id: u64) -> Result<(), String> {
        let mut pipes = self.pipes.lock().unwrap();
        match pipes.remove(&pipe_id) {
            Some(_) => Ok(()),
            None => Err(format!("Pipe {} not found", pipe_id)),
        }
    }

    /// Read from pipe
    pub fn read(&self, pipe_id: u64, count: usize) -> Result<Vec<u8>, String> {
        let mut pipes = self.pipes.lock().unwrap();
        match pipes.get_mut(&pipe_id) {
            Some(pipe) => pipe.read(count),
            None => Err(format!("Pipe {} not found", pipe_id)),
        }
    }

    /// Write to pipe
    pub fn write(&self, pipe_id: u64, data: &[u8]) -> Result<usize, String> {
        let mut pipes = self.pipes.lock().unwrap();
        match pipes.get_mut(&pipe_id) {
            Some(pipe) => pipe.write(data),
            None => Err(format!("Pipe {} not found", pipe_id)),
        }
    }

    /// Close read end
    pub fn close_read(&self, pipe_id: u64) -> Result<(), String> {
        let mut pipes = self.pipes.lock().unwrap();
        match pipes.get_mut(&pipe_id) {
            Some(pipe) => {
                pipe.close_read();
                Ok(())
            }
            None => Err(format!("Pipe {} not found", pipe_id)),
        }
    }

    /// Close write end
    pub fn close_write(&self, pipe_id: u64) -> Result<(), String> {
        let mut pipes = self.pipes.lock().unwrap();
        match pipes.get_mut(&pipe_id) {
            Some(pipe) => {
                pipe.close_write();
                Ok(())
            }
            None => Err(format!("Pipe {} not found", pipe_id)),
        }
    }

    /// Get pipe count
    pub fn pipe_count(&self) -> usize {
        let pipes = self.pipes.lock().unwrap();
        pipes.len()
    }
}

impl Default for PipeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipe_flags() {
        let flags = PipeFlags::new().with_non_blocking(true).with_close_on_exec(true);
        assert!(flags.non_blocking);
        assert!(flags.close_on_exec);
    }

    #[test]
    fn test_pipe() {
        let pipe = Pipe::new(1, 1024);
        assert_eq!(pipe.id, 1);
        assert_eq!(pipe.read_end, 2);
        assert_eq!(pipe.write_end, 3);
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipe_write_read() {
        let mut pipe = Pipe::new(1, 1024);

        let data = b"Hello, World!";
        let written = pipe.write(data).unwrap();
        assert_eq!(written, data.len());

        let read = pipe.read(data.len()).unwrap();
        assert_eq!(read, data.to_vec());
    }

    #[test]
    fn test_pipe_partial_write() {
        let mut pipe = Pipe::new(1, 5);

        let data = b"Hello, World!";
        let written = pipe.write(data).unwrap();
        assert_eq!(written, 5);
    }

    #[test]
    fn test_pipe_write_closed() {
        let mut pipe = Pipe::new(1, 1024);
        pipe.close_write();

        let data = b"Hello";
        assert!(pipe.write(data).is_err());
    }

    #[test]
    fn test_pipe_read_closed() {
        let mut pipe = Pipe::new(1, 1024);
        pipe.close_read();

        assert!(pipe.read(10).is_err());
    }

    #[test]
    fn test_pipe_available() {
        let mut pipe = Pipe::new(1, 1024);

        let data = b"Hello";
        pipe.write(data).unwrap();

        assert_eq!(pipe.available_read(), 5);
        assert_eq!(pipe.available_write(), 1019);
    }

    #[test]
    fn test_pipe_full() {
        let mut pipe = Pipe::new(1, 5);

        let data = b"Hello";
        pipe.write(data).unwrap();

        assert!(pipe.is_full());
    }

    #[test]
    fn test_pipe_manager() {
        let manager = PipeManager::new();

        let (read_end, write_end) = manager.create_pipe(1024).unwrap();
        assert_eq!(read_end, 2);
        assert_eq!(write_end, 3);

        assert_eq!(manager.pipe_count(), 1);
    }

    #[test]
    fn test_pipe_manager_write_read() {
        let manager = PipeManager::new();

        let (_read_end, _write_end) = manager.create_pipe(1024).unwrap();
        let pipe_id = 1;

        let data = b"Hello, World!";
        manager.write(pipe_id, data).unwrap();
        let read = manager.read(pipe_id, data.len()).unwrap();

        assert_eq!(read, data.to_vec());
    }

    #[test]
    fn test_pipe_manager_close() {
        let manager = PipeManager::new();

        let (_read_end, _write_end) = manager.create_pipe(1024).unwrap();
        let pipe_id = 1;

        manager.close_write(pipe_id).unwrap();
        assert!(manager.write(pipe_id, b"Hello").is_err());
    }

    #[test]
    fn test_pipe_manager_remove() {
        let manager = PipeManager::new();

        let (_read_end, _write_end) = manager.create_pipe(1024).unwrap();
        let pipe_id = 1;

        manager.remove_pipe(pipe_id).unwrap();
        assert_eq!(manager.pipe_count(), 0);
    }

    #[test]
    fn test_pipe_manager_invalid() {
        let manager = PipeManager::new();
        assert!(manager.read(999, 10).is_err());
        assert!(manager.write(999, b"Hello").is_err());
    }
}
