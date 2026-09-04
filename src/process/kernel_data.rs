/// Advanced Kernel Internals and Scheduler Data Structures for SigmaOS
/// Inspired by Windows NT (EPROCESS, KPROCESS, ETHREAD, KTHREAD, KPCR/KPRCB),
/// Linux task_struct, BSD vmspace, and iOS Mach thread models.
use std::vec::Vec;

/// Represents the alterable waiting state of a thread dispatcher (Windows NT/BSD style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadWaitMode {
    /// Wait can be interrupted by user-space asynchronous procedure calls (APCs) or signals (Linux/BSD TASK_INTERRUPTIBLE)
    UserModeAlterable,
    /// Wait cannot be interrupted, must be woken up explicitly by kernel events (Linux TASK_UNINTERRUPTIBLE)
    KernelModeNonAlterable,
}

/// Represents the execution state of a thread
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Initialized,
    Ready,
    Running,
    Waiting,
    Terminated,
}

/// Memory Descriptor List (MDL) - tracks page locks, physical backing pages, and virtual ranges
pub struct MemoryDescriptorList {
    pub virtual_address: usize,
    pub byte_count: usize,
    pub is_pinned: bool,
    pub physical_pages: Vec<usize>, // List of raw page frames backing this range
}

impl MemoryDescriptorList {
    pub fn new(virtual_address: usize, byte_count: usize) -> Self {
        MemoryDescriptorList {
            virtual_address,
            byte_count,
            is_pinned: false,
            physical_pages: Vec::new(),
        }
    }

    pub fn pin_pages(&mut self, pages: &[usize]) {
        self.is_pinned = true;
        self.physical_pages = Vec::new();
        for &page in pages {
            self.physical_pages.push(page);
        }
    }

    pub fn unpin_pages(&mut self) {
        self.is_pinned = false;
        self.physical_pages = Vec::new();
    }
}

/// Kernel Thread block (KTHREAD) - Hardware/OS core scheduling information
pub struct KThread {
    pub state: ThreadState,
    pub base_priority: u8,
    pub dynamic_priority: u8,
    pub quantum: i32,
    pub affinity_mask: usize,
    pub wait_mode: ThreadWaitMode,
    // Stack bounds tracking
    pub stack_base: usize,
    pub stack_limit: usize,
    pub kernel_stack_ptr: usize,
}

impl KThread {
    pub fn new(stack_base: usize, stack_limit: usize, base_priority: u8) -> Self {
        KThread {
            state: ThreadState::Initialized,
            base_priority,
            dynamic_priority: base_priority,
            quantum: 60, // Default tick quantum
            affinity_mask: 0xFFFFFFFF,
            wait_mode: ThreadWaitMode::UserModeAlterable,
            stack_base,
            stack_limit,
            kernel_stack_ptr: stack_base,
        }
    }

    pub fn decrement_quantum(&mut self, ticks: i32) -> bool {
        self.quantum = self.quantum.saturating_sub(ticks);
        self.quantum <= 0
    }

    pub fn boost_priority(&mut self, boost: u8) {
        self.dynamic_priority = self.base_priority.saturating_add(boost).min(31);
    }

    pub fn reset_priority(&mut self) {
        self.dynamic_priority = self.base_priority;
    }
}

/// Executive Thread block (ETHREAD) - High-level process container/API information
pub struct EThread {
    pub thread_id: usize,
    pub process_id: usize,
    pub start_address: usize,
    pub creation_time: u64,
    pub exit_status: i32,
    pub kthread: KThread,
}

impl EThread {
    pub fn new(
        thread_id: usize,
        process_id: usize,
        start_address: usize,
        stack_base: usize,
        stack_limit: usize,
        base_priority: u8,
    ) -> Self {
        EThread {
            thread_id,
            process_id,
            start_address,
            creation_time: 12345678,
            exit_status: 0,
            kthread: KThread::new(stack_base, stack_limit, base_priority),
        }
    }
}

/// Virtual Address Space Descriptor representing mapped memory segments
#[derive(Debug, Clone)]
pub struct VasDescriptor {
    pub start_address: usize,
    pub end_address: usize,
    pub permissions: u8, // Read=4, Write=2, Execute=1
}

/// Kernel Process block (KPROCESS) - Core scheduler and virtual memory tables
pub struct KProcess {
    pub directory_table_base: usize, // CR3 page directory physical address
    pub default_quantum: i32,
    pub active_threads: Vec<usize>,
    pub base_priority: u8,
}

impl KProcess {
    pub fn new(cr3: usize, base_priority: u8) -> Self {
        KProcess {
            directory_table_base: cr3,
            default_quantum: 60,
            active_threads: Vec::new(),
            base_priority,
        }
    }
}

/// Executive Process block (EPROCESS) - User-space process metadata and handles
pub struct EProcess {
    pub process_id: usize,
    pub parent_process_id: usize,
    pub image_file_name: [u8; 16],
    pub va_descriptors: Vec<VasDescriptor>,
    pub peak_memory_usage: usize,
    pub security_token: usize,
    pub kprocess: KProcess,
}

impl EProcess {
    pub fn new(pid: usize, ppid: usize, name: &[u8], cr3: usize, base_priority: u8) -> Self {
        let mut name_arr = [0u8; 16];
        let len = name.len().min(15);
        name_arr[..len].copy_from_slice(&name[..len]);

        EProcess {
            process_id: pid,
            parent_process_id: ppid,
            image_file_name: name_arr,
            va_descriptors: Vec::new(),
            peak_memory_usage: 0,
            security_token: 0xFFFF,
            kprocess: KProcess::new(cr3, base_priority),
        }
    }

