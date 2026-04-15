/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN CAPABILITY SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_CAPABILITY_SHARD_H
#define SOVEREIGN_CAPABILITY_SHARD_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_u32  sigma_cap_grant          (const char* resource, sigma_u32 rights);
sigma_bool sigma_cap_verify         (sigma_u32 handle, sigma_u32 required_rights);
void       SovereignCapabilityShard_Init (void);
void       SovereignCapability_Audit     (void);

#endif /* SOVEREIGN_CAPABILITY_SHARD_H */
