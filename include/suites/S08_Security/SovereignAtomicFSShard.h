/* S SIGMAOS: SOVEREIGN ATOMICFS SHARD HEADER */
#ifndef SOVEREIGN_ATOMICFS_SHARD_H
#define SOVEREIGN_ATOMICFS_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void sigma_atomic_write (sigma_u64 atom_id, sigma_bool state);
void SovereignAtomicFSShard_Init (void);

#endif
