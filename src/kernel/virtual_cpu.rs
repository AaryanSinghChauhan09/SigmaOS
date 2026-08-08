// Sovereign Virtual CPU and Ring Privilege Separation Simulator
<<<<<<< HEAD
// Implements x86/ARM CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.
// Also supports function invocation, arithmetic status flags, branching, Thumb state, switch cases, JIT compilation,
// self-modifying code, lock-prefixed atomic/synchronization primitives, and software interrupt mechanisms.
||||||| 23ef22a4a
// Implements x86 CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.
// Extended to support Windows-inspired NT kernel abstractions: APCs, DPCs, IRQL preemption, and Thread dispatcher.
// Extended to support CISC-style block memory movement, bitwise shifts, and memory barrier instructions.
=======
// Implements x86 CPU Modes, Ring privilege isolation (Rings 0-3), Register Sets, and Instruction Data Movement.
// Enhanced with Model Specific Registers (MSRs), lazy FP/SSE state saving (Linux/BSD style), and Exception trap vector routines.
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
<<<<<<< HEAD

// Flag bit positions in rflags (aligned with standard x86 rflags)
pub const FLAG_CF: u64 = 1 << 0;  // Carry Flag
pub const FLAG_ZF: u64 = 1 << 6;  // Zero Flag
pub const FLAG_SF: u64 = 1 << 7;  // Sign Flag
pub const FLAG_OF: u64 = 1 << 11; // Overflow Flag
||||||| 23ef22a4a
use core::sync::atomic::Ordering;
=======
use core::sync::atomic::{AtomicUsize, Ordering};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
<<<<<<< HEAD
    StackUnderflow = 5,
    InvalidBranchTarget = 6,
    SegmentationFault = 7,
    InvalidInterruptVector = 8,
    InterruptHandlerNotFound = 9,
    JitCacheMiss = 10,
    InvalidPrivilegeLevel = 11,
    DivisionByZero = 12,
||||||| 23ef22a4a
    AlignmentFault = 5,
    InvalidAddress = 6,
    IrqlViolation = 7,
    ThreadSuspended = 8,
    ApcDeliveryFailed = 9,
}

/// ARM-inspired addressing modes for LDR & STR instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,   // Immediate address or value load
    Offset,      // base + offset, base unchanged
    PreIndexed,  // base + offset, base updated before access
    PostIndexed, // base, base updated after access
}

/// ARM-inspired block data transfer modes for LDM & STM instructions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockTransferMode {
    IncrementAfter,  // IA: Increment address after each transfer
    IncrementBefore, // IB: Increment address before each transfer
    DecrementAfter,  // DA: Decrement address after each transfer
    DecrementBefore, // DB: Decrement address before each transfer
=======
    InvalidInstruction = 5,
    FloatingPointStateNotSaved = 6,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

/// x86 CPU Execution Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuMode {
    RealMode,      // 16-bit real addressing
    ProtectedMode, // 32-bit protected segments
    LongMode,      // 64-bit paging active
}

/// CPU Ring Privilege Separation levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuRing {
    Ring0 = 0, // Kernel Core (unrestricted)
    Ring1 = 1, // Device Drivers (SDF / isolated)
    Ring2 = 2, // System Services (init system)
    Ring3 = 3, // Userland Applications (most restricted)
}

<<<<<<< HEAD
/// Complete Virtual Register Set (inspired by x86-64 and ARM)
#[derive(Debug, Clone, Copy, Default)]
||||||| 23ef22a4a
/// Complete hybrid Virtual Register Set
#[derive(Debug, Clone, Copy)]
=======
/// Complete x86 Virtual Register Set
#[derive(Debug, Clone, Copy)]
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
<<<<<<< HEAD
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64, // Base Frame Pointer
    pub rsp: u64, // Stack Pointer
    pub rip: u64, // Instruction Pointer
    pub rflags: u64, // Status flags: ZF, SF, CF, OF
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable)
    pub cr3: u64, // Control Register 3: Page Table Base Address
}

/// Virtual bytecode instructions simulated dynamically
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    MovRegReg { dest: String, src: String },
    MovRegVal { dest: String, val: u64 },
    Push { reg: String },
    Pop { reg: String },
    Add { dest: String, src: String },
    AddVal { dest: String, val: u64 },
    Sub { dest: String, src: String },
    SubVal { dest: String, val: u64 },
    Mul { src: String },
    Div { src: String },
    And { dest: String, src: String },
    Or { dest: String, src: String },
    Xor { dest: String, src: String },
    Shl { dest: String, shift: u8 },
    Shr { dest: String, shift: u8 },
    Cmp { reg: String, val: u64 },
    CmpReg { reg1: String, reg2: String },
    Jmp { target: u64 },
    Je { target: u64 },
    Jne { target: u64 },
    Jg { target: u64 },
    Jl { target: u64 },
    Call { target: u64 },
    Ret,
    Syscall,
    Int { vector: u8 },
    SwitchCase { index_reg: String, targets: Vec<u64> },
    LockXchg { reg: String, mem_addr: usize },
    Cmpxchg { reg: String, expected: u64, desired: u64, mem_addr: usize },
    TestAndSet { mem_addr: usize },
    WriteMem { addr: usize, val: u8 },
||||||| 23ef22a4a
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable), Bit 3 is TS (Task Switched)
    pub cr3: u64, // Control Register 3: Page Table Base Address
    pub cr4: u64, // Control Register 4: Os Support for SSE/XSAVE
    pub rip: u64, // Instruction Pointer
    pub rsp: u64, // Stack Pointer
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
    // CPU Status Flags
    pub zf: bool, // Zero Flag
    pub cf: bool, // Carry Flag
    pub sf: bool, // Sign Flag
}

/// Windows NT inspired Interrupt Request Levels (IRQLs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Irql {
    PassiveLevel = 0,  // PASSIVE_LEVEL: Normal thread execution, user/kernel APCs active
    ApcLevel = 1,      // APC_LEVEL: Thread APC delivery level, thread scheduling disabled
    DispatchLevel = 2, // DISPATCH_LEVEL: DPC execution and thread scheduler level, no thread context
    Dirql = 3,         // DIRQL: Device Interrupt Request Level (hardware drivers)
}

/// Windows NT inspired Asynchronous Procedure Call (APC) Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApcType {
    SpecialKernel, // Kernel-mode, bypasses thread state & alert restrictions, high priority
    NormalKernel,  // Kernel-mode, runs in kernel-mode, runs below special kernel APCs
    User,          // User-mode, runs only when thread enters alertable state
}

/// Windows NT inspired Processor Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessorMode {
    KernelMode,
    UserMode,
}

/// Windows NT inspired Thread State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Running,
    Suspended,
    Terminated,
}

/// Windows NT inspired Asynchronous Procedure Call (APC) structure
#[derive(Clone)]
pub struct SovereignApc {
    pub apc_type: ApcType,
    pub kernel_routine: fn(&mut SovereignVirtualCPU, u64) -> Result<(), CpuError>,
    pub rundown_routine: Option<fn(&mut SovereignVirtualCPU, u64) -> Result<(), CpuError>>,
    pub normal_routine: Option<fn(&mut SovereignVirtualCPU, u64, u64) -> Result<(), CpuError>>,
    pub normal_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub freed_on_delivery: bool, // Simulates allocation from non-paged pool
}

/// Windows NT inspired Deferred Procedure Call (DPC) structure
#[derive(Clone)]
pub struct SovereignDpc {
    pub deferred_routine: fn(&mut SovereignVirtualCPU, u64, u64, u64) -> Result<(), CpuError>,
    pub deferred_context: u64,
    pub system_argument1: u64,
    pub system_argument2: u64,
    pub importance: u32, // Priority/importance: higher value = higher precedence
}

/// Windows NT inspired Work Item (kernel worker task)
#[derive(Clone, Copy)]
pub struct WorkItem {
    pub worker_routine: fn(&mut SovereignVirtualCPU, u64) -> Result<(), CpuError>,
    pub parameter: u64,
    pub is_queued: bool,
}

/// Windows NT inspired KTHREAD equivalent structure inside SigmaOS CPU Context
#[derive(Clone)]
pub struct SovereignThread {
    pub id: u64,
    pub parent_id: u64,
    pub state: ThreadState,
    pub suspend_count: u32,
    pub alertable: bool,
    pub kernel_apc_disable: bool,
    pub apc_queue: Vec<SovereignApc>,
}

/// Windows NT inspired KPRCB (Kernel Processor Control Block) equivalent
#[derive(Clone)]
pub struct SovereignKprcb {
    pub current_irql: Irql,
    pub dpc_queue: Vec<SovereignDpc>,
    pub work_item_queue: Vec<WorkItem>,
    pub active_thread_id: u64,
}

/// Model Specific Registers (MSRs) for Fast System Call routing (Intel/AMD standard)
#[derive(Debug, Clone, Copy)]
pub struct ModelSpecificRegisters {
    pub efer: u64,   // Extended Feature Enable Register
    pub star: u64,   // Segment selector for SYSENTER/SYSEXIT
    pub lstar: u64,  // Target RIP for 64-bit SYSCALL
    pub sfmask: u64, // RFLAGS mask for SYSCALL
=======
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable), Bit 3 is TS (Task Switched)
    pub cr3: u64, // Control Register 3: Page Table Base Address
    pub cr4: u64, // Control Register 4: Os Support for SSE/XSAVE
    pub rip: u64, // Instruction Pointer
    pub rsp: u64, // Stack Pointer
}

