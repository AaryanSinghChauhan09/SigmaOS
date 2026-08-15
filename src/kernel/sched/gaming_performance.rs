//! Gaming Performance & Advanced Distro SMP Extensions for SigmaOS
//! Inspired by SteamOS GameMode, Pop!_OS System76 Scheduler, CachyOS UKSM, and DragonFly BSD LWKT.

use crate::klib::Vec;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// CPU Power Governor Profiles for Dynamic Performance Switching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerGovernor {
    Powersave,
    OnDemand,
    Schedutil,
    Performance,
}

/// Sovereign GameMode Engine (SteamOS / Pop!_OS System76 Scheduler inspired)
/// Manages real-time CPU priority boosting, GPU performance mode, and governor overrides.
pub struct SovereignGameMode {
    active: bool,
    target_pid: Option<u64>,
    previous_governor: PowerGovernor,
    current_governor: PowerGovernor,
    priority_boost: i32,
    pinned_memory_bytes: u64,
}

impl SovereignGameMode {
    pub fn new() -> Self {
        Self {
            active: false,
            target_pid: None,
            previous_governor: PowerGovernor::Schedutil,
            current_governor: PowerGovernor::Schedutil,
            priority_boost: 0,
            pinned_memory_bytes: 0,
        }
    }

    /// Activates GameMode for a specific process PID (boosting priority & setting performance governor)
    pub fn enable_for_process(&mut self, pid: u64, memory_to_pin: u64) -> bool {
        self.active = true;
        self.target_pid = Some(pid);
        self.previous_governor = self.current_governor;
        self.current_governor = PowerGovernor::Performance;
        self.priority_boost = -10; // High priority nice value
        self.pinned_memory_bytes = memory_to_pin;
        true
    }

    /// Deactivates GameMode, restoring original CPU governor and process priorities
    pub fn disable(&mut self) -> bool {
        if !self.active {
            return false;
        }
        self.active = false;
        self.target_pid = None;
        self.current_governor = self.previous_governor;
        self.priority_boost = 0;
        self.pinned_memory_bytes = 0;
        true
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn active_pid(&self) -> Option<u64> {
        self.target_pid
    }

    pub fn governor(&self) -> PowerGovernor {
        self.current_governor
    }

    pub fn priority_boost(&self) -> i32 {
        self.priority_boost
    }

    pub fn pinned_memory_bytes(&self) -> u64 {
        self.pinned_memory_bytes
    }
}

/// Lightweight Kernel Thread (LWKT) Message (DragonFly BSD LWKT inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub fn calculate_fnv1a(data: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LwktMessage {
    pub msg_id: u64,
    pub sender_cpu: usize,
    pub target_cpu: usize,
    pub payload: Vec<u8>,
    pub processed: bool,
}

/// DragonFly BSD Lightweight Kernel Threads (LWKT) Per-CPU Lockless Work Queue
pub struct DragonFlySmpQueueManager {
    cpu_count: usize,
    per_cpu_queues: Vec<Vec<LwktMessage>>,
    next_msg_id: u64,
}

impl DragonFlySmpQueueManager {
    pub fn new(cpu_count: usize) -> Self {
        let mut queues = Vec::new();
        for _ in 0..cpu_count {
            queues.push(Vec::new());
        }
        Self {
            cpu_count,
            per_cpu_queues: queues,
            next_msg_id: 1,
        }
    }

    /// Enqueues an asynchronous LWKT message to a target CPU core queue
    pub fn send_message(&mut self, sender_cpu: usize, target_cpu: usize, payload: Vec<u8>) -> Option<u64> {
        if target_cpu >= self.cpu_count || sender_cpu >= self.cpu_count {
            return None;
        }

        let msg_id = self.next_msg_id;
        self.next_msg_id += 1;

        let msg = LwktMessage {
            msg_id,
            sender_cpu,
            target_cpu,
            payload,
            processed: false,
        };

        self.per_cpu_queues[target_cpu].push(msg);
        Some(msg_id)
    }

    /// Processes pending LWKT messages for a given CPU core
    pub fn process_cpu_queue(&mut self, cpu_id: usize) -> usize {
        if cpu_id >= self.cpu_count {
            return 0;
        }

        let queue = &mut self.per_cpu_queues[cpu_id];
        let count = queue.len();
        for msg in queue.iter_mut() {
            msg.processed = true;
        }
        queue.clear();
        count
    }

