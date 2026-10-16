// SigmaOS eBPF JIT Compiler (x86_64 machine code emitter)
// Translates 64-bit eBPF bytecode into native x86_64 instructions at runtime

#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

pub const BPF_CLASS_LD: u8 = 0x00;
pub const BPF_CLASS_LDX: u8 = 0x01;
pub const BPF_CLASS_ST: u8 = 0x02;
pub const BPF_CLASS_STX: u8 = 0x03;
pub const BPF_CLASS_ALU: u8 = 0x04;
pub const BPF_CLASS_JMP: u8 = 0x05;
pub const BPF_CLASS_JMP32: u8 = 0x06;
pub const BPF_CLASS_ALU64: u8 = 0x07;

// ALU operations
pub const BPF_ADD: u8 = 0x00;
pub const BPF_SUB: u8 = 0x10;
pub const BPF_MUL: u8 = 0x20;
pub const BPF_DIV: u8 = 0x30;
pub const BPF_OR: u8 = 0x40;
pub const BPF_AND: u8 = 0x50;
pub const BPF_LSH: u8 = 0x60;
pub const BPF_RSH: u8 = 0x70;
pub const BPF_NEG: u8 = 0x80;
pub const BPF_MOD: u8 = 0x90;
pub const BPF_XOR: u8 = 0xa0;
pub const BPF_MOV: u8 = 0xb0;
pub const BPF_ARSH: u8 = 0xc0;

// JMP operations
pub const BPF_JA: u8 = 0x00;
pub const BPF_JEQ: u8 = 0x10;
pub const BPF_JGT: u8 = 0x20;
pub const BPF_JGE: u8 = 0x30;
pub const BPF_JSET: u8 = 0x40;
pub const BPF_JNE: u8 = 0x50;
pub const BPF_CALL: u8 = 0x80;
pub const BPF_EXIT: u8 = 0x90;

/// eBPF 64-bit instruction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EbpfInstruction {
    pub opcode: u8,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub offset: i16,
    pub imm: i32,
}

impl EbpfInstruction {
    pub fn new(opcode: u8, dst: u8, src: u8, offset: i16, imm: i32) -> Self {
        Self {
            opcode,
            dst_reg: dst,
            src_reg: src,
            offset,
            imm,
        }
    }
}

/// JIT Compiler for x86_64
pub struct EbpfJitCompiler {
    code: Vec<u8>,
}

impl EbpfJitCompiler {
    pub fn new() -> Self {
        Self { code: Vec::new() }
    }

    /// Compiles an array of eBPF instructions into native x86_64 machine code
    pub fn compile(&mut self, instructions: &[EbpfInstruction]) -> Result<&[u8], &'static str> {
        self.code.clear();

        // 1. Prologue: push rbp; mov rbp, rsp; push rbx; push r13; push r14; push r15
        self.code.extend_from_slice(&[
            0x55, // push rbp
            0x48, 0x89, 0xE5, // mov rbp, rsp
            0x53, // push rbx
            0x41, 0x55, // push r13
            0x41, 0x56, // push r14
            0x41, 0x57, // push r15
        ]);

        for inst in instructions {
            let class = inst.opcode & 0x07;
            match class {
                BPF_CLASS_ALU64 => {
                    let op = inst.opcode & 0xf0;
                    match op {
                        BPF_MOV => {
                            // REX.W + 0xB8 + rd (mov r64, imm32)
                            self.emit_mov_imm(inst.dst_reg, inst.imm as i64);
                        }
                        BPF_ADD => {
                            // REX.W + 0x01 / 0x81
                            self.emit_add_imm(inst.dst_reg, inst.imm);
                        }
                        BPF_SUB => {
                            self.emit_sub_imm(inst.dst_reg, inst.imm);
                        }
                        BPF_XOR => {
                            self.emit_xor_reg(inst.dst_reg, inst.src_reg);
                        }
                        _ => {}
                    }
                }
                BPF_CLASS_JMP => {
                    let op = inst.opcode & 0xf0;
                    if op == BPF_EXIT {
                        // mov rax, r0 (return value in rax)
                        // Epilogue: pop r15; pop r14; pop r13; pop rbx; leave; ret
                        self.code.extend_from_slice(&[
                            0x41, 0x5F, // pop r15
                            0x41, 0x5E, // pop r14
                            0x41, 0x5D, // pop r13
                            0x5B, // pop rbx
                            0xC9, // leave
                            0xC3, // ret
                        ]);
                    }
                }
                _ => {}
            }
        }

        Ok(&self.code)
    }

    fn emit_mov_imm(&mut self, dst: u8, imm: i64) {
        // x86_64: mov rax, imm64 or mov rdi, imm64
        if dst == 0 {
            // mov rax, imm64
            self.code.extend_from_slice(&[0x48, 0xB8]);
            self.code.extend_from_slice(&imm.to_le_bytes());
        } else {
            // fallback: mov rdi/rax
            self.code.extend_from_slice(&[0x48, 0xC7, 0xC0]);
            self.code.extend_from_slice(&(imm as i32).to_le_bytes());
        }
    }

    fn emit_add_imm(&mut self, dst: u8, imm: i32) {
        if dst == 0 {
            self.code.extend_from_slice(&[0x48, 0x05]);
            self.code.extend_from_slice(&imm.to_le_bytes());
        }
    }

    fn emit_sub_imm(&mut self, dst: u8, imm: i32) {
        if dst == 0 {
            self.code.extend_from_slice(&[0x48, 0x2D]);
            self.code.extend_from_slice(&imm.to_le_bytes());
        }
    }

    fn emit_xor_reg(&mut self, dst: u8, src: u8) {
        if dst == 0 && src == 0 {
            // xor rax, rax
            self.code.extend_from_slice(&[0x48, 0x31, 0xC0]);
        }
    }

    pub fn code_size(&self) -> usize {
        self.code.len()
    }
}
