/* Σ SIGMAOS: SOVEREIGN BUTLER SHARD HEADER */
#ifndef SOVEREIGN_BUTLER_SHARD_H
#define SOVEREIGN_BUTLER_SHARD_H
#include "sigma_types.h"

void sigma_butler_request (const char* cmd);
void sigma_butler_tick    (void);
void SovereignButlerShard_Init (void);
void SovereignButler_Audit    (void);

#endif
