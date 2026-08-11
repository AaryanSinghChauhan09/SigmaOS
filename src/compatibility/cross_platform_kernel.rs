// SigmaOS Sovereign Cross-Platform Kernel Internals Layer
// Parity bridge representing low-level kernel mechanisms from Linux, Windows, and BSD

#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap as BTreeMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
use core::sync::atomic::{AtomicU8, Ordering};

// =========================================================================
// 1. Virtual Address Space & 4-Level Page Table Layout
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageAccessMode {
    KernelMode,
    UserMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryArch {
    X86_64,
    ARM64,
}

/// Represents individual page table entry attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    pub physical_frame: u64,
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub accessed: bool,
    pub dirty: bool,
}

impl Default for PageTableEntry {
    fn default() -> Self {
        Self {
            physical_frame: 0,
            present: false,
            writable: false,
            user_accessible: false,
            accessed: false,
            dirty: false,
        }
    }
}

/// Simulated PML4/PDPT/PD/PT 4-Level structures representing physical/virtual mapping databases
#[derive(Debug, Clone)]
pub struct PageDirectory {
    pub arch: MemoryArch,
    pub page_directory_base: u64, // CR3 on x86_64, TTBR0 on ARM64
    pub ttbr1_base: Option<u64>,  // TTBR1 for ARM64 kernel space
    // Maps Level (4 = PML4, 3 = PDPT, 2 = PD, 1 = PT) -> Index -> Entry
    pub tables: BTreeMap<(u8, u64), PageTableEntry>,
    // Register mapping for Page Fault Tracking
    pub cr2: u64,
}

impl PageDirectory {
    pub fn new(arch: MemoryArch, base_register: u64) -> Self {
        let ttbr1 = if arch == MemoryArch::ARM64 {
            Some(base_register + 0x1000)
        } else {
            None
        };

        Self {
            arch,
            page_directory_base: base_register,
            ttbr1_base: ttbr1,
            tables: BTreeMap::new(),
            cr2: 0,
        }
    }

    /// Map a complete 4-level page entry hierarchy
    pub fn map_page_4level(
        &mut self,
        virtual_addr: u64,
        physical_frame: u64,
        mode: PageAccessMode,
        writable: bool,
    ) {
        let pml4_idx = (virtual_addr >> 39) & 0x1FF;
        let pdpt_idx = (virtual_addr >> 30) & 0x1FF;
        let pd_idx = (virtual_addr >> 21) & 0x1FF;
        let pt_idx = (virtual_addr >> 12) & 0x1FF;

        let user_acc = mode == PageAccessMode::UserMode;

        // Populate PML4
        self.tables.insert((4, pml4_idx), PageTableEntry {
            physical_frame: self.page_directory_base >> 12,
            present: true,
            writable,
            user_accessible: user_acc,
            accessed: true,
            dirty: false,
        });

        // Populate PDPT
        self.tables.insert((3, pdpt_idx), PageTableEntry {
            physical_frame: (self.page_directory_base + 0x1000) >> 12,
            present: true,
            writable,
            user_accessible: user_acc,
            accessed: true,
            dirty: false,
        });

        // Populate PD
        self.tables.insert((2, pd_idx), PageTableEntry {
            physical_frame: (self.page_directory_base + 0x2000) >> 12,
            present: true,
            writable,
            user_accessible: user_acc,
            accessed: true,
            dirty: false,
        });

        // Populate PT
        self.tables.insert((1, pt_idx), PageTableEntry {
            physical_frame,
            present: true,
            writable,
            user_accessible: user_acc,
            accessed: true,
            dirty: true,
        });
    }

    /// Complete 4-Level Page Table Translation Walk Simulator
    pub fn walk_page_tables(&mut self, virtual_addr: u64) -> Result<u64, &'static str> {
        let pml4_idx = (virtual_addr >> 39) & 0x1FF;
        let pdpt_idx = (virtual_addr >> 30) & 0x1FF;
        let pd_idx = (virtual_addr >> 21) & 0x1FF;
        let pt_idx = (virtual_addr >> 12) & 0x1FF;
        let offset = virtual_addr & 0xFFF;

