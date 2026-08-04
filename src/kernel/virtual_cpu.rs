// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
    AlignmentFault = 5,
    InvalidAddress = 6,
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
}

/// Sovereign Virtual CPU managing execution state and privilege boundaries
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
    pub memory: Vec<u8>,
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
            },
            stack_memory: vec![0; 128], // 128 stack frames
            memory: vec![0; 4096],     // 4096 bytes of simulated RAM
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
}
