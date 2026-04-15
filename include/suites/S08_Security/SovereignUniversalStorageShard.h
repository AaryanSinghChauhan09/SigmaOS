/* S SIGMAOS: SOVEREIGN UNIVERSALSTORAGE SHARD HEADER */
#ifndef SOVEREIGN_UNIVERSALSTORAGE_SHARD_H
#define SOVEREIGN_UNIVERSALSTORAGE_SHARD_H
#include "sigma_types.h"

void sigma_storage_commit (const char* name, const void* data, sigma_sz_t size);
void SovereignUniversalStorageShard_Init (void);

#endif
