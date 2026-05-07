/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM CALL GATE (SSG)
 * =========================================================================
 * Mission: Zero-latency, interrupt-driven shard transitions.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALL_H
#define SIGMA_SYSCALL_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_SYS_YIELD    = 0x01,
    SIGMA_SYS_SPAWN    = 0x02,
    SIGMA_SYS_MALLOC   = 0x03,
    SIGMA_SYS_FREE     = 0x04,
    SIGMA_SYS_SEND     = 0x05,
    SIGMA_SYS_RECEIVE  = 0x06,
    SIGMA_SYS_VFS_OPEN = 0x07
} sigma_syscall_id_t;

/* --- System Call Primitives --- */
void      syscall_init(void);
sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);
sigma_u64 syscall_get_total_calls(void);

#ifdef __cplusplus
}

class SigmaOS::Kernel::Syscall::SovereignSyscallEngine {
public:
    static SigmaOS::Kernel::Syscall::SovereignSyscallEngine& getInstance() {
        static SigmaOS::Kernel::Syscall::SovereignSyscallEngine instance;
        return instance;
    }

    void init();
    sigma_u32 dispatch(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);
    sigma_u64 getTotalCalls() const { return this->total_calls; }

private:
    SigmaOS::Kernel::Syscall::SovereignSyscallEngine() : total_calls(0), initialized(0) {}
    
    sigma_u64 total_calls;
    sigma_u32 initialized;
};
#endif

#endif /* SIGMA_SYSCALL_H */
