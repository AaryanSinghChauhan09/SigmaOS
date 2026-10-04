// eBPF JIT Compiler for x86_64
// Translates verified eBPF bytecode to native x86_64 machine code

#![no_std]
extern crate alloc;

use super::instructions::{BpfInsn, BpfOpcode};
use super::instructions::{BPF_CLASS_ALU, BPF_CLASS_ALU64, BPF_CLASS_JMP, BPF_OP_EXIT};
use super::instructions::{BPF_OP_ADD, BPF_OP_DIV, BPF_OP_MOV, BPF_OP_MUL, BPF_OP_SUB};
use super::instructions::{BPF_OP_AND, BPF_OP_LSH, BPF_OP_OR, BPF_OP_RSH, BPF_OP_XOR};
use super::instructions::{BPF_OP_CALL, BPF_OP_JGE, BPF_OP_JGT, BPF_OP_JLE, BPF_OP_JLT};
use super::instructions::{BPF_OP_JA, BPF_OP_JEQ, BPF_OP_JNE, BPF_SRC_IMM, BPF_SRC_REG};
use super::verifier::VerifiedProg;
use alloc::vec::Vec;

/// x86_64 register encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum X86Register {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rbx = 3,
    Rsp = 4,
    Rbp = 5,
    Rsi = 6,
    Rdi = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

/// eBPF R0-R10 to x86_64 register mapping (Linux-compatible)
pub const X86_REG_MAP: [X86Register; 11] = [
    X86Register::Rax, // R0 (return value)
    X86Register::Rdi, // R1 (arg1)
    X86Register::Rsi, // R2 (arg2)
    X86Register::Rdx, // R3 (arg3)
    X86Register::Rcx, // R4 (arg4)
    X86Register::R8,  // R5 (arg5)
    X86Register::Rbx, // R6 (callee-saved)
    X86Register::R13, // R7 (callee-saved)
    X86Register::R14, // R8 (callee-saved)
    X86Register::R15, // R9 (callee-saved)
    X86Register::Rbp, // R10 (frame pointer, read-only)
];

/// JIT compilation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JitError {
    UnsupportedInstruction(u8),
    ImmediateTooLarge,
    PatchFailed,
}

/// Compiled native code
#[derive(Debug, Clone)]
pub struct JitCode {
    pub code: Vec<u8>,
    pub entry_offset: usize,
}

impl JitCode {
    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.code
    }
}

/// eBPF to x86_64 JIT compiler
pub struct JitCompiler {
    code_buffer: Vec<u8>,
    pc_map: Vec<usize>, // Maps eBPF PC to x86 offset
}

impl JitCompiler {
    pub fn new() -> Self {
        Self {
            code_buffer: Vec::new(),
            pc_map: Vec::new(),
        }
    }

    /// Compile verified eBPF program to native x86_64
    pub fn compile(&mut self, prog: &VerifiedProg) -> Result<JitCode, JitError> {
        self.code_buffer.clear();
        self.pc_map = alloc::vec![0; prog.insns.len()];

        // Emit prologue
        self.emit_prologue(prog.stack_depth);

        // Compile each instruction
        for (pc, insn) in prog.insns.iter().enumerate() {
            self.pc_map[pc] = self.code_buffer.len();
            self.compile_instruction(insn)?;
        }

        // Emit epilogue (if not already emitted by EXIT)
        // self.emit_epilogue();

        Ok(JitCode {
            code: self.code_buffer.clone(),
            entry_offset: 0,
        })
    }

    /// Emit function prologue
    fn emit_prologue(&mut self, stack_depth: u32) {
        // push rbp
        self.emit_u8(0x55);

        // mov rbp, rsp
        self.emit_u8(0x48);
        self.emit_u8(0x89);
        self.emit_u8(0xe5);

        // push callee-saved registers
        // push rbx
        self.emit_u8(0x53);
        // push r13
        self.emit_u8(0x41);
        self.emit_u8(0x55);
        // push r14
        self.emit_u8(0x41);
        self.emit_u8(0x56);
        // push r15
        self.emit_u8(0x41);
        self.emit_u8(0x57);

        // sub rsp, stack_depth (align to 16 bytes)
        if stack_depth > 0 {
            let aligned_depth = ((stack_depth + 15) / 16) * 16;
            // sub rsp, imm32
            self.emit_u8(0x48);
            self.emit_u8(0x81);
            self.emit_u8(0xec);
            self.emit_u32(aligned_depth);
        }
    }

