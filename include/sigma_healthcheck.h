/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SYSTEM HEALTH CHECK (S-HEALTHCHECK)
 * =========================================================================
 * Mission: A comprehensive periodic audit of all shard states, hardware
 * sensors, and security posture â€” producing a live system health score.
 * =========================================================================
 */

#ifndef SIGMA_HEALTHCHECK_H
#define SIGMA_HEALTHCHECK_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t health_score;   /* 0-100 */
    uint32_t active_shards;
    uint32_t thermal_celsius;
    uint32_t battery_percent;
    bool security_posture_ok;
} sigma_health_report_t;

/* --- Health Check Primitives --- */
void healthcheck_init(void);
sigma_health_report_t healthcheck_run_full_audit(void);
void healthcheck_render_dashboard(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HEALTHCHECK_H */
