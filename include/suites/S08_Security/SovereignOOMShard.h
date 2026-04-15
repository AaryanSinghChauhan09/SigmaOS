/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN OOM SHARD HEADER
 * =========================================================================
 */
#ifndef SOVEREIGN_OOM_SHARD_H
#define SOVEREIGN_OOM_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_oom_register (const char* name, sigma_u32 pid,
                                  sigma_u64 mem_kb, sigma_i32 score,
                                  sigma_bool prot);
void        sigma_oom_sweep    (sigma_u64 free_mem_kb);
void        SovereignOOMShard_Init (void);
void        SovereignOOM_Audit      (void);

#endif /* SOVEREIGN_OOM_SHARD_H */
