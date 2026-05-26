/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN SECURITY SANDBOX (CBAC)
 * =============================================================================
 * Mission: Capability-Based Access Control enforcing strict constraints on
 *          every process in the system.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_SANDBOX_H
#define SIGMA_SANDBOX_H

#include "../sigma_kernel_types.h"

/* Security Capabilities (Bitmask) */
#define CAP_FS_READ          BIT(0)
#define CAP_FS_WRITE         BIT(1)
#define CAP_NET_BIND         BIT(2)
#define CAP_NET_BROADCAST    BIT(3)
#define CAP_IPC_SEND         BIT(4)
#define CAP_IPC_RECV         BIT(5)
#define CAP_HARDWARE_IO      BIT(6)
#define CAP_SYS_ADMIN        BIT(7)

typedef struct {
    sigma_u32  profile_id;
    sigma_u64  capability_mask;
    sigma_bool enforce_memory_isolation;
    sigma_bool drop_privileges_on_exec;
} sigma_sandbox_profile_t;

#ifdef __cplusplus
extern "C" {
#endif

void       sandbox_init(void);
sigma_u32  sandbox_create_profile(sigma_u64 capabilities);
int        sandbox_apply_profile(sigma_u32 pid, sigma_u32 profile_id);
sigma_bool sandbox_check_capability(sigma_u32 pid, sigma_u64 requested_cap);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SANDBOX_H */
