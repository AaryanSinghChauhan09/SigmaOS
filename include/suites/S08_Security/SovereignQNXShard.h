/* S SIGMAOS: SOVEREIGN QNX SHARD HEADER */
#ifndef SOVEREIGN_QNX_SHARD_H
#define SOVEREIGN_QNX_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

void sigma_qnx_msg_send (sigma_u32 target_pid, const void* data);
void SovereignQNXShard_Init (void);

#endif
