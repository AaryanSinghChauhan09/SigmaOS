// Sovereign Virtual CPU and Ring Privilege Separation Simulator
// Implements x86 and ARM CPU Modes, Ring privilege isolation (Ring 0, 1, 2, 3), Register Sets, and Instruction Data Movement.

extern crate alloc;

use alloc::vec::Vec;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuError {
    Success = 0,
    InvalidRegister = 1,
    PrivilegeViolation = 2,
    StackOverflow = 3,
    PagingDisabled = 4,
    MemoryAccessViolation = 5,
    InvalidAddressingMode = 6,
    AlignmentFault = 7,
}

/// x86/ARM CPU Execution Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuMode {
    RealMode,      // 16-bit real addressing
    ProtectedMode, // 32-bit protected segments
    LongMode,      // 64-bit paging active
    ArmMode,       // ARM mode execution active
}

/// CPU Ring Privilege Separation levels (Inspiration: x86 Rings & ARM EL levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CpuRing {
    Ring0 = 0, // Kernel Core (unrestricted - EL1/EL2/EL3 equivalent)
    Ring1 = 1, // Device Drivers (SDF / isolated)
    Ring2 = 2, // System Services (init system)
    Ring3 = 3, // Userland Applications (most restricted - EL0 equivalent)
}

/// Extended Register Set incorporating both x86/x64 and ARM architecture registers
#[derive(Debug, Clone, Copy, Default)]
pub struct RegisterSet {
    // --- x86/x64 general-purpose registers ---
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64, // Source Index (string instructions)
    pub rdi: u64, // Destination Index (string instructions)
    pub rbp: u64, // Base Pointer
    pub rflags: u64, // Status flags register (ZF, SF, CF, OF)

    // --- x86 Segments ---
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub ss: u16,

    // --- x86 Control Registers ---
    pub cr0: u64, // Control Register 0: Bit 0 is PE (Protection Enable)
    pub cr3: u64, // Control Register 3: Page Table Base Address

    // --- ARM general-purpose registers ---
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
    pub lr: u32,   // Link Register (R14)
    pub pc: u32,   // Program Counter (R15)
    pub cpsr: u32, // Current Program Status Register

    // --- Common registers ---
    pub rip: u64, // x86 Instruction Pointer
    pub rsp: u64, // Stack Pointer
}

