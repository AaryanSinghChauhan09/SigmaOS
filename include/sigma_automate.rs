/// SigmaOS: @file sigma_automate.h
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

// ─── Module: sigma::sigma_automate ─────────────────────

/// TriggerDescriptor — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub type: SigmaU64,
    pub interval_sec: SigmaU32,
}

/// ActionDescriptor — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub type: SigmaU64,
    pub run_in_sandbox: SigmaBool,
}

/// Playbook — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub id: SigmaU32,
    pub name: [u8; 64],
    pub trigger: SigmaU64,
    pub num_actions: SigmaU32,
    pub enabled: SigmaBool,
}

/// AutomateRegistry — hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct {s_name} {{
    pub count: SigmaU32,
}

