#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
#![allow(clippy::new_without_default)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

// SigmaOS Sovereign Traffic Control (tc) Qdiscs
// Implements Linux Traffic Control queuing disciplines in 100% safe Rust.
//
// Inspired by Linux tc(8) qdiscs:
//   - HTB (Hierarchical Token Bucket) — bandwidth sharing/limiting
//   - FQ-CoDel (Fair Queuing CoDel) — latency control
//   - HFSC (Hierarchical Fair-Service Curve) — latency + bandwidth
//   - TBF (Token Bucket Filter) — simple rate limiting
//   - PRIO (Priority scheduler) — strict priority queues


#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── Packet descriptor ────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Packet {
    pub len: u32,
    pub priority: u8,    // 0=highest
    pub dscp: u8,        // Differentiated Services Code Point
    pub enqueue_tick: u64,
    pub flow_id: u32,    // for per-flow fair queuing
}

impl Packet {
    pub fn new(len: u32, priority: u8, flow_id: u32, tick: u64) -> Self {
        Packet { len, priority, dscp: priority << 2, enqueue_tick: tick, flow_id }
    }
}

// ─── TBF — Token Bucket Filter ────────────────────────────────────────────────

pub struct TbfQdisc {
    pub rate_bps: u64,       // bytes per second
    pub burst_bytes: u64,    // bucket capacity
    pub tokens: u64,         // current token count (bytes)
    pub last_refill_tick: u64,
    pub ns_per_byte: u64,    // nanoseconds per byte at rate
    pub enqueued: u64,
    pub dropped: u64,
    pub queue: Vec<Packet>,
    pub queue_limit: usize,
}

impl TbfQdisc {
    pub fn new(rate_bps: u64, burst_bytes: u64, queue_limit: usize) -> Self {
        let ns_per_byte = if rate_bps > 0 { 1_000_000_000 / rate_bps } else { 0 };
        TbfQdisc {
            rate_bps,
            burst_bytes,
            tokens: burst_bytes,
            last_refill_tick: 0,
            ns_per_byte,
            enqueued: 0,
            dropped: 0,
            queue: Vec::new(),
            queue_limit,
        }
    }

    /// Advance simulation clock by `ns` nanoseconds, refilling tokens.
    pub fn tick(&mut self, now_ns: u64) {
        if now_ns <= self.last_refill_tick { return; }
        let elapsed = now_ns - self.last_refill_tick;
        let new_tokens = if self.ns_per_byte > 0 { elapsed / self.ns_per_byte } else { 0 };
        self.tokens = (self.tokens + new_tokens).min(self.burst_bytes);
        self.last_refill_tick = now_ns;
    }

    pub fn enqueue(&mut self, pkt: Packet, now_ns: u64) -> bool {
        self.tick(now_ns);
        if self.queue.len() >= self.queue_limit {
            self.dropped = self.dropped.saturating_add(1);
            return false;
        }
        self.queue.push(pkt);
        self.enqueued = self.enqueued.saturating_add(1);
        true
    }

    /// Dequeue if tokens allow.
    pub fn dequeue(&mut self) -> Option<Packet> {
        if self.queue.is_empty() { return None; }
        let pkt_len = self.queue[0].len as u64;
        if self.tokens < pkt_len {
            return None; // Rate-limited — wait for tokens
        }
        self.tokens -= pkt_len;
        Some(self.queue.remove(0))
    }

    pub fn utilization_pct(&self) -> u32 {
        if self.burst_bytes == 0 { return 0; }
        let used = self.burst_bytes - self.tokens;
        ((used * 100) / self.burst_bytes) as u32
    }
}

// ─── PRIO — Strict Priority Scheduler ────────────────────────────────────────

pub struct PrioQdisc {
    pub bands: u8,
    pub queues: Vec<Vec<Packet>>, // one queue per band
    pub enqueued: u64,
    pub dequeued: u64,
    pub per_band_count: Vec<u64>,
}

impl PrioQdisc {
    pub fn new(bands: u8) -> Self {
        let queues = (0..bands).map(|_| Vec::new()).collect();
        let per_band_count = vec![0u64; bands as usize];
        PrioQdisc { bands, queues, enqueued: 0, dequeued: 0, per_band_count }
    }

    pub fn enqueue(&mut self, pkt: Packet) -> bool {
        let band = (pkt.priority as usize).min(self.queues.len().saturating_sub(1));
        self.queues[band].push(pkt);
        self.enqueued = self.enqueued.saturating_add(1);
        if band < self.per_band_count.len() {
            self.per_band_count[band] = self.per_band_count[band].saturating_add(1);
        }
        true
    }

    /// Dequeue from highest-priority (lowest index) non-empty band.
    pub fn dequeue(&mut self) -> Option<Packet> {
        for band in &mut self.queues {
            if !band.is_empty() {
                self.dequeued = self.dequeued.saturating_add(1);
                return Some(band.remove(0));
            }
        }
        None
    }

