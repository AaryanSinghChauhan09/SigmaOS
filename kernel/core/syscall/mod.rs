// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL CORE SYSCALL SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Sovereign Syscall layer.
//! This completely replaces the legacy C/C++ syscall implementations
//! (SovereignSyscall.cpp, dispatcher, panic, ipc, signal, etc.) with
//! memory-safe, zero-dependency Rust structs following OOP-like design.
//!
//! Exposes a C-ABI compatible interface for integration with the rest of
//! the SigmaOS kernel.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod gate;
pub mod ipc;
pub mod panic;
