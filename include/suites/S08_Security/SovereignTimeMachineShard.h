/* S SIGMAOS: SOVEREIGN TIMEMACHINE SHARD HEADER */
#ifndef SOVEREIGN_TIMEMACHINE_SHARD_H
#define SOVEREIGN_TIMEMACHINE_SHARD_H
#include "sigma_types.h"

void sigma_timemachine_snap    (void);
void sigma_timemachine_restore (const char* target_time);
void SovereignTimeMachineShard_Init (void);
void SovereignTimeMachine_Audit     (void);

#endif
