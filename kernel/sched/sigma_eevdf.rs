// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/sched/sigma_eevdf.rs — EEVDF Scheduler Implementation
//
// EEVDF (Earliest Eligible Virtual Deadline First) is Linux's modern scheduler
// that improves upon CFS by using virtual deadlines for better latency and fairness.
// This implementation follows the Linux 6.x EEVDF design with OOP principles.
//
// Key features:
// - Virtual deadline-based scheduling (not just vruntime)
// - Lag tracking for fairness correction
// - Eligibility window for task selection
// - O(1) eligibility check with O(log n) insertion/removal
// - No external dependencies, pure Rust implementation

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Constants (Linux-inspired values)
// ─────────────────────────────────────────────────────────────────────────────

pub const EEVDF_MAX_TASKS: usize = 1024;
pub const NICE_0_WEIGHT: u64 = 1024;
pub const NICE_0_LOAD: u64 = 1024;
pub const SYSCTL_SCHED_LATENCY: u64 = 20_000_000; // 20ms in nanoseconds
pub const SYSCTL_SCHED_MIN_GRANULARITY: u64 = 1_000_000; // 1ms
pub const SYSCTL_SCHED_WAKEUP_GRANULARITY: u64 = 500_000; // 0.5ms
pub const MAX_LATENCY_MULTIPLIER: u64 = 8;
pub const NIL: u32 = u32::MAX;

// ─────────────────────────────────────────────────────────────────────────────
// Prio-to-weight table (Linux sched.c, 40 entries for nice -20..+19)
// ─────────────────────────────────────────────────────────────────────────────

const PRIO_TO_WEIGHT: [u64; 40] = [
    88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916,
     9548,  7620,  6100,  4904,  3906,  3121,  2501,  1991,  1586,  1277,
     1024,   820,   655,   526,   423,   335,   272,   215,   172,   137,
      110,    87,    70,    56,    45,    36,    29,    23,    18,    15,
];

// ─────────────────────────────────────────────────────────────────────────────
// Task Entity with OOP-style encapsulation
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct EevdfEntity {
    pub tid: u32,
    pub pid: u32,
    pub nice: i32,
    pub weight: u64,
    pub vruntime: u64,
    pub deadline: u64,
    pub slice: u64,
    pub lag: i64,
    pub eligible: bool,
    pub active: bool,
}

impl EevdfEntity {
    pub const fn empty() -> Self {
        Self {
            tid: NIL,
            pid: 0,
            nice: 0,
            weight: NICE_0_WEIGHT,
            vruntime: 0,
            deadline: 0,
            slice: 0,
            lag: 0,
            eligible: false,
            active: false,
        }
    }

    // Calculate virtual deadline from vruntime and weight
    pub fn calc_virtual_deadline(&self, vruntime: u64, weight: u64) -> u64 {
        // virtual_deadline = vruntime + (slice * NICE_0_WEIGHT / weight)
        // This ensures tasks with higher weight get tighter deadlines
        let scaled_slice = self.slice * NICE_0_WEIGHT / weight.max(1);
        vruntime.saturating_add(scaled_slice)
    }

    // Update lag based on actual vs ideal runtime
    pub fn update_lag(&mut self, actual_runtime: u64, ideal_runtime: u64) {
        self.lag = ideal_runtime as i64 - actual_runtime as i64;
    }

    // Check if task is eligible to run
    pub fn is_eligible(&self, min_vruntime: u64) -> bool {
        self.vruntime >= min_vruntime.saturating_sub(self.slice)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Red-Black Tree for deadline ordering (O(log n) operations)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq)]
enum RbColor { Red, Black }

#[derive(Copy, Clone)]
struct RbNode {
    tid: u32,
    deadline: u64,
    parent: u32,
    left: u32,
    right: u32,
    color: RbColor,
}

impl RbNode {
    const fn empty() -> Self {
        Self {
            tid: NIL,
            deadline: 0,
            parent: NIL,
            left: NIL,
            right: NIL,
            color: RbColor::Black,
        }
    }
}

pub struct EevdfRbTree {
    nodes: [RbNode; EEVDF_MAX_TASKS],
    root: u32,
    count: usize,
}

