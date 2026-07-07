//! SigmaOS — Round-Robin + Priority Scheduler
//! Full preemptive scheduler with task states, priority queues, and time slicing.
//! No std, no allocator — fixed-size task table.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── Scheduler Constants ─────────────────────────────────────────────────────
const MAX_TASKS:        usize = 256;
const MAX_PRIORITY:     u8 = 32;
const DEFAULT_QUANTUM:  U64 = 10_000_000; // 10ms in nanoseconds
const MIN_QUANTUM:      U64 = 1_000_000;  // 1ms
const MAX_QUANTUM:      U64 = 100_000_000; // 100ms
const IDLE_TASK_ID:     U32 = 0;

// ── Task State ──────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum TaskState {
    Free      = 0,
    Ready     = 1,
    Running   = 2,
    Blocked   = 3,
    Sleeping  = 4,
    Zombie    = 5,
    Stopped   = 6,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum SchedPolicy {
    RoundRobin = 0,
    Fifo       = 1,
    Priority   = 2,
    CFS        = 3,  // Completely Fair Scheduler
}

// ── CPU Context (x86-64) ────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuContext {
    pub rax: U64, pub rbx: U64, pub rcx: U64, pub rdx: U64,
    pub rsi: U64, pub rdi: U64, pub rbp: U64, pub rsp: U64,
    pub r8:  U64, pub r9:  U64, pub r10: U64, pub r11: U64,
    pub r12: U64, pub r13: U64, pub r14: U64, pub r15: U64,
    pub rip: U64, pub rflags: U64,
    pub cs:  U64, pub ss:  U64,
    pub cr3: U64,  // Page table root
    pub fs_base: U64, pub gs_base: U64,
    // FPU/SSE state offset (saved separately)
    pub fpu_state: [U8; 512],
}

impl CpuContext {
    pub const fn zero() -> Self {
        CpuContext {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0, rsp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rflags: 0x202, // IF set
            cs: 0x08, ss: 0x10,    // kernel CS/SS
            cr3: 0, fs_base: 0, gs_base: 0,
            fpu_state: [0u8; 512],
        }
    }
}

// ── Task Control Block ──────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub tid:          U32,
    pub pid:          U32,         // Parent process ID
    pub state:        TaskState,
    pub priority:     U8,          // 0 = highest, 31 = lowest
    pub policy:       SchedPolicy,
    pub ctx:          CpuContext,
    pub quantum_ns:   U64,         // Time quantum in nanoseconds
    pub runtime_ns:   U64,         // Total accumulated runtime
    pub vruntime_ns:  U64,         // Virtual runtime (for CFS)
    pub wake_time_ns: U64,         // Wake time for sleeping tasks
    pub exit_code:    i32,
    pub kernel_stack: U64,         // Kernel stack pointer
    pub user_stack:   U64,         // User stack pointer
    pub name:         [U8; 32],
    pub name_len:     usize,
    pub cpu_affinity: U32,         // Bitmask of allowed CPUs
    pub nice:         i8,          // Nice value (-20 to 19)
}

impl TaskControlBlock {
    pub const fn empty() -> Self {
        TaskControlBlock {
            tid: 0, pid: 0,
            state: TaskState::Free,
            priority: 16,
            policy: SchedPolicy::RoundRobin,
            ctx: CpuContext::zero(),
            quantum_ns: DEFAULT_QUANTUM,
            runtime_ns: 0, vruntime_ns: 0,
            wake_time_ns: 0,
            exit_code: 0,
            kernel_stack: 0, user_stack: 0,
            name: [0u8; 32], name_len: 0,
            cpu_affinity: 0xFFFF_FFFF,
            nice: 0,
        }
    }
}

// ── Ready Queue (per-priority) ──────────────────────────────────────────────
const QUEUE_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct ReadyQueue {
    pub tasks: [U32; QUEUE_SIZE], // task IDs
    pub head: usize,
    pub tail: usize,
    pub count: usize,
}

impl ReadyQueue {
    pub const fn new() -> Self {
        ReadyQueue { tasks: [0; QUEUE_SIZE], head: 0, tail: 0, count: 0 }
    }

