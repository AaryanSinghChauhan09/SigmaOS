// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/process_manager.rs — Process & thread management
//
// Implements fork(), exec(), exit(), wait(), process table,
// task context blocks, and wires into the scheduler.
//
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ── PID allocator ─────────────────────────────────────────────────────────
static NEXT_PID: AtomicU32 = AtomicU32::new(2); // 0=idle, 1=init
fn alloc_pid() -> u32 { NEXT_PID.fetch_add(1, Ordering::Relaxed) }

// ── Task context (matches arch/x86_64/context_switch.asm) ─────────────────
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TaskContext {
    pub rsp:    u64,
    pub r15:    u64,
    pub r14:    u64,
    pub r13:    u64,
    pub r12:    u64,
    pub rbp:    u64,
    pub rbx:    u64,
    pub rip:    u64,  // return address / entry point
    pub cr3:    u64,  // page table physical address
    pub rflags: u64,
}

// ── Task state ────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum TaskState {
    Unused   = 0,
    Embryo   = 1,  // being created
    Sleeping = 2,  // blocked on I/O or event
    Runnable = 3,  // ready to run
    Running  = 4,  // currently on CPU
    Zombie   = 5,  // exited, waiting for parent wait()
}

// ── Process Control Block ─────────────────────────────────────────────────
const KSTACK_SIZE:   usize = 65536;     // 64 KB kernel stack per task
const MAX_TASKS:     usize = 256;
const MAX_OPEN_FDS:  usize = 256;
const TASK_NAME_LEN: usize = 32;

#[repr(C)]
pub struct Task {
    pub pid:       u32,
    pub ppid:      u32,
    pub state:     TaskState,
    pub exit_code: i32,
    pub ctx:       TaskContext,
    pub name:      [u8; TASK_NAME_LEN],
    pub open_fds:  [i32; MAX_OPEN_FDS],  // -1 = unused
    pub cwd_ino:   u64,
    pub uid:       u32,
    pub gid:       u32,
    pub kstack:    [u8; KSTACK_SIZE],  // kernel stack
    pub sched_policy: u8,
    pub mlfq_level:   u8,
    pub vruntime:     u64,
    pub deadline:     u64,
}

impl Task {
    pub const fn empty() -> Self {
        Self {
            pid: 0, ppid: 0, state: TaskState::Unused, exit_code: 0,
            ctx: TaskContext {
                rsp: 0, r15: 0, r14: 0, r13: 0, r12: 0,
                rbp: 0, rbx: 0, rip: 0, cr3: 0, rflags: 0x202, // IF set
            },
            name:        [0u8; TASK_NAME_LEN],
            open_fds:    [-1i32; MAX_OPEN_FDS],
            cwd_ino:     0,
            uid: 0, gid: 0,
            kstack:      [0u8; KSTACK_SIZE],
            sched_policy: 0,
            mlfq_level:   0,
            vruntime:     0,
            deadline:     0,
        }
    }

    pub fn set_name(&mut self, name: &[u8]) {
        let len = name.len().min(TASK_NAME_LEN - 1);
        self.name[..len].copy_from_slice(&name[..len]);
        self.name[len] = 0;
    }

    /// Set up a kernel-mode stack so the task can be context-switched to.
    /// `entry` = function pointer (no_std fn() -> !)
    pub fn setup_kstack(&mut self, entry: u64, arg: u64) {
        // Place initial "stack frame" at top of kstack
        let kstack_top = self.kstack.as_ptr() as u64 + KSTACK_SIZE as u64;
        // Align to 16 bytes, leave room for return address
        let rsp = (kstack_top - 8) & !0xF;

        // Write entry point as "return address" on stack
        unsafe {
            let sp_ptr = rsp as *mut u64;
            *sp_ptr = entry;
        }

        self.ctx.rsp    = rsp;
        self.ctx.rip    = entry;
        self.ctx.rflags = 0x202; // interrupts enabled
        // arg in rdi (first arg via System V ABI) — stored in r15 slot
        // which we'll pop into rdi in the trampoline
        self.ctx.r15 = arg;
    }
}

// ── Process table ─────────────────────────────────────────────────────────
pub struct ProcessManager {
    tasks:       [Task; MAX_TASKS],
    current_pid: u32,
    task_count:  usize,
    initialized: bool,
}

