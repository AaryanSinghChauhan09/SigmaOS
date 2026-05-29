/*
 * SigmaOS Meta-Distro — unified subsystem registry (competitor features as modules).
 */
#ifndef SIGMA_META_DISTRO_H
#define SIGMA_META_DISTRO_H

#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_FEATURE_GAMING      = 1u << 0,
    SIGMA_FEATURE_PERFORMANCE = 1u << 1,
    SIGMA_FEATURE_PACKAGES    = 1u << 2,
    SIGMA_FEATURE_IMMUTABLE   = 1u << 3,
    SIGMA_FEATURE_CONTAINERS  = 1u << 4,
    SIGMA_FEATURE_RECOVERY    = 1u << 5,
    SIGMA_FEATURE_DESKTOP     = 1u << 6,
} sigma_meta_feature_t;

#define SIGMA_META_ALL_FEATURES 0x7Fu

void sigma_meta_distro_init(sigma_u32 feature_mask);
const char* sigma_meta_distro_status_json(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_META_DISTRO_H */
