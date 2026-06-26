#ifndef SIGMA_SCHED_H
#define SIGMA_SCHED_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_SCHED_PROFILE_BALANCED = 0,
    SIGMA_SCHED_PROFILE_PERFORMANCE = 1,
    SIGMA_SCHED_PROFILE_POWERSAVE = 2,
} sigma_sched_profile_t;

void sigma_sched_init(void);
void sigma_sched_set_performance(void);
void sigma_sched_set_powersave(void);
const char* sigma_sched_active_profile_name(void);

void sigma_sched_profiles_init(void);
void sigma_sched_profile_apply(sigma_sched_profile_t profile);
const char* sigma_sched_profile_name(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SCHED_H */
