#![no_std]

pub mod translator;
pub mod elf_loader;

/// System call context passing registers
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallContext {
    pub nr: u64,
    pub args: [u64; 6],
    pub ret: i64,
}

impl SyscallContext {
    pub fn new(nr: u64, args: [u64; 6]) -> Self {
        Self { nr, args, ret: 0 }
    }
}
