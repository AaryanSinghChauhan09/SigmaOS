// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_process.rs — Process Management (fork/exec/wait)
// Language: Rust #![no_std]
// Pattern: OOP via ProcessManager struct + Process struct

#![no_std]

use core::sync::atomic::{AtomicU32, Ordering};

// ── PID Allocator ─────────────────────────────────────────────────────────────

static NEXT_PID: AtomicU32 = AtomicU32::new(2); // 1 = init

fn alloc_pid() -> u32 {
    NEXT_PID.fetch_add(1, Ordering::Relaxed)
}

// ── Process State ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProcessState {
    Running,
    Sleeping,
    Stopped,
    Zombie,
    Dead,
}

// ── Signal Set ────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default)]
pub struct SigSet(pub u64);

impl SigSet {
    pub fn add(&mut self, sig: u8)    { self.0 |=  (1u64 << (sig - 1)); }
    pub fn del(&mut self, sig: u8)    { self.0 &= !(1u64 << (sig - 1)); }
    pub fn has(&self,     sig: u8) -> bool { (self.0 >> (sig - 1)) & 1 != 0 }
    pub fn empty(&self)            -> bool { self.0 == 0 }
}

pub const SIGKILL: u8 = 9;
pub const SIGTERM: u8 = 15;
pub const SIGCHLD: u8 = 17;
pub const SIGSTOP: u8 = 19;
pub const SIGCONT: u8 = 18;

// ── Process Control Block ─────────────────────────────────────────────────────

pub const MAX_CHILDREN: usize = 32;
pub const MAX_FDS:       usize = 64;

#[derive(Clone, Copy)]
pub struct Process {
    pub pid:       u32,
    pub ppid:      u32,
    pub state:     ProcessState,
    pub exit_code: i32,

    // Address space
    pub cr3:       usize, // page table root (physical)
    pub stack_top: usize,
    pub heap_brk:  usize,

    // Scheduler
    pub priority:  u8,
    pub cpu_time:  u64,  // ticks consumed

    // File descriptors (simple: just track which FDs are open)
    pub open_fds:  u64,  // bitmask

    // Security
    pub pledge_caps: u64,
    pub uid:         u32,
    pub gid:         u32,

    // Signals
    pub sig_pending: SigSet,
    pub sig_mask:    SigSet,

    // Children
    pub children:   [u32; MAX_CHILDREN],
    pub n_children: usize,
}

impl Process {
    pub fn new(pid: u32, ppid: u32, cr3: usize) -> Self {
        Self {
            pid, ppid,
            state:     ProcessState::Running,
            exit_code: 0,
            cr3, stack_top: 0, heap_brk: 0,
            priority:  128,
            cpu_time:  0,
            open_fds:  0x7, // stdin/stdout/stderr open by default
            pledge_caps: u64::MAX, // unrestricted at birth
            uid: 0, gid: 0,
            sig_pending: SigSet::default(),
            sig_mask:    SigSet::default(),
            children:    [0u32; MAX_CHILDREN],
            n_children:  0,
        }
    }
}

// ── Process Table ─────────────────────────────────────────────────────────────

pub const MAX_PROCESSES: usize = 256;

pub struct ProcessManager {
    table:   [Option<Process>; MAX_PROCESSES],
    count:   usize,
    current: Option<u32>, // PID of running process
}

impl ProcessManager {
    pub const fn new() -> Self {
        Self {
            table:   [const { None }; MAX_PROCESSES],
            count:   0,
            current: None,
        }
    }

    fn slot_for_pid(&mut self, pid: u32) -> Option<usize> {
        self.table.iter().position(|s| matches!(s, Some(p) if p.pid == pid))
    }

    fn free_slot(&self) -> Option<usize> {
        self.table.iter().position(|s| s.is_none())
    }

    /// Create init (PID 1) at boot
    pub fn spawn_init(&mut self, cr3: usize) -> u32 {
        let p = Process::new(1, 0, cr3);
        self.table[0] = Some(p);
        self.count += 1;
        self.current = Some(1);
        1
    }

