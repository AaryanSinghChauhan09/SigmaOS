// Lock-free Ring Buffer for eBPF events
// Single-producer, single-consumer (SPSC) ring buffer

#![no_std]
extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;
use alloc::vec;

/// Ring buffer errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RingBufError {
    CapacityNotPowerOfTwo,
    CapacityTooSmall,
    TooLarge,
    NoSpace,
    Contention,
    InvalidOffset,
}

/// Lock-free ring buffer
pub struct BpfRingBuf {
    buf: Vec<u8>,
    mask: usize,
    capacity: usize,
    cons_pos: AtomicUsize,  // Consumer position
    prod_pos: AtomicUsize,  // Producer position
}

impl BpfRingBuf {
    /// Create a new ring buffer with power-of-2 capacity
    pub fn new(capacity: usize) -> Result<Self, RingBufError> {
        if capacity < 4096 {
            return Err(RingBufError::CapacityTooSmall);
        }
        
        if !capacity.is_power_of_two() {
            return Err(RingBufError::CapacityNotPowerOfTwo);
        }
        
        Ok(Self {
            buf: vec![0u8; capacity],
            mask: capacity - 1,
            capacity,
            cons_pos: AtomicUsize::new(0),
            prod_pos: AtomicUsize::new(0),
        })
    }
    
    /// Reserve space for writing (lock-free)
    pub fn reserve(&self, size: usize) -> Option<usize> {
        if size > self.capacity / 2 {
            return None;
        }
        
        // Align to 8-byte boundary
        let padded_len = (size + 7) & !7;
        let total_len = padded_len + 8; // +8 for header
        
        let head = self.prod_pos.load(Ordering::Acquire);
        let tail = self.cons_pos.load(Ordering::Acquire);
        
        let available = self.capacity - (head.wrapping_sub(tail));
        
        if total_len > available {
            return None;
        }
        
        // Try to atomically reserve space
        match self.prod_pos.compare_exchange(
            head,
            head.wrapping_add(total_len),
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            Ok(_) => Some(head),
            Err(_) => None, // Contention - retry by caller
        }
    }
    
    /// Submit data to reserved space
    pub fn submit(&mut self, offset: usize, data: &[u8]) {
        let len = data.len();
        
        // Write header (length)
        let header_offset = offset & self.mask;
        self.write_u64(header_offset, len as u64);
        
        // Write data
        let data_offset = (offset + 8) & self.mask;
        for (i, &byte) in data.iter().enumerate() {
            let pos = (data_offset + i) & self.mask;
            self.buf[pos] = byte;
        }
    }
    
    /// Discard reserved space without writing
    pub fn discard(&mut self, _offset: usize) -> Result<(), RingBufError> {
        // In a full implementation, mark as discarded
        // For now, just succeed
        Ok(())
    }
    
    /// Consume next sample (reader side)
    pub fn consume(&self) -> Option<Vec<u8>> {
        let tail = self.cons_pos.load(Ordering::Acquire);
        let head = self.prod_pos.load(Ordering::Acquire);
        
        if tail == head {
            return None; // Empty
        }
        
        // Read header
        let header_offset = tail & self.mask;
        let len = self.read_u64(header_offset) as usize;
        
        if len == 0 || len > self.capacity {
            return None;
        }
        
        // Read data
        let mut data = Vec::with_capacity(len);
        let data_offset = (tail + 8) & self.mask;
        
        for i in 0..len {
            let pos = (data_offset + i) & self.mask;
            data.push(self.buf[pos]);
        }
        
        // Advance consumer position
        let padded_len = (len + 7) & !7;
        let total_len = padded_len + 8;
        self.cons_pos.store(tail.wrapping_add(total_len), Ordering::Release);
        
        Some(data)
    }
    
    /// Get capacity in bytes
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Approximate used bytes
    pub fn len(&self) -> usize {
        let head = self.prod_pos.load(Ordering::Relaxed);
        let tail = self.cons_pos.load(Ordering::Relaxed);
        head.wrapping_sub(tail)
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Write u64 at offset
    fn write_u64(&mut self, offset: usize, val: u64) {
        let bytes = val.to_le_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            let pos = (offset + i) & self.mask;
            self.buf[pos] = byte;
        }
    }
    
    /// Read u64 from offset
    fn read_u64(&self, offset: usize) -> u64 {
        let mut bytes = [0u8; 8];
        for (i, byte) in bytes.iter_mut().enumerate() {
            let pos = (offset + i) & self.mask;
            *byte = self.buf[pos];
        }
        u64::from_le_bytes(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ringbuf_creation() {
        let rb = BpfRingBuf::new(4096).unwrap();
        assert_eq!(rb.capacity(), 4096);
        assert!(rb.is_empty());
    }
    
    #[test]
    fn test_ringbuf_not_power_of_two() {
        let result = BpfRingBuf::new(4000);
        assert_eq!(result.unwrap_err(), RingBufError::CapacityNotPowerOfTwo);
    }
    
    #[test]
    fn test_ringbuf_too_small() {
        let result = BpfRingBuf::new(1024);
        assert_eq!(result.unwrap_err(), RingBufError::CapacityTooSmall);
    }
    
    #[test]
    fn test_ringbuf_reserve_submit_consume() {
        let mut rb = BpfRingBuf::new(4096).unwrap();
        
        // Reserve space
        let offset = rb.reserve(64).unwrap();
        
        // Submit data
        let data = b"Hello, eBPF ring buffer!";
        rb.submit(offset, data);
        
        // Consume data
        let consumed = rb.consume().unwrap();
        assert_eq!(&consumed[..data.len()], data);
    }
    
    #[test]
    fn test_ringbuf_empty_consume() {
        let rb = BpfRingBuf::new(4096).unwrap();
        assert!(rb.consume().is_none());
    }
    
    #[test]
    fn test_ringbuf_reserve_too_large() {
        let rb = BpfRingBuf::new(4096).unwrap();
        
        // Try to reserve more than half capacity
        assert!(rb.reserve(3000).is_none());
    }
    
    #[test]
    fn test_ringbuf_multiple_entries() {
        let mut rb = BpfRingBuf::new(8192).unwrap();
        
        // Write multiple entries
        for i in 0..5 {
            let offset = rb.reserve(16).unwrap();
            let data = alloc::format!("Entry {}", i);
            rb.submit(offset, data.as_bytes());
        }
        
        // Read them back
        for i in 0..5 {
            let consumed = rb.consume().unwrap();
            let expected = alloc::format!("Entry {}", i);
            assert_eq!(&consumed[..expected.len()], expected.as_bytes());
        }
        
        assert!(rb.consume().is_none());
    }
}
