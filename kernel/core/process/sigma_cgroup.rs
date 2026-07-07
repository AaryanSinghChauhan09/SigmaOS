/// SigmaOS: sigma_cgroup.rs - Control Groups (cgroups) implementation
/// Inspired by Linux cgroups v2 for resource isolation and management
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

const MAX_CGROUPS: usize = 256;
const MAX_PROCS_PER_CGROUP: usize = 128;

/// Cgroup controller types
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum CgroupController {
    CPU = 0,
    Memory = 1,
    IO = 2,
    Pids = 3,
    CpuSet = 4,
    RDMA = 5,
    HugeTLB = 6,
}

/// Cgroup state
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub enum CgroupState {
    Created = 0,
    Active = 1,
    Frozen = 2,
    Deleted = 3,
}

/// Cgroup statistics
#[repr(C)]
pub struct CgroupStats {
    pub cpu_usage_ns: SigmaU64,
    pub cpu_user_ns: SigmaU64,
    pub cpu_system_ns: SigmaU64,
    pub memory_usage_bytes: SigmaU64,
    pub memory_peak_bytes: SigmaU64,
    pub memory_swap_bytes: SigmaU64,
    pub memory_cache_bytes: SigmaU64,
    pub io_read_bytes: SigmaU64,
    pub io_write_bytes: SigmaU64,
    pub pids_current: SigmaU32,
    pub pids_max: SigmaI32,
}

/// Cgroup configuration
#[repr(C)]
pub struct CgroupConfig {
    pub cpu_weight: SigmaU32,
    pub cpu_max: SigmaI64,  // -1 for max
    pub cpu_period: SigmaU64,
    pub memory_max: SigmaU64,
    pub memory_swap_max: SigmaU64,
    pub pids_max: SigmaI32,
    pub io_weight: SigmaU16,
    pub cpus_allowed: SigmaU64,  // Bitmask
    pub mems_allowed: SigmaU64,  // Bitmask
}

/// Cgroup structure
#[repr(C)]
pub struct sigma_cgroup {
    pub id: SigmaU32,
    pub name: [SigmaU8; 64],
    pub parent_id: SigmaU32,
    pub state: CgroupState,
    pub config: CgroupConfig,
    pub stats: CgroupStats,
    pub processes: [SigmaI32; MAX_PROCS_PER_CGROUP],
    pub proc_count: SigmaU32,
    pub active_controllers: SigmaU32,
}

static mut GLOBAL_CGROUPS: [Option<sigma_cgroup>; MAX_CGROUPS] = [None; MAX_CGROUPS];
static mut NEXT_CGROUP_ID: SigmaU32 = 1;

/// Initialize cgroup subsystem
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_init() -> SigmaI32 {
    for i in 0..MAX_CGROUPS {
        GLOBAL_CGROUPS[i] = None;
    }
    NEXT_CGROUP_ID = 1;
    0
}

/// Create new cgroup
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_create(
    name: *const SigmaU8,
    parent_id: SigmaU32,
    controllers: SigmaU32,
) -> SigmaI32 {
    if name.is_null() {
        return -1;
    }

    // Find free slot
    let mut idx = -1;
    for i in 0..MAX_CGROUPS {
        if GLOBAL_CGROUPS[i].is_none() {
            idx = i as SigmaI32;
            break;
        }
    }

    if idx < 0 {
        return -2; // No space
    }

    let id = NEXT_CGROUP_ID;
    NEXT_CGROUP_ID += 1;

    let mut cg = sigma_cgroup {
        id: id,
        name: [0; 64],
        parent_id: parent_id,
        state: CgroupState::Created,
        config: CgroupConfig {
            cpu_weight: 100,
            cpu_max: -1,
            cpu_period: 100000,
            memory_max: 0,
            memory_swap_max: 0,
            pids_max: -1,
            io_weight: 100,
            cpus_allowed: 0xFFFFFFFFFFFFFFFF,
            mems_allowed: 0xFFFFFFFFFFFFFFFF,
        },
        stats: CgroupStats {
            cpu_usage_ns: 0,
            cpu_user_ns: 0,
            cpu_system_ns: 0,
            memory_usage_bytes: 0,
            memory_peak_bytes: 0,
            memory_swap_bytes: 0,
            memory_cache_bytes: 0,
            io_read_bytes: 0,
            io_write_bytes: 0,
            pids_current: 0,
            pids_max: -1,
        },
        processes: [0; MAX_PROCS_PER_CGROUP],
        proc_count: 0,
        active_controllers: controllers,
    };

    // Copy name
    let mut i = 0;
    while i < 63 && *name.add(i) != 0 {
        cg.name[i] = *name.add(i);
        i += 1;
    }
    cg.name[i] = 0;

    GLOBAL_CGROUPS[idx as usize] = Some(cg);
    SigmaI32(id)
}