    pub fn map_virtual_region(&mut self, start: usize, size: usize, permissions: u8) {
        let descriptor = VasDescriptor {
            start_address: start,
            end_address: start + size,
            permissions,
        };
        self.va_descriptors.push(descriptor);
        self.peak_memory_usage = self.peak_memory_usage.saturating_add(size);
    }
}

/// Processor Control Block (KPRCB) - Per-CPU hardware scheduling queues and states
pub struct KPrcb {
    pub cpu_id: usize,
    pub current_thread_id: Option<usize>,
    pub next_thread_id: Option<usize>,
    pub idle_thread_id: usize,
    pub interrupt_depth: u32,
    pub dpc_queue_depth: u32,
}

impl KPrcb {
    pub fn new(cpu_id: usize, idle_thread_id: usize) -> Self {
        KPrcb {
            cpu_id,
            current_thread_id: None,
            next_thread_id: None,
            idle_thread_id,
            interrupt_depth: 0,
            dpc_queue_depth: 0,
        }
    }
}

/// Processor Control Region (KPCR) - Per-CPU architecture descriptors (GDT, IDT, segment maps)
pub struct Kpcr {
    pub self_ptr: usize,
    pub prcb: KPrcb,
    pub current_irql: u8,
}

impl Kpcr {
    pub fn new(self_ptr: usize, cpu_id: usize, idle_thread_id: usize) -> Self {
        Kpcr {
            self_ptr,
            prcb: KPrcb::new(cpu_id, idle_thread_id),
            current_irql: 0,
        }
    }
}

/// Kernel Debugger Interface Shim (KD Port Interface)
pub struct KernelDebuggerShim {
    pub enabled: bool,
    pub attached: bool,
    pub register_view: [u64; 16],
}

impl Default for KernelDebuggerShim {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelDebuggerShim {
    pub fn new() -> Self {
        KernelDebuggerShim {
            enabled: true,
            attached: false,
            register_view: [0; 16],
        }
    }

    pub fn attach_debugger(&mut self) {
        self.attached = true;
    }

    pub fn detach_debugger(&mut self) {
        self.attached = false;
    }

    pub fn write_register(&mut self, index: usize, value: u64) -> Result<(), &'static str> {
        if index < self.register_view.len() {
            self.register_view[index] = value;
            Ok(())
        } else {
            Err("Register index out of bounds")
        }
    }

    pub fn read_register(&self, index: usize) -> Result<u64, &'static str> {
        if index < self.register_view.len() {
            Ok(self.register_view[index])
        } else {
            Err("Register index out of bounds")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eprocess_kprocess_mapping() {
        let mut proc = EProcess::new(1001, 1, b"kernel_init", 0x1000, 16);
        assert_eq!(proc.process_id, 1001);
        assert_eq!(proc.kprocess.directory_table_base, 0x1000);
        assert_eq!(proc.kprocess.base_priority, 16);

        proc.map_virtual_region(0x400000, 0x10000, 7); // Map text section with RWX permissions
        assert_eq!(proc.va_descriptors.len(), 1);
        assert_eq!(proc.va_descriptors[0].start_address, 0x400000);
        assert_eq!(proc.va_descriptors[0].permissions, 7);
    }

    #[test]
    fn test_ethread_kthread_scheduling() {
        let mut ethread = EThread::new(2001, 1001, 0x401000, 0x7FFFF000, 0x7FFF8000, 8);
        assert_eq!(ethread.thread_id, 2001);
        assert_eq!(ethread.kthread.stack_base, 0x7FFFF000);
        assert_eq!(ethread.kthread.base_priority, 8);
        assert_eq!(ethread.kthread.wait_mode, ThreadWaitMode::UserModeAlterable);

        // Test quantum decrement
        let quantum_expired = ethread.kthread.decrement_quantum(15);
        assert!(!quantum_expired);
        assert_eq!(ethread.kthread.quantum, 45);

        // Test priority boost
        ethread.kthread.boost_priority(4);
        assert_eq!(ethread.kthread.dynamic_priority, 12);

        // Test priority reset
        ethread.kthread.reset_priority();
        assert_eq!(ethread.kthread.dynamic_priority, 8);
    }

    #[test]
    fn test_memory_descriptor_list_pinnings() {
        let mut mdl = MemoryDescriptorList::new(0x200000, 4096);
        assert_eq!(mdl.virtual_address, 0x200000);
        assert!(!mdl.is_pinned);

        mdl.pin_pages(&[0x1A000, 0x1B000]);
        assert!(mdl.is_pinned);
        assert_eq!(mdl.physical_pages.len(), 2);
        assert_eq!(mdl.physical_pages[0], 0x1A000);

        mdl.unpin_pages();
        assert!(!mdl.is_pinned);
        assert_eq!(mdl.physical_pages.len(), 0);
    }

    #[test]
    fn test_kpcr_kprcb_regions() {
        let kpcr = Kpcr::new(0xFFFFF80000000000, 0, 9999);
        assert_eq!(kpcr.self_ptr, 0xFFFFF80000000000);
        assert_eq!(kpcr.prcb.cpu_id, 0);
        assert_eq!(kpcr.prcb.idle_thread_id, 9999);
        assert_eq!(kpcr.current_irql, 0);
    }

    #[test]
    fn test_kernel_debugger_shim() {
        let mut kd = KernelDebuggerShim::new();
        assert!(kd.enabled);
        assert!(!kd.attached);

        kd.attach_debugger();
        assert!(kd.attached);

        assert!(kd.write_register(0, 0xABCD1234).is_ok());
        assert_eq!(kd.read_register(0).unwrap(), 0xABCD1234);

        assert!(kd.write_register(16, 0).is_err());
    }
}