impl ProcessManager {
    pub const fn new() -> Self {
        Self {
            tasks:       [const { Task::empty() }; MAX_TASKS],
            current_pid: 0,
            task_count:  0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        // Create idle task (PID 0)
        self.tasks[0].pid   = 0;
        self.tasks[0].ppid  = 0;
        self.tasks[0].state = TaskState::Running;
        self.tasks[0].set_name(b"idle");

        // Create init task (PID 1)
        self.tasks[1].pid   = 1;
        self.tasks[1].ppid  = 0;
        self.tasks[1].state = TaskState::Embryo;
        self.tasks[1].set_name(b"init");
        self.tasks[1].setup_kstack(init_entry as u64, 0);
        self.tasks[1].state = TaskState::Runnable;

        self.current_pid = 0;
        self.task_count  = 2;
        self.initialized = true;

        // Register both tasks with scheduler
        unsafe {
            extern "C" {
                fn sched_add_task(pid: u32, policy: u8, deadline: u64, level: u8) -> i32;
            }
            sched_add_task(0, 0, 0, 3); // idle: MLFQ Q3
            sched_add_task(1, 0, 0, 0); // init: MLFQ Q0
        }
    }

    fn find_free_slot(&self) -> Option<usize> {
        for i in 0..MAX_TASKS {
            if self.tasks[i].state == TaskState::Unused { return Some(i); }
        }
        None
    }

    fn find_by_pid(&self, pid: u32) -> Option<usize> {
        for i in 0..MAX_TASKS {
            if self.tasks[i].pid == pid &&
               self.tasks[i].state != TaskState::Unused {
                return Some(i);
            }
        }
        None
    }

    /// fork(): duplicate current task → new task
    pub fn fork(&mut self) -> i32 {
        let parent_idx = match self.find_by_pid(self.current_pid) {
            Some(i) => i, None => return -12,
        };
        let child_slot = match self.find_free_slot() {
            Some(i) => i, None => return -12,
        };
        let child_pid = alloc_pid();

        // Copy parent task to child
        let parent_ctx = self.tasks[parent_idx].ctx;
        let parent_fds = self.tasks[parent_idx].open_fds;
        let parent_cwd = self.tasks[parent_idx].cwd_ino;

        let child = &mut self.tasks[child_slot];
        child.pid         = child_pid;
        child.ppid        = self.current_pid;
        child.state       = TaskState::Runnable;
        child.ctx         = parent_ctx;
        child.ctx.r15     = 0;       // child fork() returns 0
        child.open_fds    = parent_fds;
        child.cwd_ino     = parent_cwd;
        child.sched_policy = 0;
        child.mlfq_level   = 0;
        child.set_name(b"child");
        self.task_count += 1;

        unsafe {
            extern "C" { fn sched_add_task(pid: u32, p: u8, d: u64, l: u8) -> i32; }
            sched_add_task(child_pid, 0, 0, 0);
        }

        child_pid as i32  // parent gets child PID
    }

    /// exit(code): mark task as zombie, notify parent
    pub fn exit(&mut self, pid: u32, code: i32) {
        if let Some(idx) = self.find_by_pid(pid) {
            self.tasks[idx].state     = TaskState::Zombie;
            self.tasks[idx].exit_code = code;
            // Wake parent if it's sleeping in wait()
            let ppid = self.tasks[idx].ppid;
            if let Some(pidx) = self.find_by_pid(ppid) {
                if self.tasks[pidx].state == TaskState::Sleeping {
                    self.tasks[pidx].state = TaskState::Runnable;
                }
            }
        }
    }

    /// wait4(): reap a zombie child
    pub fn wait4(&mut self, parent_pid: u32) -> (i32, i32) {
        // Find a zombie child of parent_pid
        for i in 0..MAX_TASKS {
            if self.tasks[i].state == TaskState::Zombie
               && self.tasks[i].ppid == parent_pid
            {
                let child_pid  = self.tasks[i].pid as i32;
                let exit_code  = self.tasks[i].exit_code;
                self.tasks[i]  = Task::empty();
                self.task_count -= 1;
                return (child_pid, exit_code);
            }
        }
        (-11, 0) // EAGAIN — no zombie child yet
    }

    /// Schedule: save current, switch to next PID
    pub unsafe fn schedule(&mut self, next_pid: u32) {
        if next_pid == self.current_pid { return; }
        let from_idx = match self.find_by_pid(self.current_pid) {
            Some(i) => i, None => return,
        };
        let to_idx = match self.find_by_pid(next_pid) {
            Some(i) => i, None => return,
        };

        self.tasks[from_idx].state = TaskState::Runnable;
        self.tasks[to_idx].state   = TaskState::Running;
        self.current_pid = next_pid;

        extern "C" {
            fn sigma_context_switch(from: *mut TaskContext, to: *const TaskContext);
        }
        let from_ctx = &mut self.tasks[from_idx].ctx as *mut TaskContext;
        let to_ctx   = &self.tasks[to_idx].ctx   as *const TaskContext;
        sigma_context_switch(from_ctx, to_ctx);
    }

    pub fn current_pid(&self) -> u32 { self.current_pid }
    pub fn task_count(&self) -> usize { self.task_count }
}

// ── Init task entry point ─────────────────────────────────────────────────
unsafe extern "C" fn init_entry() -> ! {
    extern "C" { fn sigma_log(msg: *const u8, len: usize); }
    let msg = b"[init] PID 1 running\n";
    sigma_log(msg.as_ptr(), msg.len());
    loop { core::arch::asm!("hlt", options(nomem, nostack)); }
}

// ── Global process manager ────────────────────────────────────────────────
static mut G_PM: ProcessManager = ProcessManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn process_manager_init() {
    G_PM.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_fork() -> i32 {
    G_PM.fork()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_exit(code: i32) {
    G_PM.exit(G_PM.current_pid(), code);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_wait4() -> i32 {
    let (child_pid, _) = G_PM.wait4(G_PM.current_pid());
    child_pid
}

#[no_mangle]
pub unsafe extern "C" fn sigma_getpid() -> u32 {
    G_PM.current_pid()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_gettid() -> u32 {
    G_PM.current_pid() // simplified: TID == PID for now
}

#[no_mangle]
pub unsafe extern "C" fn sigma_schedule(next_pid: u32) {
    G_PM.schedule(next_pid);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_sleep_ms(ms: u64) {
    // Spin-sleep using jiffies counter
    extern "C" { fn sigma_jiffies() -> u64; }
    let start = sigma_jiffies();
    while sigma_jiffies() < start + ms {
        core::arch::asm!("pause", options(nomem, nostack));
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_task_count() -> usize {
    G_PM.task_count()
}
