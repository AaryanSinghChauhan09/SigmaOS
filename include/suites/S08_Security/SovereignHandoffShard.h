/* S SIGMAOS: SOVEREIGN HANDOFF SHARD HEADER */
#ifndef SOVEREIGN_HANDOFF_SHARD_H
#define SOVEREIGN_HANDOFF_SHARD_H
#include "sigma_types.h"

void sigma_handoff_push (const char* app_context);
void sigma_handoff_pull (void);
void SovereignHandoffShard_Init (void);
void SovereignHandoff_Audit     (void);

#endif
