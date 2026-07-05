#![no_std]
#![allow(dead_code)]

/// SigmaOS eBPF Subsystem
/// A minimal, `no_std` Extended Berkeley Packet Filter interpreter and verifier.
/// Inspired by Linux kernel/bpf/core.c.

use core::sync::atomic::{AtomicU32, Ordering};

const MAX_INSTS: usize = 4096;
const MAX_STACK: usize = 512;

/// eBPF Instruction Format (64-bit)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct BpfInsn {
    pub code: u8,
    pub dst_reg: u8, // 4 bits dest, 4 bits src (packed in struct as a full byte for simplicity in this ABI, or half/half if we follow exact Linux spec)
    pub src_reg: u8,
    pub off: i16,
    pub imm: i32,
}

impl BpfInsn {
    pub const fn new(code: u8, dst: u8, src: u8, off: i16, imm: i32) -> Self {
        Self { code, dst_reg: dst, src_reg: src, off, imm }
    }
}

// Instruction Classes
const BPF_CLASS_ALU64: u8 = 0x07;
const BPF_CLASS_JMP:   u8 = 0x05;
const BPF_CLASS_LDX:   u8 = 0x01;
const BPF_CLASS_STX:   u8 = 0x03;

// ALU Opcodes
const BPF_ADD:  u8 = 0x00;
const BPF_SUB:  u8 = 0x10;
const BPF_MUL:  u8 = 0x20;
const BPF_DIV:  u8 = 0x30;
const BPF_OR:   u8 = 0x40;
const BPF_AND:  u8 = 0x50;
const BPF_LSH:  u8 = 0x60;
const BPF_RSH:  u8 = 0x70;
const BPF_XOR:  u8 = 0xa0;
const BPF_MOV:  u8 = 0xb0;

// JMP Opcodes
const BPF_JA:   u8 = 0x00;
const BPF_JEQ:  u8 = 0x10;
const BPF_JGT:  u8 = 0x20;
const BPF_JGE:  u8 = 0x30;
const BPF_JSET: u8 = 0x40;
const BPF_JNE:  u8 = 0x50;
const BPF_JSGT: u8 = 0x60;
const BPF_JSGE: u8 = 0x70;
const BPF_EXIT: u8 = 0x90;

// Source operand
const BPF_K:    u8 = 0x00;
const BPF_X:    u8 = 0x08;

pub struct BpfProgram {
    insns: [BpfInsn; MAX_INSTS],
    len: usize,
    id: u32,
}

impl BpfProgram {
    pub const fn new() -> Self {
        Self {
            insns: [BpfInsn::new(0,0,0,0,0); MAX_INSTS],
            len: 0,
            id: 0,
        }
    }
    
