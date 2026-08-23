// 1. Instructions and CPU Initialization

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArchitectureClass {
    X86_64,
    AArch64,
    RiscV64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionCyclePhase {
    Fetch,
    Decode,
    Execute,
    Writeback,
    Commit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorInitState {
    Offline,
    RealMode,
    ProtectedMode,
    LongMode,
    Ready,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CpuRegisters {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64, // PML4 Page directory base register
    pub cr4: u64,
}

// 2. Interrupt Request Levels (IRQLs) & Faults

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Irql {
    PassiveLevel = 0,   // User/normal thread execution
    ApcLevel = 1,       // Asynchronous Procedure Calls
    DispatchLevel = 2,  // Scheduler/DPC execution, No paging allowed!
    Dirql = 3,          // Device Interrupt Request Level
    HighLevel = 4,      // Hardware profiling/high priority halts
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareException {
    DivideByZero = 0,
    PageFault = 14,
    GeneralProtectionFault = 13,
    DoubleFault = 8,
}

// 3. VMM Pool Memory & MDLs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolType {
    NonPagedPool, // Guaranteed to stay in physical memory
    PagedPool,    // Can be paged out to swap space (Invalid at IRQL >= DispatchLevel!)
}

pub struct LookasideList {
    pub pool_type: PoolType,
    pub block_size: usize,
    pub cached_blocks: Vec<Vec<u8>>,
}

impl LookasideList {
    pub fn new(pool_type: PoolType, block_size: usize) -> Self {
        Self {
            pool_type,
            block_size,
            cached_blocks: Vec::new(),
        }
    }

    pub fn alloc_block(&mut self) -> Vec<u8> {
        self.cached_blocks.pop().unwrap_or_else(|| vec![0u8; self.block_size])
    }

    pub fn free_block(&mut self, block: Vec<u8>) {
        if self.cached_blocks.len() < 8 && block.len() == self.block_size {
            self.cached_blocks.push(block);
        }
    }
}

// 7. Multi-Architecture HAL Abstractions (x86_64, AArch64, RISC-V 64)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct X86_64Hal {
    pub lapic_base: u64,
    pub ioapic_base: u64,
    pub cr0: u64,
    pub cr4: u64,
    pub efer: u64,
}

impl X86_64Hal {
    pub fn new() -> Self {
        Self {
            lapic_base: 0xFEE0_0000,
            ioapic_base: 0xFEC0_0000,
            cr0: 0x8001_0033, // PE, WP, PG set
            cr4: 0x0000_06B0, // PAE, PGE, OSFXSR, OSXMMEXCPT
            efer: 0x0000_0D01, // LME, LMA, NXE
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AArch64Hal {
    pub gicd_base: u64, // GIC Distributor
    pub gicc_base: u64, // GIC Cpu Interface
    pub ttbr0: u64,     // User Page Table Base
    pub ttbr1: u64,     // Kernel Page Table Base
    pub sctlr_el1: u64, // System Control Register EL1
}

impl AArch64Hal {
    pub fn new() -> Self {
        Self {
            gicd_base: 0x0800_0000,
            gicc_base: 0x0801_0000,
            ttbr0: 0x0000_4000,
            ttbr1: 0x0000_8000,
            sctlr_el1: 0x30D0_0800, // MMU, Caches enabled
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RiscV64Hal {
    pub plic_base: u64,  // Platform Level Interrupt Controller
    pub clint_base: u64, // Core Local Interruptor (Timers/IPI)
    pub satp: u64,       // Supervisor Address Translation and Protection (Sv39/Sv48)
    pub sstatus: u64,    // Supervisor Status Register
}

impl RiscV64Hal {
    pub fn new() -> Self {
        Self {
            plic_base: 0x0C00_0000,
            clint_base: 0x0200_0000,
            satp: (8u64 << 60) | 0x0000_8000, // Sv39 mode (MODE = 8)
            sstatus: 0x0000_0020,             // SIE (Supervisor Interrupt Enable)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_arch_hal_initialization() {
        let x86 = X86_64Hal::new();
        assert_eq!(x86.lapic_base, 0xFEE0_0000);
        assert_ne!(x86.cr0 & (1 << 31), 0); // Paging enabled bit

        let arm = AArch64Hal::new();
        assert_eq!(arm.gicd_base, 0x0800_0000);

        let riscv = RiscV64Hal::new();
        assert_eq!(riscv.satp >> 60, 8); // Sv39 mode bit
    }

    #[test]
    fn test_irql_transition_and_pool_validation() {
        let mut engine = ArchitectureEngine::new();
        assert_eq!(engine.current_irql, Irql::PassiveLevel);

        let old_irql = engine.raise_irql(Irql::DispatchLevel).unwrap();
        assert_eq!(old_irql, Irql::PassiveLevel);
        assert_eq!(engine.current_irql, Irql::DispatchLevel);

        // PagedPool allocation at DispatchLevel must trigger fault exception
        let err = engine.allocate_pool(PoolType::PagedPool);
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), HardwareException::DoubleFault);

        // NonPagedPool allocation succeeds at DispatchLevel
        assert!(engine.allocate_pool(PoolType::NonPagedPool).is_ok());

        assert!(engine.lower_irql(Irql::PassiveLevel).is_ok());
        assert_eq!(engine.current_irql, Irql::PassiveLevel);
    }
}

/// Memory Descriptor List (MDL) mapping virtual buffer to locked physical pages
pub struct MemoryDescriptorList {
    pub virtual_address: usize,
    pub byte_count: usize,
    pub locked_physical_pages: Vec<usize>, // List of physical page frame numbers
    pub is_locked: bool,
}

impl MemoryDescriptorList {
    pub fn new(virtual_address: usize, byte_count: usize) -> Self {
        Self {
            virtual_address,
            byte_count,
            locked_physical_pages: Vec::new(),
            is_locked: false,
        }
    }

    /// Locks virtual buffer into physical pages (Standard Linux/Windows VMM behavior)
    pub fn lock_pages(&mut self) {
        let page_size = 4096;
        let num_pages = (self.byte_count + page_size - 1) / page_size;
        self.locked_physical_pages.clear();
        for i in 0..num_pages {
            // Map virtual page to a simulated physical page Frame Number (PFN)
            let pfn = (self.virtual_address / page_size) + i + 0x10000;
            self.locked_physical_pages.push(pfn);
        }
        self.is_locked = true;
    }

    pub fn unlock_pages(&mut self) {
        self.locked_physical_pages.clear();
        self.is_locked = false;
    }
}

// 4. Processes & Threads

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Running,
    Ready,
    Waiting,
    Transition,
    Terminated,
}

/// Thread Control Block (TCB)
pub struct Tcb {
    pub thread_id: usize,
    pub parent_process_id: usize,
    pub state: ThreadState,
    pub priority: u8,
    pub registers: CpuRegisters,
    pub stack_base: usize,
    pub stack_limit: usize,
}

impl Tcb {
    pub fn new(thread_id: usize, parent_process_id: usize, rip: u64, rsp: u64) -> Self {
        let mut registers = CpuRegisters::default();
        registers.rip = rip;
        registers.rsp = rsp;
        Self {
            thread_id,
            parent_process_id,
            state: ThreadState::Ready,
            priority: 8, // Default normal priority
            registers,
            stack_base: rsp as usize,
            stack_limit: (rsp - 0x100000) as usize, // 1MB stack
        }
    }
}

/// Process Control Block (PCB)
pub struct Pcb {
    pub process_id: usize,
    pub page_directory_base: usize, // CR3 value
    pub thread_list: Vec<Tcb>,
    pub environment_variables: Vec<(String, String)>,
}

impl Pcb {
    pub fn new(process_id: usize, cr3: usize) -> Self {
        Self {
            process_id,
            page_directory_base: cr3,
            thread_list: Vec::new(),
            environment_variables: Vec::new(),
        }
    }
}

// 5. System Call SSDT Tables

pub type SyscallHandler = fn(args: &[usize]) -> usize;

pub struct SystemServiceDescriptorTable {
    pub service_table: Vec<Option<SyscallHandler>>,
}

impl SystemServiceDescriptorTable {
    pub fn new() -> Self {
        let mut service_table = Vec::new();
        // Pre-allocate slots for standard system services (up to 64)
        for _ in 0..64 {
            service_table.push(None);
        }
        Self { service_table }
    }

    pub fn register_service(&mut self, id: usize, handler: SyscallHandler) {
        if id < self.service_table.len() {
            self.service_table[id] = Some(handler);
        }
    }
}

// 6. Unified Architecture Engine

pub struct ArchitectureEngine {
    pub init_state: ProcessorInitState,
    pub current_irql: Irql,
    pub ssdt: SystemServiceDescriptorTable,
    pub lookaside_nonpaged: LookasideList,
    pub lookaside_paged: LookasideList,
    pub running_pcb: Option<Pcb>,
}

impl ArchitectureEngine {
    pub fn new() -> Self {
        Self {
            init_state: ProcessorInitState::Offline,
            current_irql: Irql::PassiveLevel,
            ssdt: SystemServiceDescriptorTable::new(),
            lookaside_nonpaged: LookasideList::new(PoolType::NonPagedPool, 1024),
            lookaside_paged: LookasideList::new(PoolType::PagedPool, 1024),
            running_pcb: None,
        }
    }

    /// Simulates low-level hardware bootstrap initialization (Real Mode -> Protected Mode -> Long Mode)
    pub fn init_processor(&mut self) -> Result<(), &'static str> {
        self.init_state = ProcessorInitState::RealMode;
        println!("[arch] Processor Bootstrap: Entered Real Mode (16-bit segmented addressing).");

        // Transition to Protected Mode
        self.init_state = ProcessorInitState::ProtectedMode;
        println!("[arch] GDT loaded. Entered Protected Mode (32-bit flat memory & CR0 enable).");

        // Enable paging levels and enter Long Mode (64-bit AMD64/x64)
        self.init_state = ProcessorInitState::LongMode;
        println!("[arch] PML4 paging directories enabled. Entered 64-bit Long Mode (EFER.LME set).");

        self.init_state = ProcessorInitState::Ready;
        println!("[arch] BSP Core initialized successfully. Ready to schedule.");
        Ok(())
    }

    /// Promote current processor's Interrupt Request Level (Windows IRQL parity)
    pub fn raise_irql(&mut self, new_irql: Irql) -> Result<Irql, &'static str> {
        if new_irql < self.current_irql {
            return Err("raise_irql: Cannot lower IRQL using raise_irql()");
        }
        let old_irql = self.current_irql;
        self.current_irql = new_irql;
        Ok(old_irql)
    }

    /// Demote current processor's Interrupt Request Level
    pub fn lower_irql(&mut self, old_irql: Irql) -> Result<(), &'static str> {
        if old_irql > self.current_irql {
            return Err("lower_irql: Cannot raise IRQL using lower_irql()");
        }
        self.current_irql = old_irql;
        Ok(())
    }

    /// Dynamic pool memory allocation honoring IRQL paging validation rules
    pub fn allocate_pool(&mut self, pool_type: PoolType) -> Result<Vec<u8>, HardwareException> {
        if pool_type == PoolType::PagedPool && self.current_irql >= Irql::DispatchLevel {
            // Standard Windows BugCheck: PAGE_FAULT_IN_NONPAGED_AREA / IRQL_NOT_LESS_OR_EQUAL
            println!("[arch-fault] FATAL: Accessing PagedPool at IRQL >= DispatchLevel! Triggering DoubleFault.");
            self.handle_fault(HardwareException::DoubleFault, None);
            return Err(HardwareException::DoubleFault);
        }

        let block = match pool_type {
            PoolType::NonPagedPool => self.lookaside_nonpaged.alloc_block(),
            PoolType::PagedPool => self.lookaside_paged.alloc_block(),
        };
        Ok(block)
    }

    /// Performs low-level trap, fault, and exception recoveries
    pub fn handle_fault(&mut self, exception: HardwareException, address: Option<usize>) {
        println!(
            "[arch-fault] HW EXCEPTION #{:?}: Faulting Address = {:?}, Core State = {:?}",
            exception, address, self.init_state
        );
        // Execute recovery actions, e.g. unmapping bad page or halting current thread
    }

    /// Simulates task switch / context-switching of thread registers and CR3 (PML4) directories
    pub fn context_switch_threads(&mut self, from_idx: usize, to_idx: usize) -> Result<(), &'static str> {
        let pcb = self.running_pcb.as_mut().ok_or("No active PCB loaded")?;
        if from_idx >= pcb.thread_list.len() || to_idx >= pcb.thread_list.len() {
            return Err("Invalid thread index bounds");
        }

        // 1. Save register context of current running thread
        pcb.thread_list[from_idx].state = ThreadState::Ready;
        let mut saved_regs = pcb.thread_list[from_idx].registers;
        saved_regs.rax = 0xAA; // Simulated saved context values

        // 2. Restore register context of target thread
        pcb.thread_list[to_idx].state = ThreadState::Running;
        let target_regs = pcb.thread_list[to_idx].registers;

        // 3. Switch page directory mapping (CR3 / PML4 register base)
        let cr3 = pcb.page_directory_base;
        println!(
            "[arch] Context Swapped: Thread #{} -> Thread #{}. CR3 page directory directory loaded: 0x{:X}.",
            pcb.thread_list[from_idx].thread_id,
            pcb.thread_list[to_idx].thread_id,
            cr3
        );

        Ok(())
    }

    /// System Call Service Dispatcher (sysenter/sysexit and syscall/sysret parity)
    pub fn dispatch_ssdt_syscall(&self, id: usize, args: &[usize]) -> Result<usize, &'static str> {
        if id >= self.ssdt.service_table.len() {
            return Err("Syscall ID exceeds SSDT size bounds");
        }
        if let Some(ref handler) = self.ssdt.service_table[id] {
            let res = handler(args);
            Ok(res)
        } else {
            Err("Syscall not registered in SSDT")
        }
    }
}
