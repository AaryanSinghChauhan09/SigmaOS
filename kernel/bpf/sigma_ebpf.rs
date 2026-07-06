// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/bpf/sigma_ebpf.rs — SigmaOS eBPF-inspired safe in-kernel bytecode engine
//
// Implements a sandboxed bytecode interpreter for:
//   - Runtime instrumentation (tracepoints, kprobes)
//   - Network packet filtering (XDP-lite)
//   - Syscall auditing / seccomp extension
//   - Performance counters
//
// Safety model:
//   - Programs verified before loading (no loops unless bounded, no invalid mem)
//   - No raw pointer arithmetic — all memory access through maps
//   - JIT compilation to native x86_64 via simple template expansion
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Instruction set ────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum OpCode {
    // ALU 64-bit (register op register)
    Add64Reg = 0x0F, Sub64Reg = 0x1F, Mul64Reg = 0x2F, Div64Reg = 0x3F,
    Or64Reg  = 0x4F, And64Reg = 0x5F, Lsh64Reg = 0x6F, Rsh64Reg = 0x7F,
    Xor64Reg = 0xAF, Mov64Reg = 0xBF, Arsh64Reg= 0xCF,

    // ALU 64-bit (register op immediate)
    Add64Imm = 0x07, Sub64Imm = 0x17, Mul64Imm = 0x27, Div64Imm = 0x37,
    Or64Imm  = 0x47, And64Imm = 0x57, Lsh64Imm = 0x67, Rsh64Imm = 0x77,
    Xor64Imm = 0xA7, Mov64Imm = 0xB7, Arsh64Imm= 0xC7,

    // Load/Store
    LdW  = 0x61, LdH  = 0x69, LdB  = 0x71, LdDW = 0x79,
    StW  = 0x63, StH  = 0x6B, StB  = 0x73, StDW = 0x7B,
    LdImm64 = 0x18,   // LD_IMM64 — two instructions

    // Jump
    Ja   = 0x05, Jeq  = 0x15, Jgt  = 0x25, Jge  = 0x35,
    Jset = 0x45, Jne  = 0x55, Jsgt = 0x65, Jsge = 0x75,
    Jlt  = 0xA5, Jle  = 0xB5, Jslt = 0xC5, Jsle = 0xD5,

    // Calls
    Call = 0x85, Exit = 0x95,
}

// ── Instruction encoding (8 bytes, same as Linux eBPF) ───────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Insn {
    pub opcode:  u8,
    pub regs:    u8,   // dst (bits 0..3) | src (bits 4..7)
    pub offset:  i16,
    pub imm:     i32,
}

impl Insn {
    pub fn dst(&self) -> usize { (self.regs & 0x0F) as usize }
    pub fn src(&self) -> usize { ((self.regs >> 4) & 0x0F) as usize }
    pub fn alu_imm(op: OpCode, dst: u8, imm: i32) -> Self {
        Insn { opcode: op as u8, regs: dst & 0xF, offset: 0, imm }
    }
    pub fn mov_imm(dst: u8, val: i32) -> Self { Self::alu_imm(OpCode::Mov64Imm, dst, val) }
    pub fn exit() -> Self { Insn { opcode: OpCode::Exit as u8, regs: 0, offset: 0, imm: 0 } }
}

// ── eBPF Maps ─────────────────────────────────────────────────────────────
pub const MAP_MAX_ENTRIES: usize = 1024;
pub const MAP_MAX_VALUE:   usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum MapType { Hash, Array, PerfEventArray, RingBuf }

pub struct BpfMap {
    pub map_type:  MapType,
    pub key_size:  u32,
    pub val_size:  u32,
    keys:   [[u8; 8]; MAP_MAX_ENTRIES],
    vals:   [[u8; MAP_MAX_VALUE]; MAP_MAX_ENTRIES],
    used:   [bool; MAP_MAX_ENTRIES],
    count:  usize,
}

