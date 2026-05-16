/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN REAL-TIME CLOCK (SRTC)
 * =========================================================================
 * Mission: Silicon-native, drift-corrected time orchestration.
 * =========================================================================
 */

#ifndef SIGMA_TIME_H
#define SIGMA_TIME_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 year;
    sigma_u8 month;
    sigma_u8 day;
    sigma_u8 hour;
    sigma_u8 minute;
    sigma_u8 second;
    sigma_u64 silicon_ticks;
} sigma_time_t;

/* --- Time Primitives --- */
void time_init(void);
sigma_time_t time_now(void);
sigma_u64 time_get_uptime_ms(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TIME_H */
