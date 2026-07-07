//! SigmaOS — Completely Fair Scheduler (CFS) Implementation
//! Native bare-metal scheduler. No stdlib, no external crates.
//! Inspired by Linux CFS but redesigned for capability-based microkernel model.

#![no_std]
#![allow(dead_code)]

// ── Primitive type aliases to avoid libc/stdlib dependency ──────────────────
type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type I64 = i64;
type Usize = usize;
type Bool = bool;

// ── Constants ────────────────────────────────────────────────────────────────
const MAX_TASKS:         Usize = 1024;
const MAX_CPUS:          Usize = 256;
const SCHED_LATENCY_NS:  U64   = 20_000_000;   // 20 ms target latency
const MIN_GRAN_NS:       U64   = 750_000;       // 0.75 ms minimum granularity
const NICE_0_LOAD:       U64   = 1024;
const NICE_TO_WEIGHT: [U64; 40] = [
    // nice: -20 .. 19
    88761, 71755, 56483, 46273, 36291,
    29154, 23254, 18705, 14949, 11916,
     9548,  7620,  6100,  4904,  3906,
     3121,  2501,  1991,  1586,  1277,
     1024,   820,   655,   526,   423,
      335,   272,   215,   172,   137,
      110,    87,    70,    56,    45,
       36,    29,    23,    18,    15,
];

// ── Red-Black tree node (intrusive, index-based — no heap allocation) ────────
#[repr(C)]
#[derive(Copy, Clone)]
struct RbNode {
    parent: U16,   // index into task table; 0xFFFF = no parent
    left:   U16,
    right:  U16,
    red:    Bool,
}

impl RbNode {
    const NULL: U16 = 0xFFFF;
    const fn nil() -> Self {
        RbNode { parent: Self::NULL, left: Self::NULL, right: Self::NULL, red: false }
    }
}

// ── Task control block ────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Task {
    pub pid:         U32,
    pub tgid:        U32,
    pub state:       TaskState,
    pub nice:        I32,
    pub weight:      U64,          // derived from nice via NICE_TO_WEIGHT
    pub vruntime:    U64,          // virtual runtime (ns)
    pub sum_exec:    U64,          // total CPU ns consumed
    pub deadline:    U64,          // EDF deadline (ns, 0 = CFS)
    pub cpu:         U8,           // pinned CPU (0xFF = any)
    pub policy:      Policy,
    pub cap_token:   U64,          // capability token (SigmaOS security)
    rb:              RbNode,
}

