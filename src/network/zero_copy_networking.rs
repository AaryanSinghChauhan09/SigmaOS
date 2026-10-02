// SPDX-License-Identifier: MIT
// SigmaOS Zero-Copy Networking Subsystem (AF_XDP / Umem)

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Pass,
    Drop,
    Tx,
    Redirect(u32),
    Aborted,
}

#[derive(Debug, Clone)]
pub struct IoCompletionEntry {
    pub addr: u64,
    pub len: u32,
    pub flags: u32,
}

#[derive(Debug)]
pub struct IoCompletionQueue {
    pub entries: Vec<IoCompletionEntry>,
    pub capacity: usize,
}

impl IoCompletionQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
        }
    }
}

#[derive(Debug)]
pub struct UmemPool {
    pub chunk_size: usize,
    pub num_chunks: usize,
    pub base_addr: u64,
}

impl UmemPool {
    pub fn new(chunk_size: usize, num_chunks: usize, base_addr: u64) -> Self {
        Self {
            chunk_size,
            num_chunks,
            base_addr,
        }
    }
}

#[derive(Debug)]
pub struct XdpRing {
    pub size: usize,
    pub head: usize,
    pub tail: usize,
}

impl XdpRing {
    pub fn new(size: usize) -> Self {
        Self { size, head: 0, tail: 0 }
    }
}

#[derive(Debug)]
pub struct SovereignZeroCopySocket {
    pub ifindex: u32,
    pub queue_id: u32,
    pub umem: UmemPool,
    pub rx_ring: XdpRing,
    pub tx_ring: XdpRing,
    pub cq: IoCompletionQueue,
}

impl SovereignZeroCopySocket {
    pub fn new(ifindex: u32, queue_id: u32, umem: UmemPool) -> Self {
        Self {
            ifindex,
            queue_id,
            umem,
            rx_ring: XdpRing::new(256),
            tx_ring: XdpRing::new(256),
            cq: IoCompletionQueue::new(256),
        }
    }
}
