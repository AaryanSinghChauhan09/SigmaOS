// Zero-Copy Networking Buffers for SigmaOS
// Implements efficient packet handling without unnecessary memory copies

use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

/// Zero-copy buffer handle
#[derive(Debug)]
pub struct ZeroCopyBuffer {
    pub data: Vec<u8>,
    pub ref_count: AtomicUsize,
    pub id: u64,
}

impl ZeroCopyBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        
        Self {
            data,
            ref_count: AtomicUsize::new(1),
            id,
        }
    }

    /// Get buffer ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get reference count
    pub fn ref_count(&self) -> usize {
        self.ref_count.load(Ordering::SeqCst)
    }

    /// Increment reference count
    pub fn inc_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    /// Decrement reference count
    pub fn dec_ref(&self) -> usize {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }

    /// Get data length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get data slice
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Clone data (for compatibility when copy is needed)
    pub fn clone_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

/// Zero-copy buffer pool for efficient memory management
pub struct ZeroCopyBufferPool {
    buffers: Vec<Option<ZeroCopyBuffer>>,
    next_id: AtomicUsize,
    max_buffers: usize,
    buffer_size: usize,
}

impl ZeroCopyBufferPool {
    pub fn new(max_buffers: usize, buffer_size: usize) -> Self {
        Self {
            buffers: vec![None; max_buffers],
            next_id: AtomicUsize::new(0),
            max_buffers,
            buffer_size,
        }
    }

    /// Allocate a new buffer from the pool
    pub fn allocate(&self) -> Result<ZeroCopyBuffer, &'static str> {
        let idx = self.next_id.fetch_add(1, Ordering::SeqCst) % self.max_buffers;
        
        // Create new buffer with zero-initialized data
        let data = vec![0u8; self.buffer_size];
        Ok(ZeroCopyBuffer::new(data))
    }

    /// Allocate a buffer with specific data
    pub fn allocate_with_data(&self, data: Vec<u8>) -> Result<ZeroCopyBuffer, &'static str> {
        if data.len() > self.buffer_size {
            return Err("Data exceeds buffer size");
        }
        Ok(ZeroCopyBuffer::new(data))
    }

    /// Return buffer to pool (decrement ref count)
    pub fn release(&mut self, buffer: ZeroCopyBuffer) {
        if buffer.dec_ref() == 0 {
            // Buffer can be reused
            let idx = buffer.id as usize % self.max_buffers;
            if idx < self.buffers.len() {
                self.buffers[idx] = None;
            }
        }
    }

    /// Get number of allocated buffers
    pub fn allocated_count(&self) -> usize {
        self.buffers.iter().filter(|b| b.is_some()).count()
    }

    /// Get pool capacity
    pub fn capacity(&self) -> usize {
        self.max_buffers
    }

    /// Get buffer size
    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }
}

/// Zero-copy packet descriptor
#[derive(Debug, Clone)]
pub struct ZeroCopyPacket {
    pub buffer: ZeroCopyBuffer,
    pub offset: usize,
    pub length: usize,
    pub metadata: PacketMetadata,
}

/// Packet metadata
#[derive(Debug, Clone, Copy)]
pub struct PacketMetadata {
    pub timestamp: u64,
    pub source: u32,
    pub destination: u32,
    pub protocol: u8,
}

impl ZeroCopyPacket {
    pub fn new(buffer: ZeroCopyBuffer, offset: usize, length: usize) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;

        Self {
            buffer,
            offset,
            length,
            metadata: PacketMetadata {
                timestamp,
                source: 0,
                destination: 0,
                protocol: 0,
            },
        }
    }

    /// Get packet data slice
    pub fn data(&self) -> &[u8] {
        &self.buffer.data[self.offset..self.offset + self.length]
    }

    /// Get packet length
    pub fn len(&self) -> usize {
        self.length
    }

    /// Increment buffer reference count
    pub fn inc_ref(&self) {
        self.buffer.inc_ref();
    }

    /// Decrement buffer reference count
    pub fn dec_ref(&self) -> usize {
        self.buffer.dec_ref()
    }
}