impl Task {
    pub const fn zero() -> Self {
        Task {
            pid: 0, tgid: 0, state: TaskState::Dead, nice: 0,
            weight: NICE_0_LOAD, vruntime: 0, sum_exec: 0,
            deadline: 0, cpu: 0xFF, policy: Policy::Normal, cap_token: 0,
            rb: RbNode::nil(),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskState {
    Running       = 0,
    Runnable      = 1,
    Sleeping      = 2,
    Stopped       = 3,
    Zombie        = 4,
    Dead          = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Policy {
    Normal    = 0,   // CFS
    RealTimeFifo = 1,
    RealTimeRR   = 2,
    Idle      = 3,
    Deadline  = 4,   // EDF
}

// ── Per-CPU run queue ────────────────────────────────────────────────────────
#[repr(C)]
pub struct RunQueue {
    pub nr_running:   U32,
    pub min_vruntime: U64,
    pub clock:        U64,         // monotonic ns counter
    rb_root:          U16,         // index of rb-tree root (0xFFFF = empty)
    pub curr:         U16,         // index of currently running task
    pub idle:         U16,         // idle task index
}

impl RunQueue {
    pub const fn new() -> Self {
        RunQueue {
            nr_running: 0,
            min_vruntime: 0,
            clock: 0,
            rb_root: RbNode::NULL,
            curr: RbNode::NULL,
            idle: RbNode::NULL,
        }
    }
}

// ── Global state ─────────────────────────────────────────────────────────────
static mut TASKS:  [Task; MAX_TASKS]    = [Task::zero(); MAX_TASKS];
static mut RQS:    [RunQueue; MAX_CPUS] = {
    let mut a = [RunQueue::new(); MAX_CPUS];
    // const initialisation — zero is valid
    a
};
static mut NR_CPUS: U32 = 1;

// ── Weight from nice ────────────────────────────────────────────────────────
fn nice_to_weight(nice: I32) -> U64 {
    let idx = (nice + 20) as Usize;
    if idx < 40 { NICE_TO_WEIGHT[idx] } else { NICE_TO_WEIGHT[39] }
}

// ── Virtual runtime delta ────────────────────────────────────────────────────
/// Convert wall-clock delta to vruntime delta.
/// vruntime_delta = wall_delta * NICE_0_LOAD / weight
fn calc_vruntime(wall_ns: U64, weight: U64) -> U64 {
    if weight == 0 { return wall_ns; }
    wall_ns.saturating_mul(NICE_0_LOAD) / weight
}

// ── Time slice ───────────────────────────────────────────────────────────────
/// Return the ideal time slice for a task given the run queue size.
fn ideal_slice(nr: U32, weight: U64, total_weight: U64) -> U64 {
    let latency = if nr < 8 { SCHED_LATENCY_NS } else { MIN_GRAN_NS * nr as U64 };
    if total_weight == 0 { return latency; }
    (latency * weight / total_weight).max(MIN_GRAN_NS)
}

// ── Rb-tree helpers (intrusive, index-based) ─────────────────────────────────
unsafe fn rb_left_rotate(rq: &mut RunQueue, x: U16) {
    let tasks = &mut TASKS;
    let y = tasks[x as Usize].rb.right;
    if y == RbNode::NULL { return; }
    tasks[x as Usize].rb.right = tasks[y as Usize].rb.left;
    if tasks[y as Usize].rb.left != RbNode::NULL {
        tasks[tasks[y as Usize].rb.left as Usize].rb.parent = x;
    }
    tasks[y as Usize].rb.parent = tasks[x as Usize].rb.parent;
    let p = tasks[x as Usize].rb.parent;
    if p == RbNode::NULL {
        rq.rb_root = y;
    } else if tasks[p as Usize].rb.left == x {
        tasks[p as Usize].rb.left = y;
    } else {
        tasks[p as Usize].rb.right = y;
    }
    tasks[y as Usize].rb.left = x;
    tasks[x as Usize].rb.parent = y;
}

unsafe fn rb_right_rotate(rq: &mut RunQueue, y: U16) {
    let tasks = &mut TASKS;
    let x = tasks[y as Usize].rb.left;
    if x == RbNode::NULL { return; }
    tasks[y as Usize].rb.left = tasks[x as Usize].rb.right;
    if tasks[x as Usize].rb.right != RbNode::NULL {
        tasks[tasks[x as Usize].rb.right as Usize].rb.parent = y;
    }
    tasks[x as Usize].rb.parent = tasks[y as Usize].rb.parent;
    let p = tasks[y as Usize].rb.parent;
    if p == RbNode::NULL {
        rq.rb_root = x;
    } else if tasks[p as Usize].rb.left == y {
        tasks[p as Usize].rb.left = x;
    } else {
        tasks[p as Usize].rb.right = x;
    }
    tasks[x as Usize].rb.right = y;
    tasks[y as Usize].rb.parent = x;
}

/// Insert a task into the rb-tree ordered by vruntime.
unsafe fn rb_insert(rq: &mut RunQueue, idx: U16) {
    let vr = TASKS[idx as Usize].vruntime;
    let tasks = &mut TASKS;
    tasks[idx as Usize].rb = RbNode { parent: RbNode::NULL, left: RbNode::NULL, right: RbNode::NULL, red: true };

    if rq.rb_root == RbNode::NULL {
        rq.rb_root = idx;
        tasks[idx as Usize].rb.red = false;
        return;
    }

    let mut cur = rq.rb_root;
    loop {
        let cur_vr = tasks[cur as Usize].vruntime;
        if vr < cur_vr {
            let l = tasks[cur as Usize].rb.left;
            if l == RbNode::NULL {
                tasks[cur as Usize].rb.left = idx;
                tasks[idx as Usize].rb.parent = cur;
                break;
            }
            cur = l;
        } else {
            let r = tasks[cur as Usize].rb.right;
            if r == RbNode::NULL {
                tasks[cur as Usize].rb.right = idx;
                tasks[idx as Usize].rb.parent = cur;
                break;
            }
            cur = r;
        }
    }

    // Fix-up red-black properties
    let mut z = idx;
    while tasks[z as Usize].rb.red {
        let p = tasks[z as Usize].rb.parent;
        if p == RbNode::NULL { break; }
        if !tasks[p as Usize].rb.red { break; }
        let g = tasks[p as Usize].rb.parent;
        if g == RbNode::NULL { break; }

        if tasks[g as Usize].rb.left == p {
            let u = tasks[g as Usize].rb.right;
            if u != RbNode::NULL && tasks[u as Usize].rb.red {
                tasks[p as Usize].rb.red = false;
                tasks[u as Usize].rb.red = false;
                tasks[g as Usize].rb.red = true;
                z = g;
            } else {
                if tasks[p as Usize].rb.right == z {
                    z = p;
                    rb_left_rotate(rq, z);
                }
                let p2 = tasks[z as Usize].rb.parent;
                let g2 = tasks[p2 as Usize].rb.parent;
                tasks[p2 as Usize].rb.red = false;
                if g2 != RbNode::NULL { tasks[g2 as Usize].rb.red = true; }
                rb_right_rotate(rq, if g2 == RbNode::NULL { rq.rb_root } else { g2 });
            }
        } else {
            let u = tasks[g as Usize].rb.left;
            if u != RbNode::NULL && tasks[u as Usize].rb.red {
                tasks[p as Usize].rb.red = false;
                tasks[u as Usize].rb.red = false;
                tasks[g as Usize].rb.red = true;
                z = g;
            } else {
                if tasks[p as Usize].rb.left == z {
                    z = p;
                    rb_right_rotate(rq, z);
                }
                let p2 = tasks[z as Usize].rb.parent;
                let g2 = tasks[p2 as Usize].rb.parent;
                tasks[p2 as Usize].rb.red = false;
                if g2 != RbNode::NULL { tasks[g2 as Usize].rb.red = true; }
                rb_left_rotate(rq, if g2 == RbNode::NULL { rq.rb_root } else { g2 });
            }
        }
    }
    tasks[rq.rb_root as Usize].rb.red = false;
}

/// Return the leftmost (minimum vruntime) task index.
unsafe fn rb_leftmost(rq: &RunQueue) -> U16 {
    if rq.rb_root == RbNode::NULL { return RbNode::NULL; }
    let mut cur = rq.rb_root;
    loop {
        let l = TASKS[cur as Usize].rb.left;
        if l == RbNode::NULL { return cur; }
        cur = l;
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Initialise the scheduler with `nr_cpus` CPUs.
#[no_mangle]
pub unsafe extern "C" fn cfs_init(nr_cpus: U32) {
    NR_CPUS = nr_cpus.min(MAX_CPUS as U32);
    for i in 0..MAX_TASKS { TASKS[i] = Task::zero(); }
    for i in 0..MAX_CPUS { RQS[i] = RunQueue::new(); }
}

/// Allocate a task slot and return its index (or U16::MAX on error).
#[no_mangle]
pub unsafe extern "C" fn cfs_task_create(
    pid: U32, tgid: U32, nice: I32, policy: U8, cap_token: U64,
) -> U16 {
    for i in 1..MAX_TASKS {
        if TASKS[i].state == TaskState::Dead {
            let weight = nice_to_weight(nice);
            TASKS[i] = Task {
                pid, tgid,
                state: TaskState::Runnable,
                nice, weight,
                vruntime: 0,
                sum_exec: 0,
                deadline: 0,
                cpu: 0xFF,
                policy: match policy {
                    1 => Policy::RealTimeFifo,
                    2 => Policy::RealTimeRR,
                    3 => Policy::Idle,
                    4 => Policy::Deadline,
                    _ => Policy::Normal,
                },
                cap_token,
                rb: RbNode::nil(),
            };
            return i as U16;
        }
    }
    U16::MAX
}

/// Enqueue task `idx` onto the run queue for CPU `cpu`.
#[no_mangle]
pub unsafe extern "C" fn cfs_enqueue(cpu: U8, idx: U16) {
    if idx as Usize >= MAX_TASKS { return; }
    let cpu = cpu as Usize;
    if cpu >= MAX_CPUS { return; }
    // Normalise vruntime to avoid starvation
    let min_vr = RQS[cpu].min_vruntime;
    if TASKS[idx as Usize].vruntime < min_vr {
        TASKS[idx as Usize].vruntime = min_vr;
    }
    TASKS[idx as Usize].state = TaskState::Runnable;
    RQS[cpu].nr_running += 1;
    rb_insert(&mut RQS[cpu], idx);
}

/// Pick the next task to run on `cpu`. Returns task index or U16::MAX for idle.
#[no_mangle]
pub unsafe extern "C" fn cfs_pick_next(cpu: U8) -> U16 {
    let cpu = cpu as Usize;
    if cpu >= MAX_CPUS { return U16::MAX; }
    let next = rb_leftmost(&RQS[cpu]);
    if next != RbNode::NULL {
        TASKS[next as Usize].state = TaskState::Running;
        RQS[cpu].curr = next;
    }
    next
}

/// Called every tick: advances clock, checks if current task exceeded slice.
/// Returns `true` if a reschedule is needed.
#[no_mangle]
pub unsafe extern "C" fn cfs_tick(cpu: U8, tick_ns: U64) -> Bool {
    let cpu = cpu as Usize;
    if cpu >= MAX_CPUS { return false; }
    let rq = &mut RQS[cpu];
    rq.clock = rq.clock.wrapping_add(tick_ns);
    let curr = rq.curr;
    if curr == RbNode::NULL { return false; }

    let task = &mut TASKS[curr as Usize];
    let vdelta = calc_vruntime(tick_ns, task.weight);
    task.vruntime = task.vruntime.wrapping_add(vdelta);
    task.sum_exec = task.sum_exec.wrapping_add(tick_ns);

    // Compute total weight of run queue (approximate via nr_running)
    let total_w = NICE_0_LOAD * rq.nr_running as U64;
    let slice = ideal_slice(rq.nr_running, task.weight, total_w);

    // Update min_vruntime
    let left = rb_leftmost(rq);
    if left != RbNode::NULL && left != curr {
        rq.min_vruntime = TASKS[left as Usize].vruntime;
    }

    // Reschedule if vruntime exceeds fair share
    let left_vr = if left != RbNode::NULL { TASKS[left as Usize].vruntime } else { task.vruntime };
    task.vruntime > left_vr + calc_vruntime(slice, task.weight)
}

/// Account exec time for current task and yield CPU willingly.
#[no_mangle]
pub unsafe extern "C" fn cfs_yield(cpu: U8) {
    let cpu = cpu as Usize;
    if cpu >= MAX_CPUS { return; }
    let curr = RQS[cpu].curr;
    if curr == RbNode::NULL { return; }
    // Bump vruntime slightly so another task gets picked
    TASKS[curr as Usize].vruntime += MIN_GRAN_NS;
}

/// Block the current task (move it out of the run queue).
#[no_mangle]
pub unsafe extern "C" fn cfs_block(cpu: U8, reason: TaskState) {
    let cpu_idx = cpu as Usize;
    if cpu_idx >= MAX_CPUS { return; }
    let curr = RQS[cpu_idx].curr;
    if curr == RbNode::NULL { return; }
    TASKS[curr as Usize].state = reason;
    if RQS[cpu_idx].nr_running > 0 { RQS[cpu_idx].nr_running -= 1; }
    RQS[cpu_idx].curr = RbNode::NULL;
}

/// Wake up a sleeping task and enqueue it.
#[no_mangle]
pub unsafe extern "C" fn cfs_wakeup(cpu: U8, idx: U16) {
    if idx as Usize >= MAX_TASKS { return; }
    if TASKS[idx as Usize].state == TaskState::Sleeping
    || TASKS[idx as Usize].state == TaskState::Stopped {
        cfs_enqueue(cpu, idx);
    }
}

/// Destroy a task slot.
#[no_mangle]
pub unsafe extern "C" fn cfs_task_destroy(idx: U16) {
    if idx as Usize >= MAX_TASKS { return; }
    TASKS[idx as Usize] = Task::zero();
}

/// Simple O(n) load-balancing: move one task from busiest CPU to least-loaded.
#[no_mangle]
pub unsafe extern "C" fn cfs_load_balance() {
    let n = NR_CPUS as Usize;
    if n < 2 { return; }
    let (mut max_cpu, mut min_cpu) = (0, 0);
    for i in 1..n {
        if RQS[i].nr_running > RQS[max_cpu].nr_running { max_cpu = i; }
        if RQS[i].nr_running < RQS[min_cpu].nr_running { min_cpu = i; }
    }
    if RQS[max_cpu].nr_running <= RQS[min_cpu].nr_running + 1 { return; }
    // Find a migratable task on max_cpu
    let mut victim = RbNode::NULL;
    for i in 0..MAX_TASKS {
        if TASKS[i].cpu == max_cpu as U8 || TASKS[i].cpu == 0xFF {
            if TASKS[i].state == TaskState::Runnable {
                victim = i as U16;
                break;
            }
        }
    }
    if victim == RbNode::NULL { return; }
    // Move victim to min_cpu
    if RQS[max_cpu].nr_running > 0 { RQS[max_cpu].nr_running -= 1; }
    TASKS[victim as Usize].cpu = min_cpu as U8;
    cfs_enqueue(min_cpu as U8, victim);
}
