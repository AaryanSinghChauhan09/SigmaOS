/* S SIGMAOS: SOVEREIGN FLOW SHARD HEADER */
#ifndef SOVEREIGN_FLOW_SHARD_H
#define SOVEREIGN_FLOW_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_flow_register (const char* trigger, const char* action);
void        sigma_flow_trigger  (const char* trigger);
void        SovereignFlowShard_Init   (void);
void        SovereignFlow_Audit       (void);

#endif
