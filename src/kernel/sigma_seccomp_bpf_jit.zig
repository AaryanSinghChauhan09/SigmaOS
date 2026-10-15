// SigmaOS Zero-Allocation Seccomp-BPF JIT & Syscall Instruction Evaluator
// Written in Zig for high-speed deterministic syscall gating.
// Inspired by Linux seccomp.c and OpenBSD pledge.

const std = @import("std");

pub const BpfOpcode = enum(u8) {
    Ret = 0x06,
    JmpEq = 0x15,
    JmpGt = 0x25,
    JmpGe = 0x35,
    LdAbs = 0x20,
};

pub const SeccompRet = enum(u32) {
    KillProcess = 0x80000000,
    Trap = 0x00030000,
    Errno = 0x00050000,
    Allow = 0x7fff0000,
};

pub const BpfInstruction = struct {
    code: BpfOpcode,
    jt: u8,
    jf: u8,
    k: u32,
};

pub const FastSeccompEvaluator = struct {
    program: [64]BpfInstruction,
    prog_len: usize,

    pub fn init() FastSeccompEvaluator {
        return FastSeccompEvaluator{
            .program = undefined,
            .prog_len = 0,
        };
    }

    pub fn add_instruction(self: *FastSeccompEvaluator, instr: BpfInstruction) bool {
        if (self.prog_len >= 64) return false;
        self.program[self.prog_len] = instr;
        self.prog_len += 1;
        return true;
    }

    pub fn evaluate(self: *const FastSeccompEvaluator, syscall_nr: u32) SeccompRet {
        var pc: usize = 0;
        var accumulator: u32 = 0;

        while (pc < self.prog_len) {
            const instr = self.program[pc];
            switch (instr.code) {
                .LdAbs => {
                    accumulator = syscall_nr;
                    pc += 1;
                },
                .JmpEq => {
                    if (accumulator == instr.k) {
                        pc += 1 + instr.jt;
                    } else {
                        pc += 1 + instr.jf;
                    }
                },
                .Ret => {
                    if (instr.k == @intFromEnum(SeccompRet.Allow)) {
                        return SeccompRet.Allow;
                    }
                    return SeccompRet.KillProcess;
                },
                else => {
                    pc += 1;
                },
            }
        }
        return SeccompRet.KillProcess;
    }
};
