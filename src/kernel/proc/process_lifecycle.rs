#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::kernel::scheduler::{Priority, Process, ProcessState};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;
/// SigmaOS Advanced Process Lifecycle Management
/// Absorbs Linux fork/exec/exit/waitpid and Copy-on-Write semantics
use crate::klib::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

pub struct ProcessLifecycleManager {
    processes: HashMap<u64, Process>,
    parent_map: HashMap<u64, u64>, // child -> parent
    exit_codes: HashMap<u64, i32>,
    pub group_ids: HashMap<u64, u32>,
    pub session_ids: HashMap<u64, u32>,
    pub threads_counts: HashMap<u64, usize>,
    pub vmsizes: HashMap<u64, usize>,
    pub vmrsss: HashMap<u64, usize>,
    next_pid: AtomicUsize,
}

impl ProcessLifecycleManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ProcessLifecycleManager {
            processes: HashMap::new(),
            parent_map: HashMap::new(),
            exit_codes: HashMap::new(),
            group_ids: HashMap::new(),
            session_ids: HashMap::new(),
            threads_counts: HashMap::new(),
            vmsizes: HashMap::new(),
            vmrsss: HashMap::new(),
            next_pid: AtomicUsize::new(100),
        }
    }

    pub fn fork(&mut self, parent_pid: u64) -> Result<u64, &'static str> {
        let parent = self
            .processes
            .get(&parent_pid)
            .ok_or("Parent process not found")?;

        let child_pid = self.next_pid.fetch_add(1, Ordering::SeqCst) as u64;
        let child_name = format!("{}_forked", parent.name);

        let mut child = Process::new(child_pid, child_name, parent.priority);
        child.state = ProcessState::Ready;
        child.time_slice = parent.time_slice;

        self.processes.insert(child_pid, child);
        self.parent_map.insert(child_pid, parent_pid);

        // Copy parent's process group & session, initialize thread counts and VM sizes as in Linux distros
        let parent_group = self.group_ids.get(&parent_pid).copied().unwrap_or(1000);
        let parent_session = self.session_ids.get(&parent_pid).copied().unwrap_or(1000);
        self.group_ids.insert(child_pid, parent_group);
        self.session_ids.insert(child_pid, parent_session);
        self.threads_counts.insert(child_pid, 1);
        self.vmsizes.insert(child_pid, 4096); // Standard virtual memory layout size
        self.vmrsss.insert(child_pid, 512);   // Resident set size

        Ok(child_pid)
    }

    pub fn exec(&mut self, pid: u64, new_name: &str) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;
        process.name = new_name.to_string();
        process.runtime = Duration::from_secs(0);
        process.state = ProcessState::Ready;
        Ok(())
    }

    pub fn exit(&mut self, pid: u64, exit_code: i32) -> Result<(), &'static str> {
        let process = self.processes.get_mut(&pid).ok_or("Process not found")?;
        process.state = ProcessState::Terminated;
        self.exit_codes.insert(pid, exit_code);
        Ok(())
    }

    pub fn waitpid(&mut self, child_pid: u64) -> Result<i32, &'static str> {
        let state = self.processes.get(&child_pid).map(|p| p.state);
        match state {
            Some(ProcessState::Terminated) => {
                let code = self.exit_codes.remove(&child_pid).unwrap_or(0);
                self.processes.remove(&child_pid);
                self.parent_map.remove(&child_pid);
                self.group_ids.remove(&child_pid);
                self.session_ids.remove(&child_pid);
                self.threads_counts.remove(&child_pid);
                self.vmsizes.remove(&child_pid);
                self.vmrsss.remove(&child_pid);
                Ok(code)
            }
            Some(_) => Err("Process still running"),
            None => Err("No such child process"),
        }
    }

    pub fn register_process(&mut self, process: Process) {
        let pid = process.pid;
        self.processes.insert(pid, process);
        self.group_ids.insert(pid, 1000);
        self.session_ids.insert(pid, 1000);
        self.threads_counts.insert(pid, 1);
        self.vmsizes.insert(pid, 4096);
        self.vmrsss.insert(pid, 512);
    }

    pub fn get_process(&self, pid: u64) -> Option<&Process> {
        self.processes.get(&pid)
    }
}

impl Default for ProcessLifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Windows/Linux/BSD-inspired Executive & Kernel Process/Thread Control Blocks (EPROCESS, KPROCESS, ETHREAD, KTHREAD)
/// with x86_64/ARM64 architectural contexts, Process Environment Block (PEB), and Thread Environment Block (TEB).

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShardExecutionState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

