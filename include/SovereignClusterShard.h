/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CLUSTER SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_CLUSTER_SHARD_H
#define SOVEREIGN_CLUSTER_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_cluster_join        (const char* name);
void        sigma_cluster_reconcile   (void);
void        SovereignClusterShard_Init (void);
void        SovereignClusterShard_Audit(void);

#endif /* SOVEREIGN_CLUSTER_SHARD_H */
