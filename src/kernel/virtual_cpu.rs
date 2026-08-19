// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.
// Extended to support Windows-inspired NT kernel abstractions: APCs, DPCs, IRQL preemption, and Thread dispatcher.
// Extended to support CISC-style block memory movement, bitwise shifts, and memory barrier instructions.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
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

/// Complete hybrid Virtual Register Set
#[derive(Debug, Clone, Copy)]
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable)
    pub cr3: u64, // Control Register 3: Page Table Base Address
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
    // 128-bit Vector SIMD Registers (xmm0..xmm7 / v0..v7)
    pub xmm: [[u8; 16]; 8],
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

/// Linux Kernel inspired `pt_regs` context frame saved on the kernel stack during syscall/exception entries
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PtRegs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbp: u64,
    pub rbx: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub orig_rax: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

/// Windows NT inspired KPRCB (Kernel Processor Control Block) equivalent
#[derive(Clone)]
pub struct SovereignKprcb {
    pub current_irql: Irql,
    pub dpc_queue: Vec<SovereignDpc>,
    pub work_item_queue: Vec<WorkItem>,
    pub active_thread_id: u64,
    pub interrupt_mask: u32,
    pub active_vector: Option<u8>,
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
    pub memory: Vec<u8>,
    pub kprcb: SovereignKprcb,
    pub threads: Vec<SovereignThread>,
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
                cr0: 0,
                cr3: 0,
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
                xmm: [[0u8; 16]; 8],
            },
            stack_memory: vec![0; 128], // 128 stack frames
            memory: vec![0; 4096],     // 4096 bytes of simulated RAM
            kprcb: SovereignKprcb {
                current_irql: Irql::PassiveLevel,
                dpc_queue: Vec::new(),
                work_item_queue: Vec::new(),
                active_thread_id: 1,
                interrupt_mask: 0,
                active_vector: None,
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
        }
    }

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
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
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

    /// Emulates load register with CISC/RISC scaled index displacement addressing:
    /// `LDR <dest_reg>, [<base_reg> + <index_reg> * <scale> + <disp>]`
    pub fn ldr_scaled(
        &mut self,
        dest_reg: &str,
        base_reg: &str,
        index_reg: &str,
        scale: u64,
        disp: i64,
    ) -> Result<(), CpuError> {
        if scale != 1 && scale != 2 && scale != 4 && scale != 8 {
            return Err(CpuError::InvalidAddress);
        }
        let base_val = self.get_register(base_reg)?;
        let index_val = self.get_register(index_reg)?;
        let scaled_offset = index_val.checked_mul(scale).ok_or(CpuError::InvalidAddress)?;
        let effective_addr = (base_val as i64)
            .checked_add(scaled_offset as i64)
            .ok_or(CpuError::InvalidAddress)?
            .checked_add(disp)
            .ok_or(CpuError::InvalidAddress)? as u64;

        let val = self.read_mem_u64(effective_addr)?;
        self.set_register(dest_reg, val)?;
        Ok(())
    }

    /// Emulates store register with CISC/RISC scaled index displacement addressing:
    /// `STR <src_reg>, [<base_reg> + <index_reg> * <scale> + <disp>]`
    pub fn str_scaled(
        &mut self,
        src_reg: &str,
        base_reg: &str,
        index_reg: &str,
        scale: u64,
        disp: i64,
    ) -> Result<(), CpuError> {
        if scale != 1 && scale != 2 && scale != 4 && scale != 8 {
            return Err(CpuError::InvalidAddress);
        }
        let base_val = self.get_register(base_reg)?;
        let index_val = self.get_register(index_reg)?;
        let src_val = self.get_register(src_reg)?;

        let scaled_offset = index_val.checked_mul(scale).ok_or(CpuError::InvalidAddress)?;
        let effective_addr = (base_val as i64)
            .checked_add(scaled_offset as i64)
            .ok_or(CpuError::InvalidAddress)?
            .checked_add(disp)
            .ok_or(CpuError::InvalidAddress)? as u64;

        self.write_mem_u64(effective_addr, src_val)?;
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
    pub fn push_stack(&mut self, val: u64) -> Result<(), CpuError> {
        if self.registers.rsp % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if self.registers.rsp < 8 {
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

    /// Simulates standard x86 assembly stack popping: `pop <dest>`
    pub fn pop_stack(&mut self, dest: &str) -> Result<(), CpuError> {
        if self.registers.rsp % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        let index = (self.registers.rsp / 8) as usize;
        if index >= self.stack_memory.len() {
            return Err(CpuError::StackOverflow);
        }
        let val = self.stack_memory[index];
        self.registers.rsp += 8;
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

    /// Vector SIMD 128-bit Block Load Instruction: `VLD1 <v_dest>, [<base_reg>]`
    pub fn vld1(&mut self, vec_idx: usize, base_reg: &str) -> Result<(), CpuError> {
        if vec_idx >= 8 {
            return Err(CpuError::InvalidRegister);
        }
        let base_addr = self.get_register(base_reg)?;
        self.check_memory_privilege(base_addr, 16)?;

        let idx = base_addr as usize;
        self.registers.xmm[vec_idx].copy_from_slice(&self.memory[idx..idx + 16]);
        Ok(())
    }

    /// Vector SIMD 128-bit Block Store Instruction: `VST1 <v_src>, [<base_reg>]`
    pub fn vst1(&mut self, vec_idx: usize, base_reg: &str) -> Result<(), CpuError> {
        if vec_idx >= 8 {
            return Err(CpuError::InvalidRegister);
        }
        let base_addr = self.get_register(base_reg)?;
        self.check_memory_privilege(base_addr, 16)?;

        let idx = base_addr as usize;
        self.memory[idx..idx + 16].copy_from_slice(&self.registers.xmm[vec_idx]);
        Ok(())
    }

    /// Hardware-Accelerated AES Round Encryption: `AESENC <xmm_dest>, <round_key>`
    /// Performs SubBytes, ShiftRows, and AddRoundKey on 128-bit SIMD register state
    pub fn emulate_aesenc(&mut self, vec_idx: usize, round_key: [u8; 16]) -> Result<(), CpuError> {
        if vec_idx >= 8 {
            return Err(CpuError::InvalidRegister);
        }
        let state = &mut self.registers.xmm[vec_idx];

        // 1. SubBytes & AddRoundKey XOR transformation
        for i in 0..16 {
            let val = state[i].wrapping_add(1) ^ round_key[i]; // Substitution + Key XOR
            state[i] = val;
        }

        // 2. ShiftRows permutation
        state.swap(1, 5);
        state.swap(2, 10);
        state.swap(3, 15);
        Ok(())
    }

    /// Hardware-Accelerated SHA-256 2-Round Transformation: `SHA256RNDS2 <xmm_dest>, <msg_word>`
    pub fn emulate_sha256rnds2(&mut self, vec_idx: usize, msg_word: u32) -> Result<(), CpuError> {
        if vec_idx >= 8 {
            return Err(CpuError::InvalidRegister);
        }
        let state = &mut self.registers.xmm[vec_idx];
        let msg_bytes = msg_word.to_le_bytes();

        for i in 0..4 {
            state[i] = state[i].wrapping_add(msg_bytes[i]);
            state[i + 4] ^= msg_bytes[3 - i];
        }
        Ok(())
    }

    /// Linux x86_64 inspired SYSCALL entry instruction emulation.
    /// Transitions from Ring 3 (User) to Ring 0 (Kernel), saving user RIP to RCX, user RFLAGS to R11,
    /// and pushing a full `PtRegs` context frame onto the kernel stack.
    pub fn emulate_syscall(&mut self, syscall_num: u64) -> Result<PtRegs, CpuError> {
        self.registers.rcx = self.registers.rip;
        self.registers.r11 = 0x202; // Standard RFLAGS IF bit set
        self.registers.rax = syscall_num;

        let frame = PtRegs {
            r15: self.registers.r15,
            r14: self.registers.r14,
            r13: self.registers.r13,
            r12: self.registers.r12,
            rbp: self.registers.rbp,
            rbx: self.registers.rbx,
            r11: self.registers.r11,
            r10: self.registers.r10,
            r9: self.registers.r9,
            r8: self.registers.r8,
            rax: self.registers.rax,
            rcx: self.registers.rcx,
            rdx: self.registers.rdx,
            rsi: self.registers.rsi,
            rdi: self.registers.rdi,
            orig_rax: syscall_num,
            rip: self.registers.rip,
            cs: 0x33, // User CS segment selector
            rflags: self.registers.r11,
            rsp: self.registers.rsp,
            ss: 0x2B, // User SS segment selector
        };

        // Transition CPU to Ring 0 (Kernel)
        self.ring = CpuRing::Ring0;
        Ok(frame)
    }

    /// Linux x86_64 inspired SYSRET exit instruction emulation.
    /// Restores user RIP from RCX, restores user RFLAGS from R11, and switches CPU privilege level back to Ring 3.
    pub fn emulate_sysret(&mut self, frame: &PtRegs) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation); // SYSRET is a privileged kernel instruction
        }

        self.registers.rip = frame.rcx;
        self.registers.rsp = frame.rsp;
        self.registers.rax = frame.rax;
        self.ring = CpuRing::Ring3; // Restore Ring 3 (User) execution
        Ok(())
    }

    /// Masks a hardware interrupt vector line on the virtual interrupt controller
    pub fn mask_interrupt(&mut self, vector: u8) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        if vector >= 32 {
            return Err(CpuError::InvalidAddress);
        }
        self.kprcb.interrupt_mask |= 1 << vector;
        Ok(())
    }

    /// Unmasks a hardware interrupt vector line on the virtual interrupt controller
    pub fn unmask_interrupt(&mut self, vector: u8) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        if vector >= 32 {
            return Err(CpuError::InvalidAddress);
        }
        self.kprcb.interrupt_mask &= !(1 << vector);
        Ok(())
    }

    /// Sends an End of Interrupt (EOI) signal to acknowledge and clear the active interrupt vector state
    pub fn send_eoi(&mut self, vector: u8) -> Result<(), CpuError> {
        if self.ring != CpuRing::Ring0 {
            return Err(CpuError::PrivilegeViolation);
        }
        if Some(vector) == self.kprcb.active_vector {
            self.kprcb.active_vector = None;
        }
        Ok(())
    }

    /// Linux ARM64 inspired SVC (Supervisor Call) software exception entry emulation.
    /// Transitions from User Mode to Kernel Mode and generates a `PtRegs` context frame.
    pub fn emulate_svc(&mut self, svc_num: u32) -> Result<PtRegs, CpuError> {
        self.registers.rax = svc_num as u64;

        let frame = PtRegs {
            r15: self.registers.r15,
            r14: self.registers.r14,
            r13: self.registers.r13,
            r12: self.registers.r12,
            rbp: self.registers.rbp,
            rbx: self.registers.rbx,
            r11: self.registers.r11,
            r10: self.registers.r10,
            r9: self.registers.r9,
            r8: self.registers.r8,
            rax: self.registers.rax,
            rcx: self.registers.rcx,
            rdx: self.registers.rdx,
            rsi: self.registers.rsi,
            rdi: self.registers.rdi,
            orig_rax: svc_num as u64,
            rip: self.registers.rip,
            cs: 0x0,
            rflags: 0,
            rsp: self.registers.rsp,
            ss: 0x0,
        };

        self.ring = CpuRing::Ring0;
        Ok(frame)
    }
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
    fn test_ldr_str_addressing_modes() {
        let mut cpu = SovereignVirtualCPU::new();

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
    }

    #[test]
    fn test_linux_syscall_sysret_pt_regs() {
        let mut cpu = SovereignVirtualCPU::new();

        // 1. Set user context in Ring 3
        cpu.ring = CpuRing::Ring3;
        cpu.registers.rip = 0x400000;
        cpu.registers.rsp = 0x7FFFF0;
        cpu.registers.rdi = 1; // stdout fd
        cpu.registers.rsi = 0x600000; // buffer addr
        cpu.registers.rdx = 12; // length

        // Execute SYSCALL (sys_write = 1)
        let frame = cpu.emulate_syscall(1).unwrap();

        // Verify transition to Kernel Ring 0, saving of user RIP in RCX and orig_rax in PtRegs
        assert_eq!(cpu.ring, CpuRing::Ring0);
        assert_eq!(cpu.registers.rcx, 0x400000);
        assert_eq!(frame.orig_rax, 1);
        assert_eq!(frame.rip, 0x400000);
        assert_eq!(frame.rsp, 0x7FFFF0);

        // Execute SYSRET to return to Ring 3
        cpu.emulate_sysret(&frame).unwrap();
        assert_eq!(cpu.ring, CpuRing::Ring3);
        assert_eq!(cpu.registers.rip, 0x400000);
    }

    #[test]
    fn test_arm64_svc_exception_entry() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.ring = CpuRing::Ring3;
        cpu.registers.rip = 0x8000;

        let frame = cpu.emulate_svc(64).unwrap(); // sys_write on ARM64
        assert_eq!(cpu.ring, CpuRing::Ring0);
        assert_eq!(frame.orig_rax, 64);
        assert_eq!(cpu.registers.rax, 64);
    }

    #[test]
    fn test_interrupt_masking_and_eoi() {
        let mut cpu = SovereignVirtualCPU::new();
        assert_eq!(cpu.kprcb.interrupt_mask, 0);

        // Mask vector 5
        cpu.mask_interrupt(5).unwrap();
        assert_eq!(cpu.kprcb.interrupt_mask, 1 << 5);

        // Unmask vector 5
        cpu.unmask_interrupt(5).unwrap();
        assert_eq!(cpu.kprcb.interrupt_mask, 0);

        // Send EOI
        cpu.kprcb.active_vector = Some(12);
        cpu.send_eoi(12).unwrap();
        assert_eq!(cpu.kprcb.active_vector, None);
    }

    #[test]
    fn test_vector_simd_block_transfers() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_register("rbx", 128).unwrap();

        // Write 16 test bytes to memory at address 128
        let sample_data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        cpu.memory[128..144].copy_from_slice(&sample_data);

        // Load 128-bit SIMD vector into xmm0
        cpu.vld1(0, "rbx").unwrap();
        assert_eq!(cpu.registers.xmm[0], sample_data);

        // Store 128-bit SIMD vector from xmm0 to memory address 256
        cpu.set_register("rcx", 256).unwrap();
        cpu.vst1(0, "rcx").unwrap();
        assert_eq!(&cpu.memory[256..272], &sample_data);
    }

    #[test]
    fn test_crypto_instruction_emulations() {
        let mut cpu = SovereignVirtualCPU::new();
        let key = [0xFFu8; 16];

        // Perform AES round encryption
        cpu.emulate_aesenc(0, key).unwrap();
        assert_ne!(cpu.registers.xmm[0], [0u8; 16]);

        // Perform SHA-256 2-round transformation
        cpu.emulate_sha256rnds2(0, 0x12345678).unwrap();
        assert_ne!(cpu.registers.xmm[0], [0u8; 16]);
    }

    #[test]
    fn test_scaled_index_displacement_addressing() {
        let mut cpu = SovereignVirtualCPU::new();

        // Base = 100, Index = 10, Scale = 8, Disp = 16 -> Address = 100 + 80 + 16 = 196
        cpu.set_register("rbx", 100).unwrap();
        cpu.set_register("rcx", 10).unwrap();
        cpu.set_register("rax", 0x1234567890ABCDEF).unwrap();

        // Store scaled
        cpu.str_scaled("rax", "rbx", "rcx", 8, 16).unwrap();
        assert_eq!(cpu.read_mem_u64(196).unwrap(), 0x1234567890ABCDEF);

        // Load scaled
        cpu.ldr_scaled("rdx", "rbx", "rcx", 8, 16).unwrap();
        assert_eq!(cpu.get_register("rdx").unwrap(), 0x1234567890ABCDEF);

        // Invalid scale (e.g. 3) should return InvalidAddress
        assert_eq!(
            cpu.ldr_scaled("rdx", "rbx", "rcx", 3, 16),
            Err(CpuError::InvalidAddress)
        );
    }
}
