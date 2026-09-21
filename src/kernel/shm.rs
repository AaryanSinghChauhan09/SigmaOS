// Linux-inspired shared memory (shm) for IPC
// Shared memory segments for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// shared memory segment
#[derive(Debug, Clone)]
pub struct ShmSegment {
    pub id: i32,
    pub key: i32,
    pub size: usize,
    pub data: Vec<u8>,
    pub attached: bool,
}

impl ShmSegment {
    pub fn new(id: i32, key: i32, size: usize) -> Self {
        ShmSegment {
            id,
            key,
            size,
            data: vec![0u8; size],
            attached: false,
        }
    }

    pub fn attach(&mut self) {
        self.attached = true;
    }

    pub fn detach(&mut self) {
        self.attached = false;
    }

    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<(), String> {
        if offset + data.len() > self.size {
            return Err("Write exceeds segment size".to_string());
        }

        for (i, &byte) in data.iter().enumerate() {
            self.data[offset + i] = byte;
        }

        Ok(())
    }

    pub fn read(&self, offset: usize, count: usize) -> Result<Vec<u8>, String> {
        if offset + count > self.size {
            return Err("Read exceeds segment size".to_string());
        }

        Ok(self.data[offset..offset + count].to_vec())
    }
}

/// shared memory manager
pub struct ShmManager {
    segments: HashMap<i32, Arc<Mutex<ShmSegment>>>,
    next_id: i32,
}

impl ShmManager {
    pub fn new() -> Self {
        ShmManager {
            segments: HashMap::new(),
            next_id: 1,
        }
    }

    /// Create a new shared memory segment
    pub fn create(&mut self, key: i32, size: usize) -> Result<i32, String> {
        let id = self.next_id;
        self.next_id += 1;

        let segment = Arc::new(Mutex::new(ShmSegment::new(id, key, size)));
        self.segments.insert(id, segment);

        Ok(id)
    }

    /// Get a shared memory segment by ID
    pub fn get(&self, id: i32) -> Option<Arc<Mutex<ShmSegment>>> {
        self.segments.get(&id).cloned()
    }

    /// Get a shared memory segment by key
    pub fn get_by_key(&self, key: i32) -> Option<Arc<Mutex<ShmSegment>>> {
        for segment in self.segments.values() {
            let seg_guard = segment.lock().unwrap();
            if seg_guard.key == key {
                return Some(segment.clone());
            }
        }
        None
    }

    /// Remove a shared memory segment
    pub fn remove(&mut self, id: i32) -> Result<(), String> {
        self.segments.remove(&id)
            .ok_or_else(|| format!("Segment not found: {}", id))?;
        Ok(())
    }

    /// Get segment count
    pub fn segment_count(&self) -> usize {
        self.segments.len()
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
    fn test_shm_segment_creation() {
        let segment = ShmSegment::new(1, 12345, 1024);
        assert_eq!(segment.id, 1);
        assert_eq!(segment.key, 12345);
        assert_eq!(segment.size, 1024);
        assert!(!segment.attached);
    }

    #[test]
    fn test_shm_segment_attach_detach() {
        let mut segment = ShmSegment::new(1, 12345, 1024);
        
        segment.attach();
        assert!(segment.attached);
        
        segment.detach();
        assert!(!segment.attached);
    }

    #[test]
    fn test_shm_segment_write() {
        let mut segment = ShmSegment::new(1, 12345, 1024);
        
        segment.write(0, b"hello").unwrap();
        assert_eq!(&segment.data[0..5], b"hello");
    }

    #[test]
    fn test_shm_segment_write_exceeds() {
        let mut segment = ShmSegment::new(1, 12345, 10);
        
        let result = segment.write(5, b"hello world");
        assert!(result.is_err());
    }

    #[test]
    fn test_shm_segment_read() {
        let mut segment = ShmSegment::new(1, 12345, 1024);
        segment.write(0, b"hello").unwrap();
        
        let data = segment.read(0, 5).unwrap();
        assert_eq!(data, b"hello");
    }

    #[test]
    fn test_shm_segment_read_exceeds() {
        let segment = ShmSegment::new(1, 12345, 10);
        
        let result = segment.read(5, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_shm_manager_creation() {
        let manager = ShmManager::new();
        assert_eq!(manager.segment_count(), 0);
    }

    #[test]
    fn test_shm_manager_create() {
        let mut manager = ShmManager::new();
        
        let id = manager.create(12345, 1024).unwrap();
        assert_eq!(id, 1);
        assert_eq!(manager.segment_count(), 1);
    }

    #[test]
    fn test_shm_manager_get() {
        let mut manager = ShmManager::new();
        
        let id = manager.create(12345, 1024).unwrap();
        let segment = manager.get(id);
        
        assert!(segment.is_some());
    }

    #[test]
    fn test_shm_manager_get_by_key() {
        let mut manager = ShmManager::new();
        
        manager.create(12345, 1024).unwrap();
        let segment = manager.get_by_key(12345);
        
        assert!(segment.is_some());
    }

    #[test]
    fn test_shm_manager_remove() {
        let mut manager = ShmManager::new();
        
        let id = manager.create(12345, 1024).unwrap();
        manager.remove(id).unwrap();
        
        assert_eq!(manager.segment_count(), 0);
    }
}
