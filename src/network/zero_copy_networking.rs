//! SigmaOS Sovereign Zero-Copy Networking
//! Implements Linux XDP (eXpress Data Path) + io_uring-style zero-copy networking
//! in 100% safe Rust with no external dependencies.
//!
//! Inspired by:
//!   - Linux XDP (AF_XDP sockets, Linux 4.18+)
//!   - Linux io_uring (Linux 5.1+)
//!   - FreeBSD sendfile(2) zero-copy send
//!   - FreeBSD UMEM / netmap zero-copy receive

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── UMEM — Unified Memory Region (mirrors AF_XDP umem) ──────────────────────

/// A fixed-size memory chunk descriptor in the UMEM pool.
#[derive(Debug, Clone)]
pub struct UmemChunk {
    pub addr: u64,   // offset within UMEM region
    pub len: u32,    // actual data length
    pub headroom: u16,
    pub in_use: bool,
}

impl UmemChunk {
    pub fn new(addr: u64, max_len: u32) -> Self {
        UmemChunk { addr, len: max_len, headroom: 256, in_use: false }
    }
}

/// UMEM pool — mirrors xsk_umem in Linux AF_XDP
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

    pub fn free_chunk(&mut self, idx: usize) -> bool {
        if idx >= self.chunks.len() { return false; }
        if !self.chunks[idx].in_use { return false; }
        self.chunks[idx].in_use = false;
        self.free_count = self.free_count.saturating_add(1);
        self.free_total = self.free_total.saturating_add(1);
        true
    }

    pub fn utilization_pct(&self) -> u32 {
        if self.total_chunks == 0 { return 0; }
        let used = self.total_chunks - self.free_count;
        (used * 100) / self.total_chunks
    }
}

// ─── Ring Descriptor (mirrors XDP fill/completion/rx/tx rings) ────────────────

pub struct PacketRingDescriptor {
    pub chunk_idx: usize,
    pub data_offset: u32,
    pub data_len: u32,
    pub flags: u32,
}

pub struct XdpRing {
    pub entries: Vec<PacketRingDescriptor>,
    pub capacity: usize,
    pub producer: usize,
    pub consumer: usize,
    pub packets_processed: u64,
    pub drops: u64,
}

impl XdpRing {
    pub fn new(capacity: usize) -> Self {
        XdpRing {
            entries: Vec::new(),
            capacity,
            producer: 0,
            consumer: 0,
            packets_processed: 0,
            drops: 0,
        }
    }

    pub fn enqueue(&mut self, desc: PacketRingDescriptor) -> bool {
        let used = self.producer.wrapping_sub(self.consumer);
        if used >= self.capacity {
            self.drops = self.drops.saturating_add(1);
            return false;
        }
        self.entries.push(desc);
        self.producer = self.producer.wrapping_add(1);
        true
    }

    pub fn dequeue(&mut self) -> Option<PacketRingDescriptor> {
        if self.producer == self.consumer { return None; }
        if self.entries.is_empty() { return None; }
        self.consumer = self.consumer.wrapping_add(1);
        self.packets_processed = self.packets_processed.saturating_add(1);
        // Drain from front (FIFO)
        if !self.entries.is_empty() {
            Some(self.entries.remove(0))
        } else { None }
    }

    pub fn available(&self) -> usize {
        self.producer.wrapping_sub(self.consumer)
    }
}

// ─── XDP Actions (mirrors XDP_PASS, XDP_DROP, XDP_TX, XDP_REDIRECT) ──────────

#[derive(Debug, Clone, PartialEq)]
pub enum XdpAction {
    Pass,        // XDP_PASS: pass packet up the stack
    Drop,        // XDP_DROP: drop at NIC driver level
    Tx,          // XDP_TX: reflect/bounce back out the same interface
    Redirect,    // XDP_REDIRECT: send to another queue or interface
    Aborted,     // XDP_ABORTED: error in XDP program
}