    pub fn load(&mut self, instrs: &[BpfInsn], id: u32) -> Result<(), &'static str> {
        if instrs.len() > MAX_INSTS {
            return Err("Program too large");
        }
        for i in 0..instrs.len() {
            self.insns[i] = instrs[i];
        }
        self.len = instrs.len();
        self.id = id;
        self.verify()
    }

    /// Minimal verifier to prevent out-of-bounds jumps and enforce EXIT.
    fn verify(&self) -> Result<(), &'static str> {
        if self.len == 0 {
            return Err("Empty program");
        }
        let last_insn = &self.insns[self.len - 1];
        if last_insn.code != (BPF_CLASS_JMP | BPF_EXIT) {
            return Err("Program must end with EXIT");
        }
        
        for i in 0..self.len {
            let insn = &self.insns[i];
            let class = insn.code & 0x07;
            if class == BPF_CLASS_JMP {
                let opcode = insn.code & 0xf0;
                if opcode != BPF_EXIT {
                    let target = i as isize + 1 + insn.off as isize;
                    if target < 0 || target >= self.len as isize {
                        return Err("Out-of-bounds jump");
                    }
                }
            }
        }
        Ok(())
    }

    /// Execute the BPF program against a context pointer.
    pub fn run(&self, ctx: *mut u8) -> u64 {
        let mut regs: [u64; 11] = [0; 11];
        let mut stack = [0u8; MAX_STACK];
        
        regs[1] = ctx as u64; // R1 = ctx
        regs[10] = stack.as_mut_ptr() as u64 + MAX_STACK as u64; // R10 = frame pointer
        
        let mut pc = 0;
        
        while pc < self.len {
            let insn = &self.insns[pc];
            let dst = insn.dst_reg as usize;
            let src = insn.src_reg as usize;
            
            let class = insn.code & 0x07;
            let source = insn.code & 0x08;
            let opcode = insn.code & 0xf0;

            if dst > 10 || src > 10 {
                return 0; // Invalid register, abort safely
            }

            match class {
                BPF_CLASS_ALU64 => {
                    let val = if source == BPF_K { insn.imm as i64 as u64 } else { regs[src] };
                    match opcode {
                        BPF_ADD => regs[dst] = regs[dst].wrapping_add(val),
                        BPF_SUB => regs[dst] = regs[dst].wrapping_sub(val),
                        BPF_MUL => regs[dst] = regs[dst].wrapping_mul(val),
                        BPF_DIV => if val != 0 { regs[dst] /= val; },
                        BPF_OR  => regs[dst] |= val,
                        BPF_AND => regs[dst] &= val,
                        BPF_LSH => regs[dst] <<= val & 63,
                        BPF_RSH => regs[dst] >>= val & 63,
                        BPF_XOR => regs[dst] ^= val,
                        BPF_MOV => regs[dst] = val,
                        _ => {}
                    }
                },
                BPF_CLASS_JMP => {
                    if opcode == BPF_EXIT {
                        return regs[0]; // R0 contains return value
                    }
                    
                    let val = if source == BPF_K { insn.imm as i64 as u64 } else { regs[src] };
                    let mut jump = false;
                    
                    match opcode {
                        BPF_JA   => jump = true,
                        BPF_JEQ  => jump = regs[dst] == val,
                        BPF_JGT  => jump = regs[dst] > val,
                        BPF_JGE  => jump = regs[dst] >= val,
                        BPF_JSET => jump = (regs[dst] & val) != 0,
                        BPF_JNE  => jump = regs[dst] != val,
                        BPF_JSGT => jump = (regs[dst] as i64) > (val as i64),
                        BPF_JSGE => jump = (regs[dst] as i64) >= (val as i64),
                        _ => {}
                    }
                    
                    if jump {
                        pc = (pc as isize + 1 + insn.off as isize) as usize;
                        continue;
                    }
                },
                BPF_CLASS_LDX => {
                    // Minimal memory load (simulated)
                    // Security warning: In a real kernel, this needs strict verification!
                    unsafe {
                        let ptr = (regs[src] as i64 + insn.off as i64) as *const u64;
                        regs[dst] = core::ptr::read_volatile(ptr);
                    }
                },
                BPF_CLASS_STX => {
                    // Minimal memory store (simulated)
                    unsafe {
                        let ptr = (regs[dst] as i64 + insn.off as i64) as *mut u64;
                        core::ptr::write_volatile(ptr, regs[src]);
                    }
                },
                _ => return 0, // Unimplemented instruction
            }
            pc += 1;
        }
        
        0
    }
}

// ── Global eBPF State ─────────────────────────────────────────────────────

static mut G_EBPF_PROG: BpfProgram = BpfProgram::new();
static BPF_ID_GEN: AtomicU32 = AtomicU32::new(1);

// ── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_bpf_load(insns: *const BpfInsn, count: usize) -> i32 {
    if insns.is_null() || count == 0 {
        return -1;
    }
    
    let slice = core::slice::from_raw_parts(insns, count);
    let id = BPF_ID_GEN.fetch_add(1, Ordering::Relaxed);
    
    match G_EBPF_PROG.load(slice, id) {
        Ok(_) => id as i32,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_bpf_run(ctx: *mut u8) -> u64 {
    if G_EBPF_PROG.len == 0 {
        return 0;
    }
    G_EBPF_PROG.run(ctx)
}
