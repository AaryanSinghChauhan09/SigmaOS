// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86/ARM CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.
// Also supports function invocation, arithmetic status flags, branching, Thumb state, switch cases, JIT compilation,
// self-modifying code, lock-prefixed atomic/synchronization primitives, and software interrupt mechanisms.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

// Flag bit positions in rflags (aligned with standard x86 rflags)
pub const FLAG_CF: u64 = 1 << 0;  // Carry Flag
pub const FLAG_ZF: u64 = 1 << 6;  // Zero Flag
pub const FLAG_SF: u64 = 1 << 7;  // Sign Flag
pub const FLAG_OF: u64 = 1 << 11; // Overflow Flag

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
    StackUnderflow = 5,
    InvalidBranchTarget = 6,
    SegmentationFault = 7,
    InvalidInterruptVector = 8,
    InterruptHandlerNotFound = 9,
    JitCacheMiss = 10,
    InvalidPrivilegeLevel = 11,
    DivisionByZero = 12,
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

/// Complete Virtual Register Set (inspired by x86-64 and ARM)
#[derive(Debug, Clone, Copy, Default)]
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
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
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
    pub ram: Vec<u8>,
    pub thumb_state: bool, // Thumb Mode (CPSR T-bit) from ARM for compact 16-bit emulation
    pub isr_table: Vec<Option<fn(&mut SovereignVirtualCPU)>>, // Software interrupt handlers
    pub jit_cache: BTreeMap<u64, Vec<Instruction>>, // Simple JIT block emulator cache (no_std compatible)
    pub code_cache_invalidated: bool, // Track self-modifying code changes
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
                rbp: 1024,
                rsp: 1024, // High stack pointer
                rip: 0,
                rflags: 0,
                cr0: 0,
                cr3: 0,
            },
            stack_memory: vec![0; 128], // 128 stack frames
            ram: vec![0; 4096], // 4KB RAM
            thumb_state: false,
            isr_table: vec![None; 256],
            jit_cache: BTreeMap::new(),
            code_cache_invalidated: false,
        }
    }

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
    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
        self.set_reg_val(dest, val)
    }

    /// Simulates standard stack pushing: `push <val>`
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

    /// Simulates standard stack popping: `pop`
    pub fn pop_stack(&mut self) -> Result<u64, CpuError> {
        let index = (self.registers.rsp / 8) as usize;
        if index >= self.stack_memory.len() {
            return Err(CpuError::StackUnderflow);
        }
        let val = self.stack_memory[index];
        self.registers.rsp += 8;
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
    fn test_function_invocation_call_ret() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.registers.rip = 100;
        cpu.call(200).unwrap();
        assert_eq!(cpu.registers.rip, 200);
        assert_eq!(cpu.registers.rsp, 1016); // pushed return address onto stack

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
    }
}