        // Perform ARM TTBR0/TTBR1 range checks
        if self.arch == MemoryArch::ARM64 {
            let is_kernel_address = (virtual_addr & (1 << 63)) != 0;
            if is_kernel_address && self.ttbr1_base.is_none() {
                self.cr2 = virtual_addr;
                return Err("ARM64 TTBR1 (kernel space base) is uninitialized");
            }
        }

        // Walk PML4 -> PDPT with safe borrow checking pattern
        let pml4_entry = match self.tables.get(&(4, pml4_idx)) {
            Some(entry) => entry,
            None => {
                self.cr2 = virtual_addr;
                return Err("PML4 translation entry not found (Page Fault)");
            }
        };
        if !pml4_entry.present {
            self.cr2 = virtual_addr;
            return Err("PML4 entry is marked not present");
        }

        // Walk PDPT -> PD
        let pdpt_entry = match self.tables.get(&(3, pdpt_idx)) {
            Some(entry) => entry,
            None => {
                self.cr2 = virtual_addr;
                return Err("PDPT translation entry not found (Page Fault)");
            }
        };
        if !pdpt_entry.present {
            self.cr2 = virtual_addr;
            return Err("PDPT entry is marked not present");
        }

        // Walk PD -> PT
        let pd_entry = match self.tables.get(&(2, pd_idx)) {
            Some(entry) => entry,
            None => {
                self.cr2 = virtual_addr;
                return Err("PD translation entry not found (Page Fault)");
            }
        };
        if !pd_entry.present {
            self.cr2 = virtual_addr;
            return Err("PD entry is marked not present");
        }

        // Walk PT -> Physical frame
        let pt_entry = match self.tables.get(&(1, pt_idx)) {
            Some(entry) => entry,
            None => {
                self.cr2 = virtual_addr;
                return Err("PT translation entry not found (Page Fault)");
            }
        };
        if !pt_entry.present {
            self.cr2 = virtual_addr;
            return Err("PT entry is marked not present");
        }

        Ok((pt_entry.physical_frame << 12) + offset)
    }

    /// Verify page permissions
    pub fn check_page_permission_4level(
        &self,
        virtual_addr: u64,
        current_privilege: PageAccessMode,
    ) -> Result<(), &'static str> {
        let pt_idx = (virtual_addr >> 12) & 0x1FF;
        if let Some(entry) = self.tables.get(&(1, pt_idx)) {
            if !entry.user_accessible && current_privilege == PageAccessMode::UserMode {
                return Err("Access Violation: User-mode thread attempting to read Kernel-mode page");
            }
            Ok(())
        } else {
            Err("Page Fault: Page not mapped")
        }
    }
}

// =========================================================================
// 2. Thread Scheduler & Processor Control Regions (KPCR/KPCRB/Sys-Coprocessor)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Terminated,
}

/// Represents OS Kernel Thread model with stacks and core attributes
#[derive(Debug, Clone)]
pub struct KThread {
    pub thread_id: u32,
    pub priority: u8,
    pub state: ThreadState,
    pub kernel_stack_base: u64,
    pub user_stack_base: u64,
    pub affinity_mask: u64,
    pub wait_reason: Option<&'static str>,
}

impl KThread {
    pub fn new(id: u32, prio: u8) -> Self {
        Self {
            thread_id: id,
            priority: prio,
            state: ThreadState::Ready,
            kernel_stack_base: 0xFFFFFFFF80000000 + (id as u64 * 0x4000),
            user_stack_base: 0x00007FFFF0000000 + (id as u64 * 0x4000),
            affinity_mask: 0x01,
            wait_reason: None,
        }
    }
}

/// Models Windows-style Deferred Procedure Call (DPC)
#[derive(Debug, Clone)]
pub struct DeferredProcedureCall {
    pub id: usize,
    pub target_routine: u64,
    pub priority: u8,
}

/// Models Windows-style Asynchronous Procedure Call (APC)
#[derive(Debug, Clone)]
pub struct AsynchronousProcedureCall {
    pub id: usize,
    pub target_routine: u64,
    pub kernel_mode: bool,
    pub arg1: u64,
}

