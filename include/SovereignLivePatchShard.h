/* Σ SIGMAOS: SOVEREIGN LIVEPATCH SHARD HEADER */
#ifndef SOVEREIGN_LIVEPATCH_SHARD_H
#define SOVEREIGN_LIVEPATCH_SHARD_H
#include "sigma_types.h"

void sigma_live_patch (const char* func_name, void* new_addr);
void SovereignLivePatchShard_Init (void);

#endif
