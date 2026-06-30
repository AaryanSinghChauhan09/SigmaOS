/// SigmaOS: Σ SigmaOS — sigma_quantum_api: Quantum-Ready APIs
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Module: Sigma::sigma_quantum_api ─────────────────────

/// QuantumJob — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub job_id: SigmaU64,
    pub num_qubits: SigmaU64,
    pub bytecode_len: SigmaU64,
    pub result_len: SigmaU64,
    pub status: SigmaU64,
}