impl EevdfRbTree {
    pub const fn new() -> Self {
        Self {
            nodes: [RbNode::empty(); EEVDF_MAX_TASKS],
            root: NIL,
            count: 0,
        }
    }

    // Insert task by deadline (O(log n))
    pub fn insert(&mut self, tid: u32, deadline: u64) {
        if (tid as usize) >= EEVDF_MAX_TASKS { return; }

        self.nodes[tid as usize] = RbNode {
            tid,
            deadline,
            parent: NIL,
            left: NIL,
            right: NIL,
            color: RbColor::Red,
        };
        self.count += 1;

        if self.root == NIL {
            self.root = tid;
            self.nodes[tid as usize].color = RbColor::Black;
            return;
        }

        let mut y = NIL;
        let mut x = self.root;
        while x != NIL {
            y = x;
            if deadline < self.nodes[x as usize].deadline {
                x = self.nodes[x as usize].left;
            } else {
                x = self.nodes[x as usize].right;
            }
        }

        self.nodes[tid as usize].parent = y;
        if deadline < self.nodes[y as usize].deadline {
            self.nodes[y as usize].left = tid;
        } else {
            self.nodes[y as usize].right = tid;
        }

        self.rb_insert_fixup(tid);
    }

    // Remove task by TID (O(log n))
    pub fn remove(&mut self, tid: u32) {
        if (tid as usize) >= EEVDF_MAX_TASKS { return; }
        if self.nodes[tid as usize].tid == NIL { return; }

        let z = tid;
        let y = z;
        let y_original_color = self.nodes[y as usize].color;
        let x;

        if self.nodes[z as usize].left == NIL {
            x = self.nodes[z as usize].right;
            self.rb_transplant(z, self.nodes[z as usize].right);
        } else if self.nodes[z as usize].right == NIL {
            x = self.nodes[z as usize].left;
            self.rb_transplant(z, self.nodes[z as usize].left);
        } else {
            let mut y = self.minimum(self.nodes[z as usize].right);
            y_original_color = self.nodes[y as usize].color;
            x = self.nodes[y as usize].right;
            if self.nodes[y as usize].parent == z {
                if x != NIL {
                    self.nodes[x as usize].parent = y;
                }
            } else {
                self.rb_transplant(y, self.nodes[y as usize].right);
                self.nodes[y as usize].right = self.nodes[z as usize].right;
                self.nodes[self.nodes[y as usize].right as usize].parent = y;
            }
            self.rb_transplant(z, y);
            self.nodes[y as usize].left = self.nodes[z as usize].left;
            self.nodes[self.nodes[y as usize].left as usize].parent = y;
            self.nodes[y as usize].color = self.nodes[z as usize].color;
        }

        if y_original_color == RbColor::Black {
            self.rb_delete_fixup(x);
        }

        self.nodes[tid as usize] = RbNode::empty();
        self.count -= 1;
    }

    // Get task with earliest deadline (O(log n) worst case, O(1) amortized)
    pub fn pick_earliest(&self) -> Option<u32> {
        if self.root == NIL { return None; }
        let mut curr = self.root;
        while self.nodes[curr as usize].left != NIL {
            curr = self.nodes[curr as usize].left;
        }
        Some(self.nodes[curr as usize].tid)
    }

    // Update deadline for a task (remove and re-insert)
    pub fn update_deadline(&mut self, tid: u32, new_deadline: u64) {
        self.remove(tid);
        self.insert(tid, new_deadline);
    }

    fn minimum(&self, node: u32) -> u32 {
        let mut curr = node;
        while self.nodes[curr as usize].left != NIL {
            curr = self.nodes[curr as usize].left;
        }
        curr
    }

    fn rb_transplant(&mut self, u: u32, v: u32) {
        if self.nodes[u as usize].parent == NIL {
            self.root = v;
        } else if u == self.nodes[self.nodes[u as usize].parent as usize].left {
            self.nodes[self.nodes[u as usize].parent as usize].left = v;
        } else {
            self.nodes[self.nodes[u as usize].parent as usize].right = v;
        }
        if v != NIL {
            self.nodes[v as usize].parent = self.nodes[u as usize].parent;
        }
    }

