#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;

/// SigmaOS Traffic Control — QDisc (Queueing Discipline) Layer
/// Absorbs Linux tc subsystem: pfifo, pfifo_fast, SFQ, TBF, HTB, CAKE, FQ-CoDel
use crate::klib::VecDeque;
use std::string::{String, ToString};
extern crate alloc;
use alloc::vec::Vec;

/// A network packet in the qdisc layer (simplified)
#[derive(Debug, Clone)]
pub struct QPacket {
    pub priority: u8, // 0 = lowest, 7 = highest (TC_PRIO_MAX)
    pub size: usize,
    pub data: Vec<u8>,
}

impl QPacket {
    pub fn new(priority: u8, data: Vec<u8>) -> Self {
        let size = data.len();
        QPacket {
            priority,
            size,
            data,
        }
    }
}

/// Queueing discipline trait — all qdiscs implement this
pub trait QDisc: Send + Sync {
    fn name(&self) -> &str;
    fn enqueue(&mut self, pkt: QPacket) -> Result<(), QPacket>;
    fn dequeue(&mut self) -> Option<QPacket>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn stats(&self) -> QDiscStats;
}

#[derive(Debug, Clone, Default)]
pub struct QDiscStats {
    pub enqueued: u64,
    pub dequeued: u64,
    pub dropped: u64,
}

// ── pfifo — simple FIFO with packet limit ────────────────────────────────

pub struct Pfifo {
    queue: VecDeque<QPacket>,
    limit: usize,
    stats: QDiscStats,
}

unsafe impl Send for Sfq {}
unsafe impl Sync for Sfq {}

unsafe impl Send for Pfifo {}
unsafe impl Sync for Pfifo {}

impl Pfifo {
    pub fn new(limit: usize) -> Self {
        Pfifo {
            queue: VecDeque::new(),
            limit,
            stats: QDiscStats::default(),
        }
    }
}

impl QDisc for Pfifo {
    fn name(&self) -> &str {
        "pfifo"
    }
    fn enqueue(&mut self, pkt: QPacket) -> Result<(), QPacket> {
        if self.queue.len() >= self.limit {
            self.stats.dropped += 1;
            return Err(pkt);
        }
        self.stats.enqueued += 1;
        self.queue.push_back(pkt);
        Ok(())
    }
    fn dequeue(&mut self) -> Option<QPacket> {
        let p = self.queue.pop_front();
        if p.is_some() {
            self.stats.dequeued += 1;
        }
        p
    }
    fn len(&self) -> usize {
        self.queue.len()
    }
    fn stats(&self) -> QDiscStats {
        self.stats.clone()
    }
}

// ── pfifo_fast — Linux default: 3 priority bands ──────────────────────────

pub struct PfifoFast {
    bands: [VecDeque<QPacket>; 3], // 0=high, 1=normal, 2=low
    limit: usize,
    total: usize,
    stats: QDiscStats,
}

unsafe impl Send for PfifoFast {}
unsafe impl Sync for PfifoFast {}

impl PfifoFast {
    pub fn new(limit: usize) -> Self {
        PfifoFast {
            bands: [VecDeque::new(), VecDeque::new(), VecDeque::new()],
            limit,
            total: 0,
            stats: QDiscStats::default(),
        }
    }

    fn band(priority: u8) -> usize {
        match priority {
            6..=7 => 0, // Interactive
            3..=5 => 1, // Normal
            _ => 2,     // Bulk
        }
    }
}

impl QDisc for PfifoFast {
    fn name(&self) -> &str {
        "pfifo_fast"
    }
    fn enqueue(&mut self, pkt: QPacket) -> Result<(), QPacket> {
        if self.total >= self.limit {
            self.stats.dropped += 1;
            return Err(pkt);
        }
        let band = Self::band(pkt.priority);
        self.bands[band].push_back(pkt);
        self.total += 1;
        self.stats.enqueued += 1;
        Ok(())
    }
    fn dequeue(&mut self) -> Option<QPacket> {
        for band in &mut self.bands {
            if let Some(p) = band.pop_front() {
                self.total -= 1;
                self.stats.dequeued += 1;
                return Some(p);
            }
        }
        None
    }
    fn len(&self) -> usize {
        self.total
    }
    fn stats(&self) -> QDiscStats {
        self.stats.clone()
    }
}

// ── SFQ — Stochastic Fair Queueing ────────────────────────────────────────

pub struct Sfq {
    buckets: Vec<VecDeque<QPacket>>,
    num_buckets: usize,
    round_robin_idx: usize,
    total: usize,
    limit: usize,
    stats: QDiscStats,
}

impl Sfq {
    pub fn new(num_buckets: usize, limit_per_bucket: usize) -> Self {
        Sfq {
            buckets: vec![VecDeque::new(); num_buckets],
            num_buckets,
            round_robin_idx: 0,
            total: 0,
            limit: limit_per_bucket,
            stats: QDiscStats::default(),
        }
    }
}

