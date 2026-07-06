// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Seccomp-BPF Syscall Filter
//! Minimal BPF interpreter for syscall filtering.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaI32 = i32;

pub const SECCOMP_RET_KILL:  SigmaU32 = 0x00000000;
pub const SECCOMP_RET_TRAP:  SigmaU32 = 0x00030000;
pub const SECCOMP_RET_ERRNO: SigmaU32 = 0x00050000;
pub const SECCOMP_RET_TRACE: SigmaU32 = 0x7ff00000;
pub const SECCOMP_RET_ALLOW: SigmaU32 = 0x7fff0000;

pub const BPF_MAXINSNS: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_filter {
    pub code: u16,
    pub jt: u8,
    pub jf: u8,
    pub k: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seccomp_data {
    pub nr: SigmaI32,
    pub arch: SigmaU32,
    pub instruction_pointer: SigmaU32,
    pub args: [SigmaU32; 6],
}

// BPF Instruction opcodes (minimal subset for seccomp)
const BPF_LD:   u16 = 0x00;
const BPF_JMP:  u16 = 0x05;
const BPF_RET:  u16 = 0x06;

const BPF_W:    u16 = 0x00;
const BPF_ABS:  u16 = 0x20;
const BPF_JEQ:  u16 = 0x10;
const BPF_K:    u16 = 0x00;

#[repr(C)]
pub struct BpfProgram {
    pub len: usize,
    pub filter: [sock_filter; BPF_MAXINSNS],
}

static mut ACTIVE_FILTER: Option<BpfProgram> = None;

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_init() {
    ACTIVE_FILTER = None;
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_set_filter(prog: *const sock_filter, len: usize) -> SigmaI32 {
    if prog.is_null() || len == 0 || len > BPF_MAXINSNS { return -1; }
    
    let mut new_prog = BpfProgram { len, filter: [sock_filter { code: 0, jt: 0, jf: 0, k: 0 }; BPF_MAXINSNS] };
    for i in 0..len {
        new_prog.filter[i] = *prog.add(i);
    }
    
    ACTIVE_FILTER = Some(new_prog);
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_seccomp_run_filter(data: *const seccomp_data) -> SigmaU32 {
    if data.is_null() { return SECCOMP_RET_KILL; }
    let data_ref = &*data;
    
    if let Some(ref prog) = ACTIVE_FILTER {
        let mut pc = 0;
        let mut a: u32 = 0;
        
        while pc < prog.len {
            let insn = &prog.filter[pc];
            match insn.code {
                c if c == (BPF_LD | BPF_W | BPF_ABS) => {
                    // Load data into accumulator based on offset k
                    // seccomp_data layout: 
                    // offset 0: nr
                    // offset 4: arch
                    // offset 8: instruction_pointer
                    // offset 12..: args
                    match insn.k {
                        0 => a = data_ref.nr as u32,
                        4 => a = data_ref.arch,
                        8 => a = data_ref.instruction_pointer,
                        12..=32 => {
                            let arg_idx = (insn.k - 12) / 4;
                            a = data_ref.args[arg_idx as usize];
                        },
                        _ => return SECCOMP_RET_KILL,
                    }
                    pc += 1;
                }
                c if c == (BPF_JMP | BPF_JEQ | BPF_K) => {
                    if a == insn.k {
                        pc += insn.jt as usize + 1;
                    } else {
                        pc += insn.jf as usize + 1;
                    }
                }
                c if c == (BPF_RET | BPF_K) => {
                    return insn.k;
                }
                _ => return SECCOMP_RET_KILL, // Unknown/unsupported instruction
            }
        }
        SECCOMP_RET_KILL // Fallthrough implies kill
    } else {
        SECCOMP_RET_ALLOW // No filter attached
    }
}