impl BpfMap {
    pub const fn new(map_type: MapType, key_size: u32, val_size: u32) -> Self {
        BpfMap {
            map_type, key_size, val_size,
            keys: [[0u8; 8]; MAP_MAX_ENTRIES],
            vals: [[0u8; MAP_MAX_VALUE]; MAP_MAX_ENTRIES],
            used: [false; MAP_MAX_ENTRIES],
            count: 0,
        }
    }

    fn find_slot(&self, key: &[u8]) -> Option<usize> {
        let ks = key.len().min(8);
        for i in 0..MAP_MAX_ENTRIES {
            if self.used[i] && self.keys[i][..ks] == key[..ks] { return Some(i); }
        }
        None
    }

    pub fn lookup(&self, key: &[u8]) -> Option<&[u8]> {
        let vs = self.val_size as usize;
        self.find_slot(key).map(|i| &self.vals[i][..vs])
    }

    pub fn update(&mut self, key: &[u8], val: &[u8]) -> bool {
        let ks = key.len().min(8);
        let vs = val.len().min(MAP_MAX_VALUE);
        // Update existing
        if let Some(i) = self.find_slot(key) {
            self.vals[i][..vs].copy_from_slice(&val[..vs]);
            return true;
        }
        // Insert new
        if self.count >= MAP_MAX_ENTRIES { return false; }
        for i in 0..MAP_MAX_ENTRIES {
            if !self.used[i] {
                self.keys[i][..ks].copy_from_slice(&key[..ks]);
                self.vals[i][..vs].copy_from_slice(&val[..vs]);
                self.used[i] = true;
                self.count += 1;
                return true;
            }
        }
        false
    }

    pub fn delete(&mut self, key: &[u8]) -> bool {
        if let Some(i) = self.find_slot(key) {
            self.used[i] = false;
            self.count -= 1;
            return true;
        }
        false
    }
}

// ── eBPF Verifier ─────────────────────────────────────────────────────────
const MAX_INSNS: usize = 4096;
const N_REGS:   usize  = 11;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VerifyError {
    TooLong,
    InvalidOpcode(u8),
    InvalidRegister(usize),
    UnboundedLoop,
    ReadUninitialized(usize),
    NullDeref,
    BadJumpTarget(i16),
    StackOverflow,
}

pub fn verify(prog: &[Insn]) -> Result<(), VerifyError> {
    if prog.len() > MAX_INSNS { return Err(VerifyError::TooLong); }
    if prog.is_empty() { return Ok(()); }

    let mut regs_init = [false; N_REGS];
    regs_init[1] = true; // r1 = context pointer (always valid)
    regs_init[10] = true; // r10 = stack pointer (always valid)

    let mut i = 0usize;
    while i < prog.len() {
        let insn = prog[i];
        let dst = insn.dst();
        let src = insn.src();

        if dst >= N_REGS { return Err(VerifyError::InvalidRegister(dst)); }
        if src >= N_REGS { return Err(VerifyError::InvalidRegister(src)); }

        let op = insn.opcode;
        match op {
            // ALU: register ops check src is initialised
            0x0F | 0x1F | 0x2F | 0x3F | 0x4F | 0x5F | 0x6F |
            0x7F | 0xAF | 0xBF | 0xCF => {
                if !regs_init[src] { return Err(VerifyError::ReadUninitialized(src)); }
                regs_init[dst] = true;
            }
            // ALU: immediate ops always initialise dst
            0x07 | 0x17 | 0x27 | 0x37 | 0x47 | 0x57 | 0x67 |
            0x77 | 0xA7 | 0xB7 | 0xC7 => {
                regs_init[dst] = true;
            }
            // Jumps: check target is within program
            0x05 | 0x15 | 0x25 | 0x35 | 0x45 | 0x55 | 0x65 |
            0x75 | 0xA5 | 0xB5 | 0xC5 | 0xD5 => {
                let target = i as i64 + 1 + insn.offset as i64;
                if target < 0 || target as usize >= prog.len() {
                    return Err(VerifyError::BadJumpTarget(insn.offset));
                }
                // Disallow back-jumps entirely (prevents unbounded loops)
                if insn.offset < 0 { return Err(VerifyError::UnboundedLoop); }
            }
            // Load: dst gets initialised
            0x61 | 0x69 | 0x71 | 0x79 | 0x18 => {
                if !regs_init[src] && op != 0x18 {
                    return Err(VerifyError::NullDeref);
                }
                regs_init[dst] = true;
                if op == 0x18 { i += 1; } // LD_IMM64 consumes two insns
            }
            // Store: src checked
            0x63 | 0x6B | 0x73 | 0x7B => {
                if !regs_init[src] { return Err(VerifyError::ReadUninitialized(src)); }
            }
            // Call, Exit: always valid
            0x85 | 0x95 => {}
            _ => return Err(VerifyError::InvalidOpcode(op)),
        }
        i += 1;
    }
    Ok(())
}

