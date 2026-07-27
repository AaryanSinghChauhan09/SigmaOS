#![no_std]
#![no_main]

/// Custom Process Management for SigmaOS
/// Implements process management without relying on std::process
/// Uses capability-based access control

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

/// Process ID
pub type ProcessID = usize;

/// Process state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProcessState {
    Uninitialized = 0,
    Created = 1,
    Running = 2,
    Sleeping = 3,
    Stopped = 4,
    Zombie = 5,
    Terminated = 6,
}

/// Process priority
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProcessPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Process capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ProcessCapability {
    pub can_create: bool,
    pub can_terminate: bool,
    pub can_suspend: bool,
    pub can_resume: bool,
    pub can_set_priority: bool,
    pub can_fork: bool,
    pub can_exec: bool,
}

impl ProcessCapability {
    pub fn new() -> Self {
        ProcessCapability {
            can_create: false,
            can_terminate: false,
            can_suspend: false,
            can_resume: false,
            can_set_priority: false,
            can_fork: false,
            can_exec: false,
        }
    }

    pub fn full() -> Self {
        ProcessCapability {
            can_create: true,
            can_terminate: true,
            can_suspend: true,
            can_resume: true,
            can_set_priority: true,
            can_fork: true,
            can_exec: true,
        }
    }
}

/// Process memory map
#[repr(C)]
pub struct ProcessMemoryMap {
    pub code_start: usize,
    pub code_end: usize,
    pub data_start: usize,
    pub data_end: usize,
    pub heap_start: usize,
    pub heap_end: usize,
    pub stack_start: usize,
    pub stack_end: usize,
}

impl ProcessMemoryMap {
    pub fn new() -> Self {
        ProcessMemoryMap {
            code_start: 0,
            code_end: 0,
            data_start: 0,
            data_end: 0,
            heap_start: 0,
            heap_end: 0,
            stack_start: 0,
            stack_end: 0,
        }
    }
}

/// Process control block
#[repr(C)]
pub struct Process {
    pub pid: ProcessID,
    pub ppid: ProcessID,
    pub state: AtomicUsize, // ProcessState as usize
    pub priority: ProcessPriority,
    pub exit_code: AtomicUsize,
    pub memory_map: ProcessMemoryMap,
    pub capability: ProcessCapability,
    pub thread_count: AtomicUsize,
}

impl Process {
    pub unsafe fn new(pid: ProcessID, ppid: ProcessID, capability: ProcessCapability) -> Self {
        Process {
            pid,
            ppid,
            state: AtomicUsize::new(ProcessState::Created as usize),
            priority: ProcessPriority::Normal,
            exit_code: AtomicUsize::new(0),
            memory_map: ProcessMemoryMap::new(),
            capability,
            thread_count: AtomicUsize::new(0),
        }
    }

    pub fn get_state(&self) -> ProcessState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: ProcessState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn set_exit_code(&self, code: usize) {
        self.exit_code.store(code, Ordering::SeqCst);
    }

    pub fn get_exit_code(&self) -> usize {
        self.exit_code.load(Ordering::SeqCst)
    }

