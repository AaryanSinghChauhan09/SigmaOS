/* S SIGMAOS: SOVEREIGN DRM SHARD HEADER */
#ifndef SOVEREIGN_DRM_SHARD_H
#define SOVEREIGN_DRM_SHARD_H
#include "suites/S01_Genesis/shards/sigma_types.h"

sigma_u32 sigma_drm_allocate_fb   (sigma_u32 width, sigma_u32 height, sigma_u32 bpp);
void      sigma_drm_atomic_commit (sigma_u32 fb_id);
void      SovereignDRMShard_Init   (void);
void      SovereignDRM_Audit       (void);

#endif
