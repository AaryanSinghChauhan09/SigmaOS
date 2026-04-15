/* S SIGMAOS: SOVEREIGN EMULATION SHARD HEADER */
#ifndef SOVEREIGN_EMULATION_SHARD_H
#define SOVEREIGN_EMULATION_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_emulate_run (const char* binary_arch);
void        SovereignEmulationShard_Init (void);

#endif