    pub fn increment_thread_count(&self) {
        self.thread_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_thread_count(&self) {
        self.thread_count.fetch_sub(1, Ordering::SeqCst);
    }

    pub fn get_thread_count(&self) -> usize {
        self.thread_count.load(Ordering::SeqCst)
    }
}

/// Process manager
pub struct ProcessManager {
    processes: [Option<NonNull<Process>>; 256],
    next_pid: AtomicUsize,
    current_process: AtomicUsize,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: [None; 256],
            next_pid: AtomicUsize::new(1),
            current_process: AtomicUsize::new(0),
        }
    }

    pub unsafe fn create_process(&mut self, ppid: ProcessID, capability: ProcessCapability) -> Option<ProcessID> {
        if !capability.can_create {
            return None;
        }

        let pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
        if pid >= 256 {
            return None;
        }

        let process = Process::new(pid, ppid, capability);
        let process_ptr = alloc(mem::size_of::<Process>()) as *mut Process;
        if process_ptr.is_null() {
            return None;
        }

        ptr::write(process_ptr, process);
        self.processes[pid] = Some(NonNull::new_unchecked(process_ptr));

        Some(pid)
    }

    pub unsafe fn get_process(&self, pid: ProcessID) -> Option<&Process> {
        if pid < 256 {
            self.processes[pid].map(|ptr| unsafe { &*ptr.as_ptr() })
        } else {
            None
        }
    }

    pub unsafe fn terminate_process(&mut self, pid: ProcessID, exit_code: usize) -> bool {
        if pid >= 256 {
            return false;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &*process_ptr.as_ptr();
            if !process.capability.can_terminate {
                return false;
            }

            process.set_state(ProcessState::Terminated);
            process.set_exit_code(exit_code);
            true
        } else {
            false
        }
    }

    pub unsafe fn suspend_process(&mut self, pid: ProcessID) -> bool {
        if pid >= 256 {
            return false;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &*process_ptr.as_ptr();
            if !process.capability.can_suspend {
                return false;
            }

            if process.get_state() == ProcessState::Running {
                process.set_state(ProcessState::Stopped);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub unsafe fn resume_process(&mut self, pid: ProcessID) -> bool {
        if pid >= 256 {
            return false;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &*process_ptr.as_ptr();
            if !process.capability.can_resume {
                return false;
            }

            if process.get_state() == ProcessState::Stopped {
                process.set_state(ProcessState::Running);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub unsafe fn set_process_priority(&mut self, pid: ProcessID, priority: ProcessPriority) -> bool {
        if pid >= 256 {
            return false;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &mut *process_ptr.as_ptr();
            if !process.capability.can_set_priority {
                return false;
            }

            process.priority = priority;
            true
        } else {
            false
        }
    }

    pub unsafe fn fork_process(&mut self, parent_pid: ProcessID) -> Option<ProcessID> {
        if parent_pid >= 256 {
            return None;
        }

        if let Some(parent_ptr) = self.processes[parent_pid] {
            let parent = &*parent_ptr.as_ptr();
            if !parent.capability.can_fork {
                return None;
            }

            let child_pid = self.next_pid.fetch_add(1, Ordering::SeqCst);
            if child_pid >= 256 {
                return None;
            }

            let child = Process::new(child_pid, parent_pid, parent.capability);
            let child_ptr = alloc(mem::size_of::<Process>()) as *mut Process;
            if child_ptr.is_null() {
                return None;
            }

            ptr::write(child_ptr, child);
            self.processes[child_pid] = Some(NonNull::new_unchecked(child_ptr));

            Some(child_pid)
        } else {
            None
        }
    }

    pub unsafe fn exec_process(&mut self, pid: ProcessID, binary: &[u8]) -> bool {
        if pid >= 256 {
            return false;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &*process_ptr.as_ptr();
            if !process.capability.can_exec {
                return false;
            }

            // In a real implementation, this would load and execute the binary
            // For now, return true
            true
        } else {
            false
        }
    }

    pub unsafe fn wait_process(&self, pid: ProcessID) -> Option<usize> {
        if pid >= 256 {
            return None;
        }

        if let Some(process_ptr) = self.processes[pid] {
            let process = &*process_ptr.as_ptr();
            
            // In a real implementation, this would wait for process to terminate
            // For now, check if already terminated
            if process.get_state() == ProcessState::Terminated {
                Some(process.get_exit_code())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_current_process(&self) -> ProcessID {
        self.current_process.load(Ordering::SeqCst)
    }

    pub unsafe fn set_current_process(&self, pid: ProcessID) {
        self.current_process.store(pid, Ordering::SeqCst);
    }

    pub unsafe fn get_parent_pid(&self, pid: ProcessID) -> Option<ProcessID> {
        if pid < 256 {
            self.processes[pid].map(|ptr| unsafe { (*ptr.as_ptr()).ppid })
        } else {
            None
        }
    }
}

/// Global process manager
static mut GLOBAL_PROCESS_MANAGER: Option<ProcessManager> = None;

/// Initialize process manager
pub unsafe fn init_process_manager() {
    GLOBAL_PROCESS_MANAGER = Some(ProcessManager::new());
}

/// Get current process ID
pub unsafe fn get_current_pid() -> ProcessID {
    if let Some(ref manager) = GLOBAL_PROCESS_MANAGER {
        manager.get_current_process()
    } else {
        0
    }
}

/// Create process
pub unsafe fn create_process(ppid: ProcessID, capability: ProcessCapability) -> Option<ProcessID> {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        manager.create_process(ppid, capability)
    } else {
        None
    }
}

/// Terminate process
pub unsafe fn terminate_process(pid: ProcessID, exit_code: usize) -> bool {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        manager.terminate_process(pid, exit_code)
    } else {
        false
    }
}

/// Suspend process
pub unsafe fn suspend_process(pid: ProcessID) -> bool {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        manager.suspend_process(pid)
    } else {
        false
    }
}

/// Resume process
pub unsafe fn resume_process(pid: ProcessID) -> bool {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        manager.resume_process(pid)
    } else {
        false
    }
}

/// Set process priority
pub unsafe fn set_process_priority(pid: ProcessID, priority: ProcessPriority) -> bool {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        manager.set_process_priority(pid, priority)
    } else {
        false
    }
}

/// Fork process
pub unsafe fn fork_process() -> Option<ProcessID> {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        let current_pid = manager.get_current_process();
        manager.fork_process(current_pid)
    } else {
        None
    }
}

/// Exec process
pub unsafe fn exec_process(binary: &[u8]) -> bool {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        let current_pid = manager.get_current_process();
        manager.exec_process(current_pid, binary)
    } else {
        false
    }
}

/// Wait for process
pub unsafe fn wait_process(pid: ProcessID) -> Option<usize> {
    if let Some(ref manager) = GLOBAL_PROCESS_MANAGER {
        manager.wait_process(pid)
    } else {
        None
    }
}

/// Exit current process
pub unsafe fn exit_process(exit_code: usize) -> ! {
    if let Some(ref mut manager) = GLOBAL_PROCESS_MANAGER {
        let current_pid = manager.get_current_process();
        manager.terminate_process(current_pid, exit_code);
    }
    loop {}
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
