/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SYSTEM CALL GATE (SSG) v2.0
 * =========================================================================
 * Mission: Zero-latency, interrupt-driven shard transitions.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALL_H
#define SIGMA_SYSCALL_H

#include "./core/sigma_types.h"

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
    SIGMA_SYS_VFS_OPEN = 0x07,
    /* FIX: Missing syscall IDs now defined */
    SIGMA_SYS_EXIT     = 0x08,
    SIGMA_SYS_READ     = 0x09,
    SIGMA_SYS_WRITE    = 0x0A,
    SIGMA_SYS_CLOSE    = 0x0B,
    SIGMA_SYS_FORK     = 0x0C,
    SIGMA_SYS_EXEC     = 0x0D,
    SIGMA_SYS_WAIT     = 0x0E,
    SIGMA_SYS_GETPID   = 0x0F,
    SIGMA_SYS_KILL     = 0x10,
    SIGMA_SYS_MMAP     = 0x11,
    SIGMA_SYS_MUNMAP   = 0x12
} sigma_syscall_id_t;

/* --- System Call Primitives --- */
void      syscall_init(void);
sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);
sigma_u64 syscall_get_total_calls(void);

#ifdef __cplusplus
}

class SovereignSyscallEngine {
public:
    static SovereignSyscallEngine& getInstance() {
        static SovereignSyscallEngine instance;
        return instance;
    }

    void init();
    sigma_u32 dispatch(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);
    sigma_u64 getTotalCalls() const { return this->total_calls; }

private:
    SovereignSyscallEngine() : total_calls(0), initialized(0) {}
    
    sigma_u64 total_calls;
    sigma_u32 initialized;
};
#endif

#endif /* SIGMA_SYSCALL_H */