    pub fn total_queued(&self) -> usize {
        self.queues.iter().map(|q| q.len()).sum()
    }
}

// ─── HTB — Hierarchical Token Bucket (simplified) ────────────────────────────

#[derive(Debug, Clone)]
pub struct HtbClass {
    pub id: u32,
    pub parent_id: u32,
    pub rate_bps: u64,
    pub ceil_bps: u64,
    pub burst_bytes: u64,
    pub tokens: u64,
    pub ctokens: u64, // ceiling tokens
    pub prio: u8,
    pub queue: Vec<Packet>,
    pub enqueued: u64,
    pub dequeued: u64,
    pub lended: u64,   // bytes borrowed from parent
    pub dropped: u64,
}

impl HtbClass {
    pub fn new(id: u32, parent_id: u32, rate_bps: u64, ceil_bps: u64, prio: u8) -> Self {
        let burst = rate_bps / 8; // 125ms burst
        HtbClass {
            id, parent_id, rate_bps, ceil_bps,
            burst_bytes: burst,
            tokens: burst,
            ctokens: ceil_bps / 8,
            prio,
            queue: Vec::new(),
            enqueued: 0, dequeued: 0, lended: 0, dropped: 0,
        }
    }

    pub fn refill_tokens(&mut self, elapsed_ns: u64) {
        if self.rate_bps > 0 {
            let new_tok = (elapsed_ns / 1_000_000_000) * self.rate_bps / 8;
            self.tokens = (self.tokens + new_tok).min(self.burst_bytes);
        }
        if self.ceil_bps > 0 {
            let ceil_rate = self.ceil_bps / 8;
            let new_ctok = (elapsed_ns / 1_000_000_000) * ceil_rate;
            let ceil_burst = self.ceil_bps / 8;
            self.ctokens = (self.ctokens + new_ctok).min(ceil_burst);
        }
    }

    pub fn enqueue_packet(&mut self, pkt: Packet) {
        self.queue.push(pkt);
        self.enqueued = self.enqueued.saturating_add(1);
    }

    pub fn try_dequeue(&mut self) -> Option<Packet> {
        if self.queue.is_empty() { return None; }
        let len = self.queue[0].len as u64;
        if self.tokens >= len {
            // In-rate: use own tokens
            self.tokens -= len;
            self.dequeued = self.dequeued.saturating_add(1);
            Some(self.queue.remove(0))
        } else if self.ctokens >= len {
            // Over-rate but under ceil: borrow (lend)
            self.ctokens -= len;
            self.lended = self.lended.saturating_add(len);
            self.dequeued = self.dequeued.saturating_add(1);
            Some(self.queue.remove(0))
        } else {
            None // Blocked at ceil
        }
    }
}

pub struct HtbQdisc {
    pub classes: Vec<HtbClass>,
    pub r2q: u32, // rate-to-quantum divisor (default 10)
    pub default_class_id: u32,
    pub last_tick_ns: u64,
}

impl HtbQdisc {
    pub fn new(default_class_id: u32) -> Self {
        HtbQdisc { classes: Vec::new(), r2q: 10, default_class_id, last_tick_ns: 0 }
    }

    pub fn add_class(&mut self, class: HtbClass) {
        self.classes.push(class);
    }

    pub fn tick(&mut self, now_ns: u64) {
        let elapsed = now_ns.saturating_sub(self.last_tick_ns);
        for cls in &mut self.classes {
            cls.refill_tokens(elapsed);
        }
        self.last_tick_ns = now_ns;
    }

    pub fn enqueue(&mut self, pkt: Packet, class_id: u32) -> bool {
        if let Some(cls) = self.classes.iter_mut().find(|c| c.id == class_id) {
            cls.enqueue_packet(pkt);
            true
        } else { false }
    }

    /// Dequeue across all classes in priority order.
    pub fn dequeue(&mut self) -> Option<Packet> {
        // Sort by prio then try each class
        let ids: Vec<(u32, u8)> = self.classes.iter().map(|c| (c.id, c.prio)).collect();
        let mut sorted_ids = ids;
        sorted_ids.sort_by_key(|&(_, p)| p);
        for (id, _) in sorted_ids {
            if let Some(cls) = self.classes.iter_mut().find(|c| c.id == id) {
                if let Some(pkt) = cls.try_dequeue() {
                    return Some(pkt);
                }
            }
        }
        None
    }

    pub fn total_queued(&self) -> usize {
        self.classes.iter().map(|c| c.queue.len()).sum()
    }
}

// ─── FQ-CoDel (Fair Queuing Controlled Delay) — simplified ───────────────────

pub struct FqCodelQdisc {
    pub target_delay_ns: u64,  // target queue latency (default 5ms)
    pub interval_ns: u64,      // CoDel interval (default 100ms)
    pub quantum: u32,          // FQ quantum in bytes
    pub flows: Vec<Vec<Packet>>,
    pub flow_count: usize,
    pub drop_count: u64,
    pub ecn_marks: u64,
    pub round_robin_idx: usize,
}

