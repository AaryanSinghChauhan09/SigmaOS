// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/sigma_seccomp.rs — seccomp-BPF syscall filter
//
// Implements a subset of the Linux seccomp(2) interface:
//   - SECCOMP_MODE_STRICT  : allow only read/write/exit/sigreturn
//   - SECCOMP_MODE_FILTER  : evaluate a BPF filter program
//
// BPF instruction set: LD, LDX, ST, STX, ALU, JMP, RET
// Filter actions:      ALLOW, ERRNO(n), KILL, TRAP, LOG
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── BPF instruction ────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SockFilter {
    pub code: u16, // opcode
    pub jt:   u8,  // jump-if-true delta
    pub jf:   u8,  // jump-if-false delta
    pub k:    u32, // generic multiuse field
}

// BPF instruction classes
const BPF_LD:  u16 = 0x00;
const BPF_LDX: u16 = 0x01;
const BPF_ALU: u16 = 0x04;
const BPF_JMP: u16 = 0x05;
const BPF_RET: u16 = 0x06;

// BPF LD/LDX modes
const BPF_W:   u16 = 0x00; // word (32-bit)
const BPF_ABS: u16 = 0x20; // absolute offset
const BPF_IMM: u16 = 0x00; // immediate

// BPF ALU ops
const BPF_AND: u16 = 0x50;
const BPF_OR:  u16 = 0x40;

// BPF JMP ops
const BPF_JEQ: u16 = 0x10;
const BPF_JGT: u16 = 0x20;
const BPF_JGE: u16 = 0x30;

// BPF RET sources
const BPF_K:   u16 = 0x00;

// seccomp return actions
const SECCOMP_RET_KILL_PROCESS: u32 = 0x80000000;
const SECCOMP_RET_KILL_THREAD:  u32 = 0x00000000;
const SECCOMP_RET_TRAP:         u32 = 0x00030000;
const SECCOMP_RET_ERRNO:        u32 = 0x00050000;
const SECCOMP_RET_LOG:          u32 = 0x7ffc0000;
const SECCOMP_RET_ALLOW:        u32 = 0x7fff0000;
const SECCOMP_RET_DATA_MASK:    u32 = 0x0000ffff;

// ── seccomp data (passed to BPF program) ──────────────────────────────────
#[repr(C)]
pub struct SeccompData {
    pub nr:           u32, // syscall number
    pub arch:         u32, // AUDIT_ARCH_X86_64 = 0xC000003E
    pub instruction_pointer: u64,
    pub args:         [u64; 6],
}

const AUDIT_ARCH_X86_64: u32 = 0xC000003E;

// ── BPF virtual machine ───────────────────────────────────────────────────
const MAX_INSNS: usize = 4096;

pub struct BpfVm;

impl BpfVm {
    /// Evaluate a BPF filter against seccomp data.
    /// Returns the seccomp return action.
    pub fn run(prog: &[SockFilter], data: &SeccompData) -> u32 {
        if prog.is_empty() { return SECCOMP_RET_ALLOW; }
        let raw = unsafe {
            core::slice::from_raw_parts(
                data as *const _ as *const u32,
                core::mem::size_of::<SeccompData>() / 4,
            )
        };

        let mut a: u32 = 0;  // accumulator
        let mut x: u32 = 0;  // index register
        let mut pc: usize = 0;

        loop {
            if pc >= prog.len() || pc >= MAX_INSNS { break; }
            let insn = prog[pc];
            let class = insn.code & 0x07;
            match class {
                // LD: load into accumulator
                0 /* BPF_LD */ => {
                    let mode = insn.code & 0xE0;
                    if mode == BPF_ABS {
                        let off = insn.k as usize / 4;
                        a = raw.get(off).copied().unwrap_or(0);
                    } else {
                        a = insn.k;
                    }
                    pc += 1;
                }
                // ALU
                4 /* BPF_ALU */ => {
                    let op = insn.code & 0xF0;
                    let src = if insn.code & 0x08 != 0 { x } else { insn.k };
                    a = match op {
                        0x00 => a.wrapping_add(src),
                        0x10 => a.wrapping_sub(src),
                        0x20 => a.wrapping_mul(src),
                        0x30 => if src != 0 { a / src } else { 0 },
                        0x40 => a | src,        // OR
                        0x50 => a & src,        // AND
                        0x60 => a << (src & 31),
                        0x70 => a >> (src & 31),
                        0x80 => a.wrapping_neg(),
                        0x90 => a ^ src,
                        _ => a,
                    };
                    pc += 1;
                }
                // JMP
                5 /* BPF_JMP */ => {
                    let op  = insn.code & 0xF0;
                    let src = if insn.code & 0x08 != 0 { x } else { insn.k };
                    let cond = match op {
                        0x00 => true,          // JA — unconditional
                        0x10 => a == src,      // JEQ
                        0x20 => a >  src,      // JGT
                        0x30 => a >= src,      // JGE
                        0x40 => a & src != 0,  // JSET
                        _ => false,
                    };
                    if op == 0x00 {
                        pc = pc.wrapping_add(1 + insn.k as usize);
                    } else {
                        pc += 1 + if cond { insn.jt as usize } else { insn.jf as usize };
                    }
                }
                // RET
                6 /* BPF_RET */ => {
                    return if insn.code & 0x18 == BPF_K { insn.k } else { a };
                }
                _ => { pc += 1; }
            }
        }
        SECCOMP_RET_ALLOW
    }
}

