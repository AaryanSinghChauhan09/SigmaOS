/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN CGROUP SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_CGROUP_SHARD_H
#define SOVEREIGN_CGROUP_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_cgroup_create  (const char* name, sigma_u32 cpu_quota,
                                   sigma_u64 mem_limit, sigma_u32 io_weight);
void        sigma_cgroup_enforce  (void);
void        SovereignCgroupShard_Init (void);
void        SovereignCgroup_Audit      (void);

#endif /* SOVEREIGN_CGROUP_SHARD_H */
