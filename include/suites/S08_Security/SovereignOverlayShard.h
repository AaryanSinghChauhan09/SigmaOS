/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OVERLAY SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_OVERLAY_SHARD_H
#define SOVEREIGN_OVERLAY_SHARD_H

#include "sigma_types.h"

void sigma_overlay_push        (const char* name, const char* mount, sigma_bool ro);
void sigma_overlay_merge       (void);
void SovereignOverlayShard_Init(void);
void SovereignOverlay_Audit     (void);

#endif /* SOVEREIGN_OVERLAY_SHARD_H */
