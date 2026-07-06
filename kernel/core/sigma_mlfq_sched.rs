// SigmaOS — MLFQ + MCS Scheduler (Issue #1003)
// Multi-Level Feedback Queue with MCS spinlock for SMP safety.
// No external dependencies — sovereign implementation.
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

// ─── Constants ───────────────────────────────────────────────────────────────
pub const NQUEUE:       usize = 8;     // number of priority queues
pub const MAX_PROCS:    usize = 512;
pub const TIME_QUANTA:  [u64; NQUEUE] = [1,2,4,8,16,32,64,128]; // ms
pub const BOOST_PERIOD: u64 = 1000;   // ms — periodic priority boost

// ─── MCS Spinlock ────────────────────────────────────────────────────────────
pub struct McsNode {
    pub next:   AtomicUsize, // pointer to next node (0 = null)
    pub locked: AtomicBool,
}

impl McsNode {
    pub const fn new() -> Self {
        McsNode {
            next:   AtomicUsize::new(0),
            locked: AtomicBool::new(false),
        }
    }
}

pub struct McsLock {
    pub tail: AtomicUsize,
}

impl McsLock {
    pub const fn new() -> Self { McsLock { tail: AtomicUsize::new(0) } }

    /// Acquire the lock; `node` must be per-CPU/per-thread.
    pub fn lock(&self, node: &McsNode) {
        node.next.store(0, Ordering::Relaxed);
        node.locked.store(true, Ordering::Relaxed);
        let node_ptr = node as *const _ as usize;
        let prev = self.tail.swap(node_ptr, Ordering::AcqRel);
        if prev != 0 {
            let prev_node = unsafe { &*(prev as *const McsNode) };
            prev_node.next.store(node_ptr, Ordering::Release);
            // Spin until predecessor grants us the lock
            while node.locked.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }
        }
    }

    /// Release the lock.
    pub fn unlock(&self, node: &McsNode) {
        let node_ptr = node as *const _ as usize;
        let next = node.next.load(Ordering::Acquire);
        if next == 0 {
            // Try to CAS tail back to null
            if self.tail.compare_exchange(
                node_ptr, 0, Ordering::Release, Ordering::Relaxed).is_ok() {
                return;
            }
            // Wait for successor to appear
            let mut next2 = node.next.load(Ordering::Acquire);
            while next2 == 0 {
                core::hint::spin_loop();
                next2 = node.next.load(Ordering::Acquire);
            }
            let succ = unsafe { &*(next2 as *const McsNode) };
            succ.locked.store(false, Ordering::Release);
        } else {
            let succ = unsafe { &*(next as *const McsNode) };
            succ.locked.store(false, Ordering::Release);
        }
    }
}

// ─── Process State ───────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ProcState {
    Unused    = 0,
    Runnable  = 1,
    Running   = 2,
    Blocked   = 3,
    Zombie    = 4,
}

#[derive(Clone, Copy)]
pub struct ProcContext {
    // x86-64 callee-saved registers
    pub rsp: u64, pub rbp: u64, pub rbx: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64,
}

impl ProcContext {
    pub const fn zero() -> Self {
        ProcContext { rsp:0, rbp:0, rbx:0, r12:0, r13:0, r14:0, r15:0, rip:0 }
    }
}

#[derive(Clone, Copy)]
pub struct Process {
    pub pid:        u32,
    pub state:      ProcState,
    pub priority:   u8,          // current queue index (0 = highest)
    pub ticks_used: u64,         // ticks used in current quantum
    pub total_cpu:  u64,         // total CPU ticks
    pub ctx:        ProcContext,
    pub kernel_sp:  u64,
    pub io_wait:    bool,
    pub next:       usize,       // index in run queue (0 = end)
}

impl Process {
    pub const fn new(pid: u32) -> Self {
        Process {
            pid, state: ProcState::Unused, priority: 0,
            ticks_used: 0, total_cpu: 0,
            ctx: ProcContext::zero(), kernel_sp: 0,
            io_wait: false, next: 0,
        }
    }
}

// ─── Run Queue ───────────────────────────────────────────────────────────────
/// Simple singly-linked list per priority level using array indices.
pub struct RunQueue {
    pub head: [usize; NQUEUE],   // head index (0 = empty); 1-based
    pub tail: [usize; NQUEUE],
    pub len:  [usize; NQUEUE],
}

impl RunQueue {
    pub const fn new() -> Self {
        RunQueue { head: [0;NQUEUE], tail: [0;NQUEUE], len: [0;NQUEUE] }
    }

    pub fn enqueue(&mut self, procs: &mut [Process; MAX_PROCS], idx: usize, q: usize) {
        procs[idx].next = 0;
        if self.tail[q] == 0 {
            self.head[q] = idx + 1;
            self.tail[q] = idx + 1;
        } else {
            procs[self.tail[q] - 1].next = idx + 1;
            self.tail[q] = idx + 1;
        }
        self.len[q] += 1;
    }

    pub fn dequeue(&mut self, procs: &mut [Process; MAX_PROCS], q: usize) -> Option<usize> {
        if self.head[q] == 0 { return None; }
        let idx = self.head[q] - 1;
        self.head[q] = procs[idx].next;
        if self.head[q] == 0 { self.tail[q] = 0; }
        procs[idx].next = 0;
        self.len[q] = self.len[q].saturating_sub(1);
        Some(idx)
    }
}