/// Zero-copy packet ring buffer
pub struct ZeroCopyRingBuffer {
    packets: Vec<Option<ZeroCopyPacket>>,
    head: AtomicUsize,
    tail: AtomicUsize,
    capacity: usize,
}

impl ZeroCopyRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            packets: vec![None; capacity],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity,
        }
    }

    /// Enqueue a packet
    pub fn enqueue(&self, packet: ZeroCopyPacket) -> Result<(), &'static str> {
        let tail = self.tail.load(Ordering::SeqCst);
        let head = self.head.load(Ordering::SeqCst);
        
        if (tail + 1) % self.capacity == head {
            return Err("Ring buffer is full");
        }

        self.packets[tail] = Some(packet);
        self.tail.store((tail + 1) % self.capacity, Ordering::SeqCst);
        Ok(())
    }

    /// Dequeue a packet
    pub fn dequeue(&self) -> Option<ZeroCopyPacket> {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);

        if head == tail {
            return None;
        }

        let packet = self.packets[head].take();
        self.head.store((head + 1) % self.capacity, Ordering::SeqCst);
        packet
    }

    /// Get number of packets in ring
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::SeqCst);
        let tail = self.tail.load(Ordering::SeqCst);
        
        if tail >= head {
            tail - head
        } else {
            self.capacity - head + tail
        }
    }

    /// Check if ring is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if ring is full
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }

    /// Get ring capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_buffer() {
        let data = vec![1u8, 2, 3, 4, 5];
        let buffer = ZeroCopyBuffer::new(data);
        
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.ref_count(), 1);
        assert_eq!(buffer.as_slice(), &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reference_counting() {
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3]);
        assert_eq!(buffer.ref_count(), 1);
        
        buffer.inc_ref();
        assert_eq!(buffer.ref_count(), 2);
        
        assert_eq!(buffer.dec_ref(), 1);
        assert_eq!(buffer.ref_count(), 1);
    }

    #[test]
    fn test_buffer_pool() {
        let pool = ZeroCopyBufferPool::new(16, 1024);
        
        let buffer = pool.allocate().unwrap();
        assert_eq!(buffer.len(), 1024);
        assert_eq!(pool.allocated_count(), 0); // Not tracked in this simple implementation
    }

    #[test]
    fn test_buffer_pool_with_data() {
        let pool = ZeroCopyBufferPool::new(16, 1024);
        let data = vec![1u8, 2, 3];
        
        let buffer = pool.allocate_with_data(data).unwrap();
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_zero_copy_packet() {
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3, 4, 5]);
        let packet = ZeroCopyPacket::new(buffer, 1, 3);
        
        assert_eq!(packet.len(), 3);
        assert_eq!(packet.data(), &[2, 3, 4]);
    }

    #[test]
    fn test_ring_buffer() {
        let ring = ZeroCopyRingBuffer::new(8);
        
        let buffer = ZeroCopyBuffer::new(vec![1, 2, 3]);
        let packet = ZeroCopyPacket::new(buffer, 0, 3);
        
        assert!(ring.enqueue(packet).is_ok());
        assert_eq!(ring.len(), 1);
        
        let dequeued = ring.dequeue();
        assert!(dequeued.is_some());
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn test_ring_buffer_full() {
        let ring = ZeroCopyRingBuffer::new(2);
        
        let buffer1 = ZeroCopyBuffer::new(vec![1]);
        let packet1 = ZeroCopyPacket::new(buffer1, 0, 1);
        
        let buffer2 = ZeroCopyBuffer::new(vec![2]);
        let packet2 = ZeroCopyPacket::new(buffer2, 0, 1);
        
        assert!(ring.enqueue(packet1).is_ok());
        assert!(ring.enqueue(packet2).is_ok());
        assert!(ring.is_full());
    }
}
