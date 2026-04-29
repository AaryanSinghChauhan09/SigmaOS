/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN BATTERY INTELLIGENCE (S-BATTERYIQ)
 * =========================================================================
 * Mission: Ultra-detailed battery health, charge cycle tracking, and
 * intelligent power routing to maximize hardware longevity.
 * =========================================================================
 */

#ifndef SIGMA_BATTERYIQ_H
#define SIGMA_BATTERYIQ_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Battery Intelligence Primitives --- */
void batteryiq_init(void);
uint32_t batteryiq_get_health_percent(void);
void batteryiq_optimize_charge(void);
void batteryiq_render_report(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_BATTERYIQ_H */
