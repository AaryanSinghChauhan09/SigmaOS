/* S SIGMAOS: SOVEREIGN CLUSTER SHARD HEADER */
#ifndef SOVEREIGN_CLUSTER_SHARD_H
#define SOVEREIGN_CLUSTER_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_cluster_join    (const char* ip);
void        sigma_cluster_balance (void);
void        SovereignClusterShard_Init (void);
void        SovereignCluster_Audit     (void);

#endif
