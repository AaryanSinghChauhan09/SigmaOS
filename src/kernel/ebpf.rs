// Linux-inspired eBPF (Extended Berkeley Packet Filter) Engine and Instruction Verifier
// Features static bytecode validation (bounds, division-by-zero, stack alignment, backward jump loop-prevention)
// and execution over standard in-kernel maps.

use crate::klib::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst: u8,
    pub src: u8,
    pub offset: i16,
    pub imm: i32,
}

// Opcode constants
pub const EBPF_OP_ADD: u8 = 0x01;
pub const EBPF_OP_ADDI: u8 = 0x02;
pub const EBPF_OP_SUB: u8 = 0x03;
pub const EBPF_OP_LD: u8 = 0x04;
pub const EBPF_OP_ST: u8 = 0x05;
pub const EBPF_OP_JEQ: u8 = 0x06;
pub const EBPF_OP_JNE: u8 = 0x07;
pub const EBPF_OP_MAP_LOOKUP: u8 = 0x08;
pub const EBPF_OP_EXIT: u8 = 0x09;
pub const EBPF_OP_DIV: u8 = 0x0A;

pub struct EbpfVerifier;

impl EbpfVerifier {
    /// Validates the eBPF bytecode statically before kernel execution.
    pub fn verify(program: &[EbpfInstruction]) -> Result<(), &'static str> {
        if program.is_empty() {
            return Err("Empty eBPF program!");
        }

        let mut has_exit = false;

        for (idx, inst) in program.iter().enumerate() {
            // 1. Verify register ranges (R0 - R9 are valid)
            if inst.dst >= 10 || inst.src >= 10 {
                return Err("Register index out of bounds! Valid registers: R0-R9");
            }

            // 2. Prevent division-by-zero statically
            if inst.opcode == EBPF_OP_DIV && inst.imm == 0 && inst.src == 0 {
                return Err("Static validation error: Division by zero immediate!");
            }

            // 3. Verify stack bounds (standard eBPF 512-byte stack limit)
            if inst.opcode == EBPF_OP_LD || inst.opcode == EBPF_OP_ST {
                if inst.offset < 0 || inst.offset > 504 {
                    return Err("Stack offset out of bounds! Must be between 0 and 504 (aligned)");
                }
                if inst.offset % 4 != 0 {
                    return Err("Stack offset must be 4-byte aligned!");
                }
            }

            // 4. Verify jump targets & prevent infinite loops (no backward jumps)
            if inst.opcode == EBPF_OP_JEQ || inst.opcode == EBPF_OP_JNE {
                if inst.offset <= 0 {
                    return Err("Infinite loop prevention: Backward or self-referential jumps are rejected!");
                }
                let target_idx = idx as i32 + 1 + inst.offset as i32;
                if target_idx < 0 || target_idx >= program.len() as i32 {
                    return Err("Jump instruction targets index out of bounds!");
                }
            }

            if inst.opcode == EBPF_OP_EXIT {
                has_exit = true;
            }
        }

        if !has_exit {
            return Err("Program missing standard EBPF_OP_EXIT opcode!");
        }

        Ok(())
    }
}