/// Processor Control Region Block (KPCRB) containing scheduler queues and context
#[derive(Debug, Clone)]
pub struct Kpcrb {
    pub current_thread_id: u32,
    pub next_thread_id: Option<u32>,
    pub idle_thread_id: u32,
    // Schedulers run/wait states
    pub run_queue: VecDeque<KThread>,
    pub wait_queue: Vec<KThread>,
    // System DPC/APC procedure queues
    pub dpc_queue: VecDeque<DeferredProcedureCall>,
    pub apc_queue: VecDeque<AsynchronousProcedureCall>,
    pub interrupt_count: u64,
    pub dpc_count: u64,
    pub apc_count: u64,
}

impl Kpcrb {
    pub fn new(idle_thread: u32) -> Self {
        Self {
            current_thread_id: idle_thread,
            next_thread_id: None,
            idle_thread_id: idle_thread,
            run_queue: VecDeque::new(),
            wait_queue: Vec::new(),
            dpc_queue: VecDeque::new(),
            apc_queue: VecDeque::new(),
            interrupt_count: 0,
            dpc_count: 0,
            apc_count: 0,
        }
    }

    pub fn queue_dpc(&mut self, dpc: DeferredProcedureCall) {
        self.dpc_queue.push_back(dpc);
    }

    pub fn queue_apc(&mut self, apc: AsynchronousProcedureCall) {
        self.apc_queue.push_back(apc);
    }

    pub fn add_thread(&mut self, thread: KThread) {
        self.run_queue.push_back(thread);
    }

    /// Block the currently running thread and queue it to the wait queue
    pub fn block_current_thread(&mut self, wait_reason: &'static str) {
        let current_id = self.current_thread_id;
        if current_id != self.idle_thread_id {
            let mut blocked_thread = KThread::new(current_id, 8);
            blocked_thread.state = ThreadState::Waiting;
            blocked_thread.wait_reason = Some(wait_reason);
            self.wait_queue.push(blocked_thread);
        }
        self.current_thread_id = self.idle_thread_id;
    }

    /// Basic scheduler round-robin execution
    pub fn schedule_next_thread(&mut self) -> Option<u32> {
        if let Some(mut next_thread) = self.run_queue.pop_front() {
            next_thread.state = ThreadState::Running;
            self.current_thread_id = next_thread.thread_id;
            Some(next_thread.thread_id)
        } else {
            self.current_thread_id = self.idle_thread_id;
            None
        }
    }

    pub fn drain_dpc_queue(&mut self) -> usize {
        let count = self.dpc_queue.len();
        self.dpc_count += count as u64;
        self.dpc_queue.clear();
        count
    }

    pub fn drain_apc_queue(&mut self) -> usize {
        let count = self.apc_queue.len();
        self.apc_count += count as u64;
        self.apc_queue.clear();
        count
    }
}

/// Processor Control Region (KPCR) mapped to FS/GS segment registers
#[derive(Debug, Clone)]
pub struct Kpcr {
    pub self_ptr: u64,            // Points to self for validation
    pub pc_region_base: u64,       // Memory address of the PCR
    pub major_version: u16,
    pub minor_version: u16,
    pub fs_segment_base: u64,     // Segment base for x86
    pub gs_segment_base: u64,     // Segment base for x64
    pub coprocessor_regs: BTreeMap<String, u64>, // ARM System Coprocessor registers (e.g. CP15, TTBR, SCTLR)
    pub kpcrb: Kpcrb,
}

impl Kpcr {
    pub fn new(major: u16, minor: u16, gs_base: u64, idle_thread: u32) -> Self {
        let mut coprocessor_regs = BTreeMap::new();
        // Initialize basic ARM System Coprocessor control registers
        coprocessor_regs.insert("SCTLR_EL1".to_string(), 0x30D00800); // System Control Register default
        coprocessor_regs.insert("CPACR_EL1".to_string(), 0x300000);   // Coprocessor Access Control Register

        Self {
            self_ptr: gs_base,
            pc_region_base: gs_base,
            major_version: major,
            minor_version: minor,
            fs_segment_base: 0,
            gs_segment_base: gs_base,
            coprocessor_regs,
            kpcrb: Kpcrb::new(idle_thread),
        }
    }

    pub fn read_coprocessor_reg(&self, reg_name: &str) -> Option<u64> {
        self.coprocessor_regs.get(reg_name).copied()
    }

