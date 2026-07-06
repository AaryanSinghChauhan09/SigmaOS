// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sched/sigma_sched.rs — Composite Scheduler: MLFQ → CFS → EDF
//
// SigmaOS uses a three-tier scheduler:
//   • MLFQ  — interactive tasks (Tier 0, highest priority)
//   • CFS   — normal fair-share tasks (Tier 1)
//   • EDF   — real-time tasks with deadlines (Tier 2)
//
// Each CPU core has its own per-CPU runqueue. Work-stealing is used for load
// balancing. Thread state is captured in `SigmaThread` and the selected next
// thread is returned to the context-switch assembly stubs.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────────────────────────────────────

pub const MAX_THREADS:     usize = 1024;
pub const MAX_CPUS:        usize = 64;
pub const MLFQ_LEVELS:     usize = 8;
pub const MLFQ_TIMESLICE:  u64   = 5;   // ticks per MLFQ level-0 slot
pub const CFS_MIN_GRAN:    u64   = 1;   // minimum CFS granularity (ticks)
pub const EDF_MAX_TASKS:   usize = 64;

pub const SCHED_NORMAL: u8 = 0;
pub const SCHED_FIFO:   u8 = 1;
pub const SCHED_RR:     u8 = 2;
pub const SCHED_EDF:    u8 = 3;

// ─────────────────────────────────────────────────────────────────────────────
// Thread State
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ThreadState {
    Free,
    Ready,
    Running,
    Blocked,
    Zombie,
}

/// Saved register context for x86-64 context switch
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuContext {
    pub rsp: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,   // saved return address / entry point
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
    pub cr3: u64,   // page table root
}

impl CpuContext {
    pub const fn zero() -> Self {
        Self {
            rsp: 0, rbp: 0, rbx: 0, r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x200, cs: 0x08, ss: 0x10, cr3: 0,
        }
    }
}

/// Per-thread descriptor
#[derive(Copy, Clone)]
pub struct SigmaThread {
    pub tid:         u32,
    pub pid:         u32,
    pub state:       ThreadState,
    pub policy:      u8,          // SCHED_NORMAL / SCHED_EDF / …
    pub priority:    i32,         // nice value (–20 … 19)
    pub vruntime:    u64,         // CFS virtual runtime (ns)
    pub deadline_ns: u64,         // EDF absolute deadline (ns since boot)
    pub timeslice:   u64,         // MLFQ remaining ticks
    pub mlfq_level:  usize,       // 0 = most interactive
    pub cpu_affinity:u64,         // bitmask of allowed CPUs
    pub cpu_id:      usize,       // currently assigned CPU
    pub ctx:         CpuContext,
    pub stack_top:   u64,
    pub stack_size:  u64,
    pub name:        [u8; 32],
}

