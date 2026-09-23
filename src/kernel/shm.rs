// Linux-inspired shared memory for IPC
// Provides shared memory segments for inter-process communication

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Shared memory permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShmPerm {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl ShmPerm {
    pub fn new() -> Self {
        Self {
            read: false,
            write: false,
            execute: false,
        }
    }

    pub fn with_read(mut self) -> Self {
        self.read = true;
        self
    }

    pub fn with_write(mut self) -> Self {
        self.write = true;
        self
    }

    pub fn with_execute(mut self) -> Self {
        self.execute = true;
        self
    }

    pub fn to_mode(&self) -> u32 {
        let mut mode = 0u32;
        if self.read { mode |= 0o400; }
        if self.write { mode |= 0o200; }
        if self.execute { mode |= 0o100; }
        mode
    }
}

impl Default for ShmPerm {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared memory segment
#[derive(Debug, Clone)]
pub struct ShmSegment {
    pub id: u64,
    pub key: i32,
    pub size: usize,
    pub data: Vec<u8>,
    pub perm: ShmPerm,
    pub attached: bool,
}

impl ShmSegment {
    pub fn new(id: u64, key: i32, size: usize, perm: ShmPerm) -> Self {
        Self {
            id,
            key,
            size,
            data: vec![0; size],
            perm,
            attached: false,
        }
    }

    /// Attach to shared memory
    pub fn attach(&mut self) -> Result<(), String> {
        if self.attached {
            return Err("Already attached".to_string());
        }
        self.attached = true;
        Ok(())
    }

    /// Detach from shared memory
    pub fn detach(&mut self) {
        self.attached = false;
    }

    /// Read from shared memory
    pub fn read(&self, offset: usize, count: usize) -> Result<Vec<u8>, String> {
        if !self.attached {
            return Err("Not attached".to_string());
        }

        if offset + count > self.size {
            return Err("Read beyond segment size".to_string());
        }

        Ok(self.data[offset..offset + count].to_vec())
    }

    /// Write to shared memory
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), String> {
        if !self.attached {
            return Err("Not attached".to_string());
        }

        if !self.perm.write {
            return Err("No write permission".to_string());
        }

        if offset + data.len() > self.size {
            return Err("Write beyond segment size".to_string());
        }

        for (i, &byte) in data.iter().enumerate() {
            self.data[offset + i] = byte;
        }

        Ok(())
    }

    /// Check if attached
    pub fn is_attached(&self) -> bool {
        self.attached
    }
}

/// Shared memory manager for system-wide shared memory management
pub struct ShmManager {
    segments: Arc<Mutex<HashMap<u64, ShmSegment>>>,
    next_segment_id: Arc<Mutex<u64>>,
}

impl ShmManager {
    pub fn new() -> Self {
        Self {
            segments: Arc::new(Mutex::new(HashMap::new())),
            next_segment_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new shared memory segment
    pub fn create_segment(&self, key: i32, size: usize, perm: ShmPerm) -> u64 {
        let mut next_id = self.next_segment_id.lock().unwrap();
        let segment_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let segment = ShmSegment::new(segment_id, key, size, perm);
        let mut segments = self.segments.lock().unwrap();
        segments.insert(segment_id, segment);

        segment_id
    }

    /// Get a segment by ID
    pub fn get_segment(&self, segment_id: u64) -> Option<ShmSegment> {
        let segments = self.segments.lock().unwrap();
        segments.get(&segment_id).cloned()
    }

    /// Get a segment by key
    pub fn get_segment_by_key(&self, key: i32) -> Option<ShmSegment> {
        let segments = self.segments.lock().unwrap();
        segments.values().find(|s| s.key == key).cloned()
    }

    /// Remove a segment
    pub fn remove_segment(&self, segment_id: u64) -> Result<(), String> {
        let mut segments = self.segments.lock().unwrap();
        match segments.remove(&segment_id) {
            Some(_) => Ok(()),
            None => Err(format!("Segment {} not found", segment_id)),
        }
    }

    /// Attach to segment
    pub fn attach(&self, segment_id: u64) -> Result<(), String> {
        let mut segments = self.segments.lock().unwrap();
        match segments.get_mut(&segment_id) {
            Some(segment) => segment.attach(),
            None => Err(format!("Segment {} not found", segment_id)),
        }
    }

    /// Detach from segment
    pub fn detach(&self, segment_id: u64) -> Result<(), String> {
        let mut segments = self.segments.lock().unwrap();
        match segments.get_mut(&segment_id) {
            Some(segment) => {
                segment.detach();
                Ok(())
            }
            None => Err(format!("Segment {} not found", segment_id)),
        }
    }

    /// Read from segment
    pub fn read(&self, segment_id: u64, offset: usize, count: usize) -> Result<Vec<u8>, String> {
        let segments = self.segments.lock().unwrap();
        match segments.get(&segment_id) {
            Some(segment) => segment.read(offset, count),
            None => Err(format!("Segment {} not found", segment_id)),
        }
    }

    /// Write to segment
    pub fn write(&self, segment_id: u64, offset: usize, data: &[u8]) -> Result<(), String> {
        let mut segments = self.segments.lock().unwrap();
        match segments.get_mut(&segment_id) {
            Some(segment) => segment.write(offset, data),
            None => Err(format!("Segment {} not found", segment_id)),
        }
    }

    /// Get segment count
    pub fn segment_count(&self) -> usize {
        let segments = self.segments.lock().unwrap();
        segments.len()
    }
}

impl Default for ShmManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shm_perm() {
        let perm = ShmPerm::new().with_read().with_write();
        assert!(perm.read);
        assert!(perm.write);
        assert!(!perm.execute);
    }

    #[test]
    fn test_shm_perm_to_mode() {
        let perm = ShmPerm::new().with_read().with_write();
        let mode = perm.to_mode();
        assert_eq!(mode, 0o600);
    }

    #[test]
    fn test_shm_segment() {
        let perm = ShmPerm::new().with_read().with_write();
        let segment = ShmSegment::new(1, 12345, 1024, perm);

        assert_eq!(segment.id, 1);
        assert_eq!(segment.key, 12345);
        assert_eq!(segment.size, 1024);
        assert!(!segment.is_attached());
    }

    #[test]
    fn test_shm_segment_attach() {
        let perm = ShmPerm::new().with_read().with_write();
        let mut segment = ShmSegment::new(1, 12345, 1024, perm);

        segment.attach().unwrap();
        assert!(segment.is_attached());

        assert!(segment.attach().is_err());
    }

    #[test]
    fn test_shm_segment_detach() {
        let perm = ShmPerm::new().with_read().with_write();
        let mut segment = ShmSegment::new(1, 12345, 1024, perm);

        segment.attach().unwrap();
        segment.detach();
        assert!(!segment.is_attached());
    }

    #[test]
    fn test_shm_segment_read() {
        let perm = ShmPerm::new().with_read().with_write();
        let mut segment = ShmSegment::new(1, 12345, 1024, perm);

        segment.attach().unwrap();
        let data = segment.read(0, 10).unwrap();

        assert_eq!(data.len(), 10);
    }

    #[test]
    fn test_shm_segment_read_not_attached() {
        let perm = ShmPerm::new().with_read().with_write();
        let segment = ShmSegment::new(1, 12345, 1024, perm);

        assert!(segment.read(0, 10).is_err());
    }

    #[test]
    fn test_shm_segment_write() {
        let perm = ShmPerm::new().with_read().with_write();
        let mut segment = ShmSegment::new(1, 12345, 1024, perm);

        segment.attach().unwrap();
        segment.write(0, b"Hello").unwrap();

        let read = segment.read(0, 5).unwrap();
        assert_eq!(read, b"Hello");
    }

    #[test]
    fn test_shm_segment_write_no_perm() {
        let perm = ShmPerm::new().with_read();
        let mut segment = ShmSegment::new(1, 12345, 1024, perm);

        segment.attach().unwrap();
        assert!(segment.write(0, b"Hello").is_err());
    }

    #[test]
    fn test_shm_manager() {
        let manager = ShmManager::new();

        let perm = ShmPerm::new().with_read().with_write();
        let segment_id = manager.create_segment(12345, 1024, perm);

        assert_eq!(segment_id, 1);
        assert_eq!(manager.segment_count(), 1);
    }

    #[test]
    fn test_shm_manager_attach_write_read() {
        let manager = ShmManager::new();

        let perm = ShmPerm::new().with_read().with_write();
        let segment_id = manager.create_segment(12345, 1024, perm);

        manager.attach(segment_id).unwrap();
        manager.write(segment_id, 0, b"Hello").unwrap();
        let read = manager.read(segment_id, 0, 5).unwrap();

        assert_eq!(read, b"Hello");
    }

    #[test]
    fn test_shm_manager_get_by_key() {
        let manager = ShmManager::new();

        let perm = ShmPerm::new().with_read().with_write();
        manager.create_segment(12345, 1024, perm);

        let segment = manager.get_segment_by_key(12345);
        assert!(segment.is_some());
    }

    #[test]
    fn test_shm_manager_remove() {
        let manager = ShmManager::new();

        let perm = ShmPerm::new().with_read().with_write();
        let segment_id = manager.create_segment(12345, 1024, perm);

        manager.remove_segment(segment_id).unwrap();
        assert_eq!(manager.segment_count(), 0);
    }

    #[test]
    fn test_shm_manager_invalid() {
        let manager = ShmManager::new();
        assert!(manager.attach(999).is_err());
        assert!(manager.write(999, 0, b"Hello").is_err());
    }
}
