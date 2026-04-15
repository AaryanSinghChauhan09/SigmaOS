/* S SIGMAOS: SOVEREIGN OPTIMAL SHARD HEADER */
#ifndef SOVEREIGN_OPTIMIZATION_SHARD_H
#define SOVEREIGN_OPTIMIZATION_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

typedef enum {
    OPT_MEM_COMPRESSION,
    OPT_PRELINK_CACHE,
    OPT_READYBOOST,
    OPT_PREDICTIVE_LMK
} SigmaOptType_t;

sigma_err_t sigma_opt_activate (SigmaOptType_t type);
void        sigma_opt_run_pass (void);
void        SovereignOptimizationShard_Init (void);
void        SovereignOptimization_Audit    (void);

#endif