impl SigmaThread {
    pub const fn empty() -> Self {
        Self {
            tid: 0, pid: 0,
            state: ThreadState::Free,
            policy: SCHED_NORMAL,
            priority: 0,
            vruntime: 0,
            deadline_ns: u64::MAX,
            timeslice: MLFQ_TIMESLICE,
            mlfq_level: 0,
            cpu_affinity: !0u64,
            cpu_id: 0,
            ctx: CpuContext::zero(),
            stack_top: 0,
            stack_size: 0,
            name: [0u8; 32],
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MLFQ Sub-scheduler
// ─────────────────────────────────────────────────────────────────────────────

/// Circular bitmap queue per MLFQ level.
/// Each level stores up to MAX_THREADS / MLFQ_LEVELS indices.
pub struct MlfqQueue {
    pub queues:  [[u32; MAX_THREADS / MLFQ_LEVELS]; MLFQ_LEVELS],
    pub heads:   [usize; MLFQ_LEVELS],
    pub tails:   [usize; MLFQ_LEVELS],
    pub counts:  [usize; MLFQ_LEVELS],
}

impl MlfqQueue {
    pub const fn new() -> Self {
        Self {
            queues:  [[u32::MAX; MAX_THREADS / MLFQ_LEVELS]; MLFQ_LEVELS],
            heads:   [0; MLFQ_LEVELS],
            tails:   [0; MLFQ_LEVELS],
            counts:  [0; MLFQ_LEVELS],
        }
    }

    pub fn enqueue(&mut self, level: usize, tid: u32) {
        let cap = MAX_THREADS / MLFQ_LEVELS;
        if self.counts[level] >= cap { return; }
        self.queues[level][self.tails[level]] = tid;
        self.tails[level] = (self.tails[level] + 1) % cap;
        self.counts[level] += 1;
    }

    pub fn dequeue(&mut self, level: usize) -> Option<u32> {
        if self.counts[level] == 0 { return None; }
        let tid = self.queues[level][self.heads[level]];
        let cap = MAX_THREADS / MLFQ_LEVELS;
        self.heads[level] = (self.heads[level] + 1) % cap;
        self.counts[level] -= 1;
        Some(tid)
    }

    /// Pick the highest-priority non-empty MLFQ level and dequeue from it.
    pub fn pick_next(&mut self) -> Option<u32> {
        for lv in 0..MLFQ_LEVELS {
            if let Some(tid) = self.dequeue(lv) {
                return Some(tid);
            }
        }
        None
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CFS Sub-scheduler (simplified red-black tree via sorted array)
// ─────────────────────────────────────────────────────────────────────────────

pub struct CfsRunqueue {
    pub tids:   [u32; MAX_THREADS],
    pub count:  usize,
    /// Minimum vruntime in the runqueue (used for new thread placement)
    pub min_vruntime: u64,
}

impl CfsRunqueue {
    pub const fn new() -> Self {
        Self { tids: [u32::MAX; MAX_THREADS], count: 0, min_vruntime: 0 }
    }

    pub fn insert(&mut self, tid: u32) {
        if self.count >= MAX_THREADS { return; }
        self.tids[self.count] = tid;
        self.count += 1;
    }

    /// Pick thread with smallest vruntime (O(n) scan; production uses rb-tree)
    pub fn pick_next(&mut self, threads: &[SigmaThread]) -> Option<u32> {
        let mut best: Option<(usize, u64)> = None;
        for i in 0..self.count {
            let tid = self.tids[i] as usize;
            if tid >= MAX_THREADS { continue; }
            let vrt = threads[tid].vruntime;
            if best.map_or(true, |(_, bv)| vrt < bv) {
                best = Some((i, vrt));
            }
        }
        best.map(|(idx, _)| {
            let tid = self.tids[idx];
            // Remove from runqueue
            self.tids[idx] = self.tids[self.count - 1];
            self.tids[self.count - 1] = u32::MAX;
            self.count -= 1;
            tid
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EDF Sub-scheduler
// ─────────────────────────────────────────────────────────────────────────────

pub struct EdfRunqueue {
    pub tids:  [u32; EDF_MAX_TASKS],
    pub count: usize,
}

impl EdfRunqueue {
    pub const fn new() -> Self {
        Self { tids: [u32::MAX; EDF_MAX_TASKS], count: 0 }
    }

    pub fn insert(&mut self, tid: u32) {
        if self.count >= EDF_MAX_TASKS { return; }
        self.tids[self.count] = tid;
        self.count += 1;
    }

    /// Pick thread with earliest deadline (O(n); production: heap)
    pub fn pick_next(&mut self, threads: &[SigmaThread]) -> Option<u32> {
        let mut best: Option<(usize, u64)> = None;
        for i in 0..self.count {
            let tid = self.tids[i] as usize;
            if tid >= MAX_THREADS { continue; }
            let dl = threads[tid].deadline_ns;
            if best.map_or(true, |(_, bd)| dl < bd) {
                best = Some((i, dl));
            }
        }
        best.map(|(idx, _)| {
            let tid = self.tids[idx];
            self.tids[idx] = self.tids[self.count - 1];
            self.tids[self.count - 1] = u32::MAX;
            self.count -= 1;
            tid
        })
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-CPU Runqueue
// ─────────────────────────────────────────────────────────────────────────────

pub struct PerCpuRunqueue {
    pub mlfq:    MlfqQueue,
    pub cfs:     CfsRunqueue,
    pub edf:     EdfRunqueue,
    pub current: u32, // TID of currently running thread (u32::MAX = idle)
}

impl PerCpuRunqueue {
    pub const fn new() -> Self {
        Self {
            mlfq:    MlfqQueue::new(),
            cfs:     CfsRunqueue::new(),
            edf:     EdfRunqueue::new(),
            current: u32::MAX,
        }
    }

    /// Enqueue a thread based on its scheduling policy
    pub fn enqueue(&mut self, t: &SigmaThread) {
        match t.policy {
            SCHED_EDF          => self.edf.insert(t.tid),
            SCHED_FIFO | SCHED_RR => self.mlfq.enqueue(0, t.tid),
            _                  => {
                if t.timeslice > 0 && t.mlfq_level < MLFQ_LEVELS {
                    self.mlfq.enqueue(t.mlfq_level, t.tid);
                } else {
                    self.cfs.insert(t.tid);
                }
            }
        }
    }

    /// Select the next thread to run. Tier order: EDF > MLFQ > CFS > idle
    pub fn pick_next(&mut self, threads: &[SigmaThread]) -> u32 {
        if let Some(tid) = self.edf.pick_next(threads)  { return tid; }
        if let Some(tid) = self.mlfq.pick_next()         { return tid; }
        if let Some(tid) = self.cfs.pick_next(threads)   { return tid; }
        u32::MAX // idle
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global Scheduler State
// ─────────────────────────────────────────────────────────────────────────────

pub struct SigmaScheduler {
    pub threads:   [SigmaThread; MAX_THREADS],
    pub runqueues: [PerCpuRunqueue; MAX_CPUS],
    pub num_cpus:  usize,
    pub tick:      u64,
    pub next_tid:  u32,
}

impl SigmaScheduler {
    pub const fn new() -> Self {
        const T: SigmaThread      = SigmaThread::empty();
        const Q: PerCpuRunqueue   = PerCpuRunqueue::new();
        Self {
            threads:   [T; MAX_THREADS],
            runqueues: [Q; MAX_CPUS],
            num_cpus:  1,
            tick:      0,
            next_tid:  1,
        }
    }

    pub fn init(&mut self, num_cpus: usize) {
        self.num_cpus = num_cpus.min(MAX_CPUS);
    }

    // ── Thread Management ────────────────────────────────────────────────────

    pub fn spawn(
        &mut self,
        pid: u32,
        entry: u64,
        stack_top: u64,
        stack_size: u64,
        policy: u8,
        priority: i32,
    ) -> Option<u32> {
        // Find a free slot
        let tid = self.next_tid as usize;
        if tid >= MAX_THREADS { return None; }
        self.next_tid += 1;

        let cpu = self.pick_cpu();
        let min_vrt = self.runqueues[cpu].cfs.min_vruntime;

        let t = &mut self.threads[tid];
        *t = SigmaThread {
            tid: tid as u32,
            pid,
            state: ThreadState::Ready,
            policy,
            priority,
            vruntime: min_vrt,
            deadline_ns: u64::MAX,
            timeslice: MLFQ_TIMESLICE,
            mlfq_level: 0,
            cpu_affinity: !0u64,
            cpu_id: cpu,
            stack_top,
            stack_size,
            name: [0u8; 32],
            ctx: CpuContext {
                rip: entry,
                rsp: stack_top,
                rflags: 0x200,
                cs: 0x08,
                ss: 0x10,
                ..CpuContext::zero()
            },
        };

        let t_copy = *t;
        self.runqueues[cpu].enqueue(&t_copy);
        Some(tid as u32)
    }

    pub fn exit(&mut self, tid: u32) {
        let idx = tid as usize;
        if idx >= MAX_THREADS { return; }
        self.threads[idx].state = ThreadState::Zombie;
    }

    pub fn block(&mut self, tid: u32) {
        let idx = tid as usize;
        if idx >= MAX_THREADS { return; }
        self.threads[idx].state = ThreadState::Blocked;
    }

    pub fn unblock(&mut self, tid: u32) {
        let idx = tid as usize;
        if idx >= MAX_THREADS { return; }
        let t = &mut self.threads[idx];
        if t.state != ThreadState::Blocked { return; }
        t.state = ThreadState::Ready;
        let cpu = t.cpu_id;
        let t_copy = *t;
        self.runqueues[cpu].enqueue(&t_copy);
    }

    // ── Tick handler (called from timer IRQ) ─────────────────────────────────

    /// Called every timer tick on CPU `cpu_id`. Returns the TID to switch to
    /// (u32::MAX = stay on idle, or no switch needed).
    pub fn tick(&mut self, cpu_id: usize, now_ns: u64) -> u32 {
        self.tick += 1;
        let cur_tid = self.runqueues[cpu_id].current as usize;

        if cur_tid < MAX_THREADS {
            let t = &mut self.threads[cur_tid];

            // CFS: advance vruntime proportional to priority weight
            let weight: u64 = match t.priority {
                n if n < 0  => (1u64 << (-n as u64).min(20)),
                0           => 1,
                n           => 1u64.max(1024 / (n as u64 + 1)),
            };
            t.vruntime += weight;

            // EDF: check if deadline missed
            if t.policy == SCHED_EDF && now_ns > t.deadline_ns {
                // Log deadline miss (in production: raise SIGXCPU or kill)
                t.state = ThreadState::Zombie;
            }

            // MLFQ: consume timeslice
            if t.policy == SCHED_NORMAL && t.mlfq_level < MLFQ_LEVELS {
                if t.timeslice > 0 {
                    t.timeslice -= 1;
                }
                if t.timeslice == 0 {
                    // Demote to next MLFQ level (or fall into CFS)
                    t.mlfq_level = (t.mlfq_level + 1).min(MLFQ_LEVELS);
                    t.timeslice = MLFQ_TIMESLICE * (1 << t.mlfq_level.min(6)) as u64;
                    t.state = ThreadState::Ready;
                    let cpu = t.cpu_id;
                    let t_copy = *t;
                    self.runqueues[cpu].enqueue(&t_copy);
                }
            }
        }

        // Priority boost every 100 ticks (MLFQ anti-starvation)
        if self.tick % 100 == 0 {
            self.priority_boost(cpu_id);
        }

        // Work-steal if this CPU's runqueue is empty
        if self.runqueues[cpu_id].edf.count == 0
            && self.runqueues[cpu_id].cfs.count == 0
            && self.runqueues[cpu_id].mlfq.counts.iter().all(|c| *c == 0)
        {
            self.work_steal(cpu_id);
        }

        let next = self.runqueues[cpu_id].pick_next(&self.threads);
        if next < MAX_THREADS as u32 {
            self.threads[next as usize].state = ThreadState::Running;
        }
        self.runqueues[cpu_id].current = next;
        next
    }

    // ── Priority Boost (anti-starvation) ─────────────────────────────────────

    fn priority_boost(&mut self, cpu_id: usize) {
        // Move all ready threads back to MLFQ level 0 to prevent starvation
        for tid in 0..MAX_THREADS {
            let t = &mut self.threads[tid];
            if t.cpu_id != cpu_id { continue; }
            if t.state == ThreadState::Ready && t.policy == SCHED_NORMAL {
                t.mlfq_level = 0;
                t.timeslice  = MLFQ_TIMESLICE;
            }
        }
    }

    // ── Work Stealing ─────────────────────────────────────────────────────────

    fn work_steal(&mut self, dst_cpu: usize) {
        // Find the busiest CPU
        let mut busiest = dst_cpu;
        let mut max_load = 0usize;
        for cpu in 0..self.num_cpus {
            if cpu == dst_cpu { continue; }
            let load = self.runqueues[cpu].cfs.count
                + self.runqueues[cpu].edf.count
                + self.runqueues[cpu].mlfq.counts.iter().sum::<usize>();
            if load > max_load {
                max_load = load;
                busiest = cpu;
            }
        }

        if busiest == dst_cpu || max_load < 2 { return; }

        // Steal half the tasks from busiest's CFS queue
        let steal = max_load / 2;
        let src_count = self.runqueues[busiest].cfs.count;
        let to_steal = steal.min(src_count);

        for _ in 0..to_steal {
            if self.runqueues[busiest].cfs.count == 0 { break; }
            let last = self.runqueues[busiest].cfs.count - 1;
            let tid = self.runqueues[busiest].cfs.tids[last];
            self.runqueues[busiest].cfs.tids[last] = u32::MAX;
            self.runqueues[busiest].cfs.count -= 1;

            if (tid as usize) < MAX_THREADS {
                self.threads[tid as usize].cpu_id = dst_cpu;
                self.runqueues[dst_cpu].cfs.insert(tid);
            }
        }
    }

    // ── CPU Selection (lowest load) ───────────────────────────────────────────

    fn pick_cpu(&self) -> usize {
        let mut min_load = usize::MAX;
        let mut best = 0;
        for cpu in 0..self.num_cpus {
            let load = self.runqueues[cpu].cfs.count
                + self.runqueues[cpu].edf.count
                + self.runqueues[cpu].mlfq.counts.iter().sum::<usize>();
            if load < min_load {
                min_load = load;
                best = cpu;
            }
        }
        best
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut SCHEDULER: SigmaScheduler = SigmaScheduler::new();
static SCHED_READY: AtomicUsize = AtomicUsize::new(0);

pub fn sched_init(num_cpus: usize) {
    unsafe { SCHEDULER.init(num_cpus); }
    SCHED_READY.store(1, Ordering::Release);
}

pub fn sched_spawn(pid: u32, entry: u64, stack_top: u64, stack_size: u64,
                   policy: u8, priority: i32) -> Option<u32> {
    unsafe { SCHEDULER.spawn(pid, entry, stack_top, stack_size, policy, priority) }
}

pub fn sched_exit(tid: u32) {
    unsafe { SCHEDULER.exit(tid); }
}

pub fn sched_block(tid: u32) {
    unsafe { SCHEDULER.block(tid); }
}

pub fn sched_unblock(tid: u32) {
    unsafe { SCHEDULER.unblock(tid); }
}

/// Timer IRQ entry-point — returns next TID to run on `cpu_id`
pub fn sched_tick(cpu_id: usize, now_ns: u64) -> u32 {
    unsafe { SCHEDULER.tick(cpu_id, now_ns) }
}