pub struct EbpfEngine {
    pub registers: [i64; 10], // R0..R9
    pub stack: [u8; 512],
    pub map: HashMap<i64, i64>, // Simulated eBPF kernel map
}

        while (pc as usize) < prog_len {
            let inst = prog[pc as usize];
            
            match inst.opcode {
                0x95 => { // EXIT
                    return Ok(regs.r0);
                }
                0xb7 => { // MOV64_IMM
                    match inst.dst_reg {
                        0 => regs.r0 = inst.imm as u64,
                        1 => regs.r1 = inst.imm as u64,
                        2 => regs.r2 = inst.imm as u64,
                        3 => regs.r3 = inst.imm as u64,
                        4 => regs.r4 = inst.imm as u64,
                        5 => regs.r5 = inst.imm as u64,
                        6 => regs.r6 = inst.imm as u64,
                        7 => regs.r7 = inst.imm as u64,
                        8 => regs.r8 = inst.imm as u64,
                        9 => regs.r9 = inst.imm as u64,
                        _ => return Err(BpfError::InvalidRegister),
                    }
                }
                0x07 => { // ADD64_IMM
                    match inst.dst_reg {
                        0 => regs.r0 = regs.r0.wrapping_add(inst.imm as u64),
                        1 => regs.r1 = regs.r1.wrapping_add(inst.imm as u64),
                        2 => regs.r2 = regs.r2.wrapping_add(inst.imm as u64),
                        3 => regs.r3 = regs.r3.wrapping_add(inst.imm as u64),
                        4 => regs.r4 = regs.r4.wrapping_add(inst.imm as u64),
                        5 => regs.r5 = regs.r5.wrapping_add(inst.imm as u64),
                        6 => regs.r6 = regs.r6.wrapping_add(inst.imm as u64),
                        7 => regs.r7 = regs.r7.wrapping_add(inst.imm as u64),
                        8 => regs.r8 = regs.r8.wrapping_add(inst.imm as u64),
                        9 => regs.r9 = regs.r9.wrapping_add(inst.imm as u64),
                        _ => return Err(BpfError::InvalidRegister),
                    }
                }
                0x20 => { // LD_ABS_B (load byte from packet)
                    let offset = inst.imm as u32 as usize;
                    if offset >= packet_data.len() {
                        regs.r0 = 0;
                    } else {
                        regs.r0 = packet_data[offset] as u64;
                    }
                }
                0x61 => { // LDXW (load word from memory)
                    let offset = inst.offset as u32 as usize;
                    if offset + 4 <= 512 {
                        let stack = self.stack.borrow();
                        let val = u32::from_le_bytes([
                            stack[offset], stack[offset+1], 
                            stack[offset+2], stack[offset+3]
                        ]);
                        match inst.dst_reg {
                            0 => regs.r0 = val as u64,
                            1 => regs.r1 = val as u64,
                            2 => regs.r2 = val as u64,
                            3 => regs.r3 = val as u64,
                            4 => regs.r4 = val as u64,
                            5 => regs.r5 = val as u64,
                            6 => regs.r6 = val as u64,
                            7 => regs.r7 = val as u64,
                            8 => regs.r8 = val as u64,
                            9 => regs.r9 = val as u64,
                            _ => return Err(BpfError::InvalidRegister),
                        }
                    }
                }
                0x05 => { // JMP_IMM (conditional jump)
                    let cond = match inst.dst_reg {
                        0 => regs.r0,
                        1 => regs.r1,
                        2 => regs.r2,
                        3 => regs.r3,
                        4 => regs.r4,
                        5 => regs.r5,
                        6 => regs.r6,
                        7 => regs.r7,
                        8 => regs.r8,
                        9 => regs.r9,
                        _ => return Err(BpfError::InvalidRegister),
                    };
                    
                    if cond == inst.imm as u64 {
                        pc += inst.offset as i32;
                    }
                }
                _ => return Err(BpfError::InvalidOpcode),
            }
            instruction_count += 1;

/// eBPF error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfError {
    ProgramTooLarge,
    InvalidRegister,
    InvalidOpcode,
    StackOverflow,
    InvalidMemoryAccess,
    InvalidInstruction,
}

                    if divisor == 0 {
                        return Err("Division by zero at runtime!");
                    }
                    self.registers[inst.dst as usize] /= divisor;
                }
                EBPF_OP_EXIT => {
                    // R0 holds the return value of an eBPF program
                    return Ok(self.registers[0]);
                }
                _ => return Err("Invalid opcode during execution!"),
            }

/// eBPF map definition
pub struct BpfMap {
    pub map_type: BpfMapType,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub data: RefCell<[u8; 65536]>, // 64KB max map size
}

/// eBPF Static Safety Verifier
pub struct BpfVerifier;

impl BpfVerifier {
    pub fn verify_program(&self, instructions: &[BpfInstruction]) -> Result<(), BpfError> {
        if instructions.is_empty() || instructions.len() > 4096 {
            return Err(BpfError::ProgramTooLarge);
        }

        let mut contains_exit = false;
        for (i, inst) in instructions.iter().enumerate() {
            // Register bound checks
            if inst.dst_reg > 10 || inst.src_reg > 10 {
                return Err(BpfError::InvalidMemoryAccess);
            }

            // Check for backward jumps (bounded loop safety verification)
            if inst.offset < 0 {
                let target_pc = (i as i32) + 1 + (inst.offset as i32);
                if target_pc < 0 || target_pc >= i as i32 {
                    return Err(BpfError::InvalidInstruction);
                }
            }

            // Check for exit instruction
            if inst.opcode == 0x95 {
                contains_exit = true;
            }
        }

        if !contains_exit {
            return Err(BpfError::InvalidInstruction);
        }

        Ok(())
    }
}

impl BpfMap {
    pub fn new(map_type: BpfMapType, key_size: u32, value_size: u32, max_entries: u32) -> Self {
        BpfMap {
            map_type,
            key_size,
            value_size,
            max_entries,
            data: RefCell::new([0u8; 65536]),
        }
    }