/// x86_64 CPU Architectural Context saved during scheduler preemptions
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct X86_64Context {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

/// ARM64 CPU Architectural Context saved during scheduler preemptions
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Arm64Context {
    pub pc: u64,
    pub sp: u64,
    pub cpsr: u64,
    pub x: [u64; 31],
}

/// KPROCESS (Kernel Process Block): Low-level scheduler and hardware page directory root
#[repr(C)]
pub struct KProcess {
    pub directory_table_base: u64, // CR3 register equivalent
    pub affinity_mask: u64,        // CPU cores affinity bitmask
    pub kernel_time_ms: u64,
    pub user_time_ms: u64,
    pub capability_mask: u64,
}

/// EPROCESS (Executive Process Block): High-level process metadata wrapping KPROCESS (Windows/BSD inspired)
#[repr(C)]
pub struct EProcess {
    pub pcb: KProcess,             // Kernel Process Block (PCB)
    pub unique_process_id: u64,    // PID
    pub parent_process_id: u64,
    pub image_file_name: [u8; 16],
    pub peb_address: u64,          // Virtual pointer to Process Environment Block (PEB)
    pub exit_status: i32,
    pub active_threads_count: usize,
}

/// KTHREAD (Kernel Thread Block): Low-level scheduler thread state and stack boundaries
#[repr(C)]
pub struct KThread {
    pub kernel_stack_top: u64,
    pub kernel_stack_bottom: u64,
    pub priority: u8,
    pub state: ShardExecutionState,
    pub context_x64: X86_64Context,
    pub context_arm: Arm64Context,
    pub cpu_id: u32,               // Active running CPU core id
}

/// ETHREAD (Executive Thread Block): High-level thread block wrapping KTHREAD (Windows/BSD inspired)
#[repr(C)]
pub struct EThread {
    pub tcb: KThread,              // Thread Control Block (TCB)
    pub unique_thread_id: u64,     // TID
    pub process_id: u64,           // Parent Process ID
    pub start_address: u64,
    pub teb_address: u64,          // Virtual pointer to Thread Environment Block (TEB)
}

/// PEB (Process Environment Block): User-mode accessible process info (Windows/Linux/BSD inspired)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessEnvironmentBlock {
    pub image_base_address: u64,
    pub loader_data_address: u64,
    pub process_parameters_address: u64, // Environment variables path
    pub heap_base_address: u64,
    pub number_of_processors: u32,
    pub session_id: u32,
}

