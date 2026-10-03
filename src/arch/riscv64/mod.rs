//! SigmaOS — RISC-V 64-bit Architecture Module
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux arch/riscv/ and FreeBSD sys/riscv/
#![allow(dead_code, unused)]

pub mod boot;
pub mod context;
pub mod mmu;
pub mod plic;
pub mod timer;
pub mod trap;

/// Initialize all RISC-V 64-bit subsystems.
/// Called once from kernel_main after early boot.
pub fn initialize() {
    mmu::init();
    plic::init();
    trap::init();
    timer::init();
}