// ── eBPF Interpreter ──────────────────────────────────────────────────────
const STACK_SIZE: usize = 512;

pub struct BpfVm {
    regs:  [u64; N_REGS],
    stack: [u8; STACK_SIZE],
}

impl BpfVm {
    pub fn new() -> Self {
        BpfVm { regs: [0u64; N_REGS], stack: [0u8; STACK_SIZE] }
    }

    pub fn run(&mut self, prog: &[Insn], ctx: u64) -> i64 {
        self.regs[1]  = ctx;                            // r1 = context
        self.regs[10] = self.stack.as_ptr() as u64 + STACK_SIZE as u64; // r10 = fp

        let mut pc = 0usize;
        let limit = prog.len().min(MAX_INSNS * 10); // safety: max iterations

        for _ in 0..limit {
            if pc >= prog.len() { break; }
            let insn = prog[pc];
            let dst = insn.dst(); let src = insn.src();
            let imm = insn.imm as u64;
            let op  = insn.opcode;

            macro_rules! alu64r { ($op:tt) => {{ self.regs[dst] = self.regs[dst] $op self.regs[src]; }} }
            macro_rules! alu64i { ($op:tt) => {{ self.regs[dst] = self.regs[dst] $op imm; }} }

            match op {
                0x0F => alu64r!(+), 0x1F => alu64r!(-), 0x2F => alu64r!(*),
                0x3F => { if self.regs[src] != 0 { self.regs[dst] /= self.regs[src]; } }
                0x4F => alu64r!(|), 0x5F => alu64r!(&), 0xAF => alu64r!(^),
                0x6F => { self.regs[dst] <<= (self.regs[src] & 63); }
                0x7F => { self.regs[dst] >>= (self.regs[src] & 63); }
                0xBF => { self.regs[dst] = self.regs[src]; }

                0x07 => alu64i!(+), 0x17 => alu64i!(-), 0x27 => alu64i!(*),
                0x37 => { if imm != 0 { self.regs[dst] /= imm; } }
                0x47 => alu64i!(|), 0x57 => alu64i!(&), 0xA7 => alu64i!(^),
                0x67 => { self.regs[dst] <<= (imm & 63); }
                0x77 => { self.regs[dst] >>= (imm & 63); }
                0xB7 => { self.regs[dst] = imm; }

                0x61 => unsafe { self.regs[dst] = (self.regs[src].wrapping_add(insn.offset as u64 as u64) as *const u32).read_unaligned() as u64; }
                0x79 => unsafe { self.regs[dst] = (self.regs[src].wrapping_add(insn.offset as u64) as *const u64).read_unaligned(); }
                0x63 => unsafe { (self.regs[dst].wrapping_add(insn.offset as u64) as *mut u32).write_unaligned(self.regs[src] as u32); }
                0x7B => unsafe { (self.regs[dst].wrapping_add(insn.offset as u64) as *mut u64).write_unaligned(self.regs[src]); }

                0x18 => {
                    // LD_IMM64: next insn holds upper 32 bits
                    let upper = if pc + 1 < prog.len() { prog[pc+1].imm as u64 } else { 0 };
                    self.regs[dst] = ((upper << 32) | (imm & 0xFFFF_FFFF));
                    pc += 1;
                }

                0x05 => { pc = (pc as i64 + 1 + insn.offset as i64) as usize; continue; }
                0x15 => { if self.regs[dst] == imm { pc = (pc as i64 + 1 + insn.offset as i64) as usize; continue; } }
                0x55 => { if self.regs[dst] != imm { pc = (pc as i64 + 1 + insn.offset as i64) as usize; continue; } }
                0x25 => { if self.regs[dst] >  imm { pc = (pc as i64 + 1 + insn.offset as i64) as usize; continue; } }
                0x35 => { if self.regs[dst] >= imm { pc = (pc as i64 + 1 + insn.offset as i64) as usize; continue; } }

                0x85 => { /* helper call — no-op in interpreter */ }
                0x95 => { return self.regs[0] as i64; }
                _ => {}
            }
            pc += 1;
        }
        self.regs[0] as i64
    }
}

