/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SPATIAL AUDIO (S-SPATIALAUDIO)
 * =========================================================================
 * Mission: Hardware-accelerated 3D positional audio natively in the
 * kernel, mapping audio sources to spatial coordinates for immersive UX.
 * =========================================================================
 */

#ifndef SIGMA_SPATIALAUDIO_H
#define SIGMA_SPATIALAUDIO_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Spatial Audio Primitives --- */
void spatialaudio_init(void);
void spatialaudio_set_listener_position(float x, float y, float z);
void spatialaudio_play_source(uint32_t source_id, float x, float y, float z);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SPATIALAUDIO_H */
