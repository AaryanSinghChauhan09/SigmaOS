// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS CFS Scheduler — Completely Fair Scheduler (vruntime, runqueue)
//! Inspired by Linux CFS. Hand-rolled min-heap instead of red-black tree (no alloc).
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

pub const CFS_MAX_TASKS: usize = 64;

/// A CFS runqueue entity — one per schedulable task.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CfsEntity {
    pub pid:      SigmaU32,
    pub vruntime: SigmaU64,   // virtual runtime in nanoseconds
    pub weight:   SigmaU32,   // nice → weight (1024 = normal)
    pub active:   bool,
}

/// Min-heap ordered by vruntime (the leftmost task runs next).
static mut CFS_HEAP: [CfsEntity; CFS_MAX_TASKS] = [CfsEntity {
    pid: 0, vruntime: 0, weight: 1024, active: false,
}; CFS_MAX_TASKS];
static mut CFS_HEAP_SIZE: usize = 0;
static mut CFS_MIN_VRUNTIME: SigmaU64 = 0;

/// Convert Linux-style nice (-20..+19) to CFS weight.
fn nice_to_weight(nice: i32) -> SigmaU32 {
    // Prio-to-weight table (Linux sched.c excerpt, 40 entries)
    const PRIO_TO_WEIGHT: [u32; 40] = [
        88761, 71755, 56483, 46273, 36291, 29154, 23254, 18705, 14949, 11916,
         9548,  7620,  6100,  4904,  3906,  3121,  2501,  1991,  1586,  1277,
         1024,   820,   655,   526,   423,   335,   272,   215,   172,   137,
          110,    87,    70,    56,    45,    36,    29,    23,    18,    15,
    ];
    let idx = (nice + 20).clamp(0, 39) as usize;
    PRIO_TO_WEIGHT[idx]
}

unsafe fn heap_sift_up(mut i: usize) {
    while i > 0 {
        let parent = (i - 1) / 2;
        if CFS_HEAP[i].vruntime < CFS_HEAP[parent].vruntime {
            let tmp = CFS_HEAP[i];
            CFS_HEAP[i] = CFS_HEAP[parent];
            CFS_HEAP[parent] = tmp;
            i = parent;
        } else { break; }
    }
}

unsafe fn heap_sift_down(mut i: usize) {
    let size = CFS_HEAP_SIZE;
    loop {
        let l = 2 * i + 1;
        let r = 2 * i + 2;
        let mut smallest = i;
        if l < size && CFS_HEAP[l].vruntime < CFS_HEAP[smallest].vruntime { smallest = l; }
        if r < size && CFS_HEAP[r].vruntime < CFS_HEAP[smallest].vruntime { smallest = r; }
        if smallest == i { break; }
        let tmp = CFS_HEAP[i];
        CFS_HEAP[i] = CFS_HEAP[smallest];
        CFS_HEAP[smallest] = tmp;
        i = smallest;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_init() {
    CFS_HEAP_SIZE = 0;
    CFS_MIN_VRUNTIME = 0;
}

/// Enqueue a task with given pid and nice value.
#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_enqueue(pid: SigmaU32, nice: SigmaI32) {
    if CFS_HEAP_SIZE >= CFS_MAX_TASKS { return; }
    let weight = nice_to_weight(nice);
    let entity = CfsEntity {
        pid,
        vruntime: CFS_MIN_VRUNTIME,  // start at min_vruntime for fairness
        weight,
        active: true,
    };
    CFS_HEAP[CFS_HEAP_SIZE] = entity;
    CFS_HEAP_SIZE += 1;
    heap_sift_up(CFS_HEAP_SIZE - 1);
}

/// Pick the task with lowest vruntime (leftmost in CFS terminology).
#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_pick_next() -> SigmaU32 {
    if CFS_HEAP_SIZE == 0 { return 0; }
    CFS_HEAP[0].pid
}

/// Account for delta_ns nanoseconds of runtime for the current task.
/// Updates vruntime and re-heapifies.
#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_update_vruntime(pid: SigmaU32, delta_ns: SigmaU64) {
    // Find the task in the heap (O(n) scan — acceptable for small N)
    for i in 0..CFS_HEAP_SIZE {
        if CFS_HEAP[i].pid == pid {
            // vruntime += delta * (NICE_0_WEIGHT / weight) — scaled runtime
            let scaled = delta_ns * 1024 / CFS_HEAP[i].weight as SigmaU64;
            CFS_HEAP[i].vruntime += scaled;
            // Update global min_vruntime
            if CFS_HEAP[0].vruntime > CFS_MIN_VRUNTIME {
                CFS_MIN_VRUNTIME = CFS_HEAP[0].vruntime;
            }
            // Re-heapify from position i
            heap_sift_down(i);
            break;
        }
    }
}

/// Dequeue a task (task exited or sleeping).
#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_dequeue(pid: SigmaU32) {
    for i in 0..CFS_HEAP_SIZE {
        if CFS_HEAP[i].pid == pid {
            CFS_HEAP_SIZE -= 1;
            CFS_HEAP[i] = CFS_HEAP[CFS_HEAP_SIZE];
            heap_sift_down(i);
            return;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_cfs_runqueue_size() -> SigmaU32 {
    CFS_HEAP_SIZE as SigmaU32
}
