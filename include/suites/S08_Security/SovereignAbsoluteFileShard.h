/* S SIGMAOS: SOVEREIGN ABSOLUTEFILE SHARD HEADER */
#ifndef SOVEREIGN_ABSOLUTEFILE_SHARD_H
#define SOVEREIGN_ABSOLUTEFILE_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void sigma_file_lock (const char* name, const void* data, sigma_sz_t size);
void SovereignAbsoluteFileShard_Init (void);

#endif
