// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// kernel/core/sigma_ebpf.rs — eBPF-equivalent bytecode filter (no_std)
// Language: Rust #![no_std] — OOP via EbpfProg + Verifier

#![no_std]

pub const MAX_INSNS:   usize = 4096;
pub const MAX_MAPS:    usize = 16;
pub const REG_COUNT:   usize = 11;
pub const STACK_SIZE:  usize = 512;

// ── Instruction Opcodes ────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpClass { Ld=0,Ldx=1,St=2,Stx=3,Alu=4,Jmp=5,Reg=6,Alu64=7 }

// ALU ops
pub const ALU_ADD: u8  = 0x00;
pub const ALU_SUB: u8  = 0x10;
pub const ALU_MUL: u8  = 0x20;
pub const ALU_DIV: u8  = 0x30;
pub const ALU_AND: u8  = 0x50;
pub const ALU_OR:  u8  = 0x40;
pub const ALU_XOR: u8  = 0xA0;
pub const ALU_MOV: u8  = 0xB0;
pub const ALU_RSH: u8  = 0x70;
pub const ALU_LSH: u8  = 0x60;
// JMP ops
pub const JMP_JA:  u8  = 0x00;
pub const JMP_JEQ: u8  = 0x10;
pub const JMP_JNE: u8  = 0x50;
pub const JMP_JGT: u8  = 0x20;
pub const JMP_JGE: u8  = 0x30;
pub const JMP_JLT: u8  = 0xA0;
pub const JMP_JLE: u8  = 0xB0;
pub const JMP_CALL: u8 = 0x80;
pub const JMP_EXIT: u8 = 0x90;

// ── Instruction ───────────────────────────────────────────────────────────────
#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Insn {
    pub opcode: u8,
    pub regs:   u8,  // dst:4 | src:4
    pub off:    i16,
    pub imm:    i32,
}

impl Insn {
    pub fn dst(&self) -> u8 { self.regs & 0xF }
    pub fn src(&self) -> u8 { (self.regs >> 4) & 0xF }
    pub fn class(&self) -> u8 { self.opcode & 0x7 }
}

// ── Map Types ──────────────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapType { Hash, Array, PerfEvent, RingBuf }

#[derive(Clone, Copy)]
pub struct MapDef {
    pub map_type:   MapType,
    pub key_size:   u32,
    pub value_size: u32,
    pub max_entries: u32,
}

// ── Verifier ──────────────────────────────────────────────────────────────────
#[derive(Debug, PartialEq, Eq)]
pub enum VerifyError {
    TooManyInsns,
    UnknownOpcode(u8),
    InvalidRegister(u8),
    UnboundedLoop,
    OobJump { at: usize, target: usize },
    DivByZero,
    UninitialisedRegister(u8),
}

pub struct Verifier;

impl Verifier {
    pub fn verify(prog: &[Insn]) -> Result<(), VerifyError> {
        if prog.len() > MAX_INSNS { return Err(VerifyError::TooManyInsns); }
        // Track initialised registers
        let mut init_regs: u16 = 1 << 1; // r1 = ctx pointer, always init
        for (i, insn) in prog.iter().enumerate() {
            let dst = insn.dst(); let src = insn.src();
            if dst > 10 { return Err(VerifyError::InvalidRegister(dst)); }
            if src > 10 { return Err(VerifyError::InvalidRegister(src)); }
            let class = insn.class();
            match class {
                0x7 | 0x4 => { // ALU64 or ALU
                    let src_ok = insn.opcode & 0x08 == 0 // immediate
                        || (init_regs >> src) & 1 != 0;
                    if !src_ok { return Err(VerifyError::UninitialisedRegister(src)); }
                    init_regs |= 1 << dst;
                }
                0x5 => { // JMP
                    let op = insn.opcode & 0xF0;
                    if op == (JMP_EXIT & 0xF0) { break; }
                    if op == (JMP_JA & 0xF0) || op == JMP_JEQ || op == JMP_JNE
                       || op == JMP_JGT || op == JMP_JGE || op == JMP_JLT || op == JMP_JLE
                    {
                        let target = i as i64 + 1 + insn.off as i64;
                        if target < 0 || target >= prog.len() as i64 {
                            return Err(VerifyError::OobJump { at: i, target: target as usize });
                        }
                        // Simplified: no backward jump (prevents unbounded loops)
                        if target <= i as i64 { return Err(VerifyError::UnboundedLoop); }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// ── Interpreter ───────────────────────────────────────────────────────────────
pub struct EbpfVm {
    pub regs:    [u64; REG_COUNT],
    pub stack:   [u8; STACK_SIZE],
    pub pc:      usize,
}

impl EbpfVm {
    pub const fn new() -> Self {
        Self { regs: [0u64; REG_COUNT], stack: [0u8; STACK_SIZE], pc: 0 }
    }

    pub fn run(&mut self, prog: &[Insn], ctx: u64) -> i64 {
        self.regs[1] = ctx;
        self.pc = 0;
        while self.pc < prog.len() {
            let insn = prog[self.pc];
            let dst = insn.dst() as usize;
            let src = insn.src() as usize;
            let imm = insn.imm as u64;
            let class = insn.class();
            match class {
                0x7 | 0x4 => {
                    let src_val = if insn.opcode & 0x08 == 0 { imm } else { self.regs[src] };
                    let op = insn.opcode & 0xF0;
                    self.regs[dst] = match op {
                        x if x == ALU_ADD  => self.regs[dst].wrapping_add(src_val),
                        x if x == ALU_SUB  => self.regs[dst].wrapping_sub(src_val),
                        x if x == ALU_MUL  => self.regs[dst].wrapping_mul(src_val),
                        x if x == ALU_DIV  => if src_val == 0 { 0 } else { self.regs[dst] / src_val },
                        x if x == ALU_AND  => self.regs[dst] & src_val,
                        x if x == ALU_OR   => self.regs[dst] | src_val,
                        x if x == ALU_XOR  => self.regs[dst] ^ src_val,
                        x if x == ALU_MOV  => src_val,
                        x if x == ALU_LSH  => self.regs[dst] << (src_val & 63),
                        x if x == ALU_RSH  => self.regs[dst] >> (src_val & 63),
                        _                  => self.regs[dst],
                    };
                }
                0x5 => {
                    let op = insn.opcode & 0xF0;
                    if op == (JMP_EXIT & 0xF0) { return self.regs[0] as i64; }
                    let src_val = if insn.opcode & 0x08 == 0 { imm } else { self.regs[src] };
                    let cond = match op {
                        x if x == JMP_JEQ => self.regs[dst] == src_val,
                        x if x == JMP_JNE => self.regs[dst] != src_val,
                        x if x == JMP_JGT => self.regs[dst] >  src_val,
                        x if x == JMP_JGE => self.regs[dst] >= src_val,
                        x if x == JMP_JLT => self.regs[dst] <  src_val,
                        x if x == JMP_JLE => self.regs[dst] <= src_val,
                        _                  => true, // JA
                    };
                    if cond {
                        self.pc = (self.pc as i64 + insn.off as i64) as usize;
                        continue;
                    }
                }
                _ => {}
            }
            self.pc += 1;
        }
        self.regs[0] as i64
    }
}
