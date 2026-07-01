// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL VIRT SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Hypervisor and Container Subsystem.
//! This completely replaces the legacy C/C++ implementations
//! (SovereignHypervisor.cpp, sigma_vcpu.cpp, sigma_container.cpp).
//!
//! Exposes a C-ABI compatible interface for integration with the rest of
//! the SigmaOS kernel.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod hypervisor;
pub mod vcpu;
pub mod container;
