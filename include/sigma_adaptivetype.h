/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ADAPTIVE TYPOGRAPHY (S-ADAPTIVETYPE)
 * =========================================================================
 * Mission: Real-time, continuous adjustment of UI font weight, scale, and 
 * spacing based on user's eye-distance and screen resolution.
 * =========================================================================
 */

#ifndef SIGMA_ADAPTIVETYPE_H
#define SIGMA_ADAPTIVETYPE_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Adaptive Typography Primitives --- */
void adaptivetype_init(void);
void adaptivetype_recalculate(float user_distance_cm, uint32_t dpi);
void adaptivetype_render_glyph(char c, uint32_t x, uint32_t y);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ADAPTIVETYPE_H */