    fn rb_insert_fixup(&mut self, mut z: u32) {
        while self.nodes[self.nodes[z as usize].parent as usize].color == RbColor::Red {
            if self.nodes[z as usize].parent == self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].left {
                let y = self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].right;
                if self.nodes[y as usize].color == RbColor::Red {
                    self.nodes[self.nodes[z as usize].parent as usize].color = RbColor::Black;
                    self.nodes[y as usize].color = RbColor::Black;
                    self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].color = RbColor::Red;
                    z = self.nodes[self.nodes[z as usize].parent as usize].parent;
                } else {
                    if z == self.nodes[self.nodes[z as usize].parent as usize].right {
                        z = self.nodes[z as usize].parent;
                        self.left_rotate(z);
                    }
                    self.nodes[self.nodes[z as usize].parent as usize].color = RbColor::Black;
                    self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].color = RbColor::Red;
                    self.right_rotate(self.nodes[self.nodes[z as usize].parent as usize].parent);
                }
            } else {
                let y = self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].left;
                if self.nodes[y as usize].color == RbColor::Red {
                    self.nodes[self.nodes[z as usize].parent as usize].color = RbColor::Black;
                    self.nodes[y as usize].color = RbColor::Black;
                    self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].color = RbColor::Red;
                    z = self.nodes[self.nodes[z as usize].parent as usize].parent;
                } else {
                    if z == self.nodes[self.nodes[z as usize].parent as usize].left {
                        z = self.nodes[z as usize].parent;
                        self.right_rotate(z);
                    }
                    self.nodes[self.nodes[z as usize].parent as usize].color = RbColor::Black;
                    self.nodes[self.nodes[self.nodes[z as usize].parent as usize].parent as usize].color = RbColor::Red;
                    self.left_rotate(self.nodes[self.nodes[z as usize].parent as usize].parent);
                }
            }
        }
        self.nodes[self.root as usize].color = RbColor::Black;
    }

    fn rb_delete_fixup(&mut self, mut x: u32) {
        while x != self.root && self.nodes[x as usize].color == RbColor::Black {
            if x == self.nodes[self.nodes[x as usize].parent as usize].left {
                let mut w = self.nodes[self.nodes[x as usize].parent as usize].right;
                if self.nodes[w as usize].color == RbColor::Red {
                    self.nodes[w as usize].color = RbColor::Black;
                    self.nodes[self.nodes[x as usize].parent as usize].color = RbColor::Red;
                    self.left_rotate(self.nodes[x as usize].parent);
                    w = self.nodes[self.nodes[x as usize].parent as usize].right;
                }
                if self.nodes[self.nodes[w as usize].left as usize].color == RbColor::Black
                    && self.nodes[self.nodes[w as usize].right as usize].color == RbColor::Black {
                    self.nodes[w as usize].color = RbColor::Red;
                    x = self.nodes[x as usize].parent;
                } else {
                    if self.nodes[self.nodes[w as usize].right as usize].color == RbColor::Black {
                        self.nodes[self.nodes[w as usize].left as usize].color = RbColor::Black;
                        self.nodes[w as usize].color = RbColor::Red;
                        self.right_rotate(w);
                        w = self.nodes[self.nodes[x as usize].parent as usize].right;
                    }
                    self.nodes[w as usize].color = self.nodes[self.nodes[x as usize].parent as usize].color;
                    self.nodes[self.nodes[x as usize].parent as usize].color = RbColor::Black;
                    self.nodes[self.nodes[w as usize].right as usize].color = RbColor::Black;
                    self.left_rotate(self.nodes[x as usize].parent);
                    x = self.root;
                }
            } else {
                let mut w = self.nodes[self.nodes[x as usize].parent as usize].left;
                if self.nodes[w as usize].color == RbColor::Red {
                    self.nodes[w as usize].color = RbColor::Black;
                    self.nodes[self.nodes[x as usize].parent as usize].color = RbColor::Red;
                    self.right_rotate(self.nodes[x as usize].parent);
                    w = self.nodes[self.nodes[x as usize].parent as usize].left;
                }
                if self.nodes[self.nodes[w as usize].right as usize].color == RbColor::Black
                    && self.nodes[self.nodes[w as usize].left as usize].color == RbColor::Black {
                    self.nodes[w as usize].color = RbColor::Red;
                    x = self.nodes[x as usize].parent;
                } else {
                    if self.nodes[self.nodes[w as usize].left as usize].color == RbColor::Black {
                        self.nodes[self.nodes[w as usize].right as usize].color = RbColor::Black;
                        self.nodes[w as usize].color = RbColor::Red;
                        self.left_rotate(w);
                        w = self.nodes[self.nodes[x as usize].parent as usize].left;
                    }
                    self.nodes[w as usize].color = self.nodes[self.nodes[x as usize].parent as usize].color;
                    self.nodes[self.nodes[x as usize].parent as usize].color = RbColor::Black;
                    self.nodes[self.nodes[w as usize].left as usize].color = RbColor::Black;
                    self.right_rotate(self.nodes[x as usize].parent);
                    x = self.root;
                }
            }
        }
        if x != NIL {
            self.nodes[x as usize].color = RbColor::Black;
        }
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
}

