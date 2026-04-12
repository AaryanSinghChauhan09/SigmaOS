/* Σ SIGMAOS: SOVEREIGN SANDBOX SHARD HEADER */
#ifndef SOVEREIGN_SANDBOX_SHARD_H
#define SOVEREIGN_SANDBOX_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_sandbox_enforce  (sigma_u32 pid, const char* profile);
void        sigma_sandbox_audit_pid (sigma_u32 pid);
void        SovereignSandboxShard_Init   (void);
void        SovereignSandbox_Audit       (void);

#endif
