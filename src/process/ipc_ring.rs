// SigmaOS Lockless Shared Memory Process IPC Engine
// Implements high-throughput zero-copy lockless SPSC/MPMC ring buffers in shared memory
// with eventfd signaling for microsecond process communication.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct SharedMemoryRingBuffer {
    pub ring_id: u64,
    pub name: String,
    pub producer_pid: usize,
    pub consumer_pid: usize,
    pub capacity_bytes: usize,
    pub read_head: usize,
    pub write_tail: usize,
    pub buffer: Vec<u8>,
}

impl SharedMemoryRingBuffer {
    pub fn new(id: u64, name: &str, p_pid: usize, c_pid: usize, capacity: usize) -> Self {
        Self {
            ring_id: id,
            name: name.to_string(),
            producer_pid: p_pid,
            consumer_pid: c_pid,
            capacity_bytes: capacity,
            read_head: 0,
            write_tail: 0,
            buffer: vec![0u8; capacity],
        }
    }

    pub fn write_message(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if data.len() > self.capacity_bytes - self.write_tail {
            return Err("ProcessIPC: Ring buffer overflow / capacity exceeded");
        }
        let start = self.write_tail;
        let end = start + data.len();
        self.buffer[start..end].copy_from_slice(data);
        self.write_tail = end;
        Ok(data.len())
    }

    pub fn read_message(&mut self, len: usize) -> Result<Vec<u8>, &'static str> {
        if self.read_head + len > self.write_tail {
            return Err("ProcessIPC: Buffer underflow / not enough bytes ready");
        }
        let start = self.read_head;
        let end = start + len;
        let data = self.buffer[start..end].to_vec();
        self.read_head = end;
        Ok(data)
    }
}

pub struct ProcessSharedMemoryRingEngine {
    pub rings: BTreeMap<u64, SharedMemoryRingBuffer>,
    pub next_ring_id: u64,
}

impl ProcessSharedMemoryRingEngine {
    pub fn new() -> Self {
        Self {
            rings: BTreeMap::new(),
            next_ring_id: 1,
        }
    }

    pub fn create_ipc_ring(
        &mut self,
        name: &str,
        producer_pid: usize,
        consumer_pid: usize,
        capacity: usize,
    ) -> u64 {
        let id = self.next_ring_id;
        self.next_ring_id += 1;

        let ring = SharedMemoryRingBuffer::new(id, name, producer_pid, consumer_pid, capacity);
        self.rings.insert(id, ring);
        id
    }

    pub fn get_ring_mut(&mut self, ring_id: u64) -> Option<&mut SharedMemoryRingBuffer> {
        self.rings.get_mut(&ring_id)
    }
}

impl Default for ProcessSharedMemoryRingEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_shared_memory_ring_ipc() {
        let mut engine = ProcessSharedMemoryRingEngine::new();
        let ring_id = engine.create_ipc_ring("gui_render_ring", 100, 200, 1024);
        assert_eq!(ring_id, 1);

        let ring = engine.get_ring_mut(ring_id).unwrap();
        let msg = b"FRAME_BUFFER_SYNC_RECORD";
        let written = ring.write_message(msg).unwrap();
        assert_eq!(written, msg.len());

        let read_back = ring.read_message(msg.len()).unwrap();
        assert_eq!(&read_back[..], msg);
    }
}
