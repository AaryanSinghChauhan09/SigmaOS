/**
 * =========================================================================
 * Σ SIGMAOS: PERFORMANCE GOVERNOR PUBLIC HEADER
 * =========================================================================
 */
#pragma once
#include "../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_PERF_POWERSAVE   = 0,
    SIGMA_PERF_BALANCED    = 1,
    SIGMA_PERF_PERFORMANCE = 2,
    SIGMA_PERF_BURST       = 3,
} sigma_perf_governor_t;

sigma_status          sigma_perf_governor_init(void);
sigma_status          sigma_perf_set_governor(sigma_perf_governor_t mode);
sigma_perf_governor_t sigma_perf_get_governor(void);
bool                  sigma_perf_has_avx512(void);
bool                  sigma_perf_has_avx2(void);
sigma_u64             sigma_perf_tsc_freq(void);
sigma_u64             sigma_perf_rdtsc_ns(void);
void                  sigma_perf_thermal_event(sigma_u32 temp_celsius);

#ifdef __cplusplus
}
#endif
