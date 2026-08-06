// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
    AlignmentFault = 5,
    SegmentationFault = 6,
    InvalidInstruction = 7,
    MemoryAccessViolation = 8,
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

/// Complete x86 and ARM Virtual Register Set
#[derive(Debug, Clone, Copy, Default)]
pub struct RegisterSet {
    // x86 / x64 General Purpose Registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64, // Stack Pointer
    pub rip: u64, // Instruction Pointer
    pub rflags: u64, // Status and control flags
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable)
    pub cr3: u64, // Control Register 3: Page Table Base Address

    // Segment Registers (x86)
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub ss: u16,

    // ARM Registers
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub lr: u32, // Link Register (R14)
    pub pc: u32, // Program Counter (R15)
    pub cpsr: u32, // Current Program Status Register
}

/// Dynamic Operands supporting RISC and CISC instructions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    Register(String),
    Immediate(u64),
    // CISC complex memory addressing mode: [base_reg + index_reg * scale + disp]
    Memory {
        base: Option<String>,
        index: Option<String>,
        scale: u64,
        disp: i64,
    },
}

/// ARM-specific addressing modes (LDR/STR)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Offset,       // [base, #offset]
    PreIndexed,   // [base, #offset]! (updates base)
    PostIndexed,  // [base], #offset (updates base after load/store)
}

/// ARM LDM/STM Block Transfer modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockTransferMode {
    IncrementAfter,  // IA
    IncrementBefore, // IB
    DecrementAfter,  // DA
    DecrementBefore, // DB
}

