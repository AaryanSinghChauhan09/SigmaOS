// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS NUMA Topology Reader
//! Reads ACPI SRAT table entries to discover NUMA nodes and CPU proximity.
//! no_std, no alloc, no external crates. All types hand-defined.

#![no_std]
#![allow(dead_code)]

type SigmaU8   = u8;
type SigmaU32  = u32;
type SigmaU64  = u64;
type SigmaI32  = i32;
type SigmaUsize= usize;

pub const NUMA_MAX_NODES: usize = 8;
pub const NUMA_MAX_CPUS:  usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NumaNode {
    pub node_id:    SigmaU32,
    pub mem_start:  SigmaU64,
    pub mem_size:   SigmaU64,   // bytes
    pub cpu_mask:   SigmaU64,   // bitmask of CPUs on this node
    pub present:    bool,
}

static mut NUMA_NODES: [NumaNode; NUMA_MAX_NODES] = [NumaNode {
    node_id: 0, mem_start: 0, mem_size: 0, cpu_mask: 0, present: false,
}; NUMA_MAX_NODES];

static mut NUMA_NODE_COUNT: SigmaU32 = 0;

/// CPU → NUMA node affinity table
static mut CPU_NUMA_MAP: [SigmaU32; NUMA_MAX_CPUS] = [0u32; NUMA_MAX_CPUS];

#[no_mangle]
pub unsafe extern "C" fn sigma_numa_init() {
    // Simulate parsing ACPI SRAT: 2 nodes for a dual-socket system
    NUMA_NODES[0] = NumaNode {
        node_id: 0,
        mem_start: 0x0000_0000,
        mem_size: 0x4000_0000,   // 1 GiB
        cpu_mask: 0x0000_00FF,   // CPUs 0-7
        present: true,
    };
    NUMA_NODES[1] = NumaNode {
        node_id: 1,
        mem_start: 0x4000_0000,
        mem_size: 0x4000_0000,   // 1 GiB
        cpu_mask: 0x0000_FF00,   // CPUs 8-15
        present: true,
    };
    NUMA_NODE_COUNT = 2;

    // Populate CPU affinity map
    for cpu in 0..8usize  { CPU_NUMA_MAP[cpu] = 0; }
    for cpu in 8..16usize { CPU_NUMA_MAP[cpu] = 1; }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_numa_node_count() -> SigmaU32 {
    NUMA_NODE_COUNT
}

/// Returns the NUMA node ID for a given CPU index.
#[no_mangle]
pub unsafe extern "C" fn sigma_numa_cpu_to_node(cpu: SigmaU32) -> SigmaU32 {
    if (cpu as usize) < NUMA_MAX_CPUS { CPU_NUMA_MAP[cpu as usize] } else { 0 }
}

/// Returns a pointer to the NumaNode descriptor for a given node ID.
#[no_mangle]
pub unsafe extern "C" fn sigma_numa_get_node(node_id: SigmaU32) -> *const NumaNode {
    if (node_id as usize) < NUMA_MAX_NODES && NUMA_NODES[node_id as usize].present {
        &NUMA_NODES[node_id as usize] as *const NumaNode
    } else {
        core::ptr::null()
    }
}

/// Selects the best NUMA node for a new memory allocation given a CPU hint.
#[no_mangle]
pub unsafe extern "C" fn sigma_numa_preferred_node(cpu_hint: SigmaU32) -> SigmaU32 {
    sigma_numa_cpu_to_node(cpu_hint)
}
