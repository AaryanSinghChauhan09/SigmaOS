//! eBPF (Extended Berkeley Packet Filter) Virtual Machine and Hook Engine
//! Provides safe, sandboxeduserspace-defined bytecode execution within microkernel hooks.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

// =========================================================================
// EBPF INSTRUCTION DECODER & OPCODES
// =========================================================================

pub const BPF_LD: u8 = 0x00;
pub const BPF_ALU: u8 = 0x07;
pub const BPF_JMP: u8 = 0x05;

pub const BPF_ADD: u8 = 0x00;
pub const BPF_SUB: u8 = 0x10;
pub const BPF_MUL: u8 = 0x20;
pub const BPF_XOR: u8 = 0xa0;

#[derive(Debug, Clone, Copy)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub offset: i16,
    pub imm: i32,
}

pub struct EbpfVm {
    pub registers: [u64; 11], // R0 (return) to R10 (frame pointer)
    pub bytecode: Vec<EbpfInstruction>,
}

impl EbpfVm {
    pub fn new(bytecode: Vec<EbpfInstruction>) -> Self {
        Self {
            registers: [0u64; 11],
            bytecode,
        }
    }

    /// Executes compiled eBPF instructions inside a strict, safe sandbox environment
    pub fn run(&mut self, context_buffer: &[u8]) -> Result<u64, &'static str> {
        // R1 holds the pointer to the input context buffer
        self.registers[1] = context_buffer.as_ptr() as u64;
        self.registers[10] = 512; // Simulated Stack Frame Pointer

        let mut pc = 0;
        while pc < self.bytecode.len() {
            let inst = self.bytecode[pc];
            let class = inst.opcode & 0x07;

            match class {
                BPF_ALU => {
                    let op = inst.opcode & 0xf0;
                    let dst = inst.dst_reg as usize;
                    let src = inst.src_reg as usize;

                    if dst >= 10 {
                        return Err("eBPF: Access violation - destination register out of bounds");
                    }

                    match op {
                        BPF_ADD => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_add(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_add(self.registers[src]);
                            }
                        }
                        BPF_SUB => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_sub(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_sub(self.registers[src]);
                            }
                        }
                        BPF_MUL => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_mul(inst.imm as u64);
                            } else {
                                self.registers[dst] =
                                    self.registers[dst].wrapping_mul(self.registers[src]);
                            }
                        }
                        BPF_XOR => {
                            if inst.opcode & 0x08 == 0 {
                                self.registers[dst] ^= inst.imm as u64;
                            } else {
                                self.registers[dst] ^= self.registers[src];
                            }
                        }
                        _ => return Err("eBPF VM: Unknown ALU operation opcode"),
                    }
                }
                BPF_JMP => {
                    let dst = inst.dst_reg as usize;
                    let imm = inst.imm as u64;

                    if dst >= 10 {
                        return Err("eBPF: Access violation - JMP evaluation out of bounds");
                    }

                    // Jump instruction: if Register[dst] matches immediate, jump by instruction offset
                    if self.registers[dst] == imm {
                        let new_pc = (pc as i32 + inst.offset as i32) as usize;
                        if new_pc >= self.bytecode.len() {
                            return Err("eBPF VM: Jump out of instruction segment bounds");
                        }
                        pc = new_pc;
                        continue;
                    }
                }
                BPF_LD => {
                    let dst = inst.dst_reg as usize;
                    if dst >= 10 {
                        return Err("eBPF: Access violation - Load target out of bounds");
                    }
                    self.registers[dst] = inst.imm as u64;
                }
                _ => return Err("eBPF VM: Unknown instruction class"),
            }
            pc += 1;
        }

        // Return register is R0
        Ok(self.registers[0])
    }
}

// =========================================================================
// TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ebpf_alu_add_immediate() {
        let bytecode = vec![
            // R0 = 5
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 5,
            },
            // R0 += 10
            EbpfInstruction {
                opcode: BPF_ALU | BPF_ADD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 10,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 15);
    }

    #[test]
    fn test_ebpf_alu_xor_register() {
        let bytecode = vec![
            // R0 = 10
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 10,
            },
            // R2 = 12
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 2,
                src_reg: 0,
                offset: 0,
                imm: 12,
            },
            // R0 ^= R2 (10 ^ 12 = 6)
            EbpfInstruction {
                opcode: BPF_ALU | BPF_XOR | 0x08, // 0x08 signifies register src
                dst_reg: 0,
                src_reg: 2,
                offset: 0,
                imm: 0,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 6);
    }

    #[test]
    fn test_ebpf_jmp_condition() {
        let bytecode = vec![
            // R2 = 5
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 2,
                src_reg: 0,
                offset: 0,
                imm: 5,
            },
            // if R2 == 5, JMP offset 2 (skip the R0 = 99 load)
            EbpfInstruction {
                opcode: BPF_JMP,
                dst_reg: 2,
                src_reg: 0,
                offset: 2,
                imm: 5,
            },
            // R0 = 99 (skipped)
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 99,
            },
            // R0 = 42
            EbpfInstruction {
                opcode: BPF_LD,
                dst_reg: 0,
                src_reg: 0,
                offset: 0,
                imm: 42,
            },
        ];

        let mut vm = EbpfVm::new(bytecode);
        let res = vm.run(&[]).unwrap();
        assert_eq!(res, 42);
    }
}