/// Sovereign Virtual CPU managing execution state, privilege boundaries, and simulated physical memory
pub struct SovereignVirtualCPU {
    pub mode: CpuMode,
    pub ring: CpuRing,
    pub registers: RegisterSet,
    pub stack_memory: Vec<u64>,
    pub memory: Vec<u8>, // Simulated 64KB physical/virtual memory space
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
                rflags: 0,
                cs: 0,
                ds: 0,
                es: 0,
                ss: 0,
                cr0: 0,
                cr3: 0,
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
                rip: 0,
                rsp: 1024, // High stack pointer
            },
            stack_memory: vec![0; 128], // 128 stack frames
            memory: vec![0; 65536],     // 64KB main memory space
        }
    }

    // --- Addressing Mode Simulators ---

    /// Simulates x86 CISC Complex Addressing Mode: Base + Index * Scale + Displacement
    pub fn calculate_cisc_address(
        &self,
        base: u64,
        index: u64,
        scale: u64,
        displacement: i64,
    ) -> Result<u64, CpuError> {
        if scale != 1 && scale != 2 && scale != 4 && scale != 8 {
            return Err(CpuError::InvalidAddressingMode);
        }
        let scaled_index = index.wrapping_mul(scale);
        let final_addr = if displacement >= 0 {
            base.wrapping_add(scaled_index).wrapping_add(displacement as u64)
        } else {
            base.wrapping_add(scaled_index).wrapping_sub((-displacement) as u64)
        };
        if final_addr >= self.memory.len() as u64 {
            return Err(CpuError::MemoryAccessViolation);
        }
        Ok(final_addr)
    }

    /// Simulates ARM Post-Indexed Addressing Mode: [Rn], #offset
    /// Returns the address, then increments/decrements Rn.
    pub fn arm_post_indexed(&mut self, reg_num: usize, offset: i32) -> Result<u32, CpuError> {
        let base_addr = self.get_arm_reg(reg_num)?;
        let updated_addr = if offset >= 0 {
            base_addr.wrapping_add(offset as u32)
        } else {
            base_addr.wrapping_sub((-offset) as u32)
        };
        self.set_arm_reg(reg_num, updated_addr)?;
        Ok(base_addr)
    }

    /// Simulates ARM Pre-Indexed Addressing Mode: [Rn, #offset]!
    /// Updates Rn first, then returns the updated address.
    pub fn arm_pre_indexed(&mut self, reg_num: usize, offset: i32) -> Result<u32, CpuError> {
        let base_addr = self.get_arm_reg(reg_num)?;
        let updated_addr = if offset >= 0 {
            base_addr.wrapping_add(offset as u32)
        } else {
            base_addr.wrapping_sub((-offset) as u32)
        };
        self.set_arm_reg(reg_num, updated_addr)?;
        Ok(updated_addr)
    }

    // --- General General-Purpose Register Access ---

    pub fn mov_val_to_reg(&mut self, dest: &str, val: u64) -> Result<(), CpuError> {
        match dest {
            "rax" => self.registers.rax = val,
            "rbx" => self.registers.rbx = val,
            "rcx" => self.registers.rcx = val,
            "rdx" => self.registers.rdx = val,
            "rsi" => self.registers.rsi = val,
            "rdi" => self.registers.rdi = val,
            "rbp" => self.registers.rbp = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    pub fn get_arm_reg(&self, reg_num: usize) -> Result<u32, CpuError> {
        match reg_num {
            0 => Ok(self.registers.r0),
            1 => Ok(self.registers.r1),
            2 => Ok(self.registers.r2),
            3 => Ok(self.registers.r3),
            4 => Ok(self.registers.r4),
            5 => Ok(self.registers.r5),
            6 => Ok(self.registers.r6),
            7 => Ok(self.registers.r7),
            8 => Ok(self.registers.r8),
            9 => Ok(self.registers.r9),
            10 => Ok(self.registers.r10),
            11 => Ok(self.registers.r11),
            12 => Ok(self.registers.r12),
            14 => Ok(self.registers.lr),
            15 => Ok(self.registers.pc),
            _ => Err(CpuError::InvalidRegister),
        }
    }

    pub fn set_arm_reg(&mut self, reg_num: usize, val: u32) -> Result<(), CpuError> {
        match reg_num {
            0 => self.registers.r0 = val,
            1 => self.registers.r1 = val,
            2 => self.registers.r2 = val,
            3 => self.registers.r3 = val,
            4 => self.registers.r4 = val,
            5 => self.registers.r5 = val,
            6 => self.registers.r6 = val,
            7 => self.registers.r7 = val,
            8 => self.registers.r8 = val,
            9 => self.registers.r9 = val,
            10 => self.registers.r10 = val,
            11 => self.registers.r11 = val,
            12 => self.registers.r12 = val,
            14 => self.registers.lr = val,
            15 => self.registers.pc = val,
            _ => return Err(CpuError::InvalidRegister),
        }
        Ok(())
    }

    // --- Memory Operations ---

    pub fn read_mem_u8(&self, addr: u64) -> Result<u8, CpuError> {
        if addr >= self.memory.len() as u64 {
            return Err(CpuError::MemoryAccessViolation);
        }
        Ok(self.memory[addr as usize])
    }

    pub fn write_mem_u8(&mut self, addr: u64, val: u8) -> Result<(), CpuError> {
        if addr >= self.memory.len() as u64 {
            return Err(CpuError::MemoryAccessViolation);
        }
        self.memory[addr as usize] = val;
        Ok(())
    }

    pub fn read_mem_u32(&self, addr: u64) -> Result<u32, CpuError> {
        if addr % 4 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if addr + 3 >= self.memory.len() as u64 {
            return Err(CpuError::MemoryAccessViolation);
        }
        let b0 = self.memory[addr as usize] as u32;
        let b1 = self.memory[addr as usize + 1] as u32;
        let b2 = self.memory[addr as usize + 2] as u32;
        let b3 = self.memory[addr as usize + 3] as u32;
        Ok(b0 | (b1 << 8) | (b2 << 16) | (b3 << 24))
    }

    pub fn write_mem_u32(&mut self, addr: u64, val: u32) -> Result<(), CpuError> {
        if addr % 4 != 0 {
            return Err(CpuError::AlignmentFault);
        }
        if addr + 3 >= self.memory.len() as u64 {
            return Err(CpuError::MemoryAccessViolation);
        }
        self.memory[addr as usize] = (val & 0xFF) as u8;
        self.memory[addr as usize + 1] = ((val >> 8) & 0xFF) as u8;
        self.memory[addr as usize + 2] = ((val >> 16) & 0xFF) as u8;
        self.memory[addr as usize + 3] = ((val >> 24) & 0xFF) as u8;
        Ok(())
    }

    // --- Complex CISC Instructions ---

    /// Bitwise Shifts & Rotates: SHL, SHR, SAR, ROL, ROR
    pub fn exec_shift_rotate(&mut self, opcode: &str, val: u64, amount: u32) -> Result<u64, CpuError> {
        let amount = amount % 64;
        let result = match opcode {
            "SHL" => val.wrapping_shl(amount),
            "SHR" => val.wrapping_shr(amount),
            "SAR" => {
                let signed = val as i64;
                signed.wrapping_shr(amount) as u64
            }
            "ROL" => val.rotate_left(amount),
            "ROR" => val.rotate_right(amount),
            _ => return Err(CpuError::InvalidAddressingMode),
        };

        // Update status flags: ZF (Zero Flag), SF (Sign Flag), CF (Carry Flag), OF (Overflow Flag)
        let mut flags = 0u64;
        if result == 0 {
            flags |= 1 << 6; // ZF (Bit 6)
        }
        if (result as i64) < 0 {
            flags |= 1 << 7; // SF (Bit 7)
        }
        self.registers.rflags = flags;
        Ok(result)
    }

    /// Block transfer/string instructions: REP MOVS / REP STOS
    pub fn exec_string_op(&mut self, opcode: &str) -> Result<(), CpuError> {
        let mut count = self.registers.rcx;
        let mut rsi = self.registers.rsi;
        let mut rdi = self.registers.rdi;

        match opcode {
            "REP MOVS" => {
                while count > 0 {
                    let val = self.read_mem_u8(rsi)?;
                    self.write_mem_u8(rdi, val)?;
                    rsi = rsi.wrapping_add(1);
                    rdi = rdi.wrapping_add(1);
                    count -= 1;
                }
            }
            "REP STOS" => {
                let fill_val = (self.registers.rax & 0xFF) as u8;
                while count > 0 {
                    self.write_mem_u8(rdi, fill_val)?;
                    rdi = rdi.wrapping_add(1);
                    count -= 1;
                }
            }
            _ => return Err(CpuError::InvalidAddressingMode),
        }

        self.registers.rcx = count;
        self.registers.rsi = rsi;
        self.registers.rdi = rdi;
        Ok(())
    }

    // --- ARM Block / Multi-Register Transfer Instructions ---

    /// ARM Store Multiple (STM) - simulates writing a set of registers to memory starting at base
    pub fn exec_stm(&mut self, base_reg: usize, reg_mask: u16) -> Result<(), CpuError> {
        let mut addr = self.get_arm_reg(base_reg)? as u64;
        for i in 0..16 {
            if (reg_mask & (1 << i)) != 0 {
                let val = self.get_arm_reg(i)?;
                self.write_mem_u32(addr, val)?;
                addr = addr.wrapping_add(4);
            }
        }
        self.set_arm_reg(base_reg, addr as u32)?;
        Ok(())
    }

    /// ARM Load Multiple (LDM) - simulates reading a set of registers from memory starting at base
    pub fn exec_ldm(&mut self, base_reg: usize, reg_mask: u16) -> Result<(), CpuError> {
        let mut addr = self.get_arm_reg(base_reg)? as u64;
        for i in 0..16 {
            if (reg_mask & (1 << i)) != 0 {
                let val = self.read_mem_u32(addr)?;
                self.set_arm_reg(i, val)?;
                addr = addr.wrapping_add(4);
            }
        }
        self.set_arm_reg(base_reg, addr as u32)?;
        Ok(())
    }

    // --- Hardware and Memory Barriers ---

    pub fn exec_barrier(&mut self, barrier_type: &str) {
        match barrier_type {
            "MFENCE" => {
                // Ensure all load and store instructions preceding MFENCE are globally visible
                core::sync::atomic::compiler_fence(Ordering::SeqCst);
            }
            "DMB" => {
                // Data Memory Barrier
                core::sync::atomic::compiler_fence(Ordering::Release);
            }
            "DSB" => {
                // Data Synchronization Barrier
                core::sync::atomic::compiler_fence(Ordering::SeqCst);
            }
            _ => {}
        }
    }

    // --- Privilege Transitions and Software Traps ---

    /// Simulates x86 software trap `syscall` or ARM `svc #imm`
    /// escalates ring to Ring0 and handles kernel trap.
    pub fn trigger_trap(&mut self, syscall_num: u64) -> Result<u64, CpuError> {
        // Elevate ring privilege to Ring0
        let original_ring = self.ring;
        self.ring = CpuRing::Ring0;

        let ret_val = match syscall_num {
            1 => {
                // sys_write (RAX is buffer address, RCX is len)
                let addr = self.registers.rax;
                let len = self.registers.rcx;
                let mut checksum = 0u64;
                for i in 0..len {
                    let b = self.read_mem_u8(addr.wrapping_add(i))? as u64;
                    checksum = checksum.wrapping_add(b);
                }
                checksum
            }
            2 => {
                // sys_get_cpu_id
                42
            }
            _ => 0xFFFFFFFFFFFFFFFF,
        };

        // Return privilege back to original ring level
        self.ring = original_ring;
        Ok(ret_val)
    }

    // --- Existing CPU Stack & Mode Functions ---

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
            CpuMode::ArmMode => {
                self.mode = CpuMode::ArmMode;
            }
        }
        Ok(())
    }

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
    fn test_cisc_scaled_indexing() {
        let cpu = SovereignVirtualCPU::new();
        let addr = cpu.calculate_cisc_address(100, 20, 4, 15).unwrap();
        assert_eq!(addr, 100 + 20 * 4 + 15);

        let addr_neg = cpu.calculate_cisc_address(1000, 5, 8, -50).unwrap();
        assert_eq!(addr_neg, 1000 + 5 * 8 - 50);

        assert!(cpu.calculate_cisc_address(100, 20, 3, 10).is_err()); // Invalid scale
    }

    #[test]
    fn test_arm_pre_post_indexing() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_arm_reg(1, 100).unwrap(); // R1 = 100

        // arm_post_indexed: return R1 (100), then set R1 = 100 + 20
        let addr_post = cpu.arm_post_indexed(1, 20).unwrap();
        assert_eq!(addr_post, 100);
        assert_eq!(cpu.get_arm_reg(1).unwrap(), 120);

        // arm_pre_indexed: set R1 = 120 - 30, return updated R1 (90)
        let addr_pre = cpu.arm_pre_indexed(1, -30).unwrap();
        assert_eq!(addr_pre, 90);
        assert_eq!(cpu.get_arm_reg(1).unwrap(), 90);
    }

    #[test]
    fn test_shifts_rotations() {
        let mut cpu = SovereignVirtualCPU::new();
        let shl = cpu.exec_shift_rotate("SHL", 0xF, 4).unwrap();
        assert_eq!(shl, 0xF0);

        let ror = cpu.exec_shift_rotate("ROR", 0xFF00000000000000u64, 8).unwrap();
        assert_eq!(ror, 0x00FF000000000000u64);
    }

    #[test]
    fn test_string_block_transfers() {
        let mut cpu = SovereignVirtualCPU::new();
        // Setup data at RSI (1000) to move to RDI (2000)
        for i in 0..5 {
            cpu.write_mem_u8(1000 + i, (i + 10) as u8).unwrap();
        }
        cpu.registers.rsi = 1000;
        cpu.registers.rdi = 2000;
        cpu.registers.rcx = 5;

        cpu.exec_string_op("REP MOVS").unwrap();

        for i in 0..5 {
            assert_eq!(cpu.read_mem_u8(2000 + i).unwrap(), (i + 10) as u8);
        }
        assert_eq!(cpu.registers.rcx, 0);
    }

    #[test]
    fn test_arm_ldm_stm() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.set_arm_reg(0, 100).unwrap();
        cpu.set_arm_reg(1, 200).unwrap();
        cpu.set_arm_reg(2, 300).unwrap();

        cpu.set_arm_reg(10, 4000).unwrap(); // Base reg R10

        // STM R10, {R0, R1, R2}
        cpu.exec_stm(10, 0b111).unwrap();

        // Target address should have advanced
        assert_eq!(cpu.get_arm_reg(10).unwrap(), 4012);

        // Clear regs
        cpu.set_arm_reg(0, 0).unwrap();
        cpu.set_arm_reg(1, 0).unwrap();
        cpu.set_arm_reg(2, 0).unwrap();

        // Restore R10 to 4000
        cpu.set_arm_reg(10, 4000).unwrap();

        // LDM R10, {R0, R1, R2}
        cpu.exec_ldm(10, 0b111).unwrap();

        assert_eq!(cpu.get_arm_reg(0).unwrap(), 100);
        assert_eq!(cpu.get_arm_reg(1).unwrap(), 200);
        assert_eq!(cpu.get_arm_reg(2).unwrap(), 300);
    }

    #[test]
    fn test_traps_and_barriers() {
        let mut cpu = SovereignVirtualCPU::new();
        cpu.exec_barrier("MFENCE");

        // Write some data in memory
        cpu.write_mem_u8(2000, 10).unwrap();
        cpu.write_mem_u8(2001, 20).unwrap();

        cpu.registers.rax = 2000;
        cpu.registers.rcx = 2;

        cpu.transition_ring(CpuRing::Ring3).unwrap(); // user mode
        assert_eq!(cpu.ring, CpuRing::Ring3);

        let checksum = cpu.trigger_trap(1).unwrap(); // sys_write
        assert_eq!(checksum, 30);
        assert_eq!(cpu.ring, CpuRing::Ring3); // restored back to user mode
    }
}