// ─── io_uring-style Completion Queue Entry ────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IoCompletionEntry {
    pub user_data: u64,
    pub result: i32,  // bytes transferred or -errno
    pub flags: u32,
}

pub struct IoCompletionQueue {
    pub entries: Vec<IoCompletionEntry>,
    pub capacity: usize,
    pub head: usize,
    pub tail: usize,
    pub total_completed: u64,
}

impl IoCompletionQueue {
    pub fn new(capacity: usize) -> Self {
        IoCompletionQueue {
            entries: Vec::new(),
            capacity,
            head: 0,
            tail: 0,
            total_completed: 0,
        }
    }

    pub fn post_completion(&mut self, user_data: u64, result: i32) -> bool {
        if self.entries.len() >= self.capacity { return false; }
        self.entries.push(IoCompletionEntry { user_data, result, flags: 0 });
        self.tail = self.tail.wrapping_add(1);
        self.total_completed = self.total_completed.saturating_add(1);
        true
    }

    pub fn consume(&mut self) -> Option<IoCompletionEntry> {
        if self.entries.is_empty() { return None; }
        self.head = self.head.wrapping_add(1);
        Some(self.entries.remove(0))
    }

    pub fn pending_count(&self) -> usize { self.entries.len() }
}

// ─── Zero-Copy Socket (AF_XDP-style) ─────────────────────────────────────────

pub struct SovereignZeroCopySocket {
    pub queue_id: u32,
    pub ifname: String,
    pub umem: UmemPool,
    pub rx_ring: XdpRing,
    pub tx_ring: XdpRing,
    pub fill_ring: XdpRing,   // kernel fills with rx descriptors
    pub completion_ring: XdpRing, // kernel notifies tx completions
    pub cq: IoCompletionQueue,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

impl SovereignZeroCopySocket {
    pub fn new(ifname: &str, queue_id: u32, umem_chunks: u32, ring_size: usize) -> Self {
        SovereignZeroCopySocket {
            queue_id,
            ifname: ifname.to_string(),
            umem: UmemPool::new(umem_chunks, 4096),
            rx_ring: XdpRing::new(ring_size),
            tx_ring: XdpRing::new(ring_size),
            fill_ring: XdpRing::new(ring_size),
            completion_ring: XdpRing::new(ring_size),
            cq: IoCompletionQueue::new(ring_size),
            rx_packets: 0,
            tx_packets: 0,
            rx_bytes: 0,
            tx_bytes: 0,
        }
    }

    /// Simulate receiving a packet zero-copy from NIC DMA region.
    pub fn rx_packet(&mut self, len: u32) -> Option<usize> {
        let chunk_idx = self.umem.alloc_chunk()?;
        if let Some(chunk) = self.umem.chunks.get_mut(chunk_idx) {
            chunk.len = len;
        }
        let desc = PacketRingDescriptor {
            chunk_idx,
            data_offset: 256, // past headroom
            data_len: len,
            flags: 0,
        };
        if self.rx_ring.enqueue(desc) {
            self.rx_packets = self.rx_packets.saturating_add(1);
            self.rx_bytes   = self.rx_bytes.saturating_add(len as u64);
            Some(chunk_idx)
        } else {
            self.umem.free_chunk(chunk_idx);
            None
        }
    }

    /// Process received packet — apply XDP action.
    pub fn process_rx(&mut self) -> Option<XdpAction> {
        let desc = self.rx_ring.dequeue()?;
        // Example: drop packets < 14 bytes (less than Ethernet header)
        let action = if desc.data_len < 14 {
            XdpAction::Drop
        } else {
            XdpAction::Pass
        };
        if action == XdpAction::Drop {
            self.umem.free_chunk(desc.chunk_idx);
        }
        Some(action)
    }

