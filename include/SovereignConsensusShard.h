/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CONSENSUS SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_CONSENSUS_SHARD_H
#define SOVEREIGN_CONSENSUS_SHARD_H

#include "sigma_types.h"

void SovereignConsensusShard_Init (void);
void sigma_quorum_elect            (void);
void sigma_quorum_replicate        (const char* entry);
void SovereignConsensus_Audit      (void);

#endif /* SOVEREIGN_CONSENSUS_SHARD_H */
