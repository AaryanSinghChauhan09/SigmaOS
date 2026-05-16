/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN INFINITE CANVAS (S-CANVAS)
 * =========================================================================
 * Mission: Break free from standard desktop boundaries by providing a 
 * hardware-accelerated, infinite 2D zoomable workspace plane.
 * =========================================================================
 */

#ifndef SIGMA_CANVAS_H
#define SIGMA_CANVAS_H

#include "../sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Infinite Canvas Primitives --- */
void canvas_init(void);
void canvas_pan(float delta_x, float delta_y);
void canvas_zoom(float delta_zoom);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CANVAS_H */