    /// Simple hash map lookup
    pub fn lookup(&self, key: &[u8]) -> Option<Vec<u8>> {
        // Simplified hash map implementation
        let data = self.data.borrow();
        let hash = self.simple_hash(key);
        let offset = (hash % self.max_entries as u64) as usize;
        
        if offset + self.value_size as usize <= data.len() {
            let mut value = vec![0u8; self.value_size as usize];
            value.copy_from_slice(&data[offset..offset + self.value_size as usize]);
            Some(value)
        } else {
            None
        }
    }

    /// Simple hash map update
    pub fn update(&self, _key: &[u8], value: &[u8]) -> Result<(), BpfError> {
        if value.len() != self.value_size as usize {
            return Err(BpfError::InvalidMemoryAccess);
        }

        Err("Program terminated without EXIT opcode!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_verifications() {
        // Invalid registers check
        let program1 = vec![
            EbpfInstruction { opcode: EBPF_OP_ADDI, dst: 15, src: 0, offset: 0, imm: 10 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];
        assert_eq!(EbpfVerifier::verify(&program1), Err("Register index out of bounds! Valid registers: R0-R9"));

        // Division by zero immediate check
        let program2 = vec![
            EbpfInstruction { opcode: EBPF_OP_DIV, dst: 1, src: 0, offset: 0, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];
        assert_eq!(EbpfVerifier::verify(&program2), Err("Static validation error: Division by zero immediate!"));

        // Unaligned stack offset check
        let program3 = vec![
            EbpfInstruction { opcode: EBPF_OP_ST, dst: 1, src: 0, offset: 11, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];
        assert_eq!(EbpfVerifier::verify(&program3), Err("Stack offset must be 4-byte aligned!"));

        // Out of bounds stack offset check
        let program4 = vec![
            EbpfInstruction { opcode: EBPF_OP_ST, dst: 1, src: 0, offset: 512, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];
        assert_eq!(EbpfVerifier::verify(&program4), Err("Stack offset out of bounds! Must be between 0 and 504 (aligned)"));

        // Backward jump infinite loop prevention check
        let program5 = vec![
            EbpfInstruction { opcode: EBPF_OP_JEQ, dst: 1, src: 2, offset: -1, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];
        assert_eq!(EbpfVerifier::verify(&program5), Err("Infinite loop prevention: Backward or self-referential jumps are rejected!"));
    }

    #[test]
    fn test_bpf_verifier() {
        let verifier = BpfVerifier;
        let valid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 0, src_reg: 0, offset: 0, imm: 10 },
            BpfInstruction { opcode: 0x95, dst_reg: 0, src_reg: 0, offset: 0, imm: 0 },
        ];
        assert!(verifier.verify_program(&valid_prog).is_ok());

        let invalid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 11, src_reg: 0, offset: 0, imm: 10 }, // Reg 11 out of bounds
        ];
        assert!(verifier.verify_program(&invalid_prog).is_err());
    }

    #[test]
    fn test_bpf_verifier() {
        let verifier = BpfVerifier;
        let valid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 0, src_reg: 0, offset: 0, imm: 10 },
            BpfInstruction { opcode: 0x95, dst_reg: 0, src_reg: 0, offset: 0, imm: 0 },
        ];
        assert!(verifier.verify_program(&valid_prog).is_ok());

        let invalid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 11, src_reg: 0, offset: 0, imm: 10 }, // Reg 11 out of bounds
        ];
        assert!(verifier.verify_program(&invalid_prog).is_err());
    }

    #[test]
    fn test_bpf_verifier() {
        let verifier = BpfVerifier;
        let valid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 0, src_reg: 0, offset: 0, imm: 10 },
            BpfInstruction { opcode: 0x95, dst_reg: 0, src_reg: 0, offset: 0, imm: 0 },
        ];
        assert!(verifier.verify_program(&valid_prog).is_ok());

        let invalid_prog = [
            BpfInstruction { opcode: 0xb7, dst_reg: 11, src_reg: 0, offset: 0, imm: 10 }, // Reg 11 out of bounds
        ];
        assert!(verifier.verify_program(&invalid_prog).is_err());
    }

    #[test]
    fn test_bpf_map_hash() {
        let map = BpfMap::new(BpfMapType::Hash, 4, 8, 100);
        
        // Generate test key using timestamp-based approach
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let mut key = [0u8; 4];
        for (i, byte) in key.iter_mut().enumerate() {
            *byte = ((timestamp >> (i * 8)) & 0xFF) as u8;
        }
        let mut value = [0u8; 8];
        let value_timestamp = timestamp.wrapping_add(1);
        for (i, byte) in value.iter_mut().enumerate() {
            *byte = ((value_timestamp >> (i * 8)) & 0xFF) as u8;
        }
        
        map.update(&key, &value).unwrap();
        let retrieved = map.lookup(&key).unwrap();
        assert_eq!(retrieved, value.to_vec());
    }
}