/// Instruction Representation for the Sovereign Instruction Set Architecture (S-ISA)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    // CISC Data Movement
    Mov(Operand, Operand),
    Push(Operand),
    Pop(Operand),
    Xchg(Operand, Operand),

    // RISC Load/Store (ARM-specific semantics)
    Ldr(String, String, i64), // LDR dest_reg, [base_reg, #offset]
    Str(String, String, i64), // STR src_reg, [base_reg, #offset]

    // Advanced x86 Bitwise Shifts / Rotations
    Shl(Operand, Operand), // Logical Shift Left
    Shr(Operand, Operand), // Logical Shift Right
    Sar(Operand, Operand), // Arithmetic Shift Right
    Rol(Operand, Operand), // Rotate Left
    Ror(Operand, Operand), // Rotate Right

    // Advanced x86 Block Transfer/String instructions
    RepMovs, // Copy bytes from DS:RSI to ES:RDI, count in RCX
    RepStos, // Fill memory at ES:RDI with AL/AX/EAX/RAX, count in RCX

    // Advanced ARM Multi-register Block Transfer
    Ldm(String, BlockTransferMode, Vec<String>), // Load Multiple registers from base
    Stm(String, BlockTransferMode, Vec<String>), // Store Multiple registers to base

    // Advanced ARM Pre/Post-Indexed LDR/STR
    LdrAdvanced(String, String, i64, AddressingMode),
    StrAdvanced(String, String, i64, AddressingMode),

    // Advanced Memory Barrier & Synchronization
    Mfence, // x86-64 Memory Fence
    Dmb,    // ARM Data Memory Barrier
    Dsb,    // ARM Data Synchronization Barrier

    // Arithmetic & Logic
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Cmp(Operand, Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Xor(Operand, Operand),

    // Control Flow
    Jmp(u64),
    Je(u64),
    Jne(u64),
    Jg(u64),
    Jl(u64),

    // Privilege & Traps (syscall / supervisor calls)
    Syscall, // x86 syscall instruction
    Svc(u32), // ARM supervisor call / swi
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
    pub memory: Vec<u8>, // Simulated Physical/Virtual Address Space (64KB)
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
                rsi: 0,
                rdi: 0,
                rbp: 0,
                rsp: 1024, // High stack pointer
                rip: 0,
                rflags: 0,
                cr0: 0,
                cr3: 0,
                cs: 0,
                ds: 0,
                es: 0,
                ss: 0,
                r0: 0,
                r1: 0,
                r2: 0,
                r3: 0,
                r4: 0,
                r5: 0,
                r6: 0,
                r7: 0,
                r8: 0,
                r9: 0,
                r10: 0,
                r11: 0,
                r12: 0,
                lr: 0,
                pc: 0,
                cpsr: 0,
            },
            stack_memory: vec![0; 128], // 128 stack frames
            memory: vec![0; 65536], // 64KB main memory
        }
    }

    /// Read value from a dynamic register by name (supports both x86 64-bit and ARM 32-bit registers)
    pub fn get_reg(&self, name: &str) -> Result<u64, CpuError> {
        match name {
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
            "cr0" => Ok(self.registers.cr0),
            "cr3" => Ok(self.registers.cr3),
            "cs" => Ok(self.registers.cs as u64),
            "ds" => Ok(self.registers.ds as u64),
            "es" => Ok(self.registers.es as u64),
            "ss" => Ok(self.registers.ss as u64),
            "r0" => Ok(self.registers.r0 as u64),
            "r1" => Ok(self.registers.r1 as u64),
            "r2" => Ok(self.registers.r2 as u64),
            "r3" => Ok(self.registers.r3 as u64),
            "r4" => Ok(self.registers.r4 as u64),
            "r5" => Ok(self.registers.r5 as u64),
            "r6" => Ok(self.registers.r6 as u64),
            "r7" => Ok(self.registers.r7 as u64),
            "r8" => Ok(self.registers.r8 as u64),
            "r9" => Ok(self.registers.r9 as u64),
            "r10" => Ok(self.registers.r10 as u64),
            "r11" => Ok(self.registers.r11 as u64),
            "r12" => Ok(self.registers.r12 as u64),
            "lr" => Ok(self.registers.lr as u64),
            "pc" => Ok(self.registers.pc as u64),
            "cpsr" => Ok(self.registers.cpsr as u64),
            _ => Err(CpuError::InvalidRegister),
        }
    }

    /// Write value to a dynamic register by name
    pub fn set_reg(&mut self, name: &str, val: u64) -> Result<(), CpuError> {
        match name {
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
            "cr0" => {
                if self.ring != CpuRing::Ring0 {
                    return Err(CpuError::PrivilegeViolation);
                }
                self.registers.cr0 = val;
            }
            "cr3" => {
                if self.ring != CpuRing::Ring0 {
                    return Err(CpuError::PrivilegeViolation);
                }
                self.registers.cr3 = val;
            }
            "cs" => self.registers.cs = val as u16,
            "ds" => self.registers.ds = val as u16,
            "es" => self.registers.es = val as u16,
            "ss" => self.registers.ss = val as u16,
            "r0" => self.registers.r0 = val as u32,
            "r1" => self.registers.r1 = val as u32,
            "r2" => self.registers.r2 = val as u32,
            "r3" => self.registers.r3 = val as u32,
            "r4" => self.registers.r4 = val as u32,
            "r5" => self.registers.r5 = val as u32,
            "r6" => self.registers.r6 = val as u32,
            "r7" => self.registers.r7 = val as u32,
            "r8" => self.registers.r8 = val as u32,
            "r9" => self.registers.r9 = val as u32,
            "r10" => self.registers.r10 = val as u32,
            "r11" => self.registers.r11 = val as u32,
            "r12" => self.registers.r12 = val as u32,
            "lr" => self.registers.lr = val as u32,
            "pc" => self.registers.pc = val as u32,
            "cpsr" => self.registers.cpsr = val as u32,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    /// CISC x86 addressing mode resolver: resolves complex addressing `[base + index * scale + displacement]`
    /// If base or index is empty/None, they are ignored.
    pub fn resolve_address(&self, base: Option<&str>, index: Option<&str>, scale: u64, displacement: i64) -> Result<u64, CpuError> {
        let mut addr: i64 = 0;
        if let Some(b) = base {
            addr = addr.checked_add(self.get_reg(b)? as i64).ok_or(CpuError::SegmentationFault)?;
        }
        if let Some(idx) = index {
            let idx_val = self.get_reg(idx)? as i64;
            let scaled = idx_val.checked_mul(scale as i64).ok_or(CpuError::SegmentationFault)?;
            addr = addr.checked_add(scaled).ok_or(CpuError::SegmentationFault)?;
        }
        addr = addr.checked_add(displacement).ok_or(CpuError::SegmentationFault)?;
        if addr < 0 || addr as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        Ok(addr as u64)
    }

    /// Read 1 byte from memory
    pub fn read_mem_u8(&self, addr: u64) -> Result<u8, CpuError> {
        if addr as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        Ok(self.memory[addr as usize])
    }

    /// Write 1 byte to memory
    pub fn write_mem_u8(&mut self, addr: u64, val: u8) -> Result<(), CpuError> {
        if addr as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        self.memory[addr as usize] = val;
        Ok(())
    }

    /// Read 2 bytes (u16) from memory (with optional alignment checks)
    pub fn read_mem_u16(&self, addr: u64, check_align: bool) -> Result<u16, CpuError> {
        if check_align && addr % 2 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 1) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        let b1 = self.memory[addr as usize] as u16;
        let b2 = self.memory[(addr + 1) as usize] as u16;
        Ok(b1 | (b2 << 8))
    }

    /// Write 2 bytes (u16) to memory
    pub fn write_mem_u16(&mut self, addr: u64, val: u16, check_align: bool) -> Result<(), CpuError> {
        if check_align && addr % 2 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 1) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        self.memory[addr as usize] = (val & 0xFF) as u8;
        self.memory[(addr + 1) as usize] = ((val >> 8) & 0xFF) as u8;
        Ok(())
    }

    /// Read 4 bytes (u32) from memory
    pub fn read_mem_u32(&self, addr: u64, check_align: bool) -> Result<u32, CpuError> {
        if check_align && addr % 4 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 3) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        let mut val: u32 = 0;
        for i in 0..4 {
            val |= (self.memory[(addr + i) as usize] as u32) << (i * 8);
        }
        Ok(val)
    }

    /// Write 4 bytes (u32) to memory
    pub fn write_mem_u32(&mut self, addr: u64, val: u32, check_align: bool) -> Result<(), CpuError> {
        if check_align && addr % 4 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 3) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        for i in 0..4 {
            self.memory[(addr + i) as usize] = ((val >> (i * 8)) & 0xFF) as u8;
        }
        Ok(())
    }

    /// Read 8 bytes (u64) from memory
    pub fn read_mem_u64(&self, addr: u64, check_align: bool) -> Result<u64, CpuError> {
        if check_align && addr % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 7) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        let mut val: u64 = 0;
        for i in 0..8 {
            val |= (self.memory[(addr + i) as usize] as u64) << (i * 8);
        }
        Ok(val)
    }

    /// Write 8 bytes (u64) to memory
    pub fn write_mem_u64(&mut self, addr: u64, val: u64, check_align: bool) -> Result<(), CpuError> {
        if check_align && addr % 8 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if (addr + 7) as usize >= self.memory.len() {
            return Err(CpuError::SegmentationFault);
        }
        for i in 0..8 {
            self.memory[(addr + i) as usize] = ((val >> (i * 8)) & 0xFF) as u8;
        }
        Ok(())
    }

    /// Simulates standard x86 assembly data movement: `mov <dest>, <src_val>`
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
        self.set_reg(dest, val)
    }

    /// Simulates standard x86 assembly stack pushing: `push <val>`
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

    /// Simulates standard x86 assembly stack popping: `pop`
    pub fn pop_stack(&mut self) -> Result<u64, CpuError> {
        let index = (self.registers.rsp / 8) as usize;
        if index >= self.stack_memory.len() {
            return Err(CpuError::StackOverflow);
        }
        let val = self.stack_memory[index];
        self.registers.rsp += 8;
        Ok(val)
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

    /// Resolves an operand value (e.g. read from register, immediate, or complex memory)
    pub fn eval_operand(&self, op: &Operand) -> Result<u64, CpuError> {
        match op {
            Operand::Register(name) => self.get_reg(name),
            Operand::Immediate(val) => Ok(*val),
            Operand::Memory { base, index, scale, disp } => {
                let base_str = base.as_deref();
                let index_str = index.as_deref();
                let addr = self.resolve_address(base_str, index_str, *scale, *disp)?;
                // Determine width from registers or use standard 64-bit/u64 load
                self.read_mem_u64(addr, false)
            }
        }
    }

    /// Writes a value to an operand (only registers or memory)
    pub fn write_operand(&mut self, op: &Operand, val: u64) -> Result<(), CpuError> {
        match op {
            Operand::Register(name) => self.set_reg(name, val),
            Operand::Immediate(_) => Err(CpuError::InvalidInstruction),
            Operand::Memory { base, index, scale, disp } => {
                let base_str = base.as_deref();
                let index_str = index.as_deref();
                let addr = self.resolve_address(base_str, index_str, *scale, *disp)?;
                self.write_mem_u64(addr, val, false)
            }
        }
    }

    /// Updates both x86 (RFLAGS) and ARM (CPSR) status flags based on mathematical results.
    /// x86 RFLAGS: ZF (bit 6), SF (bit 7), CF (bit 0), OF (bit 11)
    /// ARM CPSR: Z (bit 30), N (bit 31), C (bit 29), V (bit 28)
    pub fn update_flags(&mut self, op1: u64, op2: u64, result: u64, is_sub: bool, is_logical: bool) {
        let mut rflags = self.registers.rflags;
        let mut cpsr = self.registers.cpsr;

        // Zero Flag (ZF / Z)
        let zf = if result == 0 { 1 } else { 0 };
        if zf == 1 {
            rflags |= 1 << 6;
            cpsr |= 1 << 30;
        } else {
            rflags &= !(1 << 6);
            cpsr &= !(1 << 30);
        }

        // Sign/Negative Flag (SF / N)
        // Check MSB of 64-bit result
        let sf = if (result & (1 << 63)) != 0 { 1 } else { 0 };
        if sf == 1 {
            rflags |= 1 << 7;
            cpsr |= 1 << 31;
        } else {
            rflags &= !(1 << 7);
            cpsr &= !(1 << 31);
        }

        // Carry and Overflow Flags
        if is_logical {
            // Logical operations clear Carry and Overflow flags
            rflags &= !(1 << 0); // Clear CF
            rflags &= !(1 << 11); // Clear OF
            cpsr &= !(1 << 29); // Clear C
            cpsr &= !(1 << 28); // Clear V
        } else if is_sub {
            // Unsigned underflow (Carry borrowing style)
            let cf = if op1 < op2 { 1 } else { 0 };
            if cf == 1 {
                rflags |= 1 << 0;
                cpsr &= !(1 << 29); // In ARM, borrow is indicated by C = 0
            } else {
                rflags &= !(1 << 0);
                cpsr |= 1 << 29; // No borrow, C = 1
            }

            // Signed overflow
            let op1_s = op1 as i64;
            let op2_s = op2 as i64;
            let (_res_s, of_bool) = op1_s.overflowing_sub(op2_s);
            if of_bool {
                rflags |= 1 << 11;
                cpsr |= 1 << 28;
            } else {
                rflags &= !(1 << 11);
                cpsr &= !(1 << 28);
            }
        } else {
            // Addition
            let (_res_u, cf_bool) = op1.overflowing_add(op2);
            if cf_bool {
                rflags |= 1 << 0;
                cpsr |= 1 << 29;
            } else {
                rflags &= !(1 << 0);
                cpsr &= !(1 << 29);
            }

            let op1_s = op1 as i64;
            let op2_s = op2 as i64;
            let (_res_s, of_bool) = op1_s.overflowing_add(op2_s);
            if of_bool {
                rflags |= 1 << 11;
                cpsr |= 1 << 28;
            } else {
                rflags &= !(1 << 11);
                cpsr &= !(1 << 28);
            }
        }

        self.registers.rflags = rflags;
        self.registers.cpsr = cpsr;
    }

    /// Evaluates if a conditional jump is met based on CPU flags
    pub fn check_cond(&self, cond: &str) -> bool {
        let rflags = self.registers.rflags;
        let zf = (rflags & (1 << 6)) != 0;
        let sf = (rflags & (1 << 7)) != 0;
        let of = (rflags & (1 << 11)) != 0;

        match cond {
            "e" | "z" => zf,
            "ne" | "nz" => !zf,
            "g" => !zf && (sf == of),
            "l" => sf != of,
            _ => false,
        }
    }

    /// Primary Interpreter Loop for Instruction Set Execution
    pub fn execute_instruction(&mut self, inst: &Instruction) -> Result<(), CpuError> {
        match inst {
            Instruction::Mov(dest, src) => {
                let val = self.eval_operand(src)?;
                self.write_operand(dest, val)?;
            }
            Instruction::Push(op) => {
                let val = self.eval_operand(op)?;
                self.push_stack(val)?;
            }
            Instruction::Pop(op) => {
                let val = self.pop_stack()?;
                self.write_operand(op, val)?;
            }
            Instruction::Xchg(op1, op2) => {
                let val1 = self.eval_operand(op1)?;
                let val2 = self.eval_operand(op2)?;
                self.write_operand(op1, val2)?;
                self.write_operand(op2, val1)?;
            }
            Instruction::Ldr(dest_reg, base_reg, offset) => {
                // ARM load: reg = [base + offset]
                let base_val = self.get_reg(base_reg)?;
                let addr = base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?;
                let val = self.read_mem_u32(addr, false)? as u64;
                self.set_reg(dest_reg, val)?;
            }
            Instruction::Str(src_reg, base_reg, offset) => {
                // ARM store: [base + offset] = reg
                let src_val = self.get_reg(src_reg)?;
                let base_val = self.get_reg(base_reg)?;
                let addr = base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?;
                self.write_mem_u32(addr, src_val as u32, false)?;
            }

            // Advanced Bitwise shifts and rotations
            Instruction::Shl(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.wrapping_shl(op2 as u32);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Shr(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.wrapping_shr(op2 as u32);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Sar(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = (op1 as i64).wrapping_shr(op2 as u32) as u64;
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Rol(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.rotate_left(op2 as u32);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Ror(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.rotate_right(op2 as u32);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }

            // Advanced block movement / x86 string instructions
            Instruction::RepMovs => {
                // Copy bytes from DS:RSI to ES:RDI, count in RCX
                let mut rsi = self.get_reg("rsi")?;
                let mut rdi = self.get_reg("rdi")?;
                let mut rcx = self.get_reg("rcx")?;
                while rcx > 0 {
                    let val = self.read_mem_u8(rsi)?;
                    self.write_mem_u8(rdi, val)?;
                    rsi += 1;
                    rdi += 1;
                    rcx -= 1;
                }
                self.set_reg("rsi", rsi)?;
                self.set_reg("rdi", rdi)?;
                self.set_reg("rcx", rcx)?;
            }
            Instruction::RepStos => {
                // Fill memory at ES:RDI with RAX byte (AL), count in RCX
                let mut rdi = self.get_reg("rdi")?;
                let mut rcx = self.get_reg("rcx")?;
                let al = (self.get_reg("rax")? & 0xFF) as u8;
                while rcx > 0 {
                    self.write_mem_u8(rdi, al)?;
                    rdi += 1;
                    rcx -= 1;
                }
                self.set_reg("rdi", rdi)?;
                self.set_reg("rcx", rcx)?;
            }

            // Advanced ARM LDM/STM multi-register block transfers
            Instruction::Ldm(base_reg, mode, regs) => {
                let mut addr = self.get_reg(base_reg)?;
                for reg in regs {
                    match mode {
                        BlockTransferMode::IncrementBefore => addr += 4,
                        BlockTransferMode::DecrementBefore => addr -= 4,
                        _ => {}
                    }
                    let val = self.read_mem_u32(addr, false)? as u64;
                    self.set_reg(reg, val)?;
                    match mode {
                        BlockTransferMode::IncrementAfter => addr += 4,
                        BlockTransferMode::DecrementAfter => addr -= 4,
                        _ => {}
                    }
                }
                self.set_reg(base_reg, addr)?;
            }
            Instruction::Stm(base_reg, mode, regs) => {
                let mut addr = self.get_reg(base_reg)?;
                for reg in regs {
                    match mode {
                        BlockTransferMode::IncrementBefore => addr += 4,
                        BlockTransferMode::DecrementBefore => addr -= 4,
                        _ => {}
                    }
                    let val = self.get_reg(reg)?;
                    self.write_mem_u32(addr, val as u32, false)?;
                    match mode {
                        BlockTransferMode::IncrementAfter => addr += 4,
                        BlockTransferMode::DecrementAfter => addr -= 4,
                        _ => {}
                    }
                }
                self.set_reg(base_reg, addr)?;
            }

            // Advanced ARM addressing modes (offset, pre-indexed, post-indexed)
            Instruction::LdrAdvanced(dest_reg, base_reg, offset, addr_mode) => {
                let base_val = self.get_reg(base_reg)?;
                let addr = match addr_mode {
                    AddressingMode::Offset | AddressingMode::PreIndexed => {
                        base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?
                    }
                    AddressingMode::PostIndexed => base_val,
                };
                let val = self.read_mem_u32(addr, false)? as u64;
                self.set_reg(dest_reg, val)?;

                match addr_mode {
                    AddressingMode::PreIndexed => {
                        self.set_reg(base_reg, addr)?;
                    }
                    AddressingMode::PostIndexed => {
                        let new_base = base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?;
                        self.set_reg(base_reg, new_base)?;
                    }
                    _ => {}
                }
            }
            Instruction::StrAdvanced(src_reg, base_reg, offset, addr_mode) => {
                let src_val = self.get_reg(src_reg)?;
                let base_val = self.get_reg(base_reg)?;
                let addr = match addr_mode {
                    AddressingMode::Offset | AddressingMode::PreIndexed => {
                        base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?
                    }
                    AddressingMode::PostIndexed => base_val,
                };
                self.write_mem_u32(addr, src_val as u32, false)?;

                match addr_mode {
                    AddressingMode::PreIndexed => {
                        self.set_reg(base_reg, addr)?;
                    }
                    AddressingMode::PostIndexed => {
                        let new_base = base_val.checked_add_signed(*offset).ok_or(CpuError::SegmentationFault)?;
                        self.set_reg(base_reg, new_base)?;
                    }
                    _ => {}
                }
            }

            // Memory fences & hardware barriers (mocked successfully)
            Instruction::Mfence | Instruction::Dmb | Instruction::Dsb => {
                // Emulates instruction execution ordering constraints
            }

            Instruction::Add(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.wrapping_add(op2);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, false);
            }
            Instruction::Sub(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1.wrapping_sub(op2);
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, true, false);
            }
            Instruction::Cmp(op1, op2) => {
                let val1 = self.eval_operand(op1)?;
                let val2 = self.eval_operand(op2)?;
                let res = val1.wrapping_sub(val2);
                self.update_flags(val1, val2, res, true, false);
            }
            Instruction::And(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1 & op2;
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Or(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1 | op2;
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Xor(dest, src) => {
                let op1 = self.eval_operand(dest)?;
                let op2 = self.eval_operand(src)?;
                let res = op1 ^ op2;
                self.write_operand(dest, res)?;
                self.update_flags(op1, op2, res, false, true);
            }
            Instruction::Jmp(target_rip) => {
                self.registers.rip = *target_rip;
                self.registers.pc = *target_rip as u32;
            }
            Instruction::Je(target_rip) => {
                if self.check_cond("e") {
                    self.registers.rip = *target_rip;
                    self.registers.pc = *target_rip as u32;
                }
            }
            Instruction::Jne(target_rip) => {
                if self.check_cond("ne") {
                    self.registers.rip = *target_rip;
                    self.registers.pc = *target_rip as u32;
                }
            }
            Instruction::Jg(target_rip) => {
                if self.check_cond("g") {
                    self.registers.rip = *target_rip;
                    self.registers.pc = *target_rip as u32;
                }
            }
            Instruction::Jl(target_rip) => {
                if self.check_cond("l") {
                    self.registers.rip = *target_rip;
                    self.registers.pc = *target_rip as u32;
                }
            }
            Instruction::Syscall => {
                // x86 Sycall privilege separation helper: escalate to Ring 0
                // We mock handling system calls by transitioning privilege rings cleanly
                if self.ring == CpuRing::Ring3 {
                    self.ring = CpuRing::Ring0;
                    // Usually we would execute handler, here we transition rings cleanly as a trap
                }
            }
            Instruction::Svc(imm) => {
                // ARM Supervisor Call privilege escalation (e.g. software interrupt)
                if self.ring == CpuRing::Ring3 {
                    self.ring = CpuRing::Ring0;
                    self.registers.r0 = *imm; // Store service number
                }
            }
        }
        Ok(())
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

        // 3. Pop value from stack
        let val = cpu.pop_stack().unwrap();
        assert_eq!(val, 999);
        assert_eq!(cpu.registers.rsp, 1024);
    }

    #[test]
    fn test_complex_addressing_modes() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_reg("rbx", 1000).unwrap();
        cpu.set_reg("rsi", 5).unwrap();

        // [rbx + rsi * 8 + 24] -> 1000 + 40 + 24 = 1064
        let addr = cpu.resolve_address(Some("rbx"), Some("rsi"), 8, 24).unwrap();
        assert_eq!(addr, 1064);

        // Write a value to that resolved address
        cpu.write_mem_u64(addr, 0xABCDEF1234567890, false).unwrap();

        // Read it back
        let read_val = cpu.read_mem_u64(addr, false).unwrap();
        assert_eq!(read_val, 0xABCDEF1234567890);
    }

    #[test]
    fn test_alignment_faults() {
        let cpu = SovereignVirtualCPU::new();
        // Trying to read u64 at misaligned address 3 with align check enabled should fail
        assert_eq!(cpu.read_mem_u64(3, true), Err(CpuError::AlignmentFault));
        // u32 misalignment
        assert_eq!(cpu.read_mem_u32(2, true), Err(CpuError::AlignmentFault));
        // u16 misalignment
        assert_eq!(cpu.read_mem_u16(1, true), Err(CpuError::AlignmentFault));
    }

    #[test]
    fn test_instruction_execution_arithmetic_logic() {
        let mut cpu = SovereignVirtualCPU::new();

        // Setup rbx = 50, rcx = 25
        cpu.execute_instruction(&Instruction::Mov(Operand::Register("rbx".to_string()), Operand::Immediate(50))).unwrap();
        cpu.execute_instruction(&Instruction::Mov(Operand::Register("rcx".to_string()), Operand::Immediate(25))).unwrap();

        // Add rcx to rbx -> rbx = 75
        cpu.execute_instruction(&Instruction::Add(Operand::Register("rbx".to_string()), Operand::Register("rcx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rbx").unwrap(), 75);

        // Sub rcx from rbx -> rbx = 50
        cpu.execute_instruction(&Instruction::Sub(Operand::Register("rbx".to_string()), Operand::Register("rcx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rbx").unwrap(), 50);

        // And rax, rbx (0 & 50 -> 0)
        cpu.execute_instruction(&Instruction::And(Operand::Register("rax".to_string()), Operand::Register("rbx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rax").unwrap(), 0);

        // Xor rax, rbx (0 ^ 50 -> 50)
        cpu.execute_instruction(&Instruction::Xor(Operand::Register("rax".to_string()), Operand::Register("rbx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rax").unwrap(), 50);
    }

    #[test]
    fn test_control_flow_and_branching() {
        let mut cpu = SovereignVirtualCPU::new();

        // cmp rbx, rcx (0 - 0 = 0 -> ZF=1, Z=1)
        cpu.execute_instruction(&Instruction::Cmp(Operand::Register("rbx".to_string()), Operand::Register("rcx".to_string()))).unwrap();

        // je to 200
        cpu.execute_instruction(&Instruction::Je(200)).unwrap();
        assert_eq!(cpu.get_reg("rip").unwrap(), 200);

        // jne to 300 should not jump (rip stays 200)
        cpu.execute_instruction(&Instruction::Jne(300)).unwrap();
        assert_eq!(cpu.get_reg("rip").unwrap(), 200);
    }

    #[test]
    fn test_privilege_separations_and_traps() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.transition_ring(CpuRing::Ring3).unwrap();
        assert_eq!(cpu.ring, CpuRing::Ring3);

        // Escalation blocked
        assert_eq!(cpu.transition_ring(CpuRing::Ring0), Err(CpuError::PrivilegeViolation));

        // Use syscall software interrupt/trap to elevate ring privilege
        cpu.execute_instruction(&Instruction::Syscall).unwrap();
        assert_eq!(cpu.ring, CpuRing::Ring0);
    }

    #[test]
    fn test_arm_ldr_str_semantics() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_reg("r1", 2000).unwrap(); // base_reg
        cpu.set_reg("r2", 0x11223344).unwrap(); // src_reg

        // Str R2 to [R1, #12]
        cpu.execute_instruction(&Instruction::Str("r2".to_string(), "r1".to_string(), 12)).unwrap();

        // Ldr R3 from [R1, #12]
        cpu.execute_instruction(&Instruction::Ldr("r3".to_string(), "r1".to_string(), 12)).unwrap();
        assert_eq!(cpu.get_reg("r3").unwrap(), 0x11223344);
    }

    #[test]
    fn test_advanced_bitwise_shifts() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_reg("rax", 0b0011).unwrap();
        cpu.set_reg("rcx", 2).unwrap();

        // SHL rax, rcx -> rax = 0b1100 (12)
        cpu.execute_instruction(&Instruction::Shl(Operand::Register("rax".to_string()), Operand::Register("rcx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rax").unwrap(), 12);

        // SHR rax, rcx -> rax = 3
        cpu.execute_instruction(&Instruction::Shr(Operand::Register("rax".to_string()), Operand::Register("rcx".to_string()))).unwrap();
        assert_eq!(cpu.get_reg("rax").unwrap(), 3);

        // SAR negative number: 0xFFFFFFFFFFFFFFFC (-4) -> shift right 1 -> -2 (0xFFFFFFFFFFFFFFFE)
        cpu.set_reg("rax", 0xFFFFFFFFFFFFFFFC).unwrap();
        cpu.execute_instruction(&Instruction::Sar(Operand::Register("rax".to_string()), Operand::Immediate(1))).unwrap();
        assert_eq!(cpu.get_reg("rax").unwrap(), 0xFFFFFFFFFFFFFFFE);
    }

    #[test]
    fn test_string_block_transfer() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.write_mem_u8(100, 0x11).unwrap();
        cpu.write_mem_u8(101, 0x22).unwrap();
        cpu.write_mem_u8(102, 0x33).unwrap();

        cpu.set_reg("rsi", 100).unwrap();
        cpu.set_reg("rdi", 200).unwrap();
        cpu.set_reg("rcx", 3).unwrap();

        // RepMovs
        cpu.execute_instruction(&Instruction::RepMovs).unwrap();
        assert_eq!(cpu.read_mem_u8(200).unwrap(), 0x11);
        assert_eq!(cpu.read_mem_u8(201).unwrap(), 0x22);
        assert_eq!(cpu.read_mem_u8(202).unwrap(), 0x33);
        assert_eq!(cpu.get_reg("rcx").unwrap(), 0);

        // RepStos
        cpu.set_reg("rax", 0xEE).unwrap();
        cpu.set_reg("rdi", 300).unwrap();
        cpu.set_reg("rcx", 4).unwrap();
        cpu.execute_instruction(&Instruction::RepStos).unwrap();
        assert_eq!(cpu.read_mem_u8(300).unwrap(), 0xEE);
        assert_eq!(cpu.read_mem_u8(303).unwrap(), 0xEE);
        assert_eq!(cpu.get_reg("rcx").unwrap(), 0);
    }

    #[test]
    fn test_arm_ldm_stm() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_reg("r0", 0x11111111).unwrap();
        cpu.set_reg("r1", 0x22222222).unwrap();
        cpu.set_reg("r2", 0x33333333).unwrap();
        cpu.set_reg("r4", 5000).unwrap(); // base

        // STM r4!, {r0, r1, r2} IncrementAfter
        let regs = vec!["r0".to_string(), "r1".to_string(), "r2".to_string()];
        cpu.execute_instruction(&Instruction::Stm("r4".to_string(), BlockTransferMode::IncrementAfter, regs.clone())).unwrap();

        // Verify base was updated to 5000 + 3 * 4 = 5012
        assert_eq!(cpu.get_reg("r4").unwrap(), 5012);

        // Clear registers
        cpu.set_reg("r0", 0).unwrap();
        cpu.set_reg("r1", 0).unwrap();
        cpu.set_reg("r2", 0).unwrap();

        // LDM from start address (5000)
        cpu.set_reg("r4", 5000).unwrap();
        cpu.execute_instruction(&Instruction::Ldm("r4".to_string(), BlockTransferMode::IncrementAfter, regs)).unwrap();
        assert_eq!(cpu.get_reg("r0").unwrap(), 0x11111111);
        assert_eq!(cpu.get_reg("r1").unwrap(), 0x22222222);
        assert_eq!(cpu.get_reg("r2").unwrap(), 0x33333333);
    }

    #[test]
    fn test_advanced_ldr_str_modes() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_reg("r1", 8000).unwrap();
        cpu.set_reg("r2", 0x99887766).unwrap();

        // Pre-indexed: write r2 to [r1, #4]!, r1 becomes 8004
        cpu.execute_instruction(&Instruction::StrAdvanced("r2".to_string(), "r1".to_string(), 4, AddressingMode::PreIndexed)).unwrap();
        assert_eq!(cpu.get_reg("r1").unwrap(), 8004);

        // Post-indexed: load from [r1], #-4, r1 updates back to 8000
        cpu.execute_instruction(&Instruction::LdrAdvanced("r3".to_string(), "r1".to_string(), -4, AddressingMode::PostIndexed)).unwrap();
        assert_eq!(cpu.get_reg("r3").unwrap(), 0x99887766);
        assert_eq!(cpu.get_reg("r1").unwrap(), 8000);
    }
}