/// Model Specific Registers (MSRs) for Fast System Call routing (Intel/AMD standard)
#[derive(Debug, Clone, Copy)]
pub struct ModelSpecificRegisters {
    pub efer: u64,   // Extended Feature Enable Register
    pub star: u64,   // Segment selector for SYSENTER/SYSEXIT
    pub lstar: u64,  // Target RIP for 64-bit SYSCALL
    pub sfmask: u64, // RFLAGS mask for SYSCALL
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
<<<<<<< HEAD
    pub ram: Vec<u8>,
    pub thumb_state: bool, // Thumb Mode (CPSR T-bit) from ARM for compact 16-bit emulation
    pub isr_table: Vec<Option<fn(&mut SovereignVirtualCPU)>>, // Software interrupt handlers
    pub jit_cache: BTreeMap<u64, Vec<Instruction>>, // Simple JIT block emulator cache (no_std compatible)
    pub code_cache_invalidated: bool, // Track self-modifying code changes
||||||| 23ef22a4a
    pub memory: Vec<u8>,
    pub kprcb: SovereignKprcb,
    pub threads: Vec<SovereignThread>,
=======
    // Lazy FP/SSE Context Tracking (Linux/BSD style)
    pub fp_dirty: bool,
    pub fp_save_area: [u64; 64], // Simulated 512-byte FXSAVE/XSAVE area
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

impl SovereignVirtualCPU {
    pub fn new() -> Self {
        Self {
            mode: CpuMode::RealMode,
            ring: CpuRing::Ring0, // Starts in Kernel Ring 0 during early boot
            registers: RegisterSet {
                rax: 0,
                rbx: 0,
                rcx: 0,
                rdx: 0,
<<<<<<< HEAD
                rsi: 0,
                rdi: 0,
                rbp: 1024,
                rsp: 1024, // High stack pointer
                rip: 0,
                rflags: 0,
                cr0: 0,
                cr3: 0,
||||||| 23ef22a4a
                cr0: 0,
                cr3: 0,
                cr4: 0,
                rip: 0,
                rsp: 1024, // High stack pointer
                rsi: 0,
                rdi: 0,
                rbp: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                r13: 0,
                r14: 0,
                r15: 0,
                zf: false,
                cf: false,
                sf: false,
            },
            msrs: ModelSpecificRegisters {
                efer: 0,
                star: 0,
                lstar: 0,
                sfmask: 0,
=======
                cr0: 0,
                cr3: 0,
                cr4: 0,
                rip: 0,
                rsp: 1024, // High stack pointer
            },
            msrs: ModelSpecificRegisters {
                efer: 0,
                star: 0,
                lstar: 0,
                sfmask: 0,
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
            },
            stack_memory: vec![0; 128], // 128 stack frames
<<<<<<< HEAD
            ram: vec![0; 4096], // 4KB RAM
            thumb_state: false,
            isr_table: vec![None; 256],
            jit_cache: BTreeMap::new(),
            code_cache_invalidated: false,
||||||| 23ef22a4a
            memory: vec![0; 4096],     // 4096 bytes of simulated RAM
            kprcb: SovereignKprcb {
                current_irql: Irql::PassiveLevel,
                dpc_queue: Vec::new(),
                work_item_queue: Vec::new(),
                active_thread_id: 1,
            },
            threads: vec![
                SovereignThread {
                    id: 1,
                    parent_id: 0,
                    state: ThreadState::Running,
                    suspend_count: 0,
                    alertable: false,
                    kernel_apc_disable: false,
                    apc_queue: Vec::new(),
                }
            ],
=======
            fp_dirty: false,
            fp_save_area: [0; 64],
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        }
    }

<<<<<<< HEAD
    /// Retrieves register value by name
    pub fn get_reg_val(&self, reg: &str) -> Result<u64, CpuError> {
        match reg {
            "rax" => Ok(self.registers.rax),
            "rbx" => Ok(self.registers.rbx),
            "rcx" => Ok(self.registers.rcx),
            "rdx" => Ok(self.registers.rdx),
            "rsi" => Ok(self.registers.rsi),
            "rdi" => Ok(self.registers.rdi),
            "rbp" => Ok(self.registers.rbp),
            "rsp" => Ok(self.registers.rsp),
            "rip" => Ok(self.registers.rip),
            "rflags" => Ok(self.registers.rflags),
            _ => Err(CpuError::InvalidRegister),
        }
    }

