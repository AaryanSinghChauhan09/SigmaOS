// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/memory/sigma_oom.rs — Out-of-Memory Killer
// Language: Rust #![no_std]
// Pattern: OOP via OomKiller struct

#![no_std]

pub const MAX_CANDIDATES: usize = 64;

#[derive(Clone, Copy)]
pub struct OomCandidate {
    pub pid:      u32,
    pub ppid:     u32,
    pub rss_kb:   u64,   // resident set size in KB
    pub score:    i32,   // oom_score: higher = kill first
    pub unkillable: bool, // true for kernel threads, init
}

impl OomCandidate {
    /// Compute OOM score: RSS-weighted, boosted if recently forked
    pub fn compute_score(rss_kb: u64, uid: u32, adj: i32) -> i32 {
        // Base score: log2(rss_kb) mapped to 0..1000
        let mut score = 0i32;
        let mut r = rss_kb;
        while r > 1 { score += 1; r >>= 1; }
        score = score.saturating_mul(10); // scale to 0..640 for 64GB
        // Root processes get lower score
        if uid == 0 { score = score.saturating_sub(30); }
        // oom_score_adj in range -1000..1000
        score.saturating_add(adj).clamp(-1000, 1000)
    }
}

pub struct OomKiller {
    candidates: [OomCandidate; MAX_CANDIDATES],
    count:      usize,
}

impl OomKiller {
    pub const fn new() -> Self {
        Self {
            candidates: [OomCandidate { pid: 0, ppid: 0, rss_kb: 0,
                                        score: 0, unkillable: false };
                         MAX_CANDIDATES],
            count: 0,
        }
    }

    pub fn add_candidate(&mut self, c: OomCandidate) -> bool {
        if self.count >= MAX_CANDIDATES { return false; }
        self.candidates[self.count] = c;
        self.count += 1;
        true
    }

    pub fn clear(&mut self) { self.count = 0; }

    /// Select the best victim: highest score, not unkillable
    pub fn select_victim(&self) -> Option<u32> {
        let mut best_score = i32::MIN;
        let mut best_pid   = None;
        for c in &self.candidates[..self.count] {
            if c.unkillable { continue; }
            if c.score > best_score {
                best_score = c.score;
                best_pid   = Some(c.pid);
            }
        }
        best_pid
    }

    /// Sort candidates by score descending (insertion sort, small N)
    pub fn sort_by_score(&mut self) {
        let n = self.count;
        for i in 1..n {
            let key = self.candidates[i];
            let mut j = i;
            while j > 0 && self.candidates[j-1].score < key.score {
                self.candidates[j] = self.candidates[j-1];
                j -= 1;
            }
            self.candidates[j] = key;
        }
    }

    pub fn top_victims(&self, out: &mut [u32], limit: usize) -> usize {
        let mut n = 0;
        for c in &self.candidates[..self.count] {
            if n >= limit { break; }
            if !c.unkillable { out[n] = c.pid; n += 1; }
        }
        n
    }
}