    pub fn write_coprocessor_reg(&mut self, reg_name: String, value: u64) {
        self.coprocessor_regs.insert(reg_name, value);
    }
}

// =========================================================================
// 3. Interrupt Request Level (IRQL) Management
// =========================================================================

/// Windows/BSD/Linux-inspired Interrupt Priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Irql {
    PassiveLevel = 0,  // User-mode threads, normal drivers
    ApcLevel = 1,      // Asynchronous Procedure Calls
    DispatchLevel = 2, // Thread scheduling, DPCs (no page faults allowed!)
    Dirql = 3,         // Device Interrupt Request Level (Hardware interrupts)
    HighLevel = 4,     // Machine checks, clock synchronization
}

/// Simulated Interrupt Request Level (IRQL) Controller with automatic procedure execution loops
#[derive(Debug)]
pub struct IrqlController {
    current_irql: AtomicU8,
}

impl IrqlController {
    pub fn new() -> Self {
        Self {
            current_irql: AtomicU8::new(Irql::PassiveLevel as u8),
        }
    }

    pub fn get_current_irql(&self) -> Irql {
        match self.current_irql.load(Ordering::SeqCst) {
            0 => Irql::PassiveLevel,
            1 => Irql::ApcLevel,
            2 => Irql::DispatchLevel,
            3 => Irql::Dirql,
            _ => Irql::HighLevel,
        }
    }

    /// KeRaiseIrql Raises the current processor priority level
    pub fn ke_raise_irql(&self, new_irql: Irql) -> Result<Irql, &'static str> {
        let current = self.get_current_irql();
        if new_irql < current {
            return Err("KeRaiseIrql: Attempting to lower IRQL via raise call (use KeLowerIrql instead)");
        }
        self.current_irql.store(new_irql as u8, Ordering::SeqCst);
        Ok(current)
    }

    /// KeLowerIrql Lowers the current processor priority level
    pub fn ke_lower_irql(&self, new_irql: Irql, kpcrb: &mut Kpcrb) -> Result<(), &'static str> {
        let current = self.get_current_irql();
        if new_irql > current {
            return Err("KeLowerIrql: Attempting to raise IRQL via lower call (use KeRaiseIrql instead)");
        }

        // Simulate automatic Software Interrupt (APC/DPC) execution loops during IRQL lowering
        if current >= Irql::DispatchLevel && new_irql < Irql::DispatchLevel {
            // Lowering below DISPATCH_LEVEL triggers automatic drain of DPC queue
            kpcrb.drain_dpc_queue();
        }
        if current >= Irql::ApcLevel && new_irql < Irql::ApcLevel {
            // Lowering below APC_LEVEL triggers automatic execution of APC queue
            kpcrb.drain_apc_queue();
        }

        self.current_irql.store(new_irql as u8, Ordering::SeqCst);
        Ok(())
    }

    /// Verify operations corresponding to IRQL level rules
    pub fn check_page_fault_safety(&self) -> Result<(), &'static str> {
        if self.get_current_irql() >= Irql::DispatchLevel {
            // Page faults at or above DISPATCH_LEVEL cause instant system crash / BugCheck 0x0A (IRQL_NOT_LESS_OR_EQUAL)
            return Err("System Crash: IRQL_NOT_LESS_OR_EQUAL (Page fault occurred at DISPATCH_LEVEL or above)");
        }
        Ok(())
    }
}

impl Default for IrqlController {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. Trap Frames, IDT, exceptions, and Calling Convention Translators
// =========================================================================

/// Models x86_64 Trap Frame saved on kernel stack during privilege transition / exception entry
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TrapFrame {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9:  u64,
    pub r8:  u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rax: u64,
    pub error_code: u64,
    pub rip: u64,
    pub cs:  u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss:  u64,
}

#[derive(Debug, Clone, Copy)]
pub struct IdtEntry {
    pub offset_low: u16,
    pub selector: u16,
    pub ist_index: u8,
    pub flags: u8,
    pub offset_middle: u16,
    pub offset_high: u32,
}

/// Interrupt Descriptor Table Register (IDTR) structure
#[derive(Debug, Clone, Copy)]
pub struct Idtr {
    pub limit: u16,
    pub base: u64,
}

/// Service Descriptor Table for System Calls supporting calling convention parameters translation
#[derive(Debug, Clone)]
pub struct SystemServiceTable {
    pub service_table: BTreeMap<u32, u64>, // Maps syscall number to handler address
    pub call_count: BTreeMap<u32, u64>,    // Syscall profiling telemetry
}

impl SystemServiceTable {
    pub fn new() -> Self {
        Self {
            service_table: BTreeMap::new(),
            call_count: BTreeMap::new(),
        }
    }