// ─────────────────────────────────────────────────────────────────────────────
// EEVDF Scheduler with OOP principles
// ─────────────────────────────────────────────────────────────────────────────

pub struct EevdfScheduler {
    entities: [EevdfEntity; EEVDF_MAX_TASKS],
    deadline_tree: EevdfRbTree,
    min_vruntime: u64,
    avg_vruntime: u64,
    total_weight: u64,
    nr_running: usize,
    next_tid: u32,
}

impl EevdfScheduler {
    pub const fn new() -> Self {
        Self {
            entities: [EevdfEntity::empty(); EEVDF_MAX_TASKS],
            deadline_tree: EevdfRbTree::new(),
            min_vruntime: 0,
            avg_vruntime: 0,
            total_weight: 0,
            nr_running: 0,
            next_tid: 1,
        }
    }

    // Convert nice value to weight (Linux-style)
    fn nice_to_weight(&self, nice: i32) -> u64 {
        let idx = (nice + 20).clamp(0, 39) as usize;
        PRIO_TO_WEIGHT[idx]
    }

    // Calculate time slice based on weight and number of running tasks
    fn calc_slice(&self, weight: u64) -> u64 {
        if self.nr_running == 0 { return SYSCTL_SCHED_LATENCY; }
        
        let slice = SYSCTL_SCHED_LATENCY * weight / self.total_weight.max(1);
        slice.max(SYSCTL_SCHED_MIN_GRANULARITY)
    }

    // Enqueue a new task
    pub fn enqueue(&mut self, pid: u32, nice: i32) -> Option<u32> {
        if self.next_tid as usize >= EEVDF_MAX_TASKS { return None; }
        
        let tid = self.next_tid;
        self.next_tid += 1;
        
        let weight = self.nice_to_weight(nice);
        let slice = self.calc_slice(weight);
        
        let entity = EevdfEntity {
            tid,
            pid,
            nice,
            weight,
            vruntime: self.min_vruntime,
            deadline: self.min_vruntime + slice * NICE_0_WEIGHT / weight.max(1),
            slice,
            lag: 0,
            eligible: true,
            active: true,
        };
        
        self.entities[tid as usize] = entity;
        self.deadline_tree.insert(tid, entity.deadline);
        self.total_weight += weight;
        self.nr_running += 1;
        
        Some(tid)
    }

    // Dequeue a task
    pub fn dequeue(&mut self, tid: u32) {
        if (tid as usize) >= EEVDF_MAX_TASKS { return; }
        
        let entity = self.entities[tid as usize];
        if !entity.active { return; }
        
        self.deadline_tree.remove(tid);
        self.total_weight -= entity.total_weight().min(self.total_weight);
        self.nr_running -= 1;
        
        self.entities[tid as usize] = EevdfEntity::empty();
    }

