// SigmaOS Sovereign Cross-Platform Kernel Internals Layer
// Parity bridge representing low-level kernel mechanisms from Linux, Windows, and BSD

#![no_std]

extern crate alloc;

use alloc::collections::{BTreeMap as HashMap, VecDeque};
use alloc::string::{String, ToString};
use alloc::format;
use core::sync::atomic::{AtomicU8, Ordering};

// =========================================================================
// 1. Virtual Address Space & Memory Layout (x86_64, ARM)
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

/// Represents virtual memory pages mapping to physical pages
#[derive(Debug, Clone)]
pub struct TranslationEntry {
    pub virtual_page: u64,
    pub physical_frame: u64,
    pub mode: PageAccessMode,
    pub writable: bool,
    pub dirty: bool,
    pub accessed: bool,
}

/// Simulated Page Directory or Translation Table Setup
#[derive(Debug, Clone)]
pub struct PageDirectory {
    pub arch: MemoryArch,
    pub page_directory_base: u64, // CR3 on x86_64, TTBR0/TTBR1 on ARM
    pub ttbr1_base: Option<u64>,  // TTBR1 for ARM Kernel space
    pub entries: HashMap<u64, TranslationEntry>,
}

impl PageDirectory {
    pub fn new(arch: MemoryArch, base_register: u64) -> Self {
        let ttbr1 = if arch == MemoryArch::ARM64 {
            Some(base_register + 0x1000) // simulated split TTBR1 kernel base
        } else {
            None
        };

        Self {
            arch,
            page_directory_base: base_register,
            ttbr1_base: ttbr1,
            entries: HashMap::new(),
        }
    }

    /// Insert simulated virtual-to-physical mapping
    pub fn map_page(
        &mut self,
        virtual_page: u64,
        physical_frame: u64,
        mode: PageAccessMode,
        writable: bool,
    ) {
        let entry = TranslationEntry {
            virtual_page,
            physical_frame,
            mode,
            writable,
            dirty: false,
            accessed: false,
        };
        self.entries.insert(virtual_page, entry);
    }

    /// Virtual-to-Physical Address Translation Simulator
    pub fn translate_address(&mut self, virtual_addr: u64) -> Result<u64, &'static str> {
        let page_size = 4096;
        let virtual_page = virtual_addr / page_size;
        let offset = virtual_addr % page_size;

        // Perform ARM TTBR0/TTBR1 split checking or standard CR3 checking
        if self.arch == MemoryArch::ARM64 {
            // Kernel address space high-bit check for ARM (TTBR1 vs TTBR0)
            let is_kernel_address = (virtual_addr & (1 << 63)) != 0;
            if is_kernel_address && self.ttbr1_base.is_none() {
                return Err("ARM TTBR1 (kernel space base) register is uninitialized");
            }
        }

        if let Some(entry) = self.entries.get_mut(&virtual_page) {
            entry.accessed = true;
            Ok(entry.physical_frame * page_size + offset)
        } else {
            Err("Page Fault: virtual page not present in translation directories")
        }
    }

    /// Verify page level permission access
    pub fn check_page_permission(
        &self,
        virtual_addr: u64,
        current_privilege: PageAccessMode,
    ) -> Result<(), &'static str> {
        let page_size = 4096;
        let virtual_page = virtual_addr / page_size;

        if let Some(entry) = self.entries.get(&virtual_page) {
            if entry.mode == PageAccessMode::KernelMode && current_privilege == PageAccessMode::UserMode {
                return Err("Access Violation: User-mode thread attempting to read Kernel-mode page");
            }
            Ok(())
        } else {
            Err("Page Fault: Page not mapped")
        }
    }
}

// =========================================================================
// 2. Processor Initialization & Control Regions (KPCR/KPCRB/Sys-Coprocessor)
// =========================================================================

/// Models Windows-style Deferred Procedure Call (DPC)
#[derive(Debug, Clone)]
pub struct DeferredProcedureCall {
    pub id: usize,
    pub target_routine: u64,
    pub priority: u8,
}

/// Processor Control Region Block (KPCRB) containing scheduler queues and context
#[derive(Debug, Clone)]
pub struct Kpcrb {
    pub current_thread_id: u32,
    pub next_thread_id: Option<u32>,
    pub idle_thread_id: u32,
    pub dpc_queue: VecDeque<DeferredProcedureCall>,
    pub interrupt_count: u64,
    pub dpc_count: u64,
}

impl Kpcrb {
    pub fn new(idle_thread: u32) -> Self {
        Self {
            current_thread_id: idle_thread,
            next_thread_id: None,
            idle_thread_id: idle_thread,
            dpc_queue: VecDeque::new(),
            interrupt_count: 0,
            dpc_count: 0,
        }
    }