    pub fn register_syscall(&mut self, sys_num: u32, handler_addr: u64) {
        self.service_table.insert(sys_num, handler_addr);
        self.call_count.insert(sys_num, 0);
    }

    /// Translate Calling Conventions parameters from Microsoft x64 (RCX, RDX, R8, R9) to System V AMD64 (RDI, RSI, RDX, R10)
    pub fn translate_and_profile_syscall(
        &mut self,
        sys_num: u32,
        rcx: u64,
        rdx: u64,
        r8: u64,
        r9: u64,
    ) -> Result<(u64, [u64; 4]), &'static str> {
        let count = self.call_count.entry(sys_num).or_insert(0);
        *count += 1;

        if let Some(&handler_addr) = self.service_table.get(&sys_num) {
            // Translate params: MS x64 params mapped into System V convention parameters
            let rdi_sysv = rcx;
            let rsi_sysv = rdx;
            let rdx_sysv = r8;
            let r10_sysv = r9;

            Ok((handler_addr, [rdi_sysv, rsi_sysv, rdx_sysv, r10_sysv]))
        } else {
            Err("Trap: Unknown system call number")
        }
    }
}

impl Default for SystemServiceTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Models Windows/Linux User Mode Scheduling (UMS) thread control state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UmsThreadState {
    Active,
    Blocked,
    Suspended,
}

#[derive(Debug, Clone)]
pub struct UmsContext {
    pub thread_id: u32,
    pub state: UmsThreadState,
    pub context_block: u64, // Virtual address of the user-allocated thread context
}

/// Master Bridge Orchestration Interface representing Kernel internals compatibility
pub struct SovereignKernelInternals {
    pub page_dir: PageDirectory,
    pub kpcr: Kpcr,
    pub irql_ctrl: IrqlController,
    pub sdt: SystemServiceTable,
    pub idtr: Idtr,
    pub ums_threads: BTreeMap<u32, UmsContext>,
    pub windbg_hooked: bool,
}

impl SovereignKernelInternals {
    pub fn new(arch: MemoryArch, base_register: u64, gs_base: u64, idle_thread: u32) -> Self {
        let mut sdt = SystemServiceTable::new();
        // Register core simulated sys call indexes
        sdt.register_syscall(1, 0xFFFFFFFF80011000); // NtCreateFile
        sdt.register_syscall(2, 0xFFFFFFFF80011400); // NtReadFile
        sdt.register_syscall(3, 0xFFFFFFFF80011800); // NtWriteFile

        Self {
            page_dir: PageDirectory::new(arch, base_register),
            kpcr: Kpcr::new(10, 0, gs_base, idle_thread),
            irql_ctrl: IrqlController::new(),
            sdt,
            idtr: Idtr { limit: 256 * 16 - 1, base: 0xFFFFFFFF80100000 },
            ums_threads: BTreeMap::new(),
            windbg_hooked: false,
        }
    }

    pub fn register_ums_thread(&mut self, thread_id: u32, context_addr: u64) {
        let thread = UmsContext {
            thread_id,
            state: UmsThreadState::Active,
            context_block: context_addr,
        };
        self.ums_threads.insert(thread_id, thread);
    }

