#![allow(dead_code)]
// SPDX-License-Identifier: MIT
// SigmaOS Comprehensive Process Model
// Includes POSIX threads, complete states, signals, ELF loading stubs, context switching,
// and advanced blocked process states (BlockedWaiting, BlockedSuspended, WaitChannels).


extern crate alloc;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Blocked,
    BlockedWaiting,
    BlockedSuspended,
    Stopped,
    Zombie,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockReason {
    IoWait,
    LockWait,
    SignalWait,
    TimerWait,
    PageFaultWait,
    ChannelWait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreadId(pub u64);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rbp: u64, pub rsi: u64, pub rdi: u64, pub r8: u64,
    pub r9: u64, pub r10: u64, pub r11: u64, pub r12: u64,
    pub r13: u64, pub r14: u64, pub r15: u64,
    pub rip: u64, pub cs: u64, pub rflags: u64, pub rsp: u64, pub ss: u64,
}

impl TrapFrame {
    pub fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0, rbp: 0, rsi: 0, rdi: 0,
            r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, cs: 0x2B, rflags: 0x202, rsp: 0, ss: 0x23,
        }
    }
}

impl Default for TrapFrame {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Thread {
    pub tid: ThreadId,
    pub pid: ProcessId,
    pub state: ProcessState,
    pub kstack: usize,
    pub context: TrapFrame,
    pub fs_base: u64,
    pub block_reason: Option<BlockReason>,
    pub priority: u32,
    pub inherited_priority: u32,
}

impl Thread {
    pub fn new(tid: ThreadId, pid: ProcessId, kstack: usize) -> Self {
        Self {
            tid,
            pid,
            state: ProcessState::New,
            kstack,
            context: TrapFrame::new(),
            fs_base: 0,
            block_reason: None,
            priority: 10,
            inherited_priority: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SigAction {
    pub handler: usize,
    pub mask: u64,
    pub flags: u64,
}

#[derive(Debug)]
pub struct Process {
    pub pid: ProcessId,
    pub ppid: ProcessId,
    pub children: Vec<ProcessId>,
    pub pgid: u64,
    pub session_id: u64,
    pub uid: u32,
    pub gid: u32,
    
    pub state: ProcessState,
    pub page_table_phys: usize,
    
    pub open_files: BTreeMap<u64, usize>,
    
    // Signals
    pub sig_pending: u64,
    pub sig_mask: u64,
    pub sig_actions: [SigAction; 64],
    
    // Memory stats
    pub brk: usize,
    pub start_brk: usize,
    pub mmap_base: usize,
    
    pub exit_code: Option<i32>,
    pub name: String,
    
    pub cwd: String,
    pub block_reason: Option<BlockReason>,
    pub is_suspended: bool,
}

static NEXT_PID: AtomicU64 = AtomicU64::new(1);
static NEXT_TID: AtomicU64 = AtomicU64::new(1);

impl Process {
    pub fn new(page_table: usize, name: &str) -> Self {
        Self {
            pid: ProcessId(NEXT_PID.fetch_add(1, Ordering::SeqCst)),
            ppid: ProcessId(0),
            children: Vec::new(),
            pgid: 0,
            session_id: 0,
            uid: 0,
            gid: 0,
            state: ProcessState::New,
            page_table_phys: page_table,
            open_files: BTreeMap::new(),
            sig_pending: 0,
            sig_mask: 0,
            sig_actions: core::array::from_fn(|_| SigAction { handler: 0, mask: 0, flags: 0 }),
            brk: 0x40000000,
            start_brk: 0x40000000,
            mmap_base: 0x700000000000,
            exit_code: None,
            name: String::from(name),
            cwd: String::from("/"),
            block_reason: None,
            is_suspended: false,
        }
    }

    pub fn fork(&self, new_pt_phys: usize) -> Self {
        let mut child = Process::new(new_pt_phys, &self.name);
        child.ppid = self.pid;
        child.pgid = self.pgid;
        child.session_id = self.session_id;
        child.uid = self.uid;
        child.gid = self.gid;
        child.open_files = self.open_files.clone();
        child.sig_mask = self.sig_mask;
        child.sig_actions = self.sig_actions.clone();
        child.brk = self.brk;
        child.start_brk = self.start_brk;
        child.mmap_base = self.mmap_base;
        child.cwd = self.cwd.clone();
        child
    }

    pub fn transition_to_blocked(&mut self, reason: BlockReason) {
        self.state = ProcessState::BlockedWaiting;
        self.block_reason = Some(reason);
    }

    pub fn suspend_blocked_process(&mut self) -> Result<(), &'static str> {
        if self.state == ProcessState::Blocked || self.state == ProcessState::BlockedWaiting {
            self.state = ProcessState::BlockedSuspended;
            self.is_suspended = true;
            Ok(())
        } else {
            Err("Process must be in blocked state to suspend")
        }
    }

    pub fn resume_blocked_process(&mut self) -> Result<(), &'static str> {
        if self.state == ProcessState::BlockedSuspended {
            self.state = ProcessState::BlockedWaiting;
            self.is_suspended = false;
            Ok(())
        } else {
            Err("Process is not in blocked-suspended state")
        }
    }
}

// ELF Types for exec()
#[repr(C)]
#[derive(Debug)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub const PT_LOAD: u32 = 1;

pub struct ProcessManager {
    processes: BTreeMap<ProcessId, Process>,
    threads: BTreeMap<ThreadId, Thread>,
    ready_queue: VecDeque<ThreadId>,
    wait_queues: BTreeMap<ProcessId, VecDeque<ThreadId>>,
    wait_channels: BTreeMap<u64, VecDeque<ThreadId>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: BTreeMap::new(),
            threads: BTreeMap::new(),
            ready_queue: VecDeque::new(),
            wait_queues: BTreeMap::new(),
            wait_channels: BTreeMap::new(),
        }
    }

