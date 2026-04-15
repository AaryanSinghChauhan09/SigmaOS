/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN HOTPATCH SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_HOTPATCH_SHARD_H
#define SOVEREIGN_HOTPATCH_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_hotpatch_load   (const char* pid, sigma_u64 target, sigma_u64 patch);
sigma_err_t sigma_hotpatch_revert (const char* pid);
void        SovereignHotpatchShard_Init (void);
void        SovereignHotpatch_Audit     (void);

#endif /* SOVEREIGN_HOTPATCH_SHARD_H */
