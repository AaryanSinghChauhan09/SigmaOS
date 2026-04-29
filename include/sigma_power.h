/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN POWER MANAGEMENT (SPM)
 * =========================================================================
 * Mission: Zero-latency power orchestration and intelligent thermal balancing.
 * =========================================================================
 */

#ifndef SIGMA_POWER_H
#define SIGMA_POWER_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_POWER_ULTRA,
    SIGMA_POWER_BALANCED,
    SIGMA_POWER_ECO,
    SIGMA_POWER_HIBERNATE
} sigma_power_profile_t;

/* --- Power Primitives --- */
void power_init(void);
void power_set_profile(sigma_power_profile_t profile);
sigma_u32 power_get_battery_pct(void);
void power_reboot(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_POWER_H */
