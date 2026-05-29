/*
 * sigma_sched.c — Clear Linux-class scheduler bridge (profiles + fair queue).
 * Implementation body: sigma_scheduler.cpp, profiles: sigma_sched_profiles.c
 */
#include "../../include/sigma_sched.h"

void sigma_sched_init(void) {
    sigma_sched_profiles_init();
    sigma_sched_profile_apply(SIGMA_SCHED_PROFILE_BALANCED);
}

void sigma_sched_set_performance(void) {
    sigma_sched_profile_apply(SIGMA_SCHED_PROFILE_PERFORMANCE);
}

void sigma_sched_set_powersave(void) {
    sigma_sched_profile_apply(SIGMA_SCHED_PROFILE_POWERSAVE);
}

const char* sigma_sched_active_profile_name(void) {
    return sigma_sched_profile_name();
}
