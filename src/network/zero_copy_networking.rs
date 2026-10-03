// SPDX-License-Identifier: MIT
// SigmaOS Network Stack - Zero-Copy Networking (AF_XDP / io_uring interface)

use std::sync::atomic::{AtomicUsize, Ordering};
use std::vec::Vec;

/// XDP Program Action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Pass,
    Drop,
    Tx,
    Redirect,
    Aborted,
}

/// Umem Memory Pool for Zero-Copy Network Packets
#[derive(Debug)]
pub struct UmemPool {
    pub frame_size: usize,
    pub frame_count: usize,
    pub frames: Vec<Vec<u8>>,
    pub free_indices: Vec<usize>,
}

impl UmemPool {
    pub fn new(frame_size: usize, frame_count: usize) -> Self {
        let mut frames = Vec::with_capacity(frame_count);
        let mut free_indices = Vec::with_capacity(frame_count);
        for i in 0..frame_count {
            frames.push(vec![0u8; frame_size]);
            free_indices.push(i);
        }
        Self {
            frame_size,
            frame_count,
            frames,
            free_indices,
        }
    }

    pub fn alloc_frame(&mut self) -> Option<usize> {
        self.free_indices.pop()
    }

    pub fn free_frame(&mut self, idx: usize) {
        if idx < self.frame_count {
            self.free_indices.push(idx);
        }
    }
}

/// XDP Descriptor Ring Buffer
#[derive(Debug)]
pub struct XdpRing {
    pub capacity: usize,
    pub descriptors: Vec<u64>,
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
}

impl XdpRing {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            descriptors: vec![0; capacity],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn produce(&mut self, desc: u64) -> bool {
        let h = self.head.load(Ordering::Relaxed);
        let t = self.tail.load(Ordering::Relaxed);
        if (h + 1) % self.capacity == t {
            return false; // Ring full
        }
        self.descriptors[h] = desc;
        self.head.store((h + 1) % self.capacity, Ordering::Release);
        true
    }

    pub fn consume(&mut self) -> Option<u64> {
        let h = self.head.load(Ordering::Acquire);
        let t = self.tail.load(Ordering::Relaxed);
        if h == t {
            return None; // Ring empty
        }
        let desc = self.descriptors[t];
        self.tail.store((t + 1) % self.capacity, Ordering::Release);
        Some(desc)
    }
}

/// Async I/O Completion Entry
#[derive(Debug, Clone, Copy)]
pub struct IoCompletionEntry {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

/// Async I/O Completion Queue
#[derive(Debug)]
pub struct IoCompletionQueue {
    pub entries: Vec<IoCompletionEntry>,
}

impl IoCompletionQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, entry: IoCompletionEntry) {
        self.entries.push(entry);
    }

    pub fn pop(&mut self) -> Option<IoCompletionEntry> {
        self.entries.pop()
    }
}

/// Sovereign AF_XDP Zero-Copy Socket
#[derive(Debug)]
pub struct SovereignZeroCopySocket {
    pub if_index: u32,
    pub queue_id: u32,
    pub rx_ring: XdpRing,
    pub tx_ring: XdpRing,
    pub fill_ring: XdpRing,
    pub comp_ring: XdpRing,
    pub umem: UmemPool,
}

impl SovereignZeroCopySocket {
    pub fn new(if_index: u32, queue_id: u32, ring_size: usize) -> Self {
        Self {
            if_index,
            queue_id,
            rx_ring: XdpRing::new(ring_size),
            tx_ring: XdpRing::new(ring_size),
            fill_ring: XdpRing::new(ring_size),
            comp_ring: XdpRing::new(ring_size),
            umem: UmemPool::new(2048, ring_size * 2),
        }
    }

    pub fn receive_packet(&mut self) -> Option<usize> {
        let desc = self.rx_ring.consume()?;
        Some(desc as usize)
    }

    pub fn send_packet(&mut self, frame_idx: usize, len: usize) -> bool {
        let desc = ((frame_idx as u64) << 32) | (len as u64);
        self.tx_ring.produce(desc)
    }
}
