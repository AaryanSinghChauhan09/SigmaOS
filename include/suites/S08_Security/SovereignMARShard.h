/* S SIGMAOS: SOVEREIGN MAR SHARD HEADER */
#ifndef SOVEREIGN_MAR_SHARD_H
#define SOVEREIGN_MAR_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void sigma_mar_execute (sigma_u8 arch_id, const void* code_blob, sigma_sz_t size);
void SovereignMARShard_Init (void);

#endif
