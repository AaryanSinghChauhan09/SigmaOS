/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN AMNESIC SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_AMNESIC_SHARD_H
#define SOVEREIGN_AMNESIC_SHARD_H

#include "sigma_types.h"

void sigma_amnesic_register    (sigma_u64 addr, sigma_u32 pages);
void sigma_amnesic_scrub       (void);
void SovereignAmnesicShard_Init(void);
void SovereignAmnesic_Audit     (void);

#endif /* SOVEREIGN_AMNESIC_SHARD_H */
