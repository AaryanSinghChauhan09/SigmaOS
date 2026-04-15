/* S SIGMAOS: SOVEREIGN SIDELOAD SHARD HEADER */
#ifndef SOVEREIGN_SIDELOAD_SHARD_H
#define SOVEREIGN_SIDELOAD_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_sideload_install (const char* filepath);
void        SovereignSideloadShard_Init (void);
void        SovereignSideload_Audit     (void);

#endif
