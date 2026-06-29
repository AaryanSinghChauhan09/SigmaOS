// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Package Manager — sigpkg (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Package Manager.
//! Replaces legacy C/C++:
//!   - usr/sigma_pkg.c
//!   - usr/SovereignPkgManager.cpp
//!
//! Entirely no_std, POSIX-free, operating over the Sovereign Syscall ABI.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod manager;
pub mod cli;
