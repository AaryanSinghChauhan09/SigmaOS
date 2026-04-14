/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SILICON PROBE HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_SILICON_PROBE_H
#define SOVEREIGN_SILICON_PROBE_H

#include "sigma_types.h"

sigma_err_t sigma_probe_register       (const char* point);
void        sigma_probe_trigger        (const char* point, sigma_u64 latency);
void        SovereignSiliconProbe_Init (void);
void        SovereignSiliconProbe_Audit(void);

#endif /* SOVEREIGN_SILICON_PROBE_H */
