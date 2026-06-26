/*
 * Meta-distro boot hook — enable competitor subsystems per profile.
 */
#include "../include/sigma_meta_distro.h"

/* PROFILE_* ids match init/sigma_profile_selector.cpp */
#define PROFILE_MINIMAL   0
#define PROFILE_DEVELOPER 1
#define PROFILE_DESKTOP   2
#define PROFILE_CLOUD     3
#define PROFILE_MOBILE    4

void sigma_meta_boot_for_profile(sigma_u32 profile_id) {
    sigma_u32 mask = SIGMA_FEATURE_IMMUTABLE | SIGMA_FEATURE_PACKAGES | SIGMA_FEATURE_RECOVERY;

    switch (profile_id) {
        case PROFILE_DESKTOP:
            mask |= SIGMA_FEATURE_DESKTOP | SIGMA_FEATURE_GAMING | SIGMA_FEATURE_PERFORMANCE;
            break;
        case PROFILE_CLOUD:
            mask |= SIGMA_FEATURE_CONTAINERS | SIGMA_FEATURE_PERFORMANCE;
            break;
        case PROFILE_DEVELOPER:
            mask |= SIGMA_FEATURE_PERFORMANCE;
            break;
        case PROFILE_MOBILE:
            mask |= SIGMA_FEATURE_DESKTOP | SIGMA_FEATURE_PERFORMANCE;
            break;
        default:
            break;
    }

    sigma_meta_distro_init(mask);
}
