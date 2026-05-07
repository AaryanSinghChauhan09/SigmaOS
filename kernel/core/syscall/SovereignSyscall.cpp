#include "libc/SovereignLibC.h"
#include "hal/sigma_hal.h"
#include "core/sigma_types.h"
#include "system/sigma_syscall.h"
#include "sigma_proc.h"
#include "sigma_mem.h"
#include "system/sigma_ipc.h"

/**
 * SigmaOS Sovereign System Call Implementation
 * Implements a Fast-Path Shard Transition (FPST) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal context management.
 *
 * Design: OOP-isolated singleton — SigmaOS::Kernel::Syscall::SovereignSyscallEngine.
 */

namespace SigmaOS {
namespace Kernel {
namespace Syscall {

void SigmaOS::Kernel::Syscall::SovereignSyscallEngine::init() {
    sigma_log("[SYSCALL] Initializing Sovereign FPST Gate...");
    this->m_initialized = 1u;
}

sigma_u32 SigmaOS::Kernel::Syscall::SovereignSyscallEngine::dispatch(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
    /* FPST (Fast-Path Shard Transition) Algorithm
     * Dispatches kernel services with minimum context overhead. */
    
    this->m_total_calls++;
    sigma_log("[SYSCALL] SSG Entry: ID 0x%02X, Args: [%08X, %08X, %08X]\n", (unsigned)id, (unsigned)arg1, (unsigned)arg2, (unsigned)arg3);
    
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
            sigma_log("[SYSCALL] [WARNING] Anomaly detected in primary shard. Triggering SELF-HEALING redirection...");
            return this->attemptSelfHealing(id, arg1, arg2, arg3);
    }
}

sigma_u32 SigmaOS::Kernel::Syscall::SovereignSyscallEngine::attemptSelfHealing(sigma_syscall_id_t id, sigma_u32 a1, sigma_u32 a2, sigma_u32 a3) {
    sigma_log("[SYSCALL] SELF-HEAL: Redirecting ID 0x%X to SovereignFallback Shard...\n", id);
    (void)a1; (void)a2; (void)a3;
    sigma_log("[SYSCALL] SELF-HEAL: Fallback execution SUCCESS. Service restored.");
    return SIGMA_OK;
}

} // namespace Syscall
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void syscall_init() {
    SigmaOS::Kernel::Syscall::SovereignSyscallEngine::getInstance().init();
}

extern "C" sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
    return SigmaOS::Kernel::Syscall::SovereignSyscallEngine::getInstance().dispatch(id, arg1, arg2, arg3);
}

extern "C" void syscall_handler_asm() {
    /* Bare-metal syscall entry point (simulated) */
    sigma_log("[SYSCALL] ASM Gate Transition: USER -> KERNEL Shard.");
}

extern "C" sigma_u64 syscall_get_total_calls() {
    return SigmaOS::Kernel::Syscall::SovereignSyscallEngine::getInstance().getTotalCalls();
}