    /// Emit function epilogue
    fn emit_epilogue(&mut self) {
        // mov rsp, rbp (or just add rsp, stack_depth if we tracked it)
        self.emit_u8(0x48);
        self.emit_u8(0x89);
        self.emit_u8(0xec);

        // pop r15
        self.emit_u8(0x41);
        self.emit_u8(0x5f);
        // pop r14
        self.emit_u8(0x41);
        self.emit_u8(0x5e);
        // pop r13
        self.emit_u8(0x41);
        self.emit_u8(0x5d);
        // pop rbx
        self.emit_u8(0x5b);

        // pop rbp
        self.emit_u8(0x5d);

        // ret
        self.emit_u8(0xc3);
    }

    /// Compile a single eBPF instruction
    fn compile_instruction(&mut self, insn: &BpfInsn) -> Result<(), JitError> {
        let class = insn.get_class();
        let op = insn.get_op();
        let src = insn.get_src();

        match class {
            BPF_CLASS_ALU64 | BPF_CLASS_ALU => {
                let dst_reg = self.map_reg(insn.dst_reg);
                let is_64 = class == BPF_CLASS_ALU64;

                match op {
                    BPF_OP_MOV => {
                        if src == BPF_SRC_IMM {
                            self.emit_mov_reg_imm(dst_reg, insn.imm as i64, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_mov_reg_reg(dst_reg, src_reg, is_64);
                        }
                    }
                    BPF_OP_ADD => {
                        if src == BPF_SRC_IMM {
                            self.emit_alu_reg_imm(dst_reg, insn.imm, 0x81, 0, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_alu_reg_reg(dst_reg, src_reg, 0x01, is_64);
                        }
                    }
                    BPF_OP_SUB => {
                        if src == BPF_SRC_IMM {
                            self.emit_alu_reg_imm(dst_reg, insn.imm, 0x81, 5, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_alu_reg_reg(dst_reg, src_reg, 0x29, is_64);
                        }
                    }
                    BPF_OP_AND => {
                        if src == BPF_SRC_IMM {
                            self.emit_alu_reg_imm(dst_reg, insn.imm, 0x81, 4, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_alu_reg_reg(dst_reg, src_reg, 0x21, is_64);
                        }
                    }
                    BPF_OP_OR => {
                        if src == BPF_SRC_IMM {
                            self.emit_alu_reg_imm(dst_reg, insn.imm, 0x81, 1, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_alu_reg_reg(dst_reg, src_reg, 0x09, is_64);
                        }
                    }
                    BPF_OP_XOR => {
                        if src == BPF_SRC_IMM {
                            self.emit_alu_reg_imm(dst_reg, insn.imm, 0x81, 6, is_64);
                        } else {
                            let src_reg = self.map_reg(insn.src_reg);
                            self.emit_alu_reg_reg(dst_reg, src_reg, 0x31, is_64);
                        }
                    }
                    _ => return Err(JitError::UnsupportedInstruction(op)),
                }
            }
            BPF_CLASS_JMP => {
                match op {
                    BPF_OP_EXIT => {
                        self.emit_epilogue();
                    }
                    BPF_OP_JA => {
                        // Unconditional jump - emit placeholder, will patch later
                        self.emit_u8(0xe9); // jmp rel32
                        self.emit_u32(0); // Placeholder for offset
                    }
                    BPF_OP_CALL => {
                        // call helper function
                        // For now, just emit a nop
                        self.emit_u8(0x90); // nop
                    }
                    _ => {
                        // Conditional jumps - simplified, just emit nop for now
                        self.emit_u8(0x90); // nop
                    }
                }
            }
            _ => return Err(JitError::UnsupportedInstruction(class)),
        }

        Ok(())
    }

    /// Map eBPF register to x86_64 register
    fn map_reg(&self, ebpf_reg: u8) -> X86Register {
        if ebpf_reg < 11 {
            X86_REG_MAP[ebpf_reg as usize]
        } else {
            X86Register::Rax // Fallback
        }
    }

    /// Emit MOV reg, imm
    fn emit_mov_reg_imm(&mut self, dst: X86Register, imm: i64, is_64: bool) {
        if is_64 {
            // REX.W prefix
            self.emit_u8(0x48 | if (dst as u8) >= 8 { 1 } else { 0 });
        } else if (dst as u8) >= 8 {
            self.emit_u8(0x41);
        }

        // mov reg, imm32 (sign-extended to 64 bits)
        self.emit_u8(0xc7);
        self.emit_u8(0xc0 | ((dst as u8) & 7));
        self.emit_u32(imm as i32 as u32);
    }

    /// Emit MOV dst, src
    fn emit_mov_reg_reg(&mut self, dst: X86Register, src: X86Register, is_64: bool) {
        if is_64 {
            // REX.W + REX.R + REX.B
            let rex = 0x48
                | (if (dst as u8) >= 8 { 1 } else { 0 })
                | (if (src as u8) >= 8 { 4 } else { 0 });
            self.emit_u8(rex);
        }

        // mov dst, src
        self.emit_u8(0x89);
        let modrm = 0xc0 | (((src as u8) & 7) << 3) | ((dst as u8) & 7);
        self.emit_u8(modrm);
    }

    /// Emit ALU operation: dst = dst op imm
    fn emit_alu_reg_imm(
        &mut self,
        dst: X86Register,
        imm: i32,
        opcode: u8,
        reg_ext: u8,
        is_64: bool,
    ) {
        if is_64 {
            self.emit_u8(0x48 | if (dst as u8) >= 8 { 1 } else { 0 });
        }

        self.emit_u8(opcode);
        self.emit_u8(0xc0 | (reg_ext << 3) | ((dst as u8) & 7));
        self.emit_u32(imm as u32);
    }

    /// Emit ALU operation: dst = dst op src
    fn emit_alu_reg_reg(&mut self, dst: X86Register, src: X86Register, opcode: u8, is_64: bool) {
        if is_64 {
            let rex = 0x48
                | (if (dst as u8) >= 8 { 1 } else { 0 })
                | (if (src as u8) >= 8 { 4 } else { 0 });
            self.emit_u8(rex);
        }

        self.emit_u8(opcode);
        let modrm = 0xc0 | (((src as u8) & 7) << 3) | ((dst as u8) & 7);
        self.emit_u8(modrm);
    }

    /// Emit a single byte
    fn emit_u8(&mut self, byte: u8) {
        self.code_buffer.push(byte);
    }

    /// Emit a 32-bit immediate
    fn emit_u32(&mut self, val: u32) {
        self.code_buffer.extend_from_slice(&val.to_le_bytes());
    }
}

impl Default for JitCompiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::ebpf::verifier::BpfVerifier;

    #[test]
    fn test_jit_compile_simple() {
        let prog = vec![BpfInsn::alu64_imm(BPF_OP_MOV, 0, 42), BpfInsn::exit_insn()];

        let verifier = BpfVerifier::new(prog);
        let verified = verifier.verify().unwrap();

        let mut compiler = JitCompiler::new();
        let code = compiler.compile(&verified).unwrap();

        assert!(!code.is_empty());
        assert!(code.len() > 10); // Should have prologue + instructions + epilogue
    }

    #[test]
    fn test_jit_prologue_emitted() {
        let prog = vec![BpfInsn::exit_insn()];

        let verifier = BpfVerifier::new(prog);
        let verified = verifier.verify().unwrap();

        let mut compiler = JitCompiler::new();
        let code = compiler.compile(&verified).unwrap();

        // Check for push rbp (0x55)
        assert_eq!(code.as_bytes()[0], 0x55);
    }

    #[test]
    fn test_register_mapping() {
        let compiler = JitCompiler::new();

        assert_eq!(compiler.map_reg(0), X86Register::Rax);
        assert_eq!(compiler.map_reg(1), X86Register::Rdi);
        assert_eq!(compiler.map_reg(10), X86Register::Rbp);
    }
}
