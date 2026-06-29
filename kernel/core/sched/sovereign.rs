// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Scheduler Core (Rust, no_std)
//! =========================================================================

use super::mlfq::{MlfqQueue, MAX_PROCESSES};

type U32 = u32;
type U64 = u64;
type I32 = i32;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Terminated = 3,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ThreadContext {
    pub pid: U32,
    pub state: ThreadState,
    pub priority: U32,
    pub slice_ticks: U32,
    pub rsp: U64,
    pub cr3: U64,
}

impl ThreadContext {
    pub const fn empty() -> Self {
        ThreadContext {
            pid: 0,
            state: ThreadState::Terminated,
            priority: 0,
            slice_ticks: 0,
            rsp: 0,
            cr3: 0,
        }
    }
}

pub struct SovereignScheduler {
    threads: [ThreadContext; MAX_PROCESSES],
    current_pid: U32,
    mlfq: MlfqQueue,
    tick_count: U64,
    initialized: bool,
}

impl SovereignScheduler {
    pub const fn new() -> Self {
        SovereignScheduler {
            threads: [ThreadContext::empty(); MAX_PROCESSES],
            current_pid: 0,
            mlfq: MlfqQueue::new(),
            tick_count: 0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        if self.initialized { return; }
        self.mlfq.init();
        self.initialized = true;
    }

    pub fn create_thread(&mut self, rsp: U64, cr3: U64) -> I32 {
        if !self.initialized { return -1; }

        let mut i = 1; // PID 0 reserved for idle
        while i < MAX_PROCESSES {
            if matches!(self.threads[i].state, ThreadState::Terminated) {
                self.threads[i].pid = i as U32;
                self.threads[i].state = ThreadState::Ready;
                self.threads[i].priority = 0; // Highest priority in MLFQ
                self.threads[i].slice_ticks = 10;
                self.threads[i].rsp = rsp;
                self.threads[i].cr3 = cr3;

                self.mlfq.enqueue(0, i as U32);
                return i as I32;
            }
            i += 1;
        }
        -1 // Out of PIDs
    }

    pub fn schedule_tick(&mut self) -> U32 {
        if !self.initialized { return 0; }
        self.tick_count += 1;

        // Current thread slice tracking
        if self.current_pid != 0 {
            let current = &mut self.threads[self.current_pid as usize];
            if current.slice_ticks > 0 {
                current.slice_ticks -= 1;
            }
            
            if current.slice_ticks == 0 && matches!(current.state, ThreadState::Running) {
                // Demote priority on time slice expiration
                let mut new_prio = current.priority + 1;
                if new_prio > 3 { new_prio = 3; }
                current.priority = new_prio;
                current.state = ThreadState::Ready;
                
                self.mlfq.enqueue(new_prio, self.current_pid);
                self.current_pid = 0; // Needs reschedule
            }
        }

        // Pick next thread if needed
        if self.current_pid == 0 {
            if let Some(next_pid) = self.mlfq.dequeue() {
                self.current_pid = next_pid;
                let next = &mut self.threads[next_pid as usize];
                next.state = ThreadState::Running;
                next.slice_ticks = 10 * (next.priority + 1); // Lower priority = longer slice
            }
        }

        self.current_pid
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_SCHED: SovereignScheduler = SovereignScheduler::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sched_init_shard() {
    G_SCHED.init();
}

#[no_mangle]
pub unsafe extern "C" fn sched_create_thread_shard(rsp: U64, cr3: U64) -> I32 {
    G_SCHED.create_thread(rsp, cr3)
}

#[no_mangle]
pub unsafe extern "C" fn sched_tick_shard() -> U32 {
    G_SCHED.schedule_tick()
}