    pub fn add_process(&mut self, p: Process) {
        self.processes.insert(p.pid, p);
    }
    
    pub fn add_thread(&mut self, mut t: Thread) {
        t.state = ProcessState::Ready;
        self.ready_queue.push_back(t.tid);
        self.threads.insert(t.tid, t);
    }

    pub fn get_process(&self, pid: ProcessId) -> Option<&Process> {
        self.processes.get(&pid)
    }

    pub fn get_process_mut(&mut self, pid: ProcessId) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }

    pub fn send_signal(&mut self, pid: ProcessId, sig: u32) -> Result<(), &'static str> {
        if let Some(p) = self.processes.get_mut(&pid) {
            if sig < 64 {
                p.sig_pending |= 1 << sig;
                // Wake up thread if blocked
                if p.state == ProcessState::Blocked || p.state == ProcessState::BlockedWaiting {
                    p.state = ProcessState::Ready;
                }
                Ok(())
            } else {
                Err("Invalid signal")
            }
        } else {
            Err("Process not found")
        }
    }

    pub fn block_thread_on_channel(&mut self, tid: ThreadId, wchan_id: u64, reason: BlockReason) -> Result<(), &'static str> {
        let t = self.threads.get_mut(&tid).ok_or("Thread not found")?;
        t.state = ProcessState::BlockedWaiting;
        t.block_reason = Some(reason);

        if let Some(p) = self.processes.get_mut(&t.pid) {
            p.transition_to_blocked(reason);
        }

        self.wait_channels.entry(wchan_id).or_default().push_back(tid);
        Ok(())
    }

    pub fn wakeup_channel(&mut self, wchan_id: u64) -> usize {
        let mut awakened = 0;
        if let Some(mut q) = self.wait_channels.remove(&wchan_id) {
            while let Some(tid) = q.pop_front() {
                if let Some(t) = self.threads.get_mut(&tid) {
                    t.state = ProcessState::Ready;
                    t.block_reason = None;
                    self.ready_queue.push_back(tid);

                    if let Some(p) = self.processes.get_mut(&t.pid) {
                        p.state = ProcessState::Ready;
                        p.block_reason = None;
                    }
                    awakened += 1;
                }
            }
        }
        awakened
    }

    pub fn waitpid(&mut self, ppid: ProcessId, pid: Option<ProcessId>, current_tid: ThreadId) -> Option<i32> {
        let parent = self.processes.get(&ppid)?;
        let children = parent.children.clone();
        
        // Check for any zombie child
        for &cpid in &children {
            if let Some(child_pid) = pid {
                if child_pid != cpid { continue; }
            }
            if let Some(child) = self.processes.get(&cpid) {
                if child.state == ProcessState::Zombie {
                    let code = child.exit_code.unwrap();
                    self.cleanup_zombie(cpid);
                    return Some(code);
                }
            }
        }

        // None are zombies, block the current thread
        let q = self.wait_queues.entry(ppid).or_default();
        q.push_back(current_tid);
        
        if let Some(t) = self.threads.get_mut(&current_tid) {
            t.state = ProcessState::BlockedWaiting;
            t.block_reason = Some(BlockReason::ChannelWait);
        }
        
        None
    }
    
    fn cleanup_zombie(&mut self, pid: ProcessId) {
        if let Some(p) = self.processes.remove(&pid) {
            if let Some(parent) = self.processes.get_mut(&p.ppid) {
                parent.children.retain(|&c| c != pid);
            }
        }
        self.threads.retain(|_, t| t.pid != pid);
    }

    pub fn exit_process(&mut self, pid: ProcessId, code: i32) {
        let mut children = Vec::new();
        let ppid = if let Some(p) = self.processes.get_mut(&pid) {
            p.state = ProcessState::Zombie;
            p.exit_code = Some(code);
            children = p.children.clone();
            p.children.clear();
            p.ppid
        } else {
            return;
        };

        for cpid in children {
            if let Some(child) = self.processes.get_mut(&cpid) {
                child.ppid = ProcessId(1);
            }
            if let Some(init) = self.processes.get_mut(&ProcessId(1)) {
                init.children.push(cpid);
            }
        }

        if let Some(q) = self.wait_queues.get_mut(&ppid) {
            while let Some(tid) = q.pop_front() {
                if let Some(t) = self.threads.get_mut(&tid) {
                    t.state = ProcessState::Ready;
                    self.ready_queue.push_back(tid);
                }
            }
        }
    }
    
    pub fn exec(&mut self, pid: ProcessId, elf_data: &[u8]) -> Result<u64, &'static str> {
        if elf_data.len() < core::mem::size_of::<Elf64Ehdr>() {
            return Err("Invalid ELF header");
        }
        
        let ehdr = unsafe { &*(elf_data.as_ptr() as *const Elf64Ehdr) };
        if ehdr.e_ident[0..4] != [0x7F, b'E', b'L', b'F'] {
            return Err("Not an ELF file");
        }
        
        if ehdr.e_type != 2 && ehdr.e_type != 3 {
            return Err("Unsupported ELF type");
        }
        
        let p = self.processes.get_mut(&pid).ok_or("Process not found")?;
        
        p.brk = 0x40000000;
        p.start_brk = 0x40000000;
        p.mmap_base = 0x700000000000;
        p.sig_actions = core::array::from_fn(|_| SigAction { handler: 0, mask: 0, flags: 0 });
        
        Ok(ehdr.e_entry)
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_blocked_state_transitions() {
        let mut pm = ProcessManager::new();
        let mut proc = Process::new(0x1000, "worker_task");
        let pid = proc.pid;
        pm.add_process(proc);

        let thread = Thread::new(ThreadId(101), pid, 0x8000);
        pm.add_thread(thread);

        // Block on I/O channel
        assert!(pm.block_thread_on_channel(ThreadId(101), 0x55, BlockReason::IoWait).is_ok());
        let p = pm.get_process(pid).unwrap();
        assert_eq!(p.state, ProcessState::BlockedWaiting);
        assert_eq!(p.block_reason, Some(BlockReason::IoWait));

        // Suspend blocked process
        let p_mut = pm.get_process_mut(pid).unwrap();
        assert!(p_mut.suspend_blocked_process().is_ok());
        assert_eq!(p_mut.state, ProcessState::BlockedSuspended);

        // Resume and wakeup
        assert!(p_mut.resume_blocked_process().is_ok());
        assert_eq!(pm.wakeup_channel(0x55), 1);
        assert_eq!(pm.get_process(pid).unwrap().state, ProcessState::Ready);
    }
}
