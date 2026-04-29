/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN THERMAL INTELLIGENCE (S-THERMALIQ)
 * =========================================================================
 * Mission: Precision silicon temperature monitoring with predictive thermal
 * throttling to prevent degradation before it happens.
 * =========================================================================
 */

#ifndef SIGMA_THERMALIQ_H
#define SIGMA_THERMALIQ_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Thermal Intelligence Primitives --- */
void thermaliq_init(void);
uint32_t thermaliq_get_package_temp(void);
void thermaliq_apply_thermal_policy(void);
void thermaliq_emergency_throttle(uint32_t threshold_celsius);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_THERMALIQ_H */