    pub fn queue_dpc(&mut self, dpc: DeferredProcedureCall) {
        self.dpc_queue.push_back(dpc);
    }

    pub fn drain_dpc_queue(&mut self) -> usize {
        let count = self.dpc_queue.len();
        self.dpc_count += count as u64;
        self.dpc_queue.clear();
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
    pub coprocessor_regs: HashMap<String, u64>, // ARM System Coprocessor registers (e.g. CP15, TTBR, SCTLR)
    pub kpcrb: Kpcrb,
}

impl Kpcr {
    pub fn new(major: u16, minor: u16, gs_base: u64, idle_thread: u32) -> Self {
        let mut coprocessor_regs = HashMap::new();
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

/// Simulated Interrupt Request Level (IRQL) Controller
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
    pub fn ke_lower_irql(&self, new_irql: Irql) -> Result<(), &'static str> {
        let current = self.get_current_irql();
        if new_irql > current {
            return Err("KeLowerIrql: Attempting to raise IRQL via lower call (use KeRaiseIrql instead)");
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
// 4. IDT, IDTR, Faults, Traps, exceptions, and Debugging Systems
// =========================================================================

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

/// Service Descriptor Table for System Calls
#[derive(Debug, Clone)]
pub struct SystemServiceTable {
    pub service_table: HashMap<u32, u64>, // Maps syscall number to handler address
    pub call_count: HashMap<u32, u64>,    // Syscall profiling telemetry
}

impl SystemServiceTable {
    pub fn new() -> Self {
        Self {
            service_table: HashMap::new(),
            call_count: HashMap::new(),
        }
    }

    pub fn register_syscall(&mut self, sys_num: u32, handler_addr: u64) {
        self.service_table.insert(sys_num, handler_addr);
        self.call_count.insert(sys_num, 0);
    }

    /// Dispatch system call with trapping/profiling hooks
    pub fn dispatch_syscall(&mut self, sys_num: u32) -> Result<u64, &'static str> {
        if let Some(&addr) = self.service_table.get(&sys_num) {
            // Profile call
            let count = self.call_count.entry(sys_num).or_insert(0);
            *count += 1;
            Ok(addr)
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
    pub ums_threads: HashMap<u32, UmsContext>,
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
            ums_threads: HashMap::new(),
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
    fn test_virtual_address_translation_and_privileges() {
        // 1. Setup x86_64 PageDirectory map
        let mut pd = PageDirectory::new(MemoryArch::X86_64, 0x1A000);
        assert_eq!(pd.page_directory_base, 0x1A000);
        assert!(pd.ttbr1_base.is_none());

        // Map virtual page 5 (virtual address 20480 to 24575) to physical frame 100
        pd.map_page(5, 100, PageAccessMode::UserMode, true);

        // Translate user space virtual address 20490 (page 5, offset 10)
        let phys_addr = pd.translate_address(20490).unwrap();
        assert_eq!(phys_addr, 100 * 4096 + 10);

        // Unmapped translation should raise Page Fault error
        assert!(pd.translate_address(50000).is_err());

        // 2. Setup ARM64 split PageDirectory map
        let mut pd_arm = PageDirectory::new(MemoryArch::ARM64, 0x5000);
        assert_eq!(pd_arm.page_directory_base, 0x5000);
        assert_eq!(pd_arm.ttbr1_base.unwrap(), 0x5000 + 0x1000);

        // Map virtual kernel-mode page 0xFFFFFFFFFFFFF000 / 4096 to frame 50
        let kernel_vpage = 0xFFFFFFFFFFFFF000u64 / 4096;
        pd_arm.map_page(kernel_vpage, 50, PageAccessMode::KernelMode, false);

        // Translate high kernel-address (with bit 63 set)
        let arm_phys = pd_arm.translate_address(0xFFFFFFFFFFFFF010).unwrap();
        assert_eq!(arm_phys, 50 * 4096 + 16);

        // Verify kernel mode page privileges
        assert!(pd_arm.check_page_permission(0xFFFFFFFFFFFFF010, PageAccessMode::KernelMode).is_ok());
        // User mode access to Kernel mode page should raise Access Violation
        let perm_res = pd_arm.check_page_permission(0xFFFFFFFFFFFFF010, PageAccessMode::UserMode);
        assert_eq!(perm_res, Err("Access Violation: User-mode thread attempting to read Kernel-mode page"));
    }

    #[test]
    fn test_processor_control_region_and_dpc_mechanics() {
        // KPCR initialization
        let mut kpcr = Kpcr::new(10, 0, 0x7FFFF7000, 999);
        assert_eq!(kpcr.self_ptr, 0x7FFFF7000);
        assert_eq!(kpcr.gs_segment_base, 0x7FFFF7000);
        assert_eq!(kpcr.kpcrb.current_thread_id, 999);

        // ARM Coprocessor registers check
        assert_eq!(kpcr.read_coprocessor_reg("SCTLR_EL1").unwrap(), 0x30D00800);
        kpcr.write_coprocessor_reg("TTBR0_EL1".to_string(), 0x12000);
        assert_eq!(kpcr.read_coprocessor_reg("TTBR0_EL1").unwrap(), 0x12000);

        // Queue DPCs
        let dpc1 = DeferredProcedureCall { id: 1, target_routine: 0x800400, priority: 2 };
        let dpc2 = DeferredProcedureCall { id: 2, target_routine: 0x800500, priority: 3 };
        kpcr.kpcrb.queue_dpc(dpc1);
        kpcr.kpcrb.queue_dpc(dpc2);
        assert_eq!(kpcr.kpcrb.dpc_queue.len(), 2);

        // Drain DPCs
        let drained = kpcr.kpcrb.drain_dpc_queue();
        assert_eq!(drained, 2);
        assert_eq!(kpcr.kpcrb.dpc_queue.len(), 0);
        assert_eq!(kpcr.kpcrb.dpc_count, 2);
    }

    #[test]
    fn test_irql_levels_and_page_fault_safety() {
        let ctrl = IrqlController::new();
        assert_eq!(ctrl.get_current_irql(), Irql::PassiveLevel);

        // Raise IRQL to DISPATCH_LEVEL
        let old_irql = ctrl.ke_raise_irql(Irql::DispatchLevel).unwrap();
        assert_eq!(old_irql, Irql::PassiveLevel);
        assert_eq!(ctrl.get_current_irql(), Irql::DispatchLevel);

        // Attempting to lower IRQL via KeRaiseIrql should fail
        assert!(ctrl.ke_raise_irql(Irql::PassiveLevel).is_err());

        // Check page fault safety at DISPATCH_LEVEL (should fail)
        assert!(ctrl.check_page_fault_safety().is_err());

        // Lower IRQL back to PassiveLevel
        ctrl.ke_lower_irql(Irql::PassiveLevel).unwrap();
        assert_eq!(ctrl.get_current_irql(), Irql::PassiveLevel);

        // Page fault safety at PassiveLevel (should pass)
        assert!(ctrl.check_page_fault_safety().is_ok());

        // Attempting to raise IRQL via KeLowerIrql should fail
        assert!(ctrl.ke_lower_irql(Irql::HighLevel).is_err());
    }

    #[test]
    fn test_system_calls_traps_and_windbg() {
        let mut sys_internals = SovereignKernelInternals::new(MemoryArch::X86_64, 0x3000, 0x7FFF100, 1);
        assert_eq!(sys_internals.idtr.limit, 4095);

        // Dispatch NtCreateFile syscall
        let addr = sys_internals.sdt.dispatch_syscall(1).unwrap();
        assert_eq!(addr, 0xFFFFFFFF80011000);
        assert_eq!(*sys_internals.sdt.call_count.get(&1).unwrap(), 1);

        // Dispatch NtWriteFile syscall
        sys_internals.sdt.dispatch_syscall(3).unwrap();
        assert_eq!(*sys_internals.sdt.call_count.get(&3).unwrap(), 1);

        // Dispatching invalid syscall number
        assert!(sys_internals.sdt.dispatch_syscall(99).is_err());

        // Hook WinDbg and query trace
        let response = sys_internals.communicate_windbg("KBUGCHECK_TRIGGERED");
        assert!(sys_internals.windbg_hooked);
        assert!(response.contains("WinDbg Hooked"));
        assert!(response.contains("PassiveLevel"));
    }

    #[test]
    fn test_user_mode_scheduling() {
        let mut sys_internals = SovereignKernelInternals::new(MemoryArch::X86_64, 0x3000, 0x7FFF100, 1);

        sys_internals.register_ums_thread(101, 0x150000);
        let thread_context = sys_internals.ums_threads.get(&101).unwrap();
        assert_eq!(thread_context.thread_id, 101);
        assert_eq!(thread_context.state, UmsThreadState::Active);
        assert_eq!(thread_context.context_block, 0x150000);

        // Change UMS thread state
        sys_internals.set_ums_thread_state(101, UmsThreadState::Blocked).unwrap();
        assert_eq!(sys_internals.ums_threads.get(&101).unwrap().state, UmsThreadState::Blocked);

        // Change state on unmapped thread id should return err
        assert!(sys_internals.set_ums_thread_state(202, UmsThreadState::Suspended).is_err());
    }
}
