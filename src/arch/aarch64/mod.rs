//! SigmaOS — AArch64 Architecture Module
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux arch/arm64/ and FreeBSD sys/arm64/
#![allow(dead_code, unused)]

pub mod boot;
pub mod context;
pub mod exception;
pub mod gic;
pub mod mmu;
pub mod timer;

/// Initialize all AArch64 subsystems.
/// Called once from kernel_main after early boot.
pub fn initialize() {
    mmu::init();
    gic::init();
    timer::init();
    exception::init();
}
