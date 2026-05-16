#ifndef SIGMA_UI_TYPES_H
#define SIGMA_UI_TYPES_H

#include "./core/sigma_types.h"

/* 
 * =========================================================================
 * SIGMAOS: SOVEREIGN UI TYPE SHARD
 * =========================================================================
 */

typedef struct SovereignUIEngine {
    const char* type_name;
    sigma_u32   layers_composited;
    sigma_u32   fps_zenith;
    sigma_bool  glass_blur_active;
    sigma_u64   frames_rendered;
} SovereignUIEngine;

#endif /* SIGMA_UI_TYPES_H */

