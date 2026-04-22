/* S SIGMAOS: SOVEREIGN HYPERVISOR SHARD HEADER */
#ifndef SOVEREIGN_HYPERVISOR_SHARD_H
#define SOVEREIGN_HYPERVISOR_SHARD_H
#include "sigma_types.h"

sigma_err_t sigma_hyp_create_guest (const char* os, sigma_u32 ram);
void        SovereignHypervisorShard_Init (void);
void        SovereignHypervisor_Audit     (void);

#endif