    pub fn set_ums_thread_state(&mut self, thread_id: u32, state: UmsThreadState) -> Result<(), &'static str> {
        if let Some(thread) = self.ums_threads.get_mut(&thread_id) {
            thread.state = state;
            Ok(())
        } else {
            Err("UMS: Thread context not found")
        }
    }

    /// WinDbg Debugging Host Interface Emulator
    pub fn communicate_windbg(&mut self, event_desc: &str) -> String {
        self.windbg_hooked = true;
        format!("WinDbg Hooked: Captured event '{}' at current IRQL: {:?}", event_desc, self.irql_ctrl.get_current_irql())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4level_page_table_translation_and_permissions() {
        let mut pd = PageDirectory::new(MemoryArch::X86_64, 0x1A000);
        assert_eq!(pd.page_directory_base, 0x1A000);

        // Map page at virtual address 0x7FFFF0000000 to physical frame 1000
        pd.map_page_4level(0x7FFFF0000000, 1000, PageAccessMode::UserMode, true);

        // Translate address successfully
        let phys = pd.walk_page_tables(0x7FFFF0000123).unwrap();
        assert_eq!(phys, (1000 << 12) + 0x123);

        // Permissions checks
        assert!(pd.check_page_permission_4level(0x7FFFF0000123, PageAccessMode::UserMode).is_ok());

        // Introduce Page Fault mapping error
        assert!(pd.walk_page_tables(0xDEADBEEF000).is_err());
        assert_eq!(pd.cr2, 0xDEADBEEF000);
    }

    #[test]
    fn test_thread_scheduling_queues() {
        let mut kpcrb = Kpcrb::new(999);
        assert_eq!(kpcrb.current_thread_id, 999);

        // Add 2 threads
        let thread1 = KThread::new(101, 15);
        let thread2 = KThread::new(102, 12);
        kpcrb.add_thread(thread1);
        kpcrb.add_thread(thread2);

        // Schedule next thread
        let scheduled = kpcrb.schedule_next_thread().unwrap();
        assert_eq!(scheduled, 101);
        assert_eq!(kpcrb.current_thread_id, 101);

        // Block current thread to WaitQueue
        kpcrb.block_current_thread("Waiting for dynamic I/O event");
        assert_eq!(kpcrb.current_thread_id, 999);
        assert_eq!(kpcrb.wait_queue.len(), 1);
        assert_eq!(kpcrb.wait_queue[0].wait_reason.unwrap(), "Waiting for dynamic I/O event");
    }

    #[test]
    fn test_irql_lowering_triggers_apc_dpc_loops() {
        let mut kpcr = Kpcr::new(10, 0, 0x7FFFF7000, 999);
        let dpc = DeferredProcedureCall { id: 1, target_routine: 0x800400, priority: 2 };
        let apc = AsynchronousProcedureCall { id: 1, target_routine: 0x800500, kernel_mode: true, arg1: 42 };

        // Queue procedures
        kpcr.kpcrb.queue_dpc(dpc);
        kpcr.kpcrb.queue_apc(apc);

        let ctrl = IrqlController::new();
        ctrl.ke_raise_irql(Irql::DispatchLevel).unwrap();

        // Lowering to ApcLevel should drain/execute DPC queue but NOT APC queue yet
        ctrl.ke_lower_irql(Irql::ApcLevel, &mut kpcr.kpcrb).unwrap();
        assert_eq!(kpcr.kpcrb.dpc_count, 1);
        assert_eq!(kpcr.kpcrb.apc_count, 0);

        // Lowering further to PassiveLevel should drain/execute APC queue automatically
        ctrl.ke_lower_irql(Irql::PassiveLevel, &mut kpcr.kpcrb).unwrap();
        assert_eq!(kpcr.kpcrb.apc_count, 1);
    }

    #[test]
    fn test_traps_and_calling_convention_translation() {
        let mut sys_internals = SovereignKernelInternals::new(MemoryArch::X86_64, 0x3000, 0x7FFF100, 1);

        // Translate and profile Microsoft x64 -> System V AMD64 syscall
        let (handler, sysv_params) = sys_internals.sdt.translate_and_profile_syscall(1, 10, 20, 30, 40).unwrap();
        assert_eq!(handler, 0xFFFFFFFF80011000);
        // Translation check
        assert_eq!(sysv_params, [10, 20, 30, 40]);
        assert_eq!(*sys_internals.sdt.call_count.get(&1).unwrap(), 1);

        // Trap Frame verification
        let trap = TrapFrame {
            rax: 1,
            rcx: 10,
            rdx: 20,
            r8: 30,
            r9: 40,
            rip: 0xFFFFFFFF80011000,
            ..Default::default()
        };
        assert_eq!(trap.rax, 1);
    }
}