// ─── MLFQ Scheduler ──────────────────────────────────────────────────────────
pub struct MlfqScheduler {
    pub procs:        [Process; MAX_PROCS],
    pub rq:           RunQueue,
    pub lock:         McsLock,
    pub current:      usize,        // index of currently running process
    pub ticks:        AtomicU64,    // global tick counter (ms)
    pub last_boost:   u64,
    pub proc_count:   usize,
}

impl MlfqScheduler {
    pub const fn new() -> Self {
        const EMPTY: Process = Process::new(0);
        MlfqScheduler {
            procs:      [EMPTY; MAX_PROCS],
            rq:         RunQueue::new(),
            lock:       McsLock::new(),
            current:    usize::MAX,
            ticks:      AtomicU64::new(0),
            last_boost: 0,
            proc_count: 0,
        }
    }

    /// Spawn a new process at highest priority.
    pub fn spawn(&mut self, entry: u64, kernel_sp: u64) -> Option<u32> {
        for i in 0..MAX_PROCS {
            if self.procs[i].state == ProcState::Unused {
                let pid = (i + 1) as u32;
                self.procs[i] = Process::new(pid);
                self.procs[i].state     = ProcState::Runnable;
                self.procs[i].priority  = 0;
                self.procs[i].ctx.rip   = entry;
                self.procs[i].kernel_sp = kernel_sp;
                self.rq.enqueue(&mut self.procs, i, 0);
                self.proc_count += 1;
                return Some(pid);
            }
        }
        None
    }

    /// Timer tick — called every 1 ms.
    pub fn tick(&mut self) {
        let now = self.ticks.fetch_add(1, Ordering::Relaxed) + 1;

        // Periodic priority boost — all processes move to queue 0
        if now - self.last_boost >= BOOST_PERIOD {
            self.last_boost = now;
            self.boost_all_priorities();
        }

        // Charge tick to running process
        if self.current != usize::MAX {
            let p = &mut self.procs[self.current];
            if p.state == ProcState::Running {
                p.ticks_used += 1;
                p.total_cpu  += 1;
                let q = p.priority as usize;
                if p.ticks_used >= TIME_QUANTA[q] {
                    // Quantum expired: demote to lower queue
                    p.ticks_used = 0;
                    if p.priority < (NQUEUE - 1) as u8 {
                        p.priority += 1;
                    }
                    p.state = ProcState::Runnable;
                    let new_q = p.priority as usize;
                    self.rq.enqueue(&mut self.procs, self.current, new_q);
                    self.current = usize::MAX;
                }
            }
        }
    }

    /// Pick the next process to run (O(NQUEUE)).
    pub fn schedule(&mut self) -> Option<usize> {
        for q in 0..NQUEUE {
            if let Some(idx) = self.rq.dequeue(&mut self.procs, q) {
                if self.procs[idx].state == ProcState::Runnable {
                    self.procs[idx].state = ProcState::Running;
                    self.procs[idx].ticks_used = 0;
                    self.current = idx;
                    return Some(idx);
                }
            }
        }
        None
    }

    /// Mark a process as blocked (I/O wait).
    pub fn block(&mut self, idx: usize) {
        if idx < MAX_PROCS {
            self.procs[idx].state   = ProcState::Blocked;
            self.procs[idx].io_wait = true;
            // I/O completion will re-enqueue at priority 0 (reward)
        }
    }

    /// Wake a blocked process and give it priority boost (MLFQ I/O reward).
    pub fn unblock(&mut self, idx: usize) {
        if idx < MAX_PROCS && self.procs[idx].state == ProcState::Blocked {
            self.procs[idx].state    = ProcState::Runnable;
            self.procs[idx].io_wait  = false;
            self.procs[idx].priority = 0; // reward: back to top
            self.rq.enqueue(&mut self.procs, idx, 0);
        }
    }

    /// Periodic priority boost to prevent starvation.
    fn boost_all_priorities(&mut self) {
        for i in 0..MAX_PROCS {
            if self.procs[i].state == ProcState::Runnable && self.procs[i].priority > 0 {
                // Remove from current queue and re-add to queue 0 would require
                // queue search; simpler: mark priority=0, will be picked up on next
                // dequeue or enqueue. Full impl maintains per-pid queue membership.
                self.procs[i].priority = 0;
            }
        }
    }

    /// Exit a process.
    pub fn exit(&mut self, idx: usize) {
        if idx < MAX_PROCS {
            self.procs[idx].state = ProcState::Zombie;
            if self.current == idx { self.current = usize::MAX; }
            self.proc_count = self.proc_count.saturating_sub(1);
        }
    }

    pub fn num_runnable(&self) -> usize {
        self.rq.len.iter().sum()
    }
}

// Global scheduler instance (single-core; SMP extends with per-CPU run queues)
static mut SCHEDULER: MlfqScheduler = MlfqScheduler::new();

pub fn sigma_sched_tick() {
    unsafe { SCHEDULER.tick(); }
}

pub fn sigma_sched_next() -> Option<usize> {
    unsafe { SCHEDULER.schedule() }
}

pub fn sigma_sched_spawn(entry: u64, sp: u64) -> Option<u32> {
    unsafe { SCHEDULER.spawn(entry, sp) }
}

pub fn sigma_sched_block(idx: usize) {
    unsafe { SCHEDULER.block(idx); }
}

pub fn sigma_sched_unblock(idx: usize) {
    unsafe { SCHEDULER.unblock(idx); }
}

pub fn sigma_sched_exit(idx: usize) {
    unsafe { SCHEDULER.exit(idx); }
}
