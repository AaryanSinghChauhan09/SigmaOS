/*
 * Silicon-aware scheduler profiles — Clear Linux-inspired tuning (Phase C).
 */
#include "../../include/sigma_kernel_types.h"

typedef enum {
    SIGMA_SCHED_PROFILE_BALANCED = 0,
    SIGMA_SCHED_PROFILE_PERFORMANCE = 1,
    SIGMA_SCHED_PROFILE_POWERSAVE = 2,
} sigma_sched_profile_t;

static sigma_sched_profile_t g_active = SIGMA_SCHED_PROFILE_BALANCED;

void sigma_sched_profiles_init(void) {
    g_active = SIGMA_SCHED_PROFILE_BALANCED;
}

void sigma_sched_profile_apply(sigma_sched_profile_t profile) {
    g_active = profile;
    /* Hook into sigma_scheduler.cpp: adjust timeslice, RT class ratio, P-state hint */
}

sigma_sched_profile_t sigma_sched_profile_get(void) {
    return g_active;
}

const char* sigma_sched_profile_name(void) {
    switch (g_active) {
        case SIGMA_SCHED_PROFILE_PERFORMANCE: return "performance";
        case SIGMA_SCHED_PROFILE_POWERSAVE: return "powersave";
        default: return "balanced";
    }
}
