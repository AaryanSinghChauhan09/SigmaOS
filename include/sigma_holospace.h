/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HOLOGRAPHIC WORKSPACE (S-HOLOSPACE)
 * =========================================================================
 * Mission: 3D spatial computing support built directly into the kernel,
 * preparing the OS for native AR/VR head-mounted displays.
 * =========================================================================
 */

#ifndef SIGMA_HOLOSPACE_H
#define SIGMA_HOLOSPACE_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- HoloSpace Primitives --- */
void holospace_init(void);
void holospace_render_spatial_volume(uint32_t app_id, float x, float y, float z);
void holospace_update_head_tracking(float pitch, float yaw, float roll);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HOLOSPACE_H */