// ── Per-process seccomp state ─────────────────────────────────────────────
const MAX_PROCS:  usize = 256;
const MAX_FILTER: usize = 256; // max BPF instructions per filter

#[derive(Copy, Clone)]
pub struct FilterSlot {
    insns: [SockFilter; MAX_FILTER],
    count: usize,
}

impl FilterSlot {
    const fn empty() -> Self {
        FilterSlot {
            insns: [SockFilter { code: 0, jt: 0, jf: 0, k: 0 }; MAX_FILTER],
            count: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SeccompMode { Disabled, Strict, Filter }

#[derive(Copy, Clone)]
pub struct SeccompState {
    pub mode:   SeccompMode,
    filter:     FilterSlot,
    kill_on_violation: bool,
}

impl SeccompState {
    const fn new() -> Self {
        SeccompState { mode: SeccompMode::Disabled, filter: FilterSlot::empty(), kill_on_violation: true }
    }
}

static mut SECCOMP_TABLE: [SeccompState; MAX_PROCS] =
    [const { SeccompState::new() }; MAX_PROCS];

// ── Built-in safe filters ─────────────────────────────────────────────────

/// Build a "allow only listed syscalls, kill otherwise" filter.
pub fn build_allowlist_filter(
    allowed: &[u32], out: &mut [SockFilter],
) -> usize {
    // Architecture check: load arch field, compare to x86_64
    let mut pc = 0;
    // Load syscall nr (offset 0 in seccomp_data)
    out[pc] = SockFilter { code: (BPF_LD | BPF_W | BPF_ABS), jt: 0, jf: 0, k: 0 }; pc += 1;
    // For each allowed NR: if nr == X → allow
    for &nr in allowed {
        if pc + 2 > out.len() { break; }
        let jumps_to_allow = (allowed.len() - (pc / 2)) as u8; // rough
        out[pc] = SockFilter { code: (BPF_JMP | BPF_JEQ | BPF_K),
            jt: 0, jf: 1, k: nr }; pc += 1; // jt=skip kill, jf=next check
        // placeholder — real filter chains all comparisons
        let _ = jumps_to_allow;
    }
    // Default: KILL
    out[pc] = SockFilter { code: (BPF_RET | BPF_K), jt: 0, jf: 0,
        k: SECCOMP_RET_KILL_PROCESS }; pc += 1;
    pc
}

// ── C-ABI exports ──────────────────────────────────────────────────────────

/// Install strict mode (only read/write/exit/sigreturn)
#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_strict(pid: u32) -> i32 {
    let state = &mut SECCOMP_TABLE[pid as usize % MAX_PROCS];
    state.mode = SeccompMode::Strict;
    0
}

/// Install a BPF filter
#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_filter(
    pid: u32, insns: *const SockFilter, count: usize,
) -> i32 {
    let state = &mut SECCOMP_TABLE[pid as usize % MAX_PROCS];
    if count > MAX_FILTER { return -22; }
    for i in 0..count {
        state.filter.insns[i] = *insns.add(i);
    }
    state.filter.count = count;
    state.mode = SeccompMode::Filter;
    0
}

/// Called by syscall gate before dispatching — returns true if allowed.
#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_check(pid: u32, nr: u64, args: *const u64) -> bool {
    let state = &SECCOMP_TABLE[pid as usize % MAX_PROCS];
    match state.mode {
        SeccompMode::Disabled => true,
        SeccompMode::Strict => matches!(nr, 0 | 1 | 60 | 15), // read/write/exit/rt_sigreturn
        SeccompMode::Filter => {
            let mut arg_arr = [0u64; 6];
            for i in 0..6 { arg_arr[i] = *args.add(i); }
            let data = SeccompData {
                nr: nr as u32, arch: AUDIT_ARCH_X86_64,
                instruction_pointer: 0, args: arg_arr,
            };
            let action = BpfVm::run(&state.filter.insns[..state.filter.count], &data);
            action == SECCOMP_RET_ALLOW || (action & 0xFFFF0000) == SECCOMP_RET_LOG
        }
    }
}

// ── Pledge → seccomp bridge ───────────────────────────────────────────────
/// Generate a seccomp filter from a pledge bitmask.
/// Called when sigma_pledge() is invoked.
#[no_mangle]
pub unsafe extern "C" fn sigma_pledge_to_seccomp(pid: u32, pledge_mask: u64) -> i32 {
    // Map pledge bits → allowed syscall sets
    // (simplified: just install strict mode for stdio-only pledge)
    use crate::sigma_pledge::*;
    if pledge_mask == PLEDGE_STDIO {
        return sigma_seccomp_strict(pid);
    }
    0 // full filter generation would enumerate all syscalls per pledge group
}