    pub fn enqueue(&mut self, tid: U32) -> bool {
        if self.count >= QUEUE_SIZE { return false; }
        self.tasks[self.tail] = tid;
        self.tail = (self.tail + 1) % QUEUE_SIZE;
        self.count += 1;
        true
    }

    pub fn dequeue(&mut self) -> Option<U32> {
        if self.count == 0 { return None; }
        let tid = self.tasks[self.head];
        self.head = (self.head + 1) % QUEUE_SIZE;
        self.count -= 1;
        Some(tid)
    }

    pub fn is_empty(&self) -> bool { self.count == 0 }
}

// ── Scheduler State ─────────────────────────────────────────────────────────
pub struct Scheduler {
    pub tasks: [TaskControlBlock; MAX_TASKS],
    pub task_count: usize,
    pub current_tid: U32,
    pub queues: [ReadyQueue; MAX_PRIORITY as usize],
    pub bitmap: U32, // Bitmask of non-empty priority levels
    pub tick_count: U64,
    pub context_switches: U64,
    pub policy: SchedPolicy,
    pub preemption_enabled: bool,
}

static mut SCHED: Scheduler = Scheduler {
    tasks: [TaskControlBlock::empty(); MAX_TASKS],
    task_count: 0,
    current_tid: IDLE_TASK_ID,
    queues: [ReadyQueue::new(); MAX_PRIORITY as usize],
    bitmap: 0,
    tick_count: 0,
    context_switches: 0,
    policy: SchedPolicy::RoundRobin,
    preemption_enabled: true,
};

// ── Scheduling Algorithm ────────────────────────────────────────────────────

unsafe fn find_highest_priority() -> Option<U32> {
    if SCHED.bitmap == 0 { return None; }
    // Find first set bit (highest priority = lowest index)
    let mut prio = 0u8;
    let mut mask = SCHED.bitmap;
    while mask & 1 == 0 && prio < MAX_PRIORITY {
        mask >>= 1;
        prio += 1;
    }
    SCHED.queues[prio as usize].dequeue().map(|tid| {
        if SCHED.queues[prio as usize].is_empty() {
            SCHED.bitmap &= !(1 << prio);
        }
        tid
    })
}

unsafe fn enqueue_task(tid: U32) {
    let prio = SCHED.tasks[tid as usize].priority;
    if prio < MAX_PRIORITY {
        SCHED.queues[prio as usize].enqueue(tid);
        SCHED.bitmap |= 1 << prio;
    }
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the scheduler with an idle task.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_init() -> i32 {
    // Create idle task (TID 0)
    let idle = &mut SCHED.tasks[0];
    idle.tid = IDLE_TASK_ID;
    idle.state = TaskState::Running;
    idle.priority = MAX_PRIORITY - 1; // Lowest priority
    idle.policy = SchedPolicy::RoundRobin;
    idle.name[..4].copy_from_slice(b"idle");
    idle.name_len = 4;
    SCHED.current_tid = IDLE_TASK_ID;
    SCHED.task_count = 1;
    SCHED.preemption_enabled = true;
    0
}

/// Create a new task. Returns task ID or negative error.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_create(
    entry_point: U64,
    stack_top: U64,
    priority: U8,
    name: *const U8,
    name_len: usize,
) -> i32 {
    // Find a free slot
    let mut slot: Option<usize> = None;
    for i in 1..MAX_TASKS {
        if SCHED.tasks[i].state == TaskState::Free {
            slot = Some(i);
            break;
        }
    }
    let idx = match slot {
        Some(i) => i,
        None => return -1, // ENOMEM
    };

    let task = &mut SCHED.tasks[idx];
    task.tid = idx as U32;
    task.pid = SCHED.current_tid;
    task.state = TaskState::Ready;
    task.priority = if priority < MAX_PRIORITY { priority } else { MAX_PRIORITY - 1 };
    task.policy = SCHED.policy;
    task.ctx = CpuContext::zero();
    task.ctx.rip = entry_point;
    task.ctx.rsp = stack_top;
    task.ctx.rflags = 0x202; // IF set
    task.quantum_ns = DEFAULT_QUANTUM;
    task.runtime_ns = 0;
    task.vruntime_ns = 0;
    task.exit_code = 0;
    task.kernel_stack = stack_top;
    task.user_stack = stack_top;

    // Copy name
    let copy_len = if name_len < 32 { name_len } else { 31 };
    if !name.is_null() {
        for i in 0..copy_len {
            task.name[i] = *name.add(i);
        }
        task.name_len = copy_len;
    }

    // Enqueue into ready queue
    enqueue_task(idx as U32);
    SCHED.task_count += 1;

    idx as i32
}