    /// Sets register value by name
    pub fn set_reg_val(&mut self, reg: &str, val: u64) -> Result<(), CpuError> {
        match reg {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
            "rsi" => self.registers.rsi = val,
            "rdi" => self.registers.rdi = val,
            "rbp" => self.registers.rbp = val,
            "rsp" => self.registers.rsp = val,
            "rip" => self.registers.rip = val,
            "rflags" => self.registers.rflags = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Simulates standard assembly data movement: `mov <dest>, <src_val>`
||||||| 23ef22a4a
    /// Helper to get a register's value dynamically by string name
    pub fn get_register(&self, name: &str) -> Result<u64, CpuError> {
        match name {
            "rax" => Ok(self.registers.rax),
            "rbx" => Ok(self.registers.rbx),
            "rcx" => Ok(self.registers.rcx),
            "rdx" => Ok(self.registers.rdx),
            "cr0" => Ok(self.registers.cr0),
            "cr3" => Ok(self.registers.cr3),
            "rip" => Ok(self.registers.rip),
            "rsp" => Ok(self.registers.rsp),
            "rsi" => Ok(self.registers.rsi),
            "rdi" => Ok(self.registers.rdi),
            "rbp" => Ok(self.registers.rbp),
            "r8" => Ok(self.registers.r8),
            "r9" => Ok(self.registers.r9),
            "r10" => Ok(self.registers.r10),
            "r11" => Ok(self.registers.r11),
            "r12" => Ok(self.registers.r12),
            "r13" => Ok(self.registers.r13),
            "r14" => Ok(self.registers.r14),
            "r15" => Ok(self.registers.r15),
            _ => Err(CpuError::InvalidRegister),
        }
    }

    /// Helper to set a register's value dynamically by string name
    pub fn set_register(&mut self, name: &str, val: u64) -> Result<(), CpuError> {
        match name {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
            "cr0" => self.registers.cr0 = val,
            "cr3" => self.registers.cr3 = val,
            "rip" => self.registers.rip = val,
            "rsp" => self.registers.rsp = val,
            "rsi" => self.registers.rsi = val,
            "rdi" => self.registers.rdi = val,
            "rbp" => self.registers.rbp = val,
            "r8" => self.registers.r8 = val,
            "r9" => self.registers.r9 = val,
            "r10" => self.registers.r10 = val,
            "r11" => self.registers.r11 = val,
            "r12" => self.registers.r12 = val,
            "r13" => self.registers.r13 = val,
            "r14" => self.registers.r14 = val,
            "r15" => self.registers.r15 = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Simulates standard x86 assembly data movement: `mov <dest>, <src_val>`
=======
    /// Simulates standard x86 assembly data movement: `mov <dest>, <src_val>`
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
<<<<<<< HEAD
        self.set_reg_val(dest, val)
    }

    /// Simulates standard stack pushing: `push <val>`
||||||| 23ef22a4a
        self.set_register(dest, val)
    }

    /// Helper for checking address boundary and privilege violations
    /// Addresses >= 2048 are kernel-protected and restricted from Ring 3
    pub fn check_memory_privilege(&self, addr: u64, size: u64) -> Result<(), CpuError> {
        let limit = addr.checked_add(size).ok_or(CpuError::InvalidAddress)?;
        if limit > self.memory.len() as u64 {
            return Err(CpuError::InvalidAddress);
        }
        if self.ring == CpuRing::Ring3 {
            // Access in kernel zone denied to User Ring 3
            if addr >= 2048 || limit > 2048 {
                return Err(CpuError::PrivilegeViolation);
            }
        }
        Ok(())
    }

    /// Helper to read a u64 in little-endian format from simulated memory
    pub fn read_mem_u64(&self, addr: u64) -> Result<u64, CpuError> {
        self.check_memory_privilege(addr, 8)?;
        let idx = addr as usize;
        let bytes = [
            self.memory[idx],
            self.memory[idx + 1],
            self.memory[idx + 2],
            self.memory[idx + 3],
            self.memory[idx + 4],
            self.memory[idx + 5],
            self.memory[idx + 6],
            self.memory[idx + 7],
        ];
        Ok(u64::from_le_bytes(bytes))
    }

    /// Helper to write a u64 in little-endian format to simulated memory
    pub fn write_mem_u64(&mut self, addr: u64, val: u64) -> Result<(), CpuError> {
        self.check_memory_privilege(addr, 8)?;
        let bytes = val.to_le_bytes();
        let idx = addr as usize;
        self.memory[idx..idx + 8].copy_from_slice(&bytes);
        Ok(())
    }

    /// Emulates load register: `LDR <dest_reg>, [<base_reg>, #offset]`
    pub fn ldr(&mut self, dest_reg: &str, base_reg: &str, offset: i64, mode: AddressingMode) -> Result<(), CpuError> {
        let base_val = self.get_register(base_reg)?;
        let target_addr = match mode {
            AddressingMode::Immediate => {
                let val = offset as u64;
                self.set_register(dest_reg, val)?;
                return Ok(());
            }
            AddressingMode::Offset => {
                (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64
            }
            AddressingMode::PreIndexed => {
                let addr = (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64;
                self.set_register(base_reg, addr)?;
                addr
            }
            AddressingMode::PostIndexed => {
                let addr = base_val;
                let next_base = (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64;
                self.set_register(base_reg, next_base)?;
                addr
            }
        };

        let val = self.read_mem_u64(target_addr)?;
        self.set_register(dest_reg, val)?;
        Ok(())
    }

    /// Emulates store register: `STR <src_reg>, [<base_reg>, #offset]`
    pub fn str(&mut self, src_reg: &str, base_reg: &str, offset: i64, mode: AddressingMode) -> Result<(), CpuError> {
        let base_val = self.get_register(base_reg)?;
        let src_val = self.get_register(src_reg)?;

        let target_addr = match mode {
            AddressingMode::Immediate => {
                offset as u64
            }
            AddressingMode::Offset => {
                (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64
            }
            AddressingMode::PreIndexed => {
                let addr = (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64;
                self.set_register(base_reg, addr)?;
                addr
            }
            AddressingMode::PostIndexed => {
                let addr = base_val;
                let next_base = (base_val as i64).checked_add(offset).ok_or(CpuError::InvalidAddress)? as u64;
                self.set_register(base_reg, next_base)?;
                addr
            }
        };

        self.write_mem_u64(target_addr, src_val)?;
        Ok(())
    }

    /// Emulates load multiple: `LDM <base_reg>[!], {reg_list}`
    pub fn ldm(&mut self, base_reg: &str, regs: &[&str], mode: BlockTransferMode, writeback: bool) -> Result<(), CpuError> {
        if regs.is_empty() {
            return Ok(());
        }
        let base_val = self.get_register(base_reg)?;
        let mut current_addr = base_val;

        for reg in regs {
            match mode {
                BlockTransferMode::IncrementAfter => {
                    let val = self.read_mem_u64(current_addr)?;
                    self.set_register(reg, val)?;
                    current_addr = current_addr.checked_add(8).ok_or(CpuError::InvalidAddress)?;
                }
                BlockTransferMode::IncrementBefore => {
                    current_addr = current_addr.checked_add(8).ok_or(CpuError::InvalidAddress)?;
                    let val = self.read_mem_u64(current_addr)?;
                    self.set_register(reg, val)?;
                }
                BlockTransferMode::DecrementAfter => {
                    let val = self.read_mem_u64(current_addr)?;
                    self.set_register(reg, val)?;
                    current_addr = current_addr.checked_sub(8).ok_or(CpuError::InvalidAddress)?;
                }
                BlockTransferMode::DecrementBefore => {
                    current_addr = current_addr.checked_sub(8).ok_or(CpuError::InvalidAddress)?;
                    let val = self.read_mem_u64(current_addr)?;
                    self.set_register(reg, val)?;
                }
            }
        }

        if writeback {
            self.set_register(base_reg, current_addr)?;
        }
        Ok(())
    }

    /// Emulates store multiple: `STM <base_reg>[!], {reg_list}`
    pub fn stm(&mut self, base_reg: &str, regs: &[&str], mode: BlockTransferMode, writeback: bool) -> Result<(), CpuError> {
        if regs.is_empty() {
            return Ok(());
        }
        let base_val = self.get_register(base_reg)?;
        let mut current_addr = base_val;

        for reg in regs {
            let val = self.get_register(reg)?;
            match mode {
                BlockTransferMode::IncrementAfter => {
                    self.write_mem_u64(current_addr, val)?;
                    current_addr = current_addr.checked_add(8).ok_or(CpuError::InvalidAddress)?;
                }
                BlockTransferMode::IncrementBefore => {
                    current_addr = current_addr.checked_add(8).ok_or(CpuError::InvalidAddress)?;
                    self.write_mem_u64(current_addr, val)?;
                }
                BlockTransferMode::DecrementAfter => {
                    self.write_mem_u64(current_addr, val)?;
                    current_addr = current_addr.checked_sub(8).ok_or(CpuError::InvalidAddress)?;
                }
                BlockTransferMode::DecrementBefore => {
                    current_addr = current_addr.checked_sub(8).ok_or(CpuError::InvalidAddress)?;
                    self.write_mem_u64(current_addr, val)?;
                }
            }
        }

        if writeback {
            self.set_register(base_reg, current_addr)?;
        }
        Ok(())
    }

    /// Simulates standard x86 assembly stack pushing: `push <val>`
=======
        match dest {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Simulates standard x86 assembly stack pushing: `push <val>`
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    pub fn push_stack(&mut self, val: u64) -> Result<(), CpuError> {
        if self.registers.rsp == 0 {
            return Err(CpuError::StackOverflow);
        }
        self.registers.rsp -= 8;
        let index = (self.registers.rsp / 8) as usize;
        if index < self.stack_memory.len() {
            self.stack_memory[index] = val;
            Ok(())
        } else {
            Err(CpuError::StackOverflow)
        }
    }

<<<<<<< HEAD
    /// Simulates standard stack popping: `pop`
    pub fn pop_stack(&mut self) -> Result<u64, CpuError> {
||||||| 23ef22a4a
    /// Simulates standard x86 assembly stack popping: `pop <dest>`
    pub fn pop_stack(&mut self, dest: &str) -> Result<(), CpuError> {
        if self.registers.rsp % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
=======
    /// Simulates standard x86 assembly stack popping: `pop`
    pub fn pop_stack(&mut self) -> Result<u64, CpuError> {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        let index = (self.registers.rsp / 8) as usize;
        if index >= self.stack_memory.len() {
            return Err(CpuError::StackUnderflow);
        }
        let val = self.stack_memory[index];
        self.registers.rsp += 8;
<<<<<<< HEAD
        Ok(val)
    }

    /// Simulates standard function call: `call <target>`
    pub fn call(&mut self, target: u64) -> Result<(), CpuError> {
        let next_rip = self.registers.rip + if self.thumb_state { 2 } else { 8 };
        self.push_stack(next_rip)?;
        self.registers.rip = target;
        Ok(())
    }

    /// Simulates standard function return: `ret`
    pub fn ret(&mut self) -> Result<(), CpuError> {
        let return_address = self.pop_stack()?;
        self.registers.rip = return_address;
        Ok(())
    }

    /// Updates flags for ADD instructions
    fn update_flags_add(&mut self, a: u64, b: u64, result: u64) {
        self.registers.rflags &= !(FLAG_ZF | FLAG_SF | FLAG_CF | FLAG_OF);
        if result == 0 {
            self.registers.rflags |= FLAG_ZF;
        }
        if (result as i64) < 0 {
            self.registers.rflags |= FLAG_SF;
        }
        if result < a {
            self.registers.rflags |= FLAG_CF;
        }
        let a_sign = a >> 63;
        let b_sign = b >> 63;
        let res_sign = result >> 63;
        if a_sign == b_sign && a_sign != res_sign {
            self.registers.rflags |= FLAG_OF;
        }
    }

    /// Updates flags for SUB/CMP instructions
    fn update_flags_sub(&mut self, a: u64, b: u64, result: u64) {
        self.registers.rflags &= !(FLAG_ZF | FLAG_SF | FLAG_CF | FLAG_OF);
        if result == 0 {
            self.registers.rflags |= FLAG_ZF;
        }
        if (result as i64) < 0 {
            self.registers.rflags |= FLAG_SF;
        }
        if a < b {
            self.registers.rflags |= FLAG_CF;
        }
        let a_sign = a >> 63;
        let b_sign = b >> 63;
        let res_sign = result >> 63;
        if a_sign != b_sign && b_sign == res_sign {
            self.registers.rflags |= FLAG_OF;
        }
    }

    /// Compare `a` and `b`, updating status flags
    pub fn cmp(&mut self, a: u64, b: u64) {
        let res = a.wrapping_sub(b);
        self.update_flags_sub(a, b, res);
    }

    /// Simulates conditional branching
    pub fn cond_jmp(&mut self, condition: &str, target: u64) -> bool {
        let zf = (self.registers.rflags & FLAG_ZF) != 0;
        let sf = (self.registers.rflags & FLAG_SF) != 0;
        let of = (self.registers.rflags & FLAG_OF) != 0;

        let should_jump = match condition {
            "e" | "z" => zf,
            "ne" | "nz" => !zf,
            "g" => !zf && (sf == of),
            "l" => sf != of,
            "ge" => sf == of,
            "le" => zf || (sf != of),
            _ => false,
        };

        if should_jump {
            self.registers.rip = target;
        }
        should_jump
    }

    /// Toggles ARM Thumb State (changes PC increment size / encoding)
    pub fn set_thumb_state(&mut self, enabled: bool) {
        self.thumb_state = enabled;
    }

    /// Executes Switch Case jump tables
    pub fn execute_switch(&mut self, index: usize, targets: &[u64]) -> Result<(), CpuError> {
        if index < targets.len() {
            self.registers.rip = targets[index];
            Ok(())
        } else {
            Err(CpuError::InvalidBranchTarget)
        }
    }

    /// Simulates dynamic JIT tracing/compilation cache and execution
    pub fn jit_compile_and_execute(&mut self, block_id: u64, instructions: Vec<Instruction>) -> Result<(), CpuError> {
        self.jit_cache.insert(block_id, instructions);
        self.execute_jit_block(block_id)
    }

    /// Executes JIT compiled trace blocks
    pub fn execute_jit_block(&mut self, block_id: u64) -> Result<(), CpuError> {
        if let Some(instructions) = self.jit_cache.get(&block_id).cloned() {
            for inst in instructions {
                self.execute_instruction(&inst)?;
            }
            Ok(())
        } else {
            Err(CpuError::JitCacheMiss)
        }
    }

    /// Self-modifying code write trap (invalidates current translation cache dynamically)
    pub fn write_memory(&mut self, addr: usize, val: u8) -> Result<(), CpuError> {
        if addr >= self.ram.len() {
            return Err(CpuError::SegmentationFault);
        }
        self.ram[addr] = val;
        // Invalidate JIT trace caches (hardware self-modifying code coherency protocol)
        self.jit_cache.clear();
        self.code_cache_invalidated = true;
        Ok(())
    }

    /// Lock-prefixed Exchange (XCHG) synchronization primitive
    pub fn lock_xchg(&mut self, reg_name: &str, mem_addr: usize) -> Result<u64, CpuError> {
        if mem_addr + 8 > self.ram.len() {
            return Err(CpuError::SegmentationFault);
        }
        let mut old_mem_bytes = [0u8; 8];
        old_mem_bytes.copy_from_slice(&self.ram[mem_addr..mem_addr + 8]);
        let old_mem_val = u64::from_le_bytes(old_mem_bytes);

        let reg_val = self.get_reg_val(reg_name)?;
        self.set_reg_val(reg_name, old_mem_val)?;

        let new_mem_bytes = reg_val.to_le_bytes();
        self.ram[mem_addr..mem_addr + 8].copy_from_slice(&new_mem_bytes);

        Ok(old_mem_val)
    }

    /// Lock-prefixed Compare-And-Swap (CMPXCHG) synchronization primitive
    pub fn cmpxchg(&mut self, reg_name: &str, expected: u64, desired: u64, mem_addr: usize) -> Result<bool, CpuError> {
        if mem_addr + 8 > self.ram.len() {
            return Err(CpuError::SegmentationFault);
        }
        let mut mem_bytes = [0u8; 8];
        mem_bytes.copy_from_slice(&self.ram[mem_addr..mem_addr + 8]);
        let current_val = u64::from_le_bytes(mem_bytes);

        if current_val == expected {
            let desired_bytes = desired.to_le_bytes();
            self.ram[mem_addr..mem_addr + 8].copy_from_slice(&desired_bytes);
            self.set_reg_val(reg_name, expected)?;
            self.registers.rflags |= FLAG_ZF;
            Ok(true)
        } else {
            self.set_reg_val(reg_name, current_val)?;
            self.registers.rflags &= !FLAG_ZF;
            Ok(false)
        }
    }

    /// Register interrupt handler callback in vector table
    pub fn register_interrupt_handler(&mut self, vector: u8, handler: fn(&mut SovereignVirtualCPU)) {
        if (vector as usize) < self.isr_table.len() {
            self.isr_table[vector as usize] = Some(handler);
        }
    }

    /// Triggers software interrupt/service traps (promotes rings during execution)
    pub fn trigger_interrupt(&mut self, vector: u8) -> Result<(), CpuError> {
        if (vector as usize) >= self.isr_table.len() {
            return Err(CpuError::InvalidInterruptVector);
        }
        if let Some(handler) = self.isr_table[vector as usize] {
            let old_ring = self.ring;
            let old_rip = self.registers.rip;
            self.push_stack(old_rip)?;
            self.push_stack(old_ring as u64)?;

            // Interrupt handlers execute in Ring 0 (Privilege escalation)
            self.ring = CpuRing::Ring0;

            handler(self);

            let popped_ring = self.pop_stack()?;
            let popped_rip = self.pop_stack()?;
            self.ring = match popped_ring {
                0 => CpuRing::Ring0,
                1 => CpuRing::Ring1,
                2 => CpuRing::Ring2,
                3 => CpuRing::Ring3,
                _ => return Err(CpuError::InvalidPrivilegeLevel),
            };
            self.registers.rip = popped_rip;
            Ok(())
        } else {
            Err(CpuError::InterruptHandlerNotFound)
        }
    }

    /// Evaluates and runs single virtual instruction
    pub fn execute_instruction(&mut self, inst: &Instruction) -> Result<(), CpuError> {
        match inst {
            Instruction::MovRegReg { dest, src } => {
                let val = self.get_reg_val(src)?;
                self.set_reg_val(dest, val)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::MovRegVal { dest, val } => {
                self.set_reg_val(dest, *val)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Push { reg } => {
                let val = self.get_reg_val(reg)?;
                self.push_stack(val)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Pop { reg } => {
                let val = self.pop_stack()?;
                self.set_reg_val(reg, val)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Add { dest, src } => {
                let a = self.get_reg_val(dest)?;
                let b = self.get_reg_val(src)?;
                let res = a.wrapping_add(b);
                self.set_reg_val(dest, res)?;
                self.update_flags_add(a, b, res);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::AddVal { dest, val } => {
                let a = self.get_reg_val(dest)?;
                let b = *val;
                let res = a.wrapping_add(b);
                self.set_reg_val(dest, res)?;
                self.update_flags_add(a, b, res);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Sub { dest, src } => {
                let a = self.get_reg_val(dest)?;
                let b = self.get_reg_val(src)?;
                let res = a.wrapping_sub(b);
                self.set_reg_val(dest, res)?;
                self.update_flags_sub(a, b, res);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::SubVal { dest, val } => {
                let a = self.get_reg_val(dest)?;
                let b = *val;
                let res = a.wrapping_sub(b);
                self.set_reg_val(dest, res)?;
                self.update_flags_sub(a, b, res);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Mul { src } => {
                let a = self.registers.rax;
                let b = self.get_reg_val(src)?;
                let res = a.wrapping_mul(b);
                self.registers.rax = res;
                self.registers.rflags &= !FLAG_ZF;
                if res == 0 {
                    self.registers.rflags |= FLAG_ZF;
                }
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Div { src } => {
                let a = self.registers.rax;
                let b = self.get_reg_val(src)?;
                if b == 0 {
                    return Err(CpuError::DivisionByZero);
                }
                let res = a / b;
                self.registers.rax = res;
                self.registers.rflags &= !FLAG_ZF;
                if res == 0 {
                    self.registers.rflags |= FLAG_ZF;
                }
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::And { dest, src } => {
                let a = self.get_reg_val(dest)?;
                let b = self.get_reg_val(src)?;
                let res = a & b;
                self.set_reg_val(dest, res)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Or { dest, src } => {
                let a = self.get_reg_val(dest)?;
                let b = self.get_reg_val(src)?;
                let res = a | b;
                self.set_reg_val(dest, res)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Xor { dest, src } => {
                let a = self.get_reg_val(dest)?;
                let b = self.get_reg_val(src)?;
                let res = a ^ b;
                self.set_reg_val(dest, res)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Shl { dest, shift } => {
                let a = self.get_reg_val(dest)?;
                let res = a.wrapping_shl(*shift as u32);
                self.set_reg_val(dest, res)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Shr { dest, shift } => {
                let a = self.get_reg_val(dest)?;
                let res = a.wrapping_shr(*shift as u32);
                self.set_reg_val(dest, res)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Cmp { reg, val } => {
                let a = self.get_reg_val(reg)?;
                self.cmp(a, *val);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::CmpReg { reg1, reg2 } => {
                let a = self.get_reg_val(reg1)?;
                let b = self.get_reg_val(reg2)?;
                self.cmp(a, b);
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Jmp { target } => {
                self.registers.rip = *target;
            }
            Instruction::Je { target } => {
                if !self.cond_jmp("e", *target) {
                    self.registers.rip += if self.thumb_state { 2 } else { 8 };
                }
            }
            Instruction::Jne { target } => {
                if !self.cond_jmp("ne", *target) {
                    self.registers.rip += if self.thumb_state { 2 } else { 8 };
                }
            }
            Instruction::Jg { target } => {
                if !self.cond_jmp("g", *target) {
                    self.registers.rip += if self.thumb_state { 2 } else { 8 };
                }
            }
            Instruction::Jl { target } => {
                if !self.cond_jmp("l", *target) {
                    self.registers.rip += if self.thumb_state { 2 } else { 8 };
                }
            }
            Instruction::Call { target } => {
                self.call(*target)?;
            }
            Instruction::Ret => {
                self.ret()?;
            }
            Instruction::Syscall => {
                self.trigger_interrupt(0x80)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Int { vector } => {
                self.trigger_interrupt(*vector)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::SwitchCase { index_reg, targets } => {
                let index = self.get_reg_val(index_reg)? as usize;
                self.execute_switch(index, targets)?;
            }
            Instruction::LockXchg { reg, mem_addr } => {
                self.lock_xchg(reg, *mem_addr)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::Cmpxchg { reg, expected, desired, mem_addr } => {
                self.cmpxchg(reg, *expected, *desired, *mem_addr)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::TestAndSet { mem_addr } => {
                if *mem_addr >= self.ram.len() {
                    return Err(CpuError::SegmentationFault);
                }
                let old = self.ram[*mem_addr];
                self.ram[*mem_addr] = 1;
                self.registers.rax = old as u64;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
            Instruction::WriteMem { addr, val } => {
                self.write_memory(*addr, *val)?;
                self.registers.rip += if self.thumb_state { 2 } else { 8 };
            }
        }
        Ok(())
||||||| 23ef22a4a
        self.set_register(dest, val)?;
        Ok(())
    }

    /// Pushes multiple registers onto the stack.
    /// Alignment of Stack Pointer (rsp) is validated.
    pub fn push_multiple(&mut self, regs: &[&str]) -> Result<(), CpuError> {
        if self.registers.rsp % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        for &reg in regs.iter().rev() {
            let val = self.get_register(reg)?;
            if self.registers.rsp < 8 {
                return Err(CpuError::StackOverflow);
            }
            self.registers.rsp -= 8;
            let index = (self.registers.rsp / 8) as usize;
            if index < self.stack_memory.len() {
                self.stack_memory[index] = val;
            } else {
                return Err(CpuError::StackOverflow);
            }
        }
        Ok(())
    }

    /// Pops multiple registers from the stack.
    /// Alignment of Stack Pointer (rsp) is validated.
    pub fn pop_multiple(&mut self, regs: &[&str]) -> Result<(), CpuError> {
        if self.registers.rsp % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        for &reg in regs {
            let index = (self.registers.rsp / 8) as usize;
            if index >= self.stack_memory.len() {
                return Err(CpuError::StackOverflow);
            }
            let val = self.stack_memory[index];
            self.registers.rsp += 8;
            self.set_register(reg, val)?;
        }
        Ok(())
=======
        Ok(val)
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }

    /// Transitions between x86 Execution Modes (Alters PE bits dynamically)
    pub fn transition_mode(&mut self, target: CpuMode) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation); // Only Ring 0 can alter CPU modes!
        }

        match target {
            CpuMode::RealMode => {
                self.registers.cr0 &= !1; // Clear PE bit 0
                self.mode = CpuMode::RealMode;
            }
            CpuMode::ProtectedMode => {
                self.registers.cr0 |= 1; // Set PE bit 0
                self.mode = CpuMode::ProtectedMode;
            }
            CpuMode::LongMode => {
                if self.registers.cr3 == 0 {
                    return Err(CpuError::PagingDisabled); // LongMode requires CR3 paging base!
                }
                self.registers.cr0 |= 1; // PE bit 0 must be active
                self.mode = CpuMode::LongMode;
            }
        }
        Ok(())
    }

    /// Transitions the active thread across CPU Rings (Privilege Separation)
    /// - Ring 0 can transition down to Ring 1, 2, or 3.
    /// - Restricted Rings (Ring 3) CANNOT escalate back to Ring 0 directly (must trigger software trap).
    pub fn transition_ring(&mut self, target: CpuRing) -> Result<(), CpuError> {
        if target > self.ring {
            // Lowering privileges is always allowed
            self.ring = target;
            Ok(())
        } else if target == self.ring {
            Ok(())
        } else {
            // Escalating privileges is blocked unless in Ring 0
            if self.ring == CpuRing::Ring0 {
                self.ring = target;
                Ok(())
            } else {
                Err(CpuError::PrivilegeViolation) // Blocked!
            }
        }
    }
<<<<<<< HEAD
||||||| 23ef22a4a

    /// Initializes a simulated physical thread object (KeInitThread Windows equivalent)
    pub fn ke_init_thread(&mut self, thread_id: u64, parent_id: u64) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        // Check if thread already exists
        if self.threads.iter().any(|t| t.id == thread_id) {
            return Ok(());
        }
        let thread = SovereignThread {
            id: thread_id,
            parent_id,
            state: ThreadState::Running,
            suspend_count: 0,
            alertable: false,
            kernel_apc_disable: false,
            apc_queue: Vec::new(),
        };
        self.threads.push(thread);
        Ok(())
    }

    /// Suspends a thread (KeSuspendThread Windows equivalent), incrementing suspend_count
    pub fn ke_suspend_thread(&mut self, thread_id: u64) -> Result<u32, CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        // Cannot suspend if processor runs at DISPATCH_LEVEL or above
        if self.kprcb.current_irql >= Irql::DispatchLevel {
            return Err(CpuError::IrqlViolation);
        }
        let pos = self.threads.iter().position(|t| t.id == thread_id).ok_or(CpuError::InvalidAddress)?;
        let thread = &mut self.threads[pos];
        thread.suspend_count += 1;
        thread.state = ThreadState::Suspended;
        Ok(thread.suspend_count)
    }

    /// Resumes a thread (KeResumeThread Windows equivalent), decrementing suspend_count
    pub fn ke_resume_thread(&mut self, thread_id: u64) -> Result<u32, CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        if self.kprcb.current_irql >= Irql::DispatchLevel {
            return Err(CpuError::IrqlViolation);
        }
        let pos = self.threads.iter().position(|t| t.id == thread_id).ok_or(CpuError::InvalidAddress)?;
        let thread = &mut self.threads[pos];
        if thread.suspend_count > 0 {
            thread.suspend_count -= 1;
        }
        if thread.suspend_count == 0 {
            thread.state = ThreadState::Running;
        }
        Ok(thread.suspend_count)
    }

    /// Raises CPU Interrupt Request Level (KeRaiseIrql equivalent)
    /// Validates environment (e.g. Ring 3 cannot modify hardware IRQLs, preventing rootkits)
    pub fn ke_raise_irql(&mut self, target_irql: Irql) -> Result<Irql, CpuError> {
        if self.ring == CpuRing::Ring3 {
            return Err(CpuError::PrivilegeViolation); // Block rootkits in user context
        }
        if target_irql < self.kprcb.current_irql {
            return Err(CpuError::IrqlViolation); // Cannot raise to lower level!
        }
        let old = self.kprcb.current_irql;
        self.kprcb.current_irql = target_irql;
        Ok(old)
    }

    /// Lowers CPU Interrupt Request Level (KeLowerIrql equivalent)
    pub fn ke_lower_irql(&mut self, target_irql: Irql) -> Result<(), CpuError> {
        if self.ring == CpuRing::Ring3 {
            return Err(CpuError::PrivilegeViolation);
        }
        if target_irql > self.kprcb.current_irql {
            return Err(CpuError::IrqlViolation); // Lowering to higher IRQL is invalid!
        }
        self.kprcb.current_irql = target_irql;
        Ok(())
    }

    /// Queues an APC onto a specified thread's queue (KiScheduleApc equivalent)
    pub fn ki_schedule_apc(&mut self, thread_id: u64, apc: SovereignApc) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        let pos = self.threads.iter().position(|t| t.id == thread_id).ok_or(CpuError::InvalidAddress)?;
        self.threads[pos].apc_queue.push(apc);
        Ok(())
    }

    /// Delivers pending APCs on the active thread context (KiDeliverApc / SchedulerApc equivalent)
    /// Respects thread states, alertable environments, and current CPU IRQL (must be < APC_LEVEL)
    pub fn scheduler_apc(&mut self) -> Result<u32, CpuError> {
        if self.kprcb.current_irql >= Irql::ApcLevel {
            return Ok(0); // APC delivery is disabled when IRQL >= APC_LEVEL
        }

        let active_id = self.kprcb.active_thread_id;
        let pos = self.threads.iter().position(|t| t.id == active_id).ok_or(CpuError::InvalidAddress)?;

        if self.threads[pos].state == ThreadState::Suspended {
            return Err(CpuError::ThreadSuspended);
        }

        // Elevate IRQL to APC_LEVEL during delivery
        let old_irql = self.ke_raise_irql(Irql::ApcLevel)?;

        let mut delivered = 0;
        let mut remaining = Vec::new();
        // Take APC queue to execute
        let queue = core::mem::take(&mut self.threads[pos].apc_queue);

        for apc in queue {
            let mut deliver = false;
            match apc.apc_type {
                ApcType::SpecialKernel => {
                    // Special Kernel APC is always delivered regardless of kernel_apc_disable
                    deliver = true;
                }
                ApcType::NormalKernel => {
                    if !self.threads[pos].kernel_apc_disable {
                        deliver = true;
                    }
                }
                ApcType::User => {
                    // User APCs delivered only if thread is explicitly alertable
                    if self.threads[pos].alertable {
                        deliver = true;
                    }
                }
            }

            if deliver {
                // Call kernel routine
                (apc.kernel_routine)(self, apc.system_argument1)?;
                // Call normal routine if present
                if let Some(normal) = apc.normal_routine {
                    (normal)(self, apc.normal_context, apc.system_argument2)?;
                }
                delivered += 1;
            } else {
                // Keep in queue or run rundown routine on process shutdown/exit
                if self.threads[pos].state == ThreadState::Terminated {
                    if let Some(rundown) = apc.rundown_routine {
                        (rundown)(self, apc.system_argument1)?;
                    }
                } else {
                    remaining.push(apc);
                }
            }
        }

        self.threads[pos].apc_queue = remaining;
        self.ke_lower_irql(old_irql)?;
        Ok(delivered)
    }

    /// Queues a Deferred Procedure Call (KeInsertQueueDpc equivalent)
    pub fn ke_insert_queue_dpc(&mut self, dpc: SovereignDpc) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        // Insert into queue and sort by importance (priority queuing)
        self.kprcb.dpc_queue.push(dpc);
        self.kprcb.dpc_queue.sort_by(|a, b| b.importance.cmp(&a.importance));
        Ok(())
    }

    /// Executes queued Deferred Procedure Calls at DISPATCH_LEVEL (KiRetireDpcList equivalent)
    pub fn ki_retire_dpc_list(&mut self) -> Result<u32, CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        if self.kprcb.current_irql < Irql::DispatchLevel {
            // Must raise IRQL to DISPATCH_LEVEL to execute DPCs
            let old_irql = self.ke_raise_irql(Irql::DispatchLevel)?;
            let count = self.execute_dpcs_internal()?;
            self.ke_lower_irql(old_irql)?;
            Ok(count)
        } else {
            self.execute_dpcs_internal()
        }
    }

    fn execute_dpcs_internal(&mut self) -> Result<u32, CpuError> {
        let queue = core::mem::take(&mut self.kprcb.dpc_queue);
        let mut count = 0;
        for dpc in queue {
            (dpc.deferred_routine)(
                self,
                dpc.deferred_context,
                dpc.system_argument1,
                dpc.system_argument2,
            )?;
            count += 1;
        }
        Ok(count)
    }

    /// Queues a system Work Item to the worker queue (ExQueueWorkItem equivalent)
    pub fn ex_queue_work_item(&mut self, mut item: WorkItem) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        item.is_queued = true;
        self.kprcb.work_item_queue.push(item);
        Ok(())
    }

    /// Process queued system Work Items inside exp_worker_thread (ExWorkerThread equivalent)
    /// Runs at PASSIVE_LEVEL inside system thread contexts
    pub fn exp_worker_thread(&mut self) -> Result<u32, CpuError> {
        if self.kprcb.current_irql != Irql::PassiveLevel {
            return Err(CpuError::IrqlViolation); // Worker threads must execute at PASSIVE_LEVEL
        }
        let queue = core::mem::take(&mut self.kprcb.work_item_queue);
        let mut count = 0;
        for item in queue {
            (item.worker_routine)(self, item.parameter)?;
            count += 1;
        }
        Ok(count)
    }

    /// Emulates CISC string copy operation: `REP MOVSB`
    /// Copies `rcx` bytes of memory from source `rsi` to destination `rdi`
    pub fn rep_movsb(&mut self) -> Result<(), CpuError> {
        let count = self.registers.rcx;
        let mut src = self.registers.rsi;
        let mut dest = self.registers.rdi;

        if count == 0 {
            return Ok(());
        }

        self.check_memory_privilege(src, count)?;
        self.check_memory_privilege(dest, count)?;

        // Perform safe non-overlapping or overlapping copies
        for _ in 0..count {
            let val = self.memory[src as usize];
            self.memory[dest as usize] = val;
            src = src.checked_add(1).ok_or(CpuError::InvalidAddress)?;
            dest = dest.checked_add(1).ok_or(CpuError::InvalidAddress)?;
        }

        self.registers.rcx = 0;
        self.registers.rsi = src;
        self.registers.rdi = dest;
        self.registers.zf = true; // Complete execution sets ZF to true
        Ok(())
    }

    /// Emulates CISC memory fill operation: `REP STOSB`
    /// Fills `rcx` bytes starting at `rdi` with the lower byte of `rax` (AL)
    pub fn rep_stosb(&mut self) -> Result<(), CpuError> {
        let count = self.registers.rcx;
        let mut dest = self.registers.rdi;
        let fill_byte = (self.registers.rax & 0xFF) as u8;

        if count == 0 {
            return Ok(());
        }

        self.check_memory_privilege(dest, count)?;

        for _ in 0..count {
            self.memory[dest as usize] = fill_byte;
            dest = dest.checked_add(1).ok_or(CpuError::InvalidAddress)?;
        }

        self.registers.rcx = 0;
        self.registers.rdi = dest;
        self.registers.zf = true;
        Ok(())
    }

    // =====================================
    // 2. Bitwise Shift and Rotation Methods
    // =====================================

    /// Emulates logical shift left: `SHL <dest_reg>, <count>`
    pub fn shl(&mut self, reg_name: &str, count: u32) -> Result<(), CpuError> {
        let val = self.get_register(reg_name)?;
        if count == 0 {
            return Ok(());
        }
        let count_mod = count % 64;
        let carry = if count_mod > 0 {
            ((val >> (64 - count_mod)) & 1) == 1
        } else {
            false
        };
        let res = val << count_mod;
        self.set_register(reg_name, res)?;

        // Update Status Flags
        self.registers.cf = carry;
        self.registers.zf = res == 0;
        self.registers.sf = (res >> 63) == 1;
        Ok(())
    }

    /// Emulates logical shift right: `SHR <dest_reg>, <count>`
    pub fn shr(&mut self, reg_name: &str, count: u32) -> Result<(), CpuError> {
        let val = self.get_register(reg_name)?;
        if count == 0 {
            return Ok(());
        }
        let count_mod = count % 64;
        let carry = if count_mod > 0 {
            ((val >> (count_mod - 1)) & 1) == 1
        } else {
            false
        };
        let res = val >> count_mod;
        self.set_register(reg_name, res)?;

        // Update Status Flags
        self.registers.cf = carry;
        self.registers.zf = res == 0;
        self.registers.sf = (res >> 63) == 1;
        Ok(())
    }

    /// Emulates arithmetic shift right (keeps sign-bit): `SAR <dest_reg>, <count>`
    pub fn sar(&mut self, reg_name: &str, count: u32) -> Result<(), CpuError> {
        let val = self.get_register(reg_name)? as i64;
        if count == 0 {
            return Ok(());
        }
        let count_mod = count % 64;
        let carry = if count_mod > 0 {
            ((val >> (count_mod - 1)) & 1) == 1
        } else {
            false
        };
        let res = val >> count_mod;
        self.set_register(reg_name, res as u64)?;

        // Update Status Flags
        self.registers.cf = carry;
        self.registers.zf = res == 0;
        self.registers.sf = res < 0;
        Ok(())
    }

    /// Emulates rotate left: `ROL <dest_reg>, <count>`
    pub fn rol(&mut self, reg_name: &str, count: u32) -> Result<(), CpuError> {
        let val = self.get_register(reg_name)?;
        if count == 0 {
            return Ok(());
        }
        let count_mod = count % 64;
        let res = val.rotate_left(count_mod);
        self.set_register(reg_name, res)?;

        self.registers.cf = (res & 1) == 1;
        self.registers.zf = res == 0;
        self.registers.sf = (res >> 63) == 1;
        Ok(())
    }

    /// Emulates rotate right: `ROR <dest_reg>, <count>`
    pub fn ror(&mut self, reg_name: &str, count: u32) -> Result<(), CpuError> {
        let val = self.get_register(reg_name)?;
        if count == 0 {
            return Ok(());
        }
        let count_mod = count % 64;
        let res = val.rotate_right(count_mod);
        self.set_register(reg_name, res)?;

        self.registers.cf = (res >> 63) == 1;
        self.registers.zf = res == 0;
        self.registers.sf = (res >> 63) == 1;
        Ok(())
    }

    // ===================================
    // 3. Hardware Memory Fence Emulations
    // ===================================

    /// x86: Serializes all load and store operations (MFENCE equivalent)
    pub fn mfence(&self) {
        core::sync::atomic::fence(Ordering::SeqCst);
    }

    /// x86: Serializes all store operations (SFENCE equivalent)
    pub fn sfence(&self) {
        core::sync::atomic::fence(Ordering::Release);
    }

    /// x86: Serializes all load operations (LFENCE equivalent)
    pub fn lfence(&self) {
        core::sync::atomic::fence(Ordering::Acquire);
    }

    /// ARM: Data Memory Barrier (DMB equivalent)
    pub fn dmb(&self) {
        core::sync::atomic::fence(Ordering::SeqCst);
    }

    /// ARM: Data Synchronization Barrier (DSB equivalent)
    pub fn dsb(&self) {
        core::sync::atomic::compiler_fence(Ordering::SeqCst);
    }
=======

    /// Write to Model Specific Register (rdmsr/wrmsr emulation)
    pub fn write_msr(&mut self, reg: u32, value: u64) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        match reg {
            0xC0000080 => self.msrs.efer = value,
            0xC0000081 => self.msrs.star = value,
            0xC0000082 => self.msrs.lstar = value,
            0xC0000084 => self.msrs.sfmask = value,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// Emulates the fast 64-bit `SYSCALL` instruction used by Linux and BSD for low-overhead user space transitions
    pub fn execute_syscall(&mut self) -> Result<(), CpuError> {
        if self.mode != CpuMode::LongMode {
            return Err(CpuError::InvalidInstruction);
        }

        // 1. Save current RIP to RCX, and save RFLAGS to R11 (simulated)
        self.registers.rcx = self.registers.rip;

        // 2. Load syscall target RIP from LSTAR MSR
        self.registers.rip = self.msrs.lstar;

        // 3. Elevate privilege to Ring 0
        self.ring = CpuRing::Ring0;

        Ok(())
    }

    /// Emulates x86 software trap / CPU exception interrupt handling (e.g. GPF, Page Fault, Soft Traps)
    /// Automatically pushes RIP, CS, and registers onto the kernel stack, and escalates ring to Ring 0.
    pub fn trigger_interrupt_trap(&mut self, vector: u8, handler_rip: u64) -> Result<(), CpuError> {
        // Save current instruction pointer and stack context
        let old_rip = self.registers.rip;
        let old_rsp = self.registers.rsp;
        let old_ring = self.ring as u64;

        // Escapes to Ring 0 first (privilege elevation)
        self.ring = CpuRing::Ring0;

        // Push execution context onto kernel stack frame (standard hardware frame)
        self.push_stack(old_rsp)?;
        self.push_stack(old_rip)?;
        self.push_stack(old_ring)?;
        self.push_stack(vector as u64)?;

        // Jump to exception service handler address
        self.registers.rip = handler_rip;

        Ok(())
    }

    /// Lazily handles floating-point/vector register context switches (Linux/BSD style).
    /// If TS (Task Switched) bit in CR0 is set, accessing FP registers triggers a Device Not Available exception.
    /// The kernel then clears TS, saves/restores the FP area, and proceeds.
    pub fn handle_lazy_fp_state_restore(
        &mut self,
        is_fp_instruction: bool,
    ) -> Result<(), CpuError> {
        let ts_bit_active = (self.registers.cr0 & (1 << 3)) != 0;
        if is_fp_instruction && ts_bit_active {
            // Trigger Device Not Available (#NM exception trap)
            // Clear TS bit in CR0
            self.registers.cr0 &= !(1 << 3);

            // Simulates copying FP state from memory (XSAVE)
            if self.fp_dirty {
                // Restore state
                self.fp_dirty = false;
            }
            Ok(())
        } else if is_fp_instruction {
            Ok(())
        } else {
            Err(CpuError::InvalidInstruction)
        }
    }
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
}

impl Default for SovereignVirtualCPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_cpu_instructions_and_data_movement() {
        let mut cpu = SovereignVirtualCPU::new();

        // 1. Move value 120 directly to register RAX
        cpu.mov_val_to_reg("rax", 120).unwrap();
        assert_eq!(cpu.registers.rax, 120);

        // 2. Push value to stack
        cpu.push_stack(999).unwrap();
        assert_eq!(cpu.registers.rsp, 1016);
    }

    #[test]
    fn test_virtual_cpu_execution_mode_switching() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.mode, CpuMode::RealMode);
        assert_eq!(cpu.registers.cr0 & 1, 0); // PE bit 0 is inactive

        // Transition to ProtectedMode (32-bit segment)
        cpu.transition_mode(CpuMode::ProtectedMode).unwrap();
        assert_eq!(cpu.mode, CpuMode::ProtectedMode);
        assert_eq!(cpu.registers.cr0 & 1, 1); // PE bit 1 is set successfully!

        // Attempting LongMode without paging CR3 should fail
        assert_eq!(
            cpu.transition_mode(CpuMode::LongMode),
            Err(CpuError::PagingDisabled)
        );

        // Set CR3 page directory pointer base address
        cpu.registers.cr3 = 0x100000;
        assert!(cpu.transition_mode(CpuMode::LongMode).is_ok());
    }

    #[test]
    fn test_virtual_cpu_ring_privilege_isolation() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.ring, CpuRing::Ring0);

        // Transition down to Userland Ring 3 (permitted)
        cpu.transition_ring(CpuRing::Ring3).unwrap();
        assert_eq!(cpu.ring, CpuRing::Ring3);

        // Attempting to escalate back to Ring 0 from Userland Ring 3 should fail (PrivilegeViolation!)
        assert_eq!(
            cpu.transition_ring(CpuRing::Ring0),
            Err(CpuError::PrivilegeViolation)
        );

        // Mode changes from Ring 3 should also fail
        assert_eq!(
            cpu.transition_mode(CpuMode::RealMode),
            Err(CpuError::PrivilegeViolation)
        );
    }

    #[test]
<<<<<<< HEAD
    fn test_function_invocation_call_ret() {
||||||| 23ef22a4a
    fn test_ldr_str_addressing_modes() {
=======
    fn test_msr_and_fast_syscall() {
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
        let mut cpu = SovereignVirtualCPU::new();
<<<<<<< HEAD
        cpu.registers.rip = 100;
        cpu.call(200).unwrap();
        assert_eq!(cpu.registers.rip, 200);
        assert_eq!(cpu.registers.rsp, 1016); // pushed return address onto stack
||||||| 23ef22a4a
=======
        cpu.registers.cr3 = 0x200000;
        cpu.transition_mode(CpuMode::LongMode).unwrap();
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

<<<<<<< HEAD
        cpu.ret().unwrap();
        assert_eq!(cpu.registers.rip, 108); // standard rip + 8 returned address
        assert_eq!(cpu.registers.rsp, 1024);
    }

    #[test]
    fn test_arithmetic_flags_and_operations() {
        let mut cpu = SovereignVirtualCPU::new();

        // Let's execute 0 - 1 subtraction to set Carry and Sign flags
        cpu.mov_val_to_reg("rax", 0).unwrap();
        cpu.mov_val_to_reg("rbx", 1).unwrap();
        cpu.execute_instruction(&Instruction::Sub { dest: "rax".to_string(), src: "rbx".to_string() }).unwrap();

        assert_eq!(cpu.registers.rax, u64::MAX);
        assert_ne!(cpu.registers.rflags & FLAG_SF, 0); // Sign flag set
        assert_ne!(cpu.registers.rflags & FLAG_CF, 0); // Carry flag set

        // Test addition resulting in zero (sets Zero Flag)
        cpu.mov_val_to_reg("rax", u64::MAX).unwrap();
        cpu.mov_val_to_reg("rbx", 1).unwrap();
        cpu.execute_instruction(&Instruction::Add { dest: "rax".to_string(), src: "rbx".to_string() }).unwrap();

        assert_eq!(cpu.registers.rax, 0);
        assert_ne!(cpu.registers.rflags & FLAG_ZF, 0); // Zero flag set
        assert_eq!(cpu.registers.rflags & FLAG_SF, 0); // Sign flag clear
    }

    #[test]
    fn test_branching_and_conditional_jumps() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.rip = 10;

        // Compare 5 with 5, setting ZF
        cpu.mov_val_to_reg("rax", 5).unwrap();
        cpu.execute_instruction(&Instruction::Cmp { reg: "rax".to_string(), val: 5 }).unwrap();
        assert_ne!(cpu.registers.rflags & FLAG_ZF, 0);

        // Conditional Jump Equal should be taken
        cpu.execute_instruction(&Instruction::Je { target: 300 }).unwrap();
        assert_eq!(cpu.registers.rip, 300);

        // Compare 10 with 5 (10 > 5), ZF is clear, and SF == OF (both 0)
        cpu.mov_val_to_reg("rax", 10).unwrap();
        cpu.execute_instruction(&Instruction::Cmp { reg: "rax".to_string(), val: 5 }).unwrap();

        // Conditional Jump Greater should be taken
        cpu.execute_instruction(&Instruction::Jg { target: 400 }).unwrap();
        assert_eq!(cpu.registers.rip, 400);
    }

    #[test]
    fn test_thumb_state_pc_increments() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.rip = 10;

        // Non-Thumb Mode (adds 8 to RIP)
        cpu.execute_instruction(&Instruction::MovRegVal { dest: "rax".to_string(), val: 42 }).unwrap();
        assert_eq!(cpu.registers.rip, 18);

        // Enable Thumb Mode (adds 2 to RIP)
        cpu.set_thumb_state(true);
        cpu.execute_instruction(&Instruction::MovRegVal { dest: "rax".to_string(), val: 99 }).unwrap();
        assert_eq!(cpu.registers.rip, 20);
    }

    #[test]
    fn test_switch_case_jump_table() {
        let mut cpu = SovereignVirtualCPU::new();
        let targets = vec![100, 200, 300];

        // Switch to case 1
        cpu.mov_val_to_reg("rcx", 1).unwrap();
        cpu.execute_instruction(&Instruction::SwitchCase { index_reg: "rcx".to_string(), targets: targets.clone() }).unwrap();
        assert_eq!(cpu.registers.rip, 200);

        // Invalid branch index should return an error
        cpu.mov_val_to_reg("rcx", 5).unwrap();
        assert_eq!(
            cpu.execute_instruction(&Instruction::SwitchCase { index_reg: "rcx".to_string(), targets }),
            Err(CpuError::InvalidBranchTarget)
        );
    }

    #[test]
    fn test_just_in_time_compilation() {
        let mut cpu = SovereignVirtualCPU::new();
        let trace = vec![
            Instruction::MovRegVal { dest: "rax".to_string(), val: 10 },
            Instruction::AddVal { dest: "rax".to_string(), val: 25 },
        ];

        // Compile trace block 42 and execute it
        cpu.jit_compile_and_execute(42, trace).unwrap();
        assert_eq!(cpu.registers.rax, 35);

        // Execute compiled trace directly via JIT cache
        cpu.registers.rax = 0;
        cpu.execute_jit_block(42).unwrap();
        assert_eq!(cpu.registers.rax, 35);
    }

    #[test]
    fn test_self_modifying_code() {
        let mut cpu = SovereignVirtualCPU::new();
        let trace = vec![
            Instruction::MovRegVal { dest: "rax".to_string(), val: 10 },
        ];

        cpu.jit_compile_and_execute(101, trace).unwrap();
        assert_eq!(cpu.registers.rax, 10);
        assert_eq!(cpu.code_cache_invalidated, false);

        // Write to memory to trigger dynamic code cache invalidation
        cpu.write_memory(128, 0xFF).unwrap();
        assert_eq!(cpu.code_cache_invalidated, true);

        // Cache miss since JIT traces were invalidated
        assert_eq!(cpu.execute_jit_block(101), Err(CpuError::JitCacheMiss));
    }

    #[test]
    fn test_synchronization_primitives() {
        let mut cpu = SovereignVirtualCPU::new();

        // Initialize mutex memory at address 16 to unlocked (0)
        let mem_addr = 16;
        let bytes_zero = 0u64.to_le_bytes();
        cpu.ram[mem_addr..mem_addr + 8].copy_from_slice(&bytes_zero);

        // Thread attempts to acquire spinlock via CMPXCHG: CAS expected = 0, desired = 1
        cpu.mov_val_to_reg("rax", 99).unwrap();
        let cas_success = cpu.cmpxchg("rax", 0, 1, mem_addr).unwrap();
        assert_eq!(cas_success, true);
        assert_ne!(cpu.registers.rflags & FLAG_ZF, 0); // ZF set on success

        // Read RAM to ensure lock was written
        let mut val_bytes = [0u8; 8];
        val_bytes.copy_from_slice(&cpu.ram[mem_addr..mem_addr + 8]);
        assert_eq!(u64::from_le_bytes(val_bytes), 1);

        // Thread attempts to acquire again: CAS expected = 0, desired = 1 (should fail since lock is 1)
        let cas_fail = cpu.cmpxchg("rax", 0, 1, mem_addr).unwrap();
        assert_eq!(cas_fail, false);
        assert_eq!(cpu.registers.rflags & FLAG_ZF, 0); // ZF clear on failure
        assert_eq!(cpu.registers.rax, 1); // rax loaded with current value 1
    }

    #[test]
    fn test_interrupt_and_context_privilege_escalation() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.transition_ring(CpuRing::Ring3).unwrap(); // set userland privilege
        assert_eq!(cpu.ring, CpuRing::Ring3);

        // Register custom software interrupt handler for Syscall/Int vector 0x20
        fn my_isr(c: &mut SovereignVirtualCPU) {
            assert_eq!(c.ring, CpuRing::Ring0); // Escalated to Kernel privilege
            c.registers.rbx = 12345;
        }
        cpu.register_interrupt_handler(0x20, my_isr);

        // Trigger interrupt
        cpu.registers.rip = 500;
        cpu.trigger_interrupt(0x20).unwrap();

        // Ensure userland context was restored afterwards and handler ran successfully
        assert_eq!(cpu.ring, CpuRing::Ring3);
        assert_eq!(cpu.registers.rbx, 12345);
        assert_eq!(cpu.registers.rip, 500);
||||||| 23ef22a4a
        // 1. STR Immediate / register write
        cpu.set_register("rax", 0xDEADBEEF).unwrap();
        cpu.set_register("rbx", 100).unwrap();

        // Offset Mode
        cpu.str("rax", "rbx", 8, AddressingMode::Offset).unwrap();
        assert_eq!(cpu.read_mem_u64(108).unwrap(), 0xDEADBEEF);
        assert_eq!(cpu.get_register("rbx").unwrap(), 100);

        // PreIndexed Mode
        cpu.set_register("rcx", 0xCAFEBABE).unwrap();
        cpu.str("rcx", "rbx", 16, AddressingMode::PreIndexed).unwrap();
        assert_eq!(cpu.read_mem_u64(116).unwrap(), 0xCAFEBABE);
        assert_eq!(cpu.get_register("rbx").unwrap(), 116);

        // PostIndexed Mode
        cpu.set_register("rdx", 0xBEEFFEED).unwrap();
        cpu.str("rdx", "rbx", 24, AddressingMode::PostIndexed).unwrap();
        assert_eq!(cpu.read_mem_u64(116).unwrap(), 0xBEEFFEED);
        assert_eq!(cpu.get_register("rbx").unwrap(), 140);

        // LDR Offset Mode
        cpu.ldr("r8", "rbx", -24, AddressingMode::Offset).unwrap();
        assert_eq!(cpu.get_register("r8").unwrap(), 0xBEEFFEED);
    }

    #[test]
    fn test_ldm_stm_block_transfers() {
        let mut cpu = SovereignVirtualCPU::new();

        cpu.set_register("r10", 256).unwrap();
        cpu.set_register("r11", 0x1111).unwrap();
        cpu.set_register("r12", 0x2222).unwrap();
        cpu.set_register("r13", 0x3333).unwrap();

        // STM IA
        cpu.stm("r10", &["r11", "r12", "r13"], BlockTransferMode::IncrementAfter, true).unwrap();
        assert_eq!(cpu.read_mem_u64(256).unwrap(), 0x1111);
        assert_eq!(cpu.read_mem_u64(264).unwrap(), 0x2222);
        assert_eq!(cpu.read_mem_u64(272).unwrap(), 0x3333);
        assert_eq!(cpu.get_register("r10").unwrap(), 280);

        // LDM DB
        cpu.set_register("r11", 0).unwrap();
        cpu.set_register("r12", 0).unwrap();
        cpu.set_register("r13", 0).unwrap();
        cpu.ldm("r10", &["r11", "r12", "r13"], BlockTransferMode::DecrementBefore, true).unwrap();
        assert_eq!(cpu.get_register("r11").unwrap(), 0x3333);
        assert_eq!(cpu.get_register("r12").unwrap(), 0x2222);
        assert_eq!(cpu.get_register("r13").unwrap(), 0x1111);
        assert_eq!(cpu.get_register("r10").unwrap(), 256);
    }

    #[test]
    fn test_push_pop_multiple_alignment_and_privileges() {
        let mut cpu = SovereignVirtualCPU::new();

        cpu.set_register("r14", 0xAAAA).unwrap();
        cpu.set_register("r15", 0xBBBB).unwrap();

        cpu.push_multiple(&["r14", "r15"]).unwrap();
        assert_eq!(cpu.registers.rsp, 1008);

        cpu.set_register("r14", 0).unwrap();
        cpu.set_register("r15", 0).unwrap();

        cpu.pop_multiple(&["r14", "r15"]).unwrap();
        assert_eq!(cpu.get_register("r14").unwrap(), 0xAAAA);
        assert_eq!(cpu.get_register("r15").unwrap(), 0xBBBB);
        assert_eq!(cpu.registers.rsp, 1024);

        // Alignment fault
        cpu.registers.rsp = 1023;
        assert_eq!(cpu.push_multiple(&["r14"]), Err(CpuError::AlignmentFault));

        // Privilege fault (Ring 3 accessing Kernel space >= 2048)
        cpu.registers.rsp = 1024;
        cpu.ring = CpuRing::Ring3;
        cpu.set_register("rax", 3000).unwrap(); // Kernel memory address
        assert_eq!(cpu.ldr("rbx", "rax", 0, AddressingMode::Offset), Err(CpuError::PrivilegeViolation));
    }

    // New NT integration tests for thread suspension & APCs
    #[test]
    fn test_thread_suspension_and_resume() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.ke_init_thread(2, 1).unwrap();
        assert_eq!(cpu.ke_suspend_thread(2).unwrap(), 1);
        assert_eq!(cpu.threads[1].state, ThreadState::Suspended);

        assert_eq!(cpu.ke_resume_thread(2).unwrap(), 0);
        assert_eq!(cpu.threads[1].state, ThreadState::Running);
    }

    #[test]
    fn test_apc_delivery_priorities() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.ke_init_thread(2, 1).unwrap();
        cpu.kprcb.active_thread_id = 2;

        fn dummy_kernel_routine(cpu: &mut SovereignVirtualCPU, arg: u64) -> Result<(), CpuError> {
            cpu.registers.rax = arg;
            Ok(())
        }

        let apc = SovereignApc {
            apc_type: ApcType::SpecialKernel,
            kernel_routine: dummy_kernel_routine,
            rundown_routine: None,
            normal_routine: None,
            normal_context: 0,
            system_argument1: 0xBAAD,
            system_argument2: 0,
            freed_on_delivery: true,
        };

        cpu.ki_schedule_apc(2, apc).unwrap();
        let delivered = cpu.scheduler_apc().unwrap();
        assert_eq!(delivered, 1);
        assert_eq!(cpu.registers.rax, 0xBAAD);
    }

    #[test]
    fn test_dpc_priority_queuing() {
        let mut cpu = SovereignVirtualCPU::new();

        fn dummy_dpc_routine(cpu: &mut SovereignVirtualCPU, ctx: u64, _arg1: u64, _arg2: u64) -> Result<(), CpuError> {
            cpu.registers.rbx = ctx;
            Ok(())
        }

        let dpc1 = SovereignDpc {
            deferred_routine: dummy_dpc_routine,
            deferred_context: 10,
            system_argument1: 0,
            system_argument2: 0,
            importance: 1,
        };
        let dpc2 = SovereignDpc {
            deferred_routine: dummy_dpc_routine,
            deferred_context: 20,
            system_argument1: 0,
            system_argument2: 0,
            importance: 5, // Higher importance should run first
        };

        cpu.ke_insert_queue_dpc(dpc1).unwrap();
        cpu.ke_insert_queue_dpc(dpc2).unwrap();

        // Retire DPCs
        cpu.ki_retire_dpc_list().unwrap();
        // Since dpc2 is retired last, registers.rbx should contain 10 (since they both ran and dpc1 ran after dpc2 due to lower importance)
        assert_eq!(cpu.registers.rbx, 10);
    }

    #[test]
    fn test_work_items_processing() {
        let mut cpu = SovereignVirtualCPU::new();

        fn dummy_work_routine(cpu: &mut SovereignVirtualCPU, param: u64) -> Result<(), CpuError> {
            cpu.registers.rcx = param;
            Ok(())
        }

        let item = WorkItem {
            worker_routine: dummy_work_routine,
            parameter: 42,
            is_queued: false,
        };

        cpu.ex_queue_work_item(item).unwrap();
        assert_eq!(cpu.exp_worker_thread().unwrap(), 1);
        assert_eq!(cpu.registers.rcx, 42);
    }

    #[test]
    fn test_irql_preemption_boundaries() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.kprcb.current_irql, Irql::PassiveLevel);

        // Elevate to DISPATCH_LEVEL
        let old = cpu.ke_raise_irql(Irql::DispatchLevel).unwrap();
        assert_eq!(old, Irql::PassiveLevel);
        assert_eq!(cpu.kprcb.current_irql, Irql::DispatchLevel);

        // Elevating to lower level (e.g., PassiveLevel while at DispatchLevel) must fail
        assert_eq!(cpu.ke_raise_irql(Irql::PassiveLevel), Err(CpuError::IrqlViolation));

        // Lower to PASSIVE_LEVEL
        cpu.ke_lower_irql(Irql::PassiveLevel).unwrap();
        assert_eq!(cpu.kprcb.current_irql, Irql::PassiveLevel);
    }

    #[test]
    fn test_cisc_rep_string_instructions() {
        let mut cpu = SovereignVirtualCPU::new();

        // 1. REP STOSB: Fills bytes 100 to 110 with 0xAA
        cpu.registers.rcx = 10;
        cpu.registers.rdi = 100;
        cpu.registers.rax = 0xAA;
        cpu.rep_stosb().unwrap();

        for i in 100..110 {
            assert_eq!(cpu.memory[i], 0xAA);
        }
        assert_eq!(cpu.registers.rcx, 0);
        assert_eq!(cpu.registers.rdi, 110);
        assert!(cpu.registers.zf);

        // 2. REP MOVSB: Copies bytes 100 to 110 to bytes 200 to 210
        cpu.registers.rcx = 10;
        cpu.registers.rsi = 100;
        cpu.registers.rdi = 200;
        cpu.rep_movsb().unwrap();

        for i in 200..210 {
            assert_eq!(cpu.memory[i], 0xAA);
        }
        assert_eq!(cpu.registers.rcx, 0);
        assert_eq!(cpu.registers.rsi, 110);
        assert_eq!(cpu.registers.rdi, 210);
    }

    #[test]
    fn test_bitwise_shifts_and_rotations() {
        let mut cpu = SovereignVirtualCPU::new();

        // shl
        cpu.set_register("rax", 1).unwrap();
        cpu.shl("rax", 4).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 16);
        assert!(!cpu.registers.cf);

        // Carry detection
        cpu.set_register("rax", 1 << 63).unwrap();
        cpu.shl("rax", 1).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 0);
        assert!(cpu.registers.cf);
        assert!(cpu.registers.zf);

        // sar
        cpu.set_register("rax", 0xFF00000000000000).unwrap();
        cpu.sar("rax", 8).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 0xFFFF000000000000);

        // rol/ror
        cpu.set_register("rax", 1).unwrap();
        cpu.rol("rax", 1).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 2);

        cpu.ror("rax", 1).unwrap();
        assert_eq!(cpu.get_register("rax").unwrap(), 1);
    }

    #[test]
    fn test_hardware_memory_barriers() {
        let cpu = SovereignVirtualCPU::new();
        cpu.mfence();
        cpu.sfence();
        cpu.lfence();
        cpu.dmb();
        cpu.dsb();
=======
        // Write to target LSTAR MSR
        cpu.write_msr(0xC0000082, 0xFFFFFFFF80100000).unwrap();
        assert_eq!(cpu.msrs.lstar, 0xFFFFFFFF80100000);

        // Lower privilege to User space Ring 3
        cpu.transition_ring(CpuRing::Ring3).unwrap();

        // Simulate a system call instruction execution
        cpu.registers.rip = 0x400000; // user rip
        cpu.execute_syscall().unwrap();

        // CPU must have jumped to kernel's fast system call handler and escalated to Ring 0
        assert_eq!(cpu.registers.rip, 0xFFFFFFFF80100000);
        assert_eq!(cpu.registers.rcx, 0x400000);
        assert_eq!(cpu.ring, CpuRing::Ring0);
    }

    #[test]
    fn test_cpu_interrupt_traps() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.rip = 0xDEADBEEF;

        // Transition down to user space Ring 3
        cpu.transition_ring(CpuRing::Ring3).unwrap();

        // Trigger a page-fault exception trap (vector 14) pointing to handler 0x8100
        cpu.trigger_interrupt_trap(14, 0x8100).unwrap();

        assert_eq!(cpu.ring, CpuRing::Ring0);
        assert_eq!(cpu.registers.rip, 0x8100);

        // Popping the context frame off the kernel stack should yield the original user state
        let vector = cpu.pop_stack().unwrap();
        let ring = cpu.pop_stack().unwrap();
        let rip = cpu.pop_stack().unwrap();

        assert_eq!(vector, 14);
        assert_eq!(ring, 3);
        assert_eq!(rip, 0xDEADBEEF);
    }

    #[test]
    fn test_lazy_fp_state_restore() {
        let mut cpu = SovereignVirtualCPU::new();

        // Active TS (Task Switched) bit in CR0
        cpu.registers.cr0 |= 1 << 3;

        // Try to access FP registers - triggers restoration and clears TS bit
        cpu.handle_lazy_fp_state_restore(true).unwrap();
        assert_eq!(cpu.registers.cr0 & (1 << 3), 0);
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
    }
}
