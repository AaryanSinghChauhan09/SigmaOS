/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN TWM SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_TWM_SHARD_H
#define SOVEREIGN_TWM_SHARD_H

#include "sigma_types.h"

void SovereignTWMShard_Init (void);
void sigma_twm_recalculate   (void);
void sigma_twm_add           (sigma_u32 win_id);
void SovereignTWM_Audit       (void);

#endif /* SOVEREIGN_TWM_SHARD_H */