impl FqCodelQdisc {
    pub fn new(flow_count: usize) -> Self {
        let flows = (0..flow_count).map(|_| Vec::new()).collect();
        FqCodelQdisc {
            target_delay_ns: 5_000_000,    // 5ms
            interval_ns: 100_000_000,       // 100ms
            quantum: 1514,
            flows,
            flow_count,
            drop_count: 0,
            ecn_marks: 0,
            round_robin_idx: 0,
        }
    }

    pub fn enqueue(&mut self, pkt: Packet) -> bool {
        if self.flow_count == 0 { return false; }
        let flow_idx = (pkt.flow_id as usize) % self.flow_count;
        // CoDel: if sojourn time > target, mark/drop
        // (simplified: if flow queue is deep, mark ECN)
        if self.flows[flow_idx].len() > 64 {
            self.ecn_marks = self.ecn_marks.saturating_add(1);
        }
        if self.flows[flow_idx].len() > 128 {
            self.drop_count = self.drop_count.saturating_add(1);
            return false; // Drop tail
        }
        self.flows[flow_idx].push(pkt);
        true
    }

    /// Round-robin dequeue across non-empty flows.
    pub fn dequeue(&mut self) -> Option<Packet> {
        if self.flow_count == 0 { return None; }
        let start = self.round_robin_idx;
        for i in 0..self.flow_count {
            let idx = (start + i) % self.flow_count;
            if !self.flows[idx].is_empty() {
                self.round_robin_idx = (idx + 1) % self.flow_count;
                return Some(self.flows[idx].remove(0));
            }
        }
        None
    }

    pub fn total_queued(&self) -> usize {
        self.flows.iter().map(|f| f.len()).sum()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tbf_rate_limiting() {
        // 1 MB/s = 1_000_000 bytes/s, 100KB burst
        let mut tbf = TbfQdisc::new(1_000_000, 100_000, 64);
        let pkt = Packet::new(50_000, 0, 1, 0);
        assert!(tbf.enqueue(pkt.clone(), 0));
        // First dequeue: uses tokens
        let d = tbf.dequeue();
        assert!(d.is_some());
        // After consuming 50KB of 100KB burst, 50KB remain
        assert!(tbf.tokens < 100_000);
    }

    #[test]
    fn test_tbf_token_refill() {
        let mut tbf = TbfQdisc::new(1_000_000, 100_000, 64);
        tbf.tokens = 0; // drain bucket
        tbf.last_refill_tick = 0;
        tbf.tick(2_000_000_000); // 2 seconds elapsed → +2MB tokens, capped at 100KB
        assert_eq!(tbf.tokens, 100_000); // Capped at burst
    }

    #[test]
    fn test_prio_strict_ordering() {
        let mut prio = PrioQdisc::new(3);
        prio.enqueue(Packet::new(100, 2, 1, 0)); // low priority
        prio.enqueue(Packet::new(100, 0, 2, 0)); // highest priority
        prio.enqueue(Packet::new(100, 1, 3, 0)); // medium
        // Should dequeue highest priority first
        let d = prio.dequeue().unwrap();
        assert_eq!(d.priority, 0); // highest priority (band 0)
    }

    #[test]
    fn test_htb_class_rate_ceiling() {
        let mut cls = HtbClass::new(1, 0, 1_000_000, 2_000_000, 0);
        cls.tokens = 0; // drain within-rate tokens
        cls.ctokens = 2_000_000 / 8; // ceiling tokens available
        let pkt = Packet::new(1000, 0, 1, 0);
        cls.enqueue_packet(pkt);
        // Should dequeue using ceiling tokens
        let d = cls.try_dequeue();
        assert!(d.is_some());
        assert!(cls.lended > 0); // borrowed from ceiling
    }

    #[test]
    fn test_fq_codel_fair_queuing() {
        let mut fq = FqCodelQdisc::new(8); // 8 flows
        // Send packets for two flows
        for i in 0..4 {
            fq.enqueue(Packet::new(100, 0, 1, i)); // flow 1
            fq.enqueue(Packet::new(100, 0, 2, i)); // flow 2
        }
        // Dequeue should alternate between flows (round-robin)
        let d1 = fq.dequeue().unwrap();
        let d2 = fq.dequeue().unwrap();
        assert_ne!(d1.flow_id, d2.flow_id); // Different flows
    }

    #[test]
    fn test_fq_codel_ecn_marking() {
        let mut fq = FqCodelQdisc::new(4);
        // Fill one flow beyond ECN threshold (>64 packets)
        for i in 0..70 {
            fq.enqueue(Packet::new(100, 0, 0, i)); // all same flow
        }
        assert!(fq.ecn_marks > 0); // ECN marks triggered
    }
}
