// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL NETWORK SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Sovereign TCP/IP Network Stack.
//! This completely replaces the legacy C/C++ network implementations
//! (SovereignNetStack.cpp, sigma_net_ipv4, sigma_firewall, etc.) with
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

pub mod net_stack;
pub mod socket;
pub mod firewall;
