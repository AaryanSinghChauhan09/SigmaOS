/* Σ SIGMAOS: SOVEREIGN REGISTRY SHARD HEADER */
#ifndef SOVEREIGN_REGISTRY_SHARD_H
#define SOVEREIGN_REGISTRY_SHARD_H
#include "sigma_types.h"

void        sigma_registry_set   (const char* key, const char* value);
const char* sigma_registry_query (const char* key);
void        SovereignRegistryShard_Init (void);

#endif
