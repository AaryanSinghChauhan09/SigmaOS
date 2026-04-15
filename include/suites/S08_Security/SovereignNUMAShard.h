/* S SIGMAOS: SOVEREIGN NUMA SHARD HEADER */
#ifndef SOVEREIGN_NUMA_SHARD_H
#define SOVEREIGN_NUMA_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"
sigma_err_t sigma_numa_add_node (sigma_u32 cpu_mask, sigma_u64 mem_mb);
sigma_u32   sigma_numa_alloc    (sigma_u32 preferred_node, sigma_u64 size_mb);
void        sigma_numa_balance  (void);
void        SovereignNUMAShard_Init (void);
void        SovereignNUMA_Audit      (void);
#endif
