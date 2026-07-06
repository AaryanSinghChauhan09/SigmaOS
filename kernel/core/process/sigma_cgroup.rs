/// SigmaOS: =============================================================================
/// Migrated from C/C++ to Rust â€” no_std, no alloc, no external crates.
/// All types hand-defined. OOP via struct + impl + trait patterns.

#![no_std]
#![allow(dead_code)]

// â”€â”€â”€ Kernel Primitive Types â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// â”€â”€â”€ Module: Sigma::sigma_cgroup â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

/// sigma_cgroup â€” hardware-compatible struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigma_cgroup {
    pub active_subsys: SigmaU32,
    pub resources: SigmaU64,
    pub active: SigmaBool,
    pub cpu_weight: SigmaU32,
    pub memory_limit_bytes: SigmaU64,
    pub memory_usage_bytes: SigmaU64,
}

static mut GLOBAL_CGROUPS: [Option<sigma_cgroup>; 16] = [None; 16];

#[no_mangle]
pub unsafe extern "C" fn cgroup_init() {
    for i in 0..16 {
        GLOBAL_CGROUPS[i] = None;
    }
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_set_memory_limit(idx: SigmaU32, limit: SigmaU64) -> SigmaI32 {
    let index = idx as usize;
    if index >= 16 {
        return -1;
    }
    if let Some(ref mut cg) = GLOBAL_CGROUPS[index] {
        cg.memory_limit_bytes = limit;
        0
    } else {
        let mut cg = sigma_cgroup {
            active_subsys: 1,
            resources: 0,
            active: true,
            cpu_weight: 100,
            memory_limit_bytes: limit,
            memory_usage_bytes: 0,
        };
        GLOBAL_CGROUPS[index] = Some(cg);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn cgroup_set_cpu_weight(idx: SigmaU32, weight: SigmaU32) -> SigmaI32 {
    let index = idx as usize;
    if index >= 16 {
        return -1;
    }
    if let Some(ref mut cg) = GLOBAL_CGROUPS[index] {
        cg.cpu_weight = weight;
        0
    } else {
        let mut cg = sigma_cgroup {
            active_subsys: 1,
            resources: 0,
            active: true,
            cpu_weight: weight,
            memory_limit_bytes: 0,
            memory_usage_bytes: 0,
        };
        GLOBAL_CGROUPS[index] = Some(cg);
        0
    }
}