    /// Zero-copy transmit a chunk.
    pub fn tx_packet(&mut self, chunk_idx: usize, len: u32) -> bool {
        let desc = PacketRingDescriptor { chunk_idx, data_offset: 256, data_len: len, flags: 0 };
        if self.tx_ring.enqueue(desc) {
            self.tx_packets = self.tx_packets.saturating_add(1);
            self.tx_bytes   = self.tx_bytes.saturating_add(len as u64);
            // Post completion immediately (simulate NIC DMA done)
            self.cq.post_completion(chunk_idx as u64, len as i32);
            true
        } else { false }
    }

    pub fn stats_summary(&self) -> String {
        let mut s = String::from("ZeroCopySocket[");
        s.push_str(&self.ifname);
        s.push_str("] rx_pkts=");
        s.push_str(&self.rx_packets.to_string());
        s.push_str(" tx_pkts=");
        s.push_str(&self.tx_packets.to_string());
        s.push_str(" rx_bytes=");
        s.push_str(&self.rx_bytes.to_string());
        s.push_str(" tx_bytes=");
        s.push_str(&self.tx_bytes.to_string());
        s.push_str(" umem_util=");
        s.push_str(&self.umem.utilization_pct().to_string());
        s.push('%');
        s
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_umem_pool_alloc_free() {
        let mut pool = UmemPool::new(8, 4096);
        assert_eq!(pool.free_count, 8);
        let idx1 = pool.alloc_chunk().unwrap();
        let idx2 = pool.alloc_chunk().unwrap();
        assert_eq!(pool.free_count, 6);
        assert!(pool.free_chunk(idx1));
        assert_eq!(pool.free_count, 7);
        assert!(!pool.free_chunk(idx1)); // double-free
        let _ = idx2;
    }

    #[test]
    fn test_xdp_ring_enqueue_dequeue() {
        let mut ring = XdpRing::new(4);
        for i in 0..4 {
            ring.enqueue(PacketRingDescriptor { chunk_idx: i, data_offset: 256, data_len: 1500, flags: 0 });
        }
        // Ring full — should drop
        assert!(!ring.enqueue(PacketRingDescriptor { chunk_idx: 99, data_offset: 0, data_len: 1, flags: 0 }));
        assert_eq!(ring.drops, 1);
        let d = ring.dequeue().unwrap();
        assert_eq!(d.chunk_idx, 0);
        assert_eq!(ring.packets_processed, 1);
    }

    #[test]
    fn test_zero_copy_socket_rx() {
        let mut sock = SovereignZeroCopySocket::new("eth0", 0, 32, 16);
        let chunk = sock.rx_packet(1500).unwrap();
        assert!(chunk < 32);
        assert_eq!(sock.rx_packets, 1);
        let action = sock.process_rx().unwrap();
        assert_eq!(action, XdpAction::Pass);
    }

    #[test]
    fn test_xdp_drop_small_packets() {
        let mut sock = SovereignZeroCopySocket::new("eth0", 0, 32, 16);
        sock.rx_packet(8); // 8 bytes < 14 byte Ethernet header — should be dropped
        let action = sock.process_rx().unwrap();
        assert_eq!(action, XdpAction::Drop);
    }

    #[test]
    fn test_zero_copy_tx() {
        let mut sock = SovereignZeroCopySocket::new("eth0", 0, 32, 16);
        let chunk = sock.umem.alloc_chunk().unwrap();
        assert!(sock.tx_packet(chunk, 64));
        assert_eq!(sock.tx_packets, 1);
        assert_eq!(sock.cq.pending_count(), 1);
        let cqe = sock.cq.consume().unwrap();
        assert_eq!(cqe.result, 64);
    }

    #[test]
    fn test_io_completion_queue() {
        let mut cq = IoCompletionQueue::new(4);
        assert!(cq.post_completion(1001, 512));
        assert!(cq.post_completion(1002, -11)); // EAGAIN
        let e = cq.consume().unwrap();
        assert_eq!(e.user_data, 1001);
        assert_eq!(e.result, 512);
        assert_eq!(cq.total_completed, 2);
    }
}