/// Delete cgroup
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_delete(id: SigmaU32) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3; // Not found
    }

    let idx = idx as usize;
    if let Some(ref cg) = GLOBAL_CGROUPS[idx] {
        if cg.proc_count > 0 {
            return -4; // Has processes
        }
    }

    GLOBAL_CGROUPS[idx] = None;
    0
}

/// Add process to cgroup
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_add_proc(cgroup_id: SigmaU32, pid: SigmaI32) -> SigmaI32 {
    let idx = find_cgroup_index(cgroup_id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        if cg.proc_count >= MAX_PROCS_PER_CGROUP as SigmaU32 {
            return -5; // Full
        }

        cg.processes[cg.proc_count as usize] = pid;
        cg.proc_count += 1;
        cg.state = CgroupState::Active;

        // Wire to scheduler
        extern "C" {
            fn sigma_sched_set_cgroup(pid: SigmaI32, cgroup_id: SigmaU32) -> SigmaI32;
        }
        sigma_sched_set_cgroup(pid, cgroup_id);

        0
    } else {
        -3
    }
}

/// Remove process from cgroup
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_remove_proc(cgroup_id: SigmaU32, pid: SigmaI32) -> SigmaI32 {
    let idx = find_cgroup_index(cgroup_id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        let mut found = false;
        let mut found_idx = 0;
        for i in 0..cg.proc_count as usize {
            if cg.processes[i] == pid {
                found = true;
                found_idx = i;
                break;
            }
        }

        if !found {
            return -6; // Process not in cgroup
        }

        // Shift remaining processes
        for i in found_idx..(cg.proc_count as usize - 1) {
            cg.processes[i] = cg.processes[i + 1];
        }
        cg.proc_count -= 1;

        if cg.proc_count == 0 {
            cg.state = CgroupState::Created;
        }

        0
    } else {
        -3
    }
}

/// Set CPU weight (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_set_cpu_weight(id: SigmaU32, weight: SigmaU32) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        cg.config.cpu_weight = weight;

        // Wire to scheduler
        extern "C" {
            fn sigma_sched_set_cgroup_cpu_weight(cgroup_id: SigmaU32, weight: SigmaU32) -> SigmaI32;
        }
        sigma_sched_set_cgroup_cpu_weight(id, weight);

        0
    } else {
        -3
    }
}

/// Set memory limit (wired to memory manager)
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_set_memory_limit(id: SigmaU32, limit: SigmaU64) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        cg.config.memory_max = limit;

        // Wire to memory manager
        extern "C" {
            fn sigma_mm_set_cgroup_memory_limit(cgroup_id: SigmaU32, limit: SigmaU64) -> SigmaI32;
        }
        sigma_mm_set_cgroup_memory_limit(id, limit);

        0
    } else {
        -3
    }
}

/// Set PIDs limit
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_set_pids_limit(id: SigmaU32, limit: SigmaI32) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        cg.config.pids_max = limit;
        cg.stats.pids_max = limit;
        0
    } else {
        -3
    }
}

/// Freeze cgroup (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_freeze(id: SigmaU32) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        cg.state = CgroupState::Frozen;

        // Wire to scheduler to freeze all processes
        extern "C" {
            fn sigma_sched_freeze_cgroup(cgroup_id: SigmaU32) -> SigmaI32;
        }
        sigma_sched_freeze_cgroup(id);

        0
    } else {
        -3
    }
}

