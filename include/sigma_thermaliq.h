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


#ifdef __cplusplus
extern "C" {
#endif

/* --- Thermal Intelligence Primitives --- */
void thermaliq_init(void);
sigma_u32 thermaliq_get_package_temp(void);
void thermaliq_apply_thermal_policy(void);
void thermaliq_emergency_throttle(sigma_u32 threshold_celsius);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_THERMALIQ_H */
