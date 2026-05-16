/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN DIGITAL WELLBEING (S-WELLBEING)
 * =========================================================================
 * Mission: Comprehensive screen-time tracking, app usage analytics,
 * and customizable daily limits with a beautiful visual report.
 * =========================================================================
 */

#ifndef SIGMA_WELLBEING_H
#define SIGMA_WELLBEING_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Digital Wellbeing Primitives --- */
void wellbeing_init(void);
void wellbeing_log_app_usage(uint32_t app_id, uint32_t seconds);
void wellbeing_render_daily_report(void);
void wellbeing_set_daily_limit(uint32_t app_id, uint32_t max_minutes);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_WELLBEING_H */