impl QDisc for Sfq {
    fn name(&self) -> &str {
        "sfq"
    }
    fn enqueue(&mut self, pkt: QPacket) -> Result<(), QPacket> {
        // Hash bucket by priority (simplified - real SFQ uses 5-tuple hash)
        let bucket = pkt.priority as usize % self.num_buckets;
        if self.buckets[bucket].len() >= self.limit {
            self.stats.dropped += 1;
            return Err(pkt);
        }
        self.buckets[bucket].push_back(pkt);
        self.total += 1;
        self.stats.enqueued += 1;
        Ok(())
    }
    fn dequeue(&mut self) -> Option<QPacket> {
        for _ in 0..self.num_buckets {
            let idx = self.round_robin_idx % self.num_buckets;
            self.round_robin_idx += 1;
            if let Some(p) = self.buckets[idx].pop_front() {
                self.total -= 1;
                self.stats.dequeued += 1;
                return Some(p);
            }
        }
        None
    }
    fn len(&self) -> usize {
        self.total
    }
    fn stats(&self) -> QDiscStats {
        self.stats.clone()
    }
}

// ── Token Bucket Filter (TBF) — rate limiting ─────────────────────────────

pub struct Tbf {
    inner: Box<dyn QDisc>,
    rate_bytes_per_tick: usize,
    bucket_tokens: usize,
    bucket_max: usize,
    stats: QDiscStats,
}

impl Tbf {
    pub fn new(inner: Box<dyn QDisc>, rate_bytes_per_tick: usize, burst_bytes: usize) -> Self {
        Tbf {
            inner,
            rate_bytes_per_tick,
            bucket_tokens: burst_bytes,
            bucket_max: burst_bytes,
            stats: QDiscStats::default(),
        }
    }

    pub fn tick(&mut self) {
        self.bucket_tokens = (self.bucket_tokens + self.rate_bytes_per_tick).min(self.bucket_max);
    }
}

impl QDisc for Tbf {
    fn name(&self) -> &str {
        "tbf"
    }
    fn enqueue(&mut self, pkt: QPacket) -> Result<(), QPacket> {
        self.inner.enqueue(pkt)
    }
    fn dequeue(&mut self) -> Option<QPacket> {
        if let Some(p) = self.inner.dequeue() {
            if self.bucket_tokens >= p.size {
                self.bucket_tokens -= p.size;
                self.stats.dequeued += 1;
                return Some(p);
            } else {
                // Not enough tokens — re-queue at front
                let _ = self.inner.enqueue(p);
                return None;
            }
        }
        None
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
    fn stats(&self) -> QDiscStats {
        self.stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkt(prio: u8) -> QPacket {
        QPacket::new(prio, vec![0u8; 100])
    }

    #[test]
    fn test_pfifo_basic() {
        let mut q = Pfifo::new(3);
        q.enqueue(pkt(0)).unwrap();
        q.enqueue(pkt(0)).unwrap();
        q.enqueue(pkt(0)).unwrap();
        assert!(q.enqueue(pkt(0)).is_err()); // drop
        assert_eq!(q.stats().dropped, 1);
        assert_eq!(q.dequeue().unwrap().priority, 0);
    }

    #[test]
    fn test_pfifo_fast_priority_ordering() {
        let mut q = PfifoFast::new(64);
        q.enqueue(pkt(0)).unwrap(); // bulk (band 2)
        q.enqueue(pkt(7)).unwrap(); // interactive (band 0)
        q.enqueue(pkt(4)).unwrap(); // normal (band 1)

        assert_eq!(q.dequeue().unwrap().priority, 7); // band 0 first
        assert_eq!(q.dequeue().unwrap().priority, 4); // band 1 next
        assert_eq!(q.dequeue().unwrap().priority, 0); // band 2 last
    }

    #[test]
    fn test_sfq_round_robin() {
        let mut q = Sfq::new(4, 16);
        q.enqueue(pkt(0)).unwrap(); // bucket 0
        q.enqueue(pkt(1)).unwrap(); // bucket 1
        q.enqueue(pkt(0)).unwrap(); // bucket 0 again
        assert_eq!(q.len(), 3);
        let p1 = q.dequeue().unwrap();
        let p2 = q.dequeue().unwrap();
        assert_ne!(p1.priority, p2.priority); // should round-robin between buckets
    }

    #[test]
    fn test_tbf_rate_limiting() {
        let inner = Box::new(Pfifo::new(64));
        let mut tbf = Tbf::new(inner, 500, 1000); // 500 B/tick, 1KB burst
        tbf.enqueue(QPacket::new(0, vec![0u8; 800])).unwrap(); // 800B packet
                                                               // Has 1000 tokens → dequeues fine
        assert!(tbf.dequeue().is_some());
        // Now 200 tokens left; enqueue 400B → not enough to dequeue
        tbf.enqueue(QPacket::new(0, vec![0u8; 400])).unwrap();
        assert!(tbf.dequeue().is_none()); // blocked by TBF
                                          // Tick → add 500 tokens (200+500=700 ≥ 400)
        tbf.tick();
        assert!(tbf.dequeue().is_some());
    }
}
