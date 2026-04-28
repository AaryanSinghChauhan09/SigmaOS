#ifndef SIGMA_CLOUD_H
#define SIGMA_CLOUD_H

#include "sigma_types.h"

typedef struct {
    const char* type_name;
    sigma_u32   active_nodes;
    sigma_u32   isolated_vpcs;
} SovereignCloudOrchestrator;

void SovereignCloud_init(SovereignCloudOrchestrator* c);
void SovereignCloud_ElasticShardScale(SovereignCloudOrchestrator* c, int nodeCount);
void SovereignCloud_VirtualVPCIsolation(SovereignCloudOrchestrator* c, const char* tenantId);
void SovereignCloud_audit(const SovereignCloudOrchestrator* c);

#endif
