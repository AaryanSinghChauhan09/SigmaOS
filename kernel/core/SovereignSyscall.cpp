#include "Lattice.h"
#include "sigma_syscall.h"
#include "sigma_hal.h"
#include "sigma_proc.h"
#include "sigma_mem.h"
#include "sigma_ipc.h"

/**
 * SigmaOS Sovereign System Call Implementation
 * Implements a Fast-Path Shard Transition (FPST) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal context management.
 */

#include "Lattice.h"
#include "sigma_syscall.h"
#include "sigma_proc.h"
#include "sigma_mem.h"
#include "sigma_ipc.h"

/**
 * SigmaOS Sovereign System Call Implementation
 * Implements a Fast-Path Shard Transition (FPST) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal context management.
 *
 * Design: OOP-isolated singleton — SovereignSyscallEngine.
 */

/* --- Sovereign Syscall Engine (OOP Isolation) --- */
static struct {
    sigma_u64 total_calls;
    sigma_u32 initialized;
} SovereignSyscallEngine = {
    .total_calls = 0u,
    .initialized = 0u
};

extern "C" void syscall_init() {
    sigma_log("[SYSCALL] Initializing Sovereign FPST Gate...");
    SovereignSyscallEngine.initialized = 1u;
}

extern "C" sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
    /* FPST (Fast-Path Shard Transition) Algorithm
     * Dispatches kernel services with minimum context overhead. */
    
    SovereignSyscallEngine.total_calls++;
    sigma_printf("[SYSCALL] SSG Entry: ID 0x%02X, Args: [%08X, %08X, %08X]\n", (unsigned)id, (unsigned)arg1, (unsigned)arg2, (unsigned)arg3);
    
    switch (id) {
        case SIGMA_SYS_YIELD:
            proc_yield();
            return SIGMA_OK;
            
        case SIGMA_SYS_MALLOC:
            return (sigma_u32)(sigma_addr_t)sigma_malloc((sigma_size_t)arg1);
            
        case SIGMA_SYS_FREE:
            sigma_free((void*)(sigma_addr_t)arg1);
            return SIGMA_OK;
            
        case SIGMA_SYS_SEND:
            /* Standardised to optimized WFAE IPC */
            return (sigma_u32)ipc_send_optimized(arg1, arg2, (sigma_u32*)(sigma_addr_t)arg3);
            
        default:
            sigma_log("[SYSCALL] [ERROR] Unknown Sovereign Syscall ID.");
            return SIGMA_ERROR;
    }
}

extern "C" void syscall_handler_asm() {
    /* Bare-metal syscall entry point (simulated) */
    sigma_log("[SYSCALL] ASM Gate Transition: USER -> KERNEL Shard.");
}

extern "C" sigma_u64 syscall_get_total_calls() {
    return SovereignSyscallEngine.total_calls;
}
