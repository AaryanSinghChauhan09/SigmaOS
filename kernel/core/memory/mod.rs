// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL CORE MEMORY SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Sovereign Memory layer.
//! This completely replaces the legacy C/C++ memory implementations
//! (SovereignPMM.cpp, sigma_slab.c, SovereignVMM.cpp) with memory-safe,
//! zero-dependency Rust structs following OOP-like design paradigms.
//!
//! Exposes a C-ABI compatible interface for integration with the rest of
//! the SigmaOS kernel.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod pmm;
pub mod slab;
pub mod vmm;

// Kernel panic handler for the memory subsystem staticlib
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            core::arch::asm!("cli; hlt", options(nomem, nostack));
        }
    }
}
