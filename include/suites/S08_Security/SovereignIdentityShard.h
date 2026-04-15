/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN IDENTITY SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_IDENTITY_SHARD_H
#define SOVEREIGN_IDENTITY_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_u32  sigma_id_mint           (const char* principal);
sigma_bool sigma_id_authenticate  (sigma_u32 ticket);
void       SovereignIdentityShard_Init (void);
void       SovereignIdentity_Audit      (void);

#endif /* SOVEREIGN_IDENTITY_SHARD_H */
