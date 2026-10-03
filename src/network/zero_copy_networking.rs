// SPDX-License-Identifier: MIT
// Zero-Copy Networking Subsystem for Sovereign OS
// Inspired by Linux AF_XDP / XSK zero-copy sockets and UMEM memory pools

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZeroCopyBufferType {
    KernelPageRing,
    SharedMemory,
    DmaRingBuffer,
}

#[derive(Debug, Clone)]
pub struct UmemChunk {
    pub addr: u64,
    pub len: u32,
    pub headroom: u16,
    pub in_use: bool,
}

impl UmemChunk {
    pub fn new(addr: u64, max_len: u32) -> Self {
        UmemChunk {
            addr,
            len: max_len,
            headroom: 256,
            in_use: false,
        }
    }
}

pub struct UmemPool {
    pub chunks: Vec<UmemChunk>,
    pub chunk_size: u32,
    pub total_chunks: u32,
    pub free_count: u32,
    pub alloc_count: u64,
    pub free_total: u64,
}

impl UmemPool {
    pub fn new(total_chunks: u32, chunk_size: u32) -> Self {
        let mut chunks = Vec::new();
        for i in 0..total_chunks {
            chunks.push(UmemChunk::new(i as u64 * chunk_size as u64, chunk_size));
        }
        UmemPool {
            chunks,
            chunk_size,
            total_chunks,
            free_count: total_chunks,
            alloc_count: 0,
            free_total: 0,
        }
    }

    pub fn alloc_chunk(&mut self) -> Option<usize> {
        for (idx, chunk) in self.chunks.iter_mut().enumerate() {
            if !chunk.in_use {
                chunk.in_use = true;
                self.free_count = self.free_count.saturating_sub(1);
                self.alloc_count = self.alloc_count.saturating_add(1);
                return Some(idx);
            }
        }
        None
    }

    pub fn free_chunk(&mut self, idx: usize) {
        if idx < self.chunks.len() && self.chunks[idx].in_use {
            self.chunks[idx].in_use = false;
            self.free_count = self.free_count.saturating_add(1);
            self.free_total = self.free_total.saturating_add(1);
        }
    }

    pub fn utilization_pct(&self) -> u32 {
        if self.total_chunks == 0 {
            return 0;
        }
        let used = self.total_chunks.saturating_sub(self.free_count) as u64;
        ((used * 100) / self.total_chunks as u64) as u32
    }
}

pub struct PacketRingDescriptor {
    pub chunk_idx: usize,
    pub data_offset: u32,
    pub data_len: u32,
    pub flags: u32,
}

pub struct XdpRing {
    pub entries: VecDeque<PacketRingDescriptor>,
    pub capacity: usize,
    pub packets_processed: u64,
    pub drops: u64,
}

impl XdpRing {
    pub fn new(capacity: usize) -> Self {
        XdpRing {
            entries: VecDeque::with_capacity(capacity),
            capacity,
            packets_processed: 0,
            drops: 0,
        }
    }

    pub fn enqueue(&mut self, desc: PacketRingDescriptor) -> bool {
        if self.entries.len() >= self.capacity {
            self.drops = self.drops.saturating_add(1);
            return false;
        }
        self.entries.push_back(desc);
        true
    }

    pub fn dequeue(&mut self) -> Option<PacketRingDescriptor> {
        let desc = self.entries.pop_front()?;
        self.packets_processed = self.packets_processed.saturating_add(1);
        Some(desc)
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

pub struct IoCompletionEntry {
    pub user_data: u64,
    pub result: i32,
    pub flags: u32,
}

pub struct IoCompletionQueue {
    pub entries: VecDeque<IoCompletionEntry>,
    pub capacity: usize,
    pub total_completed: u64,
}

impl IoCompletionQueue {
    pub fn new(capacity: usize) -> Self {
        IoCompletionQueue {
            entries: VecDeque::with_capacity(capacity),
            capacity,
            total_completed: 0,
        }
    }

    pub fn post_completion(&mut self, user_data: u64, result: i32) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push_back(IoCompletionEntry {
            user_data,
            result,
            flags: 0,
        });
        self.total_completed = self.total_completed.saturating_add(1);
        true
    }

    pub fn consume(&mut self) -> Option<IoCompletionEntry> {
        self.entries.pop_front()
    }
}

pub struct ZeroCopySocket {
    pub ifname: String,
    pub queue_id: u32,
    pub umem: UmemPool,
    pub rx_ring: XdpRing,
    pub tx_ring: XdpRing,
    pub cq: IoCompletionQueue,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
}

impl ZeroCopySocket {
    pub fn new(ifname: &str, queue_id: u32, ring_size: usize, umem_chunks: u32, chunk_size: u32) -> Self {
        ZeroCopySocket {
            ifname: ifname.to_string(),
            queue_id,
            umem: UmemPool::new(umem_chunks, chunk_size),
            rx_ring: XdpRing::new(ring_size),
            tx_ring: XdpRing::new(ring_size),
            cq: IoCompletionQueue::new(ring_size),
            rx_packets: 0,
            rx_bytes: 0,
            tx_packets: 0,
            tx_bytes: 0,
        }
    }

    pub fn rx_packet(&mut self) -> Option<u32> {
        let desc = self.rx_ring.dequeue()?;
        self.rx_packets = self.rx_packets.saturating_add(1);
        self.rx_bytes = self.rx_bytes.saturating_add(desc.data_len as u64);
        let len = desc.data_len;
        self.umem.free_chunk(desc.chunk_idx);
        Some(len)
    }

    pub fn tx_packet(&mut self, chunk_idx: usize, len: u32) -> bool {
        let desc = PacketRingDescriptor {
            chunk_idx,
            data_offset: 256,
            data_len: len,
            flags: 0,
        };
        if self.tx_ring.enqueue(desc) {
            self.tx_packets = self.tx_packets.saturating_add(1);
            self.tx_bytes = self.tx_bytes.saturating_add(len as u64);
            self.cq.post_completion(chunk_idx as u64, len as i32);
            true
        } else {
            false
        }
    }

    pub fn stats_summary(&self) -> String {
        format!(
            "ZeroCopySocket[{}] rx_pkts={} rx_bytes={} tx_pkts={} tx_bytes={}",
            self.ifname, self.rx_packets, self.rx_bytes, self.tx_packets, self.tx_bytes
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_umem_pool_alloc_free() {
        let mut pool = UmemPool::new(10, 2048);
        assert_eq!(pool.free_count, 10);
        let idx = pool.alloc_chunk().unwrap();
        assert_eq!(pool.free_count, 9);
        pool.free_chunk(idx);
        assert_eq!(pool.free_count, 10);
    }

    #[test]
    fn test_zero_copy_socket_tx_rx() {
        let mut sock = ZeroCopySocket::new("eth0", 0, 16, 16, 2048);
        let chunk_idx = sock.umem.alloc_chunk().unwrap();
        assert!(sock.tx_packet(chunk_idx, 512));
        assert_eq!(sock.tx_packets, 1);
        let cqe = sock.cq.consume().unwrap();
        assert_eq!(cqe.result, 512);
    }
}
