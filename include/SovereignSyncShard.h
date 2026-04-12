/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYNC SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SYNC_SHARD_H
#define SOVEREIGN_SYNC_SHARD_H

#include "sigma_types.h"

void SovereignSyncShard_Init (void);
void sigma_sync_push           (const char* uid);
void sigma_sync_reconcile      (void);
void SovereignSync_Audit       (void);

#endif /* SOVEREIGN_SYNC_SHARD_H */