    // Pick next task to run (EEVDF algorithm)
    pub fn pick_next(&mut self) -> Option<u32> {
        if self.nr_running == 0 { return None; }
        
        // Find eligible task with earliest deadline
        let mut best_tid = NIL;
        let mut best_deadline = u64::MAX;
        
        for tid in 0..EEVDF_MAX_TASKS as u32 {
            let entity = self.entities[tid as usize];
            if !entity.active { continue; }
            
            // Check eligibility
            if entity.is_eligible(self.min_vruntime) && entity.deadline < best_deadline {
                best_deadline = entity.deadline;
                best_tid = tid;
            }
        }
        
        if best_tid != NIL {
            Some(best_tid)
        } else {
            // No eligible tasks, pick the one with smallest vruntime
            let mut min_vruntime = u64::MAX;
            for tid in 0..EEVDF_MAX_TASKS as u32 {
                let entity = self.entities[tid as usize];
                if entity.active && entity.vruntime < min_vruntime {
                    min_vruntime = entity.vruntime;
                    best_tid = tid;
                }
            }
            Some(best_tid)
        }
    }

    // Update task runtime after accounting
    pub fn update_runtime(&mut self, tid: u32, delta_ns: u64) {
        if (tid as usize) >= EEVDF_MAX_TASKS { return; }
        
        let entity = &mut self.entities[tid as usize];
        if !entity.active { return; }
        
        // Update vruntime: delta_scaled = delta * NICE_0_WEIGHT / weight
        let delta_scaled = delta_ns * NICE_0_WEIGHT / entity.weight.max(1);
        entity.vruntime += delta_scaled;
        
        // Update lag
        let ideal_runtime = delta_ns * self.total_weight / entity.weight.max(1);
        entity.update_lag(delta_ns, ideal_runtime);
        
        // Recalculate deadline
        let new_deadline = entity.calc_virtual_deadline(entity.vruntime, entity.weight);
        entity.deadline = new_deadline;
        
        // Update tree
        self.deadline_tree.update_deadline(tid, new_deadline);
        
        // Update min_vruntime
        self.update_min_vruntime();
    }

    // Update min_vruntime across all tasks
    fn update_min_vruntime(&mut self) {
        let mut min_vr = u64::MAX;
        let mut sum_vr = 0u64;
        let mut count = 0usize;
        
        for tid in 0..EEVDF_MAX_TASKS as u32 {
            let entity = self.entities[tid as usize];
            if entity.active {
                min_vr = min_vr.min(entity.vruntime);
                sum_vr += entity.vruntime;
                count += 1;
            }
        }
        
        if count > 0 {
            self.min_vruntime = min_vr;
            self.avg_vruntime = sum_vr / count as u64;
        }
    }

    // Get task entity
    pub fn get_entity(&self, tid: u32) -> Option<&EevdfEntity> {
        if (tid as usize) >= EEVDF_MAX_TASKS { return None; }
        let entity = self.entities[tid as usize];
        if entity.active { Some(&entity) } else { None }
    }

    // Get number of running tasks
    pub fn nr_running(&self) -> usize {
        self.nr_running
    }

    // Get total weight
    pub fn total_weight(&self) -> u64 {
        self.total_weight
    }
}

impl EevdfEntity {
    fn total_weight(&self) -> u64 {
        self.weight
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut EEVDF_SCHED: EevdfScheduler = EevdfScheduler::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_init() {
    EEVDF_SCHED = EevdfScheduler::new();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_enqueue(pid: u32, nice: i32) -> u32 {
    EEVDF_SCHED.enqueue(pid, nice).unwrap_or(NIL)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_dequeue(tid: u32) {
    EEVDF_SCHED.dequeue(tid);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_pick_next() -> u32 {
    EEVDF_SCHED.pick_next().unwrap_or(NIL)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_update_runtime(tid: u32, delta_ns: u64) {
    EEVDF_SCHED.update_runtime(tid, delta_ns);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_nr_running() -> usize {
    EEVDF_SCHED.nr_running()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_get_vruntime(tid: u32) -> u64 {
    EEVDF_SCHED.get_entity(tid).map(|e| e.vruntime).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_get_deadline(tid: u32) -> u64 {
    EEVDF_SCHED.get_entity(tid).map(|e| e.deadline).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_eevdf_get_lag(tid: u32) -> i64 {
    EEVDF_SCHED.get_entity(tid).map(|e| e.lag).unwrap_or(0)
}
