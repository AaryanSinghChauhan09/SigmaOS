/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: MULTI-CORE ORCHESTRATION (v1.0)
 * =============================================================================
 * Principles: CPU-Local State (GS-Base) & Scalable Sharding.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct CPU {
    u32     id;
    void*   current_task;
    u64     kernel_stack;
} cpu_t;

/* Set GS Base to CPU structure (Silicon-Direct) */
void cpu_init_local(cpu_t* cpu) {
    u64 addr = (u64)cpu;
    u32 low = addr & 0xFFFFFFFF;
    u32 high = addr >> 32;
    
    /* MSR_GS_BASE = 0xC0000101 */
    __asm__ __volatile__ (
        "wrmsr"
        : : "c"(0xC0000101), "a"(low), "d"(high) : "memory"
    );
}

/* Retrieve CPU ID from GS-Base */
u32 cpu_get_id() {
    u32 id;
    __asm__ __volatile__ ("mov %%gs:0, %0" : "=r"(id));
    return id;
}
