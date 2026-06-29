// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL CORE SCHEDULER SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Sovereign Scheduler layer.
//! This replaces the legacy C/C++ scheduler implementations
//! (sigma_sched_sovereign.cpp, sigma_mlfq.c) with a memory-safe,
//! zero-dependency Rust implementation.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod sovereign;
pub mod mlfq;
