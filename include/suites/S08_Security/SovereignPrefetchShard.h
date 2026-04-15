/* S SIGMAOS: SOVEREIGN PREFETCH SHARD HEADER */
#ifndef SOVEREIGN_PREFETCH_SHARD_H
#define SOVEREIGN_PREFETCH_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void        sigma_prefetch_warm    (const char* name);
void        sigma_prefetch_predict (void);
void        SovereignPrefetchShard_Init (void);
void        SovereignPrefetch_Audit     (void);

#endif
