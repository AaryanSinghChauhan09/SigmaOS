/* S SIGMAOS: SOVEREIGN GARBAGE SHARD HEADER */
#ifndef SOVEREIGN_GARBAGE_SHARD_H
#define SOVEREIGN_GARBAGE_SHARD_H
#include "sigma_types.h"

void        sigma_gc_sweep     (void);
void        sigma_gc_proactive (void);
void        SovereignGarbageShard_Init (void);
void        SovereignGarbage_Audit    (void);

#endif