/// Schedule the next task. Returns the TID of the selected task.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_schedule() -> U32 {
    SCHED.tick_count += 1;

    // Put current task back in queue if it's still runnable
    let cur = SCHED.current_tid;
    if SCHED.tasks[cur as usize].state == TaskState::Running {
        SCHED.tasks[cur as usize].state = TaskState::Ready;
        enqueue_task(cur);
    }

    // Pick next task
    let next = find_highest_priority().unwrap_or(IDLE_TASK_ID);
    SCHED.tasks[next as usize].state = TaskState::Running;

    if next != cur {
        SCHED.context_switches += 1;
    }
    SCHED.current_tid = next;
    next
}

/// Block the current task.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_block() -> U32 {
    let cur = SCHED.current_tid;
    SCHED.tasks[cur as usize].state = TaskState::Blocked;
    sigma_sched_schedule()
}

/// Wake a blocked task by TID.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_wake(tid: U32) -> i32 {
    if tid as usize >= MAX_TASKS { return -1; }
    let task = &mut SCHED.tasks[tid as usize];
    if task.state != TaskState::Blocked && task.state != TaskState::Sleeping {
        return -2;
    }
    task.state = TaskState::Ready;
    enqueue_task(tid);
    0
}

/// Sleep the current task for the given nanoseconds.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_sleep(ns: U64) -> U32 {
    let cur = SCHED.current_tid;
    SCHED.tasks[cur as usize].state = TaskState::Sleeping;
    SCHED.tasks[cur as usize].wake_time_ns =
        SCHED.tasks[cur as usize].runtime_ns + ns;
    sigma_sched_schedule()
}

/// Exit the current task.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_exit(code: i32) -> U32 {
    let cur = SCHED.current_tid;
    SCHED.tasks[cur as usize].state = TaskState::Zombie;
    SCHED.tasks[cur as usize].exit_code = code;
    SCHED.task_count -= 1;
    sigma_sched_schedule()
}

/// Yield the current task's remaining quantum.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_yield() -> U32 {
    sigma_sched_schedule()
}

/// Set scheduling policy.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_set_policy(policy: U8) {
    SCHED.policy = match policy {
        0 => SchedPolicy::RoundRobin,
        1 => SchedPolicy::Fifo,
        2 => SchedPolicy::Priority,
        3 => SchedPolicy::CFS,
        _ => SchedPolicy::RoundRobin,
    };
}

/// Get the current task ID.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_current_tid() -> U32 {
    SCHED.current_tid
}

/// Get total context switches.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_context_switches() -> U64 {
    SCHED.context_switches
}

/// Get number of active tasks.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_task_count() -> U32 {
    SCHED.task_count as U32
}

/// Check sleeping tasks and wake them if their time is up.
/// Call this from the timer interrupt handler.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_tick(current_time_ns: U64) {
    for i in 1..MAX_TASKS {
        if SCHED.tasks[i].state == TaskState::Sleeping {
            if current_time_ns >= SCHED.tasks[i].wake_time_ns {
                SCHED.tasks[i].state = TaskState::Ready;
                enqueue_task(i as U32);
            }
        }
    }
    // Add runtime to current task
    let cur = SCHED.current_tid as usize;
    SCHED.tasks[cur].runtime_ns += DEFAULT_QUANTUM;
    // CFS: update vruntime based on nice value
    let weight = match SCHED.tasks[cur].nice {
        n if n < 0 => (DEFAULT_QUANTUM as i64 * 100 / (120 + n as i64)) as U64,
        n => (DEFAULT_QUANTUM as i64 * 100 / (120 + n as i64)) as U64,
    };
    SCHED.tasks[cur].vruntime_ns += weight;
}

/// Disable preemption.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_preempt_disable() {
    SCHED.preemption_enabled = false;
}

/// Enable preemption.
#[no_mangle]
pub unsafe extern "C" fn sigma_sched_preempt_enable() {
    SCHED.preemption_enabled = true;
}
