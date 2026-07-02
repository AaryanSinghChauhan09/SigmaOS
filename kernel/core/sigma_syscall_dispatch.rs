// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_syscall_dispatch.rs — Syscall Dispatch Table
// Replaces: sigma_syscall_dispatch.cpp (C++ stub, removed)
//
// 30 sovereign syscalls — no POSIX dependency, no libc
// Language: Rust #![no_std]
// Pattern: function pointer table + OOP SyscallHandler trait

#![no_std]

// ── Syscall Numbers ──────────────────────────────────────────────────────────

#[repr(u32)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Syscall {
    Read       = 0,
    Write      = 1,
    Open       = 2,
    Close      = 3,
    Exit       = 4,
    Fork       = 5,
    Exec       = 6,
    Wait       = 7,
    Mmap       = 8,
    Munmap     = 9,
    Brk        = 10,
    Getpid     = 11,
    Getppid    = 12,
    Kill       = 13,
    Signal     = 14,
    Stat       = 15,
    Fstat      = 16,
    Lseek      = 17,
    Dup        = 18,
    Dup2       = 19,
    Pipe       = 20,
    Chdir      = 21,
    Getcwd     = 22,
    Mkdir      = 23,
    Rmdir      = 24,
    Unlink     = 25,
    Rename     = 26,
    ClockGettime = 27,
    Nanosleep  = 28,
    Ioctl      = 29,
    // Sigma-native extensions
    SigmaPledge  = 30,
    SigmaUnveil  = 31,
    SigmaAttest  = 32,
}

impl Syscall {
    pub fn from_u32(n: u32) -> Option<Self> {
        match n {
            0  => Some(Self::Read),       1  => Some(Self::Write),
            2  => Some(Self::Open),       3  => Some(Self::Close),
            4  => Some(Self::Exit),       5  => Some(Self::Fork),
            6  => Some(Self::Exec),       7  => Some(Self::Wait),
            8  => Some(Self::Mmap),       9  => Some(Self::Munmap),
            10 => Some(Self::Brk),        11 => Some(Self::Getpid),
            12 => Some(Self::Getppid),    13 => Some(Self::Kill),
            14 => Some(Self::Signal),     15 => Some(Self::Stat),
            16 => Some(Self::Fstat),      17 => Some(Self::Lseek),
            18 => Some(Self::Dup),        19 => Some(Self::Dup2),
            20 => Some(Self::Pipe),       21 => Some(Self::Chdir),
            22 => Some(Self::Getcwd),     23 => Some(Self::Mkdir),
            24 => Some(Self::Rmdir),      25 => Some(Self::Unlink),
            26 => Some(Self::Rename),     27 => Some(Self::ClockGettime),
            28 => Some(Self::Nanosleep),  29 => Some(Self::Ioctl),
            30 => Some(Self::SigmaPledge), 31 => Some(Self::SigmaUnveil),
            32 => Some(Self::SigmaAttest), _  => None,
        }
    }
}

// ── Syscall Context ──────────────────────────────────────────────────────────

/// Registers passed by userland on syscall entry (x86-64 calling convention)
#[repr(C)]
pub struct SyscallRegs {
    pub rax: u64,  // syscall number (on entry) / return value (on exit)
    pub rdi: u64,  // arg1
    pub rsi: u64,  // arg2
    pub rdx: u64,  // arg3
    pub r10: u64,  // arg4 (rcx clobbered by syscall instruction)
    pub r8:  u64,  // arg5
    pub r9:  u64,  // arg6
    pub rip: u64,  // saved instruction pointer
    pub rsp: u64,  // saved stack pointer
}

// ── Return values ────────────────────────────────────────────────────────────

pub const SIGMA_OK:     i64 = 0;
pub const SIGMA_EPERM:  i64 = -1;
pub const SIGMA_ENOENT: i64 = -2;
pub const SIGMA_EINVAL: i64 = -3;
pub const SIGMA_ENOMEM: i64 = -4;
pub const SIGMA_ENOSYS: i64 = -38;

// ── Handler Trait (OOP) ──────────────────────────────────────────────────────

pub trait SyscallHandler {
    fn sys_read (&self, fd: i32, buf: usize, len: usize) -> i64;
    fn sys_write(&self, fd: i32, buf: usize, len: usize) -> i64;
    fn sys_open (&self, path: usize, flags: u32, mode: u32) -> i64;
    fn sys_close(&self, fd: i32) -> i64;
    fn sys_exit (&self, code: i32) -> i64;
    fn sys_getpid(&self) -> i64;
    fn sys_mmap  (&self, addr: usize, len: usize, prot: u32, flags: u32) -> i64;
    fn sys_pledge(&self, caps: usize, len: usize) -> i64;
    fn sys_unveil(&self, path: usize, perms: usize) -> i64;
}

// ── Dispatch Table ───────────────────────────────────────────────────────────

/// Dispatch a syscall to the appropriate handler.
/// Called from the assembly syscall entry point.
///
/// # Safety
/// Must be called with a valid SyscallRegs from Ring-3 entry.
pub unsafe fn dispatch(regs: &mut SyscallRegs, handler: &dyn SyscallHandler) {
    let nr = regs.rax as u32;
    let result: i64 = match Syscall::from_u32(nr) {
        Some(Syscall::Read)    => handler.sys_read(
            regs.rdi as i32, regs.rsi as usize, regs.rdx as usize),
        Some(Syscall::Write)   => handler.sys_write(
            regs.rdi as i32, regs.rsi as usize, regs.rdx as usize),
        Some(Syscall::Open)    => handler.sys_open(
            regs.rdi as usize, regs.rsi as u32, regs.rdx as u32),
        Some(Syscall::Close)   => handler.sys_close(regs.rdi as i32),
        Some(Syscall::Exit)    => handler.sys_exit(regs.rdi as i32),
        Some(Syscall::Getpid)  => handler.sys_getpid(),
        Some(Syscall::Mmap)    => handler.sys_mmap(
            regs.rdi as usize, regs.rsi as usize,
            regs.rdx as u32,   regs.r10 as u32),
        Some(Syscall::SigmaPledge) => handler.sys_pledge(
            regs.rdi as usize, regs.rsi as usize),
        Some(Syscall::SigmaUnveil) => handler.sys_unveil(
            regs.rdi as usize, regs.rsi as usize),
        None => SIGMA_ENOSYS,
        _    => SIGMA_ENOSYS, // stub for unimplemented syscalls
    };
    regs.rax = result as u64;
}
