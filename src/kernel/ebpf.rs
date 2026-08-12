// Linux-inspired eBPF (Extended Berkeley Packet Filter) Engine and Instruction Verifier
// Features static bytecode validation (bounds, division-by-zero, stack alignment, backward jump loop-prevention)
// and execution over standard in-kernel maps.

use std::collections::HashMap;

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

impl EbpfEngine {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Seed default key-value maps
        map.insert(100, 999);
        map.insert(101, 888);

        Self {
            registers: [0; 10],
            stack: [0; 512],
            map,
        }
    }

    /// Executes the verified eBPF bytecode.
    pub fn execute(&mut self, program: &[EbpfInstruction]) -> Result<i64, &'static str> {
        let mut rip = 0;
        let mut instruction_count = 0;
        let max_instructions = 1000; // Prevent unexpected runaway execution

        while rip < program.len() {
            if instruction_count >= max_instructions {
                return Err("Execution limit reached! Infinite loop protection triggered.");
            }
            instruction_count += 1;

            let inst = program[rip];
            match inst.opcode {
                EBPF_OP_ADD => {
                    self.registers[inst.dst as usize] = self.registers[inst.dst as usize]
                        .wrapping_add(self.registers[inst.src as usize]);
                }
                EBPF_OP_ADDI => {
                    self.registers[inst.dst as usize] = self.registers[inst.dst as usize]
                        .wrapping_add(inst.imm as i64);
                }
                EBPF_OP_SUB => {
                    self.registers[inst.dst as usize] = self.registers[inst.dst as usize]
                        .wrapping_sub(self.registers[inst.src as usize]);
                }
                EBPF_OP_LD => {
                    // Load 4-byte little endian value from stack
                    let offset = inst.offset as usize;
                    let val = i32::from_le_bytes([
                        self.stack[offset],
                        self.stack[offset + 1],
                        self.stack[offset + 2],
                        self.stack[offset + 3],
                    ]);
                    self.registers[inst.dst as usize] = val as i64;
                }
                EBPF_OP_ST => {
                    // Store 4-byte value to stack
                    let offset = inst.offset as usize;
                    let bytes = (self.registers[inst.dst as usize] as i32).to_le_bytes();
                    self.stack[offset..offset + 4].copy_from_slice(&bytes);
                }
                EBPF_OP_JEQ => {
                    if self.registers[inst.dst as usize] == self.registers[inst.src as usize] {
                        rip += inst.offset as usize;
                    }
                }
                EBPF_OP_JNE => {
                    if self.registers[inst.dst as usize] != self.registers[inst.src as usize] {
                        rip += inst.offset as usize;
                    }
                }
                EBPF_OP_MAP_LOOKUP => {
                    let key = self.registers[inst.src as usize];
                    if let Some(&val) = self.map.get(&key) {
                        self.registers[inst.dst as usize] = val;
                    } else {
                        self.registers[inst.dst as usize] = 0; // Null equivalent
                    }
                }
                EBPF_OP_DIV => {
                    let divisor = if inst.src == 0 {
                        inst.imm as i64
                    } else {
                        self.registers[inst.src as usize]
                    };

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

            rip += 1;
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
    fn test_ebpf_execution_success() {
        // Valid program:
        // R1 = 5
        // R2 = 10
        // R0 = R1 + R2
        // EXIT
        let program = vec![
            EbpfInstruction { opcode: EBPF_OP_ADDI, dst: 1, src: 0, offset: 0, imm: 5 },
            EbpfInstruction { opcode: EBPF_OP_ADDI, dst: 2, src: 0, offset: 0, imm: 10 },
            EbpfInstruction { opcode: EBPF_OP_ADD, dst: 0, src: 1, offset: 0, imm: 0 }, // R0 = R0 + R1
            EbpfInstruction { opcode: EBPF_OP_ADD, dst: 0, src: 2, offset: 0, imm: 0 }, // R0 = R0 + R2
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];

        assert!(EbpfVerifier::verify(&program).is_ok());

        let mut engine = EbpfEngine::new();
        let res = engine.execute(&program).unwrap();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_ebpf_stack_and_maps() {
        // R1 = 1234
        // Stack[8] = R1
        // R2 = Stack[8]
        // R0 = R2
        // R3 = 101
        // R4 = Map[R3] (key 101 -> value 888)
        // R0 = R0 + R4
        // EXIT
        let program = vec![
            EbpfInstruction { opcode: EBPF_OP_ADDI, dst: 1, src: 0, offset: 0, imm: 1234 },
            EbpfInstruction { opcode: EBPF_OP_ST, dst: 1, src: 0, offset: 8, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_LD, dst: 2, src: 0, offset: 8, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_ADD, dst: 0, src: 2, offset: 0, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_ADDI, dst: 3, src: 0, offset: 0, imm: 101 },
            EbpfInstruction { opcode: EBPF_OP_MAP_LOOKUP, dst: 4, src: 3, offset: 0, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_ADD, dst: 0, src: 4, offset: 0, imm: 0 },
            EbpfInstruction { opcode: EBPF_OP_EXIT, dst: 0, src: 0, offset: 0, imm: 0 },
        ];

        assert!(EbpfVerifier::verify(&program).is_ok());

        let mut engine = EbpfEngine::new();
        let res = engine.execute(&program).unwrap();
        assert_eq!(res, 1234 + 888);
    }
}
