// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: KERNEL TELEMETRY SUBSYSTEM (Rust, no_std)
//! =========================================================================
//!
//! Root module for the SigmaOS Telemetry Stack.
//! This completely replaces the legacy C/C++ implementations
//! (SovereignTelemetryShard.cpp) with memory-safe, zero-dependency Rust structs
//! following OOP-like design.
//!
//! Exposes a C-ABI compatible interface for integration with the rest of
//! the SigmaOS kernel for Zero-Trust auditing.
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

/// Per-shard telemetry collection (CPU, IPC, memory, page faults)
pub mod shard;

/// The Sovereign Profiler — high-frequency metrics aggregation and broadcast.
/// Provides lock-free per-shard metrics at 10Hz via /sigma/metrics shared memory.
pub mod sovereign_profiler;
