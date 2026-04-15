/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SILICON CONTAINER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SILICON_CONTAINER_H
#define SOVEREIGN_SILICON_CONTAINER_H

#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_err_t sigma_container_spawn          (const char* name, sigma_u64 memory_limit);
void        SovereignSiliconContainer_Init (void);
void        SovereignSiliconContainer_Audit(void);

#endif /* SOVEREIGN_SILICON_CONTAINER_H */