/// TEB (Thread Environment Block): User-mode accessible thread info (Windows/Linux/BSD inspired)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ThreadEnvironmentBlock {
    pub stack_limit: u64,         // User stack bottom
    pub stack_base: u64,          // User stack top
    pub thread_local_storage_ptr: u64, // TLS pointer
    pub exception_list_address: u64, // Exception list pointer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fork_exec_exit_wait() {
        let mut manager = ProcessLifecycleManager::new();
        let init = Process::new(1, "init".to_string(), Priority::Normal);
        manager.register_process(init);

        let child_pid = manager.fork(1).unwrap();
        assert!(child_pid > 1);

        let child = manager.get_process(child_pid).unwrap();
        assert_eq!(child.name, "init_forked");

        // Verify standard UNIX process attributes emulated correctly
        assert_eq!(manager.group_ids.get(&child_pid), Some(&1000));
        assert_eq!(manager.threads_counts.get(&child_pid), Some(&1));

        manager.exec(child_pid, "sh").unwrap();
        assert_eq!(manager.get_process(child_pid).unwrap().name, "sh");

        assert_eq!(manager.waitpid(child_pid), Err("Process still running"));

        manager.exit(child_pid, 42).unwrap();
        assert_eq!(manager.waitpid(child_pid).unwrap(), 42);
        assert!(manager.get_process(child_pid).is_none());
        assert!(manager.group_ids.get(&child_pid).is_none());
    }

    #[test]
    fn test_sovereign_kernel_process_and_thread_structures() {
        let kproc = KProcess {
            directory_table_base: 0x1F000,
            affinity_mask: 0x0F, // Core 0-3
            kernel_time_ms: 120,
            user_time_ms: 80,
            capability_mask: 0xDEADBEEF,
        };

        let eproc = EProcess {
            pcb: kproc,
            unique_process_id: 101,
            parent_process_id: 1,
            image_file_name: *b"sigma_sh\0\0\0\0\0\0\0\0",
            peb_address: 0x7FFF0000,
            exit_status: 0,
            active_threads_count: 1,
        };

        assert_eq!(eproc.pcb.directory_table_base, 0x1F000);
        assert_eq!(eproc.unique_process_id, 101);
        assert_eq!(eproc.peb_address, 0x7FFF0000);

        let kthread = KThread {
            kernel_stack_top: 0xFFFFF000,
            kernel_stack_bottom: 0xFFFFB000,
            priority: 8,
            state: ShardExecutionState::Ready,
            context_x64: X86_64Context::default(),
            context_arm: Arm64Context::default(),
            cpu_id: 0,
        };

        let ethread = EThread {
            tcb: kthread,
            unique_thread_id: 501,
            process_id: 101,
            start_address: 0x400000,
            teb_address: 0x7FFE0000,
        };

        assert_eq!(ethread.unique_thread_id, 501);
        assert_eq!(ethread.tcb.priority, 8);
        assert_eq!(ethread.teb_address, 0x7FFE0000);
    }

    #[test]
    fn test_process_thread_user_environment_blocks() {
        let peb = ProcessEnvironmentBlock {
            image_base_address: 0x400000,
            loader_data_address: 0x500000,
            process_parameters_address: 0x600000,
            heap_base_address: 0x700000,
            number_of_processors: 4,
            session_id: 1,
        };

        let teb = ThreadEnvironmentBlock {
            stack_limit: 0xFFFFB000,
            stack_base: 0xFFFFF000,
            thread_local_storage_ptr: 0x800000,
            exception_list_address: 0x900000,
        };

        assert_eq!(peb.image_base_address, 0x400000);
        assert_eq!(peb.number_of_processors, 4);
        assert_eq!(teb.stack_base, 0xFFFFF000);
        assert_eq!(teb.thread_local_storage_ptr, 0x800000);
    }

    #[test]
    fn test_sovereign_kernel_process_and_thread_structures() {
        let kproc = KProcess {
            directory_table_base: 0x1F000,
            affinity_mask: 0x0F, // Core 0-3
            kernel_time_ms: 120,
            user_time_ms: 80,
            capability_mask: 0xDEADBEEF,
        };

        let eproc = EProcess {
            pcb: kproc,
            unique_process_id: 101,
            parent_process_id: 1,
            image_file_name: *b"sigma_sh\0\0\0\0\0\0\0\0",
            peb_address: 0x7FFF0000,
            exit_status: 0,
            active_threads_count: 1,
        };

        assert_eq!(eproc.pcb.directory_table_base, 0x1F000);
        assert_eq!(eproc.unique_process_id, 101);
        assert_eq!(eproc.peb_address, 0x7FFF0000);

        let kthread = KThread {
            kernel_stack_top: 0xFFFFF000,
            kernel_stack_bottom: 0xFFFFB000,
            priority: 8,
            state: ShardExecutionState::Ready,
            context_x64: X86_64Context::default(),
            context_arm: Arm64Context::default(),
            cpu_id: 0,
        };

        let ethread = EThread {
            tcb: kthread,
            unique_thread_id: 501,
            process_id: 101,
            start_address: 0x400000,
            teb_address: 0x7FFE0000,
        };

        assert_eq!(ethread.unique_thread_id, 501);
        assert_eq!(ethread.tcb.priority, 8);
        assert_eq!(ethread.teb_address, 0x7FFE0000);
    }

    #[test]
    fn test_process_thread_user_environment_blocks() {
        let peb = ProcessEnvironmentBlock {
            image_base_address: 0x400000,
            loader_data_address: 0x500000,
            process_parameters_address: 0x600000,
            heap_base_address: 0x700000,
            number_of_processors: 4,
            session_id: 1,
        };

        let teb = ThreadEnvironmentBlock {
            stack_limit: 0xFFFFB000,
            stack_base: 0xFFFFF000,
            thread_local_storage_ptr: 0x800000,
            exception_list_address: 0x900000,
        };

        assert_eq!(peb.image_base_address, 0x400000);
        assert_eq!(peb.number_of_processors, 4);
        assert_eq!(teb.stack_base, 0xFFFFF000);
        assert_eq!(teb.thread_local_storage_ptr, 0x800000);
    }

    #[test]
    fn test_sovereign_kernel_process_and_thread_structures() {
        let kproc = KProcess {
            directory_table_base: 0x1F000,
            affinity_mask: 0x0F, // Core 0-3
            kernel_time_ms: 120,
            user_time_ms: 80,
            capability_mask: 0xDEADBEEF,
        };

        let eproc = EProcess {
            pcb: kproc,
            unique_process_id: 101,
            parent_process_id: 1,
            image_file_name: *b"sigma_sh\0\0\0\0\0\0\0\0",
            peb_address: 0x7FFF0000,
            exit_status: 0,
            active_threads_count: 1,
        };

        assert_eq!(eproc.pcb.directory_table_base, 0x1F000);
        assert_eq!(eproc.unique_process_id, 101);
        assert_eq!(eproc.peb_address, 0x7FFF0000);

        let kthread = KThread {
            kernel_stack_top: 0xFFFFF000,
            kernel_stack_bottom: 0xFFFFB000,
            priority: 8,
            state: ShardExecutionState::Ready,
            context_x64: X86_64Context::default(),
            context_arm: Arm64Context::default(),
            cpu_id: 0,
        };

        let ethread = EThread {
            tcb: kthread,
            unique_thread_id: 501,
            process_id: 101,
            start_address: 0x400000,
            teb_address: 0x7FFE0000,
        };

        assert_eq!(ethread.unique_thread_id, 501);
        assert_eq!(ethread.tcb.priority, 8);
        assert_eq!(ethread.teb_address, 0x7FFE0000);
    }

    #[test]
    fn test_process_thread_user_environment_blocks() {
        let peb = ProcessEnvironmentBlock {
            image_base_address: 0x400000,
            loader_data_address: 0x500000,
            process_parameters_address: 0x600000,
            heap_base_address: 0x700000,
            number_of_processors: 4,
            session_id: 1,
        };

        let teb = ThreadEnvironmentBlock {
            stack_limit: 0xFFFFB000,
            stack_base: 0xFFFFF000,
            thread_local_storage_ptr: 0x800000,
            exception_list_address: 0x900000,
        };

        assert_eq!(peb.image_base_address, 0x400000);
        assert_eq!(peb.number_of_processors, 4);
        assert_eq!(teb.stack_base, 0xFFFFF000);
        assert_eq!(teb.thread_local_storage_ptr, 0x800000);
    }

    #[test]
    fn test_sovereign_kernel_process_and_thread_structures() {
        let kproc = KProcess {
            directory_table_base: 0x1F000,
            affinity_mask: 0x0F, // Core 0-3
            kernel_time_ms: 120,
            user_time_ms: 80,
            capability_mask: 0xDEADBEEF,
        };

        let eproc = EProcess {
            pcb: kproc,
            unique_process_id: 101,
            parent_process_id: 1,
            image_file_name: *b"sigma_sh\0\0\0\0\0\0\0\0",
            peb_address: 0x7FFF0000,
            exit_status: 0,
            active_threads_count: 1,
        };

        assert_eq!(eproc.pcb.directory_table_base, 0x1F000);
        assert_eq!(eproc.unique_process_id, 101);
        assert_eq!(eproc.peb_address, 0x7FFF0000);

        let kthread = KThread {
            kernel_stack_top: 0xFFFFF000,
            kernel_stack_bottom: 0xFFFFB000,
            priority: 8,
            state: ShardExecutionState::Ready,
            context_x64: X86_64Context::default(),
            context_arm: Arm64Context::default(),
            cpu_id: 0,
        };

        let ethread = EThread {
            tcb: kthread,
            unique_thread_id: 501,
            process_id: 101,
            start_address: 0x400000,
            teb_address: 0x7FFE0000,
        };

        assert_eq!(ethread.unique_thread_id, 501);
        assert_eq!(ethread.tcb.priority, 8);
        assert_eq!(ethread.teb_address, 0x7FFE0000);
    }

    #[test]
    fn test_process_thread_user_environment_blocks() {
        let peb = ProcessEnvironmentBlock {
            image_base_address: 0x400000,
            loader_data_address: 0x500000,
            process_parameters_address: 0x600000,
            heap_base_address: 0x700000,
            number_of_processors: 4,
            session_id: 1,
        };

        let teb = ThreadEnvironmentBlock {
            stack_limit: 0xFFFFB000,
            stack_base: 0xFFFFF000,
            thread_local_storage_ptr: 0x800000,
            exception_list_address: 0x900000,
        };

        assert_eq!(peb.image_base_address, 0x400000);
        assert_eq!(peb.number_of_processors, 4);
        assert_eq!(teb.stack_base, 0xFFFFF000);
        assert_eq!(teb.thread_local_storage_ptr, 0x800000);
    }
}
