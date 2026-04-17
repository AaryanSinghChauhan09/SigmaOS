#ifndef SIGMA_CONTAINER_RUNTIME_PRO_H
#define SIGMA_CONTAINER_RUNTIME_PRO_H

#include "../../kernel/includes/SovereignCommon.h"

// SigmaOS Advanced Container Runtime Interface
// Exceeds Linux Docker/LXC by utilizing deep hardware-level Sovereign Shards

typedef struct {
    char container_id[64];
    void* memory_namespace_ptr;
    void* cpu_affinity_mask;
    uint32_t isolation_level; // 0 = standard, 1 = strict, 2 = absolute silicon isolation
} SigmaContainer;

void init_sigma_container(SigmaContainer* container, const char* image_name, uint32_t isolation);
void start_sigma_container(SigmaContainer* container);
void stop_sigma_container(SigmaContainer* container);

#endif // SIGMA_CONTAINER_RUNTIME_PRO_H