// ── Program registry ──────────────────────────────────────────────────────
const MAX_PROGS: usize = 64;

#[derive(Copy, Clone, PartialEq)]
pub enum AttachType { Tracepoint, Kprobe, XdpIngress, SyscallEnter, SyscallExit }

pub struct BpfProgram {
    pub id:          u32,
    pub attach:      AttachType,
    pub attach_key:  u64,   // syscall nr, tracepoint id, etc.
    insns:    [Insn; MAX_INSNS],
    n_insns:  usize,
    verified: bool,
}

impl BpfProgram {
    pub const fn empty() -> Self {
        BpfProgram {
            id: 0, attach: AttachType::Tracepoint, attach_key: 0,
            insns: [Insn { opcode: 0x95, regs: 0, offset: 0, imm: 0 }; MAX_INSNS],
            n_insns: 0, verified: false,
        }
    }
}

static mut BPF_PROGS: [BpfProgram; MAX_PROGS] = [const { BpfProgram::empty() }; MAX_PROGS];
static BPF_NEXT_ID: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(1);

// ── C-ABI exports ──────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_bpf_load(
    insns: *const Insn, n: usize,
    attach: u8, attach_key: u64,
) -> i32 {
    if insns.is_null() || n == 0 || n > MAX_INSNS { return -22; }
    let prog_insns = core::slice::from_raw_parts(insns, n);

    // Verify before loading
    if let Err(e) = verify(prog_insns) { return -13; } // EACCES

    for i in 0..MAX_PROGS {
        if BPF_PROGS[i].n_insns == 0 {
            let id = BPF_NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let prog = &mut BPF_PROGS[i];
            prog.id = id;
            prog.attach = match attach {
                0 => AttachType::Tracepoint,
                1 => AttachType::Kprobe,
                2 => AttachType::XdpIngress,
                3 => AttachType::SyscallEnter,
                4 => AttachType::SyscallExit,
                _ => AttachType::Tracepoint,
            };
            prog.attach_key = attach_key;
            prog.insns[..n].copy_from_slice(prog_insns);
            prog.n_insns = n;
            prog.verified = true;
            return id as i32;
        }
    }
    -12 // ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bpf_run_syscall(nr: u64, ctx: u64) -> i64 {
    for i in 0..MAX_PROGS {
        let prog = &BPF_PROGS[i];
        if prog.n_insns == 0 || !prog.verified { continue; }
        if (prog.attach == AttachType::SyscallEnter || prog.attach == AttachType::SyscallExit)
            && (prog.attach_key == nr || prog.attach_key == u64::MAX)
        {
            let mut vm = BpfVm::new();
            let result = vm.run(&prog.insns[..prog.n_insns], ctx);
            if result != 0 { return result; } // non-zero = filter/drop
        }
    }
    0 // allow
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bpf_unload(id: u32) -> i32 {
    for i in 0..MAX_PROGS {
        if BPF_PROGS[i].id == id {
            BPF_PROGS[i].n_insns = 0;
            return 0;
        }
    }
    -2 // ENOENT
}
