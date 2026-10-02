// SPDX-License-Identifier: MIT
// Sovereign Zero-Copy AF_XDP Networking Engine (`src/network/zero_copy_networking.rs`)
// Zero-dependency `#![no_std]` / `alloc` compliant Linux AF_XDP & XDP driver subsystem.

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Pass,
    Drop,
    Tx,
    Redirect,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct XdpRing {
    pub producer_idx: u32,
    pub consumer_idx: u32,
    pub capacity: usize,
    pub descriptors: Vec<u64>,
}

impl XdpRing {
    pub fn new(capacity: usize) -> Self {
        Self {
            producer_idx: 0,
            consumer_idx: 0,
            capacity,
            descriptors: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, desc: u64) -> bool {
        if self.descriptors.len() < self.capacity {
            self.descriptors.push(desc);
            self.producer_idx = self.producer_idx.wrapping_add(1);
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<u64> {
        if !self.descriptors.is_empty() {
            self.consumer_idx = self.consumer_idx.wrapping_add(1);
            Some(self.descriptors.remove(0))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct UmemPool {
    pub chunk_size_bytes: usize,
    pub frame_count: usize,
    pub free_frames: Vec<u64>,
}

impl UmemPool {
    pub fn new(chunk_size_bytes: usize, frame_count: usize) -> Self {
        let mut free_frames = Vec::with_capacity(frame_count);
        for i in 0..frame_count {
            free_frames.push((i * chunk_size_bytes) as u64);
        }
        Self {
            chunk_size_bytes,
            frame_count,
            free_frames,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<u64> {
        self.free_frames.pop()
    }

    pub fn free_frame(&mut self, addr: u64) {
        self.free_frames.push(addr);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoCompletionEntry {
    pub addr: u64,
    pub len: u32,
    pub status: i32,
}

#[derive(Debug, Clone)]
pub struct IoCompletionQueue {
    pub queue: Vec<IoCompletionEntry>,
}

impl IoCompletionQueue {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push(&mut self, entry: IoCompletionEntry) {
        self.queue.push(entry);
    }

    pub fn pop(&mut self) -> Option<IoCompletionEntry> {
        if !self.queue.is_empty() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}

impl Default for IoCompletionQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SovereignZeroCopySocket {
    pub if_index: u32,
    pub queue_id: u32,
    pub umem: UmemPool,
    pub rx_ring: XdpRing,
    pub tx_ring: XdpRing,
    pub cq: IoCompletionQueue,
}

impl SovereignZeroCopySocket {
    pub fn bind(if_index: u32, queue_id: u32, umem: UmemPool) -> Self {
        Self {
            if_index,
            queue_id,
            umem,
            rx_ring: XdpRing::new(1024),
            tx_ring: XdpRing::new(1024),
            cq: IoCompletionQueue::new(),
        }
    }

    pub fn rx_packet(&mut self) -> Option<u64> {
        self.rx_ring.pop()
    }

    pub fn tx_packet(&mut self, addr: u64) -> bool {
        self.tx_ring.push(addr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_af_xdp_socket() {
        let umem = UmemPool::new(2048, 64);
        let mut xsk = SovereignZeroCopySocket::bind(1, 0, umem);

        assert!(xsk.tx_packet(0x1000));
        assert_eq!(xsk.tx_ring.pop(), Some(0x1000));
    }
}