/// Thaw cgroup (wired to scheduler)
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_thaw(id: SigmaU32) -> SigmaI32 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref mut cg) = GLOBAL_CGROUPS[idx] {
        cg.state = CgroupState::Active;

        // Wire to scheduler to thaw all processes
        extern "C" {
            fn sigma_sched_thaw_cgroup(cgroup_id: SigmaU32) -> SigmaI32;
        }
        sigma_sched_thaw_cgroup(id);

        0
    } else {
        -3
    }
}

/// Get cgroup statistics
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_get_stats(id: SigmaU32, stats: *mut CgroupStats) -> SigmaI32 {
    if stats.is_null() {
        return -1;
    }

    let idx = find_cgroup_index(id);
    if idx < 0 {
        return -3;
    }

    let idx = idx as usize;
    if let Some(ref cg) = GLOBAL_CGROUPS[idx] {
        // Update stats from kernel subsystems
        extern "C" {
            fn sigma_sched_get_cgroup_cpu_usage(cgroup_id: SigmaU32) -> SigmaU64;
            fn sigma_mm_get_cgroup_memory_usage(cgroup_id: SigmaU32) -> SigmaU64;
        }

        let mut updated_stats = cg.stats;
        updated_stats.cpu_usage_ns = sigma_sched_get_cgroup_cpu_usage(id);
        updated_stats.memory_usage_bytes = sigma_mm_get_cgroup_memory_usage(id);
        updated_stats.pids_current = cg.proc_count;

        *stats = updated_stats;
        0
    } else {
        -3
    }
}

/// Set CPU shares (legacy API, maps to weight)
#[no_mangle]
pub unsafe extern "C" fn cgroup_set_cpu_weight(idx: SigmaU32, weight: SigmaU32) -> SigmaI32 {
    sigma_cgroup_set_cpu_weight(idx, weight)
}

/// Set memory limit (legacy API)
#[no_mangle]
pub unsafe extern "C" fn cgroup_set_memory_limit(idx: SigmaU32, limit: SigmaU64) -> SigmaI32 {
    sigma_cgroup_set_memory_limit(idx, limit)
}

/// Get CPU usage
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_get_cpu_usage(id: SigmaU32) -> SigmaU64 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return 0;
    }

    let idx = idx as usize;
    if let Some(ref cg) = GLOBAL_CGROUPS[idx] {
        extern "C" {
            fn sigma_sched_get_cgroup_cpu_usage(cgroup_id: SigmaU32) -> SigmaU64;
        }
        sigma_sched_get_cgroup_cpu_usage(id)
    } else {
        0
    }
}

/// Get memory usage
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_get_memory_usage(id: SigmaU32) -> SigmaU64 {
    let idx = find_cgroup_index(id);
    if idx < 0 {
        return 0;
    }

    let idx = idx as usize;
    if let Some(ref cg) = GLOBAL_CGROUPS[idx] {
        extern "C" {
            fn sigma_mm_get_cgroup_memory_usage(cgroup_id: SigmaU32) -> SigmaU64;
        }
        sigma_mm_get_cgroup_memory_usage(id)
    } else {
        0
    }
}

/// Helper: Find cgroup index by ID
unsafe fn find_cgroup_index(id: SigmaU32) -> SigmaI32 {
    for i in 0..MAX_CGROUPS {
        if let Some(ref cg) = GLOBAL_CGROUPS[i] {
            if cg.id == id {
                return i as SigmaI32;
            }
        }
    }
    -1
}

/// Check if cgroup exists
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_exists(id: SigmaU32) -> SigmaBool {
    find_cgroup_index(id) >= 0
}

/// Get cgroup by name
#[no_mangle]
pub unsafe extern "C" fn sigma_cgroup_get_by_name(name: *const SigmaU8) -> SigmaI32 {
    if name.is_null() {
        return -1;
    }

    for i in 0..MAX_CGROUPS {
        if let Some(ref cg) = GLOBAL_CGROUPS[i] {
            let mut match = true;
            let mut j = 0;
            while j < 64 {
                if cg.name[j] == 0 && *name.add(j) == 0 {
                    break;
                }
                if cg.name[j] != *name.add(j) {
                    match = false;
                    break;
                }
                j += 1;
            }
            if match {
                return SigmaI32(cg.id);
            }
        }
    }
    -3
}

/// Initialize legacy API
#[no_mangle]
pub unsafe extern "C" fn cgroup_init() -> SigmaI32 {
    sigma_cgroup_init()
}
