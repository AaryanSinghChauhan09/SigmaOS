/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN AESTHETIC ENGINE HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_AESTHETIC_SHARD_H
#define SOVEREIGN_AESTHETIC_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

void sigma_aesthetic_apply_glass   (sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h);
void sigma_aesthetic_set_theme     (const char* name, sigma_u32 color, sigma_u32 blur);
void SovereignAestheticShard_Init (void);
void SovereignAesthetic_Audit     (void);

#endif /* SOVEREIGN_AESTHETIC_SHARD_H */
