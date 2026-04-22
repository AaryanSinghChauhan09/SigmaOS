/* S SIGMAOS: SOVEREIGN QUBES SHARD HEADER */
#ifndef SOVEREIGN_QUBES_SHARD_H
#define SOVEREIGN_QUBES_SHARD_H
#include "sigma_types.h"

void sigma_qubes_isolate (const char* group_name, sigma_u32 security_level);
void SovereignQubesShard_Init (void);

#endif