    pub fn pending_messages_count(&self, cpu_id: usize) -> usize {
        if cpu_id < self.cpu_count {
            self.per_cpu_queues[cpu_id].len()
        } else {
            0
        }
    }
}

/// Ultra Kernel Samepage Merging (UKSM) Page Deduplicator (CachyOS / Linux UKSM inspired)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageFrame {
    pub page_id: u64,
    pub data: Vec<u8>,
    pub hash: u64,
    pub is_shared: bool,
    pub ref_count: usize,
}

pub struct UksmPageDeduplicator {
    page_size: usize,
    pages: Vec<PageFrame>,
    scanned_count: u64,
    merged_count: u64,
    saved_bytes: u64,
}

impl UksmPageDeduplicator {
    pub fn new(page_size: usize) -> Self {
        Self {
            page_size,
            pages: Vec::new(),
            scanned_count: 0,
            merged_count: 0,
            saved_bytes: 0,
        }
    }

    pub fn register_page(&mut self, page_id: u64, data: Vec<u8>) {
        let hash = calculate_fnv1a(&data);
        self.pages.push(PageFrame {
            page_id,
            data,
            hash,
            is_shared: false,
            ref_count: 1,
        });
        self.scanned_count += 1;
    }

    /// Scans registered pages, identifies identical content, and merges them into shared pages
    pub fn run_deduplication_pass(&mut self) -> usize {
        let mut merges = 0;
        let len = self.pages.len();

        for i in 0..len {
            if self.pages[i].is_shared && self.pages[i].ref_count > 1 {
                continue;
            }

            for j in (i + 1)..len {
                if self.pages[i].hash == self.pages[j].hash
                    && self.pages[i].data == self.pages[j].data
                    && !self.pages[j].is_shared
                {
                    self.pages[i].is_shared = true;
                    self.pages[i].ref_count += 1;

                    self.pages[j].is_shared = true;
                    self.pages[j].ref_count = 0; // Mark merged out

                    merges += 1;
                    self.merged_count += 1;
                    self.saved_bytes += self.page_size as u64;
                }
            }
        }

        merges
    }

    pub fn stats(&self) -> (u64, u64, u64) {
        (self.scanned_count, self.merged_count, self.saved_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mode() {
        let mut game_mode = SovereignGameMode::new();
        assert!(!game_mode.is_active());
        assert_eq!(game_mode.governor(), PowerGovernor::Schedutil);

        assert!(game_mode.enable_for_process(4096, 1024 * 1024 * 512));
        assert!(game_mode.is_active());
        assert_eq!(game_mode.active_pid(), Some(4096));
        assert_eq!(game_mode.governor(), PowerGovernor::Performance);
        assert_eq!(game_mode.priority_boost(), -10);
        assert_eq!(game_mode.pinned_memory_bytes(), 1024 * 1024 * 512);

        assert!(game_mode.disable());
        assert!(!game_mode.is_active());
        assert_eq!(game_mode.governor(), PowerGovernor::Schedutil);
    }

    #[test]
    fn test_dragonfly_smp_queue() {
        let mut smp = DragonFlySmpQueueManager::new(4);
        let msg_id = smp.send_message(0, 2, vec![1, 2, 3, 4]).unwrap();
        assert_eq!(msg_id, 1);
        assert_eq!(smp.pending_messages_count(2), 1);
        assert_eq!(smp.pending_messages_count(0), 0);

        let processed = smp.process_cpu_queue(2);
        assert_eq!(processed, 1);
        assert_eq!(smp.pending_messages_count(2), 0);
    }

    #[test]
    fn test_uksm_deduplicator() {
        let mut uksm = UksmPageDeduplicator::new(4096);
        uksm.register_page(100, vec![0xAA; 4096]);
        uksm.register_page(101, vec![0xAA; 4096]);
        uksm.register_page(102, vec![0xBB; 4096]);

        let merges = uksm.run_deduplication_pass();
        assert_eq!(merges, 1);

        let (scanned, merged, saved) = uksm.stats();
        assert_eq!(scanned, 3);
        assert_eq!(merged, 1);
        assert_eq!(saved, 4096);
    }
}
