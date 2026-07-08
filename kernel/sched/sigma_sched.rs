// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sched/sigma_sched.rs — Composite Scheduler: MLFQ → CFS → EDF
//
// SigmaOS uses a three-tier scheduler:
//   • MLFQ  — interactive tasks (Tier 0, highest priority)
//   • CFS   — normal fair-share tasks (Tier 1) (Red-Black Tree)
//   • EDF   — real-time tasks with deadlines (Tier 2) (Binary Min-Heap)
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
pub const NIL:             u32   = u32::MAX;

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
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
    pub cr3: u64,
}

impl CpuContext {
    pub const fn zero() -> Self {
        Self {
            rsp: 0, rbp: 0, rbx: 0, r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x200, cs: 0x08, ss: 0x10, cr3: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct SigmaThread {
    pub tid:         u32,
    pub pid:         u32,
    pub state:       ThreadState,
    pub policy:      u8,
    pub priority:    i32,
    pub vruntime:    u64,
    pub deadline_ns: u64,
    pub timeslice:   u64,
    pub mlfq_level:  usize,
    pub cpu_affinity:u64,
    pub cpu_id:      usize,
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

pub struct MlfqQueue {
    pub queues:  [[u32; MAX_THREADS / MLFQ_LEVELS]; MLFQ_LEVELS],
    pub heads:   [usize; MLFQ_LEVELS],
    pub tails:   [usize; MLFQ_LEVELS],
    pub counts:  [usize; MLFQ_LEVELS],
}

impl MlfqQueue {
    pub const fn new() -> Self {
        Self {
            queues:  [[NIL; MAX_THREADS / MLFQ_LEVELS]; MLFQ_LEVELS],
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
// CFS Sub-scheduler: Array-Backed Red-Black Tree (O(log n))
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
enum Color { Red, Black }

#[derive(Copy, Clone)]
struct RbNode {
    tid: u32,
    vruntime: u64,
    parent: u32,
    left: u32,
    right: u32,
    color: Color,
}

impl RbNode {
    const fn empty() -> Self {
        Self { tid: NIL, vruntime: 0, parent: NIL, left: NIL, right: NIL, color: Color::Black }
    }
}

pub struct CfsRunqueue {
    nodes: [RbNode; MAX_THREADS],
    root: u32,
    pub count: usize,
    pub min_vruntime: u64,
}

impl CfsRunqueue {
    pub const fn new() -> Self {
        Self { nodes: [RbNode::empty(); MAX_THREADS], root: NIL, count: 0, min_vruntime: 0 }
    }

    pub fn insert(&mut self, tid: u32, vruntime: u64) {
        let node_idx = tid; 
        if (node_idx as usize) >= MAX_THREADS { return; }

        self.nodes[node_idx as usize] = RbNode {
            tid,
            vruntime,
            parent: NIL,
            left: NIL,
            right: NIL,
            color: Color::Red,
        };
        self.count += 1;

        if self.root == NIL {
            self.root = node_idx;
            self.nodes[node_idx as usize].color = Color::Black;
            return;
        }

        let mut y = NIL;
        let mut x = self.root;
        while x != NIL {
            y = x;
            if vruntime < self.nodes[x as usize].vruntime {
                x = self.nodes[x as usize].left;
            } else {
                x = self.nodes[x as usize].right;
            }
        }

        self.nodes[node_idx as usize].parent = y;
        if vruntime < self.nodes[y as usize].vruntime {
            self.nodes[y as usize].left = node_idx;
        } else {
            self.nodes[y as usize].right = node_idx;
        }

        let mut curr = node_idx;
        while curr != self.root && self.nodes[self.nodes[curr as usize].parent as usize].color == Color::Red {
            let parent = self.nodes[curr as usize].parent;
            let grandparent = self.nodes[parent as usize].parent;
            if grandparent == NIL { break; }

            if parent == self.nodes[grandparent as usize].left {
                let uncle = self.nodes[grandparent as usize].right;
                if uncle != NIL && self.nodes[uncle as usize].color == Color::Red {
                    self.nodes[parent as usize].color = Color::Black;
                    self.nodes[uncle as usize].color = Color::Black;
                    self.nodes[grandparent as usize].color = Color::Red;
                    curr = grandparent;
                } else {
                    if curr == self.nodes[parent as usize].right {
                        curr = parent;
                        self.left_rotate(curr);
                    }
                    let p = self.nodes[curr as usize].parent;
                    let g = self.nodes[p as usize].parent;
                    self.nodes[p as usize].color = Color::Black;
                    self.nodes[g as usize].color = Color::Red;
                    self.right_rotate(g);
                }
            } else {
                let uncle = self.nodes[grandparent as usize].left;
                if uncle != NIL && self.nodes[uncle as usize].color == Color::Red {
                    self.nodes[parent as usize].color = Color::Black;
                    self.nodes[uncle as usize].color = Color::Black;
                    self.nodes[grandparent as usize].color = Color::Red;
                    curr = grandparent;
                } else {
                    if curr == self.nodes[parent as usize].left {
                        curr = parent;
                        self.right_rotate(curr);
                    }
                    let p = self.nodes[curr as usize].parent;
                    let g = self.nodes[p as usize].parent;
                    self.nodes[p as usize].color = Color::Black;
                    self.nodes[g as usize].color = Color::Red;
                    self.left_rotate(g);
                }
            }
        }
        self.nodes[self.root as usize].color = Color::Black;
    }

    fn left_rotate(&mut self, x: u32) {
        let y = self.nodes[x as usize].right;
        if y == NIL { return; }
        self.nodes[x as usize].right = self.nodes[y as usize].left;
        if self.nodes[y as usize].left != NIL {
            self.nodes[self.nodes[y as usize].left as usize].parent = x;
        }
        self.nodes[y as usize].parent = self.nodes[x as usize].parent;
        if self.nodes[x as usize].parent == NIL {
            self.root = y;
        } else if x == self.nodes[self.nodes[x as usize].parent as usize].left {
            self.nodes[self.nodes[x as usize].parent as usize].left = y;
        } else {
            self.nodes[self.nodes[x as usize].parent as usize].right = y;
        }
        self.nodes[y as usize].left = x;
        self.nodes[x as usize].parent = y;
    }

    fn right_rotate(&mut self, y: u32) {
        let x = self.nodes[y as usize].left;
        if x == NIL { return; }
        self.nodes[y as usize].left = self.nodes[x as usize].right;
        if self.nodes[x as usize].right != NIL {
            self.nodes[self.nodes[x as usize].right as usize].parent = y;
        }
        self.nodes[x as usize].parent = self.nodes[y as usize].parent;
        if self.nodes[y as usize].parent == NIL {
            self.root = x;
        } else if y == self.nodes[self.nodes[y as usize].parent as usize].right {
            self.nodes[self.nodes[y as usize].parent as usize].right = x;
        } else {
            self.nodes[self.nodes[y as usize].parent as usize].left = x;
        }
        self.nodes[x as usize].right = y;
        self.nodes[y as usize].parent = x;
    }

    pub fn pick_next(&mut self) -> Option<u32> {
        if self.root == NIL { return None; }
        
        let mut curr = self.root;
        while self.nodes[curr as usize].left != NIL {
            curr = self.nodes[curr as usize].left;
        }

        let tid = self.nodes[curr as usize].tid;
        self.min_vruntime = self.nodes[curr as usize].vruntime;
        
        let parent = self.nodes[curr as usize].parent;
        let right = self.nodes[curr as usize].right;
        if parent == NIL {
            self.root = right;
        } else if curr == self.nodes[parent as usize].left {
            self.nodes[parent as usize].left = right;
        } else {
            self.nodes[parent as usize].right = right;
        }
        if right != NIL {
            self.nodes[right as usize].parent = parent;
        }
        
        self.nodes[curr as usize] = RbNode::empty();
        self.count -= 1;
        Some(tid)
    }

    pub fn extract_any(&mut self) -> Option<u32> {
        self.pick_next()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// EDF Sub-scheduler: Array-Backed Binary Min-Heap (O(log n))
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone)]
struct EdfNode {
    tid: u32,
    deadline_ns: u64,
}

pub struct EdfRunqueue {
    nodes: [EdfNode; EDF_MAX_TASKS],
    pub count: usize,
}

impl EdfRunqueue {
    pub const fn new() -> Self {
        Self { nodes: [EdfNode { tid: NIL, deadline_ns: 0 }; EDF_MAX_TASKS], count: 0 }
    }

    pub fn insert(&mut self, tid: u32, deadline_ns: u64) {
        if self.count >= EDF_MAX_TASKS { return; }
        self.nodes[self.count] = EdfNode { tid, deadline_ns };
        self.bubble_up(self.count);
        self.count += 1;
    }

    pub fn pick_next(&mut self) -> Option<u32> {
        if self.count == 0 { return None; }
        let root_tid = self.nodes[0].tid;
        self.count -= 1;
        if self.count > 0 {
            self.nodes[0] = self.nodes[self.count];
            self.trickle_down(0);
        }
        self.nodes[self.count] = EdfNode { tid: NIL, deadline_ns: 0 };
        Some(root_tid)
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.nodes[idx].deadline_ns >= self.nodes[parent].deadline_ns { break; }
            self.nodes.swap(idx, parent);
            idx = parent;
        }
    }

    fn trickle_down(&mut self, mut idx: usize) {
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut smallest = idx;
            
            if left < self.count && self.nodes[left].deadline_ns < self.nodes[smallest].deadline_ns {
                smallest = left;
            }
            if right < self.count && self.nodes[right].deadline_ns < self.nodes[smallest].deadline_ns {
                smallest = right;
            }
            if smallest == idx { break; }
            self.nodes.swap(idx, smallest);
            idx = smallest;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Per-CPU Runqueue
// ─────────────────────────────────────────────────────────────────────────────

pub struct PerCpuRunqueue {
    pub mlfq:    MlfqQueue,
    pub cfs:     CfsRunqueue,
    pub edf:     EdfRunqueue,
    pub current: u32,
}

impl PerCpuRunqueue {
    pub const fn new() -> Self {
        Self {
            mlfq:    MlfqQueue::new(),
            cfs:     CfsRunqueue::new(),
            edf:     EdfRunqueue::new(),
            current: NIL,
        }
    }

    pub fn enqueue(&mut self, t: &SigmaThread) {
        match t.policy {
            SCHED_EDF          => self.edf.insert(t.tid, t.deadline_ns),
            SCHED_FIFO | SCHED_RR => self.mlfq.enqueue(0, t.tid),
            _                  => {
                if t.timeslice > 0 && t.mlfq_level < MLFQ_LEVELS {
                    self.mlfq.enqueue(t.mlfq_level, t.tid);
                } else {
                    self.cfs.insert(t.tid, t.vruntime);
                }
            }
        }
    }

    pub fn pick_next(&mut self) -> u32 {
        if let Some(tid) = self.edf.pick_next()  { return tid; }
        if let Some(tid) = self.mlfq.pick_next() { return tid; }
        if let Some(tid) = self.cfs.pick_next()  { return tid; }
        NIL
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

    pub fn spawn(
        &mut self,
        pid: u32,
        entry: u64,
        stack_top: u64,
        stack_size: u64,
        policy: u8,
        priority: i32,
    ) -> Option<u32> {
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

    pub fn tick(&mut self, cpu_id: usize, now_ns: u64) -> u32 {
        self.tick += 1;
        let cur_tid = self.runqueues[cpu_id].current as usize;

        if cur_tid < MAX_THREADS {
            let t = &mut self.threads[cur_tid];

            let weight: u64 = match t.priority {
                n if n < 0  => (1u64 << (-n as u64).min(20)),
                0           => 1,
                n           => 1u64.max(1024 / (n as u64 + 1)),
            };
            t.vruntime += weight;

            if t.policy == SCHED_EDF && now_ns > t.deadline_ns {
                t.state = ThreadState::Zombie;
            }

            if t.policy == SCHED_NORMAL && t.mlfq_level < MLFQ_LEVELS {
                if t.timeslice > 0 {
                    t.timeslice -= 1;
                }
                if t.timeslice == 0 {
                    t.mlfq_level = (t.mlfq_level + 1).min(MLFQ_LEVELS);
                    t.timeslice = MLFQ_TIMESLICE * (1 << t.mlfq_level.min(6)) as u64;
                    t.state = ThreadState::Ready;
                    let cpu = t.cpu_id;
                    let t_copy = *t;
                    self.runqueues[cpu].enqueue(&t_copy);
                }
            }
        }

        if self.tick % 100 == 0 {
            self.priority_boost(cpu_id);
        }

        if self.runqueues[cpu_id].edf.count == 0
            && self.runqueues[cpu_id].cfs.count == 0
            && self.runqueues[cpu_id].mlfq.counts.iter().all(|c| *c == 0)
        {
            self.work_steal(cpu_id);
        }

        let next = self.runqueues[cpu_id].pick_next();
        if next < MAX_THREADS as u32 {
            self.threads[next as usize].state = ThreadState::Running;
        }
        self.runqueues[cpu_id].current = next;
        next
    }

    fn priority_boost(&mut self, cpu_id: usize) {
        for tid in 0..MAX_THREADS {
            let t = &mut self.threads[tid];
            if t.cpu_id != cpu_id { continue; }
            if t.state == ThreadState::Ready && t.policy == SCHED_NORMAL {
                t.mlfq_level = 0;
                t.timeslice  = MLFQ_TIMESLICE;
            }
        }
    }

    fn work_steal(&mut self, dst_cpu: usize) {
        if self.num_cpus < 2 { return; }
        
        let seed = self.tick.wrapping_add(dst_cpu as u64);
        let mut r = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cpu1 = ((r >> 32) as usize) % self.num_cpus;
        r = r.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cpu2 = ((r >> 32) as usize) % self.num_cpus;

        let load1 = self.runqueues[cpu1].cfs.count + self.runqueues[cpu1].edf.count + self.runqueues[cpu1].mlfq.counts.iter().sum::<usize>();
        let load2 = self.runqueues[cpu2].cfs.count + self.runqueues[cpu2].edf.count + self.runqueues[cpu2].mlfq.counts.iter().sum::<usize>();

        let (mut busiest, mut max_load) = if load1 > load2 { (cpu1, load1) } else { (cpu2, load2) };
        if busiest == dst_cpu {
            busiest = if busiest == cpu1 { cpu2 } else { cpu1 };
            max_load = if busiest == cpu1 { load1 } else { load2 };
        }

        if busiest == dst_cpu || max_load < 2 { return; }

        let steal = max_load / 2;
        let src_count = self.runqueues[busiest].cfs.count;
        let to_steal = steal.min(src_count);

        for _ in 0..to_steal {
            if let Some(tid) = self.runqueues[busiest].cfs.extract_any() {
                if (tid as usize) < MAX_THREADS {
                    self.threads[tid as usize].cpu_id = dst_cpu;
                    self.runqueues[dst_cpu].cfs.insert(tid, self.threads[tid as usize].vruntime);
                }
            } else {
                break;
            }
        }
    }

    fn pick_cpu(&self) -> usize {
        if self.num_cpus < 2 { return 0; }
        
        let seed = self.tick.wrapping_add(self.next_tid as u64);
        let mut r = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cpu1 = ((r >> 32) as usize) % self.num_cpus;
        r = r.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let cpu2 = ((r >> 32) as usize) % self.num_cpus;

        let load1 = self.runqueues[cpu1].cfs.count + self.runqueues[cpu1].edf.count + self.runqueues[cpu1].mlfq.counts.iter().sum::<usize>();
        let load2 = self.runqueues[cpu2].cfs.count + self.runqueues[cpu2].edf.count + self.runqueues[cpu2].mlfq.counts.iter().sum::<usize>();

        if load1 < load2 { cpu1 } else { cpu2 }
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

pub fn sched_tick(cpu_id: usize, now_ns: u64) -> u32 {
    unsafe { SCHEDULER.tick(cpu_id, now_ns) }
}
