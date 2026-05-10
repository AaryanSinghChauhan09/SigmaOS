#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: MULTI-CORE ORCHESTRATION (v1.0)
 * =============================================================================
 * Principles: CPU-Local State (GS-Base) & Scalable Sharding.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

typedef struct CPU {
    sigma_u32     id;
    void*   current_task;
    sigma_u64     kernel_stack;
} cpu_t;

/* Set GS Base to CPU structure (Silicon-Direct) */
void cpu_init_local(cpu_t* cpu) {
    sigma_u64 addr = (sigma_u64)cpu;
    sigma_u32 low = addr & 0xFFFFFFFF;
    sigma_u32 high = addr >> 32;
    
    /* MSR_GS_BASE = 0xC0000101 */
    __asm__ __volatile__ (
        "wrmsr"
        : : "c"(0xC0000101), "a"(low), "d"(high) : "memory"
    );
}

/* Retrieve CPU ID from GS-Base */
sigma_u32 cpu_get_id() {
    sigma_u32 id;
    __asm__ __volatile__ ("mov %%gs:0, %0" : "=r"(id));
    return id;
}