    /// Fork: clone parent process
    pub fn fork(&mut self, parent_pid: u32, child_cr3: usize) -> Option<u32> {
        let slot = self.free_slot()?;
        let parent = self.table
            .iter().flatten()
            .find(|p| p.pid == parent_pid)
            .copied()?;

        let child_pid = alloc_pid();
        let mut child = parent;
        child.pid      = child_pid;
        child.ppid     = parent_pid;
        child.cr3      = child_cr3;
        child.cpu_time = 0;
        child.sig_pending = SigSet::default();

        // Add child to parent's list
        if let Some(pslot) = self.slot_for_pid(parent_pid) {
            if let Some(ref mut p) = self.table[pslot] {
                if p.n_children < MAX_CHILDREN {
                    p.children[p.n_children] = child_pid;
                    p.n_children += 1;
                }
            }
        }

        self.table[slot] = Some(child);
        self.count += 1;
        Some(child_pid)
    }

    /// Replace process image (exec): reset address space
    pub fn exec(&mut self, pid: u32, new_cr3: usize, entry: usize, stack: usize) -> bool {
        if let Some(slot) = self.slot_for_pid(pid) {
            if let Some(ref mut p) = self.table[slot] {
                p.cr3       = new_cr3;
                p.stack_top = stack;
                p.heap_brk  = 0;
                p.open_fds  = 0x7; // reset to stdin/stdout/stderr
                p.pledge_caps = u64::MAX;
                return true;
            }
        }
        false
    }

    /// Wait for any child to exit; returns (child_pid, exit_code) or None
    pub fn wait(&mut self, parent_pid: u32) -> Option<(u32, i32)> {
        let children: [u32; MAX_CHILDREN];
        let n_children: usize;
        {
            let p = self.table.iter().flatten()
                .find(|p| p.pid == parent_pid)?;
            children   = p.children;
            n_children = p.n_children;
        }
        for i in 0..n_children {
            let cid = children[i];
            if let Some(slot) = self.slot_for_pid(cid) {
                if let Some(ref c) = self.table[slot] {
                    if c.state == ProcessState::Zombie {
                        let exit = c.exit_code;
                        self.table[slot] = None; // reap
                        self.count -= 1;
                        // Remove from parent's children list
                        if let Some(pslot) = self.slot_for_pid(parent_pid) {
                            if let Some(ref mut parent) = self.table[pslot] {
                                parent.children[i] = 0;
                                // compact
                                for j in i..parent.n_children-1 {
                                    parent.children[j] = parent.children[j+1];
                                }
                                parent.n_children -= 1;
                            }
                        }
                        return Some((cid, exit));
                    }
                }
            }
        }
        None
    }

    /// Send signal to process
    pub fn kill(&mut self, pid: u32, sig: u8) -> bool {
        if let Some(slot) = self.slot_for_pid(pid) {
            if let Some(ref mut p) = self.table[slot] {
                match sig {
                    SIGKILL => { p.state = ProcessState::Zombie; p.exit_code = -9; }
                    SIGSTOP => { p.state = ProcessState::Stopped; }
                    SIGCONT => { if p.state == ProcessState::Stopped { p.state = ProcessState::Running; } }
                    _ => { p.sig_pending.add(sig); }
                }
                return true;
            }
        }
        false
    }

    /// Mark process as exited (zombie until waited)
    pub fn exit(&mut self, pid: u32, code: i32) {
        if let Some(slot) = self.slot_for_pid(pid) {
            if let Some(ref mut p) = self.table[slot] {
                p.state     = ProcessState::Zombie;
                p.exit_code = code;
            }
        }
        // Notify parent with SIGCHLD
        let ppid = self.table.iter().flatten()
            .find(|p| p.pid == pid).map(|p| p.ppid);
        if let Some(pp) = ppid { self.kill(pp, SIGCHLD); }
    }

    pub fn get(&self, pid: u32) -> Option<&Process> {
        self.table.iter().flatten().find(|p| p.pid == pid)
    }

    pub fn current_pid(&self) -> Option<u32> { self.current }
    pub fn set_current(&mut self, pid: u32)  { self.current = Some(pid); }
    pub fn count(&self) -> usize             { self.count }
}
