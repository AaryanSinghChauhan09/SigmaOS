/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM CALL GATE (SSG)
 * =========================================================================
 * Mission: Zero-latency, interrupt-driven shard transitions.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALL_H
#define SIGMA_SYSCALL_H

#include <sigma_types.h>

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
sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SYSCALL_H */
