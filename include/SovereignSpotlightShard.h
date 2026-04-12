/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SPOTLIGHT SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SPOTLIGHT_SHARD_H
#define SOVEREIGN_SPOTLIGHT_SHARD_H

#include "sigma_types.h"

void sigma_spotlight_index (const char* name, const char* info);
void sigma_spotlight_query (const char* query);
void SovereignSpotlightShard_Init (void);
void SovereignSpotlight_Audit     (void);

#endif /* SOVEREIGN_SPOTLIGHT_SHARD_H */
