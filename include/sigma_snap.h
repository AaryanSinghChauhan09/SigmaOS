/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN WINDOW SNAPPING (S-SNAP)
 * =========================================================================
 * Mission: Intelligent, AI-assisted window tiling and snapping that
 * learns the user's preferred layouts and auto-arranges applications.
 * =========================================================================
 */

#ifndef SIGMA_SNAP_H
#define SIGMA_SNAP_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SNAP_ZONE_LEFT_HALF,
    SNAP_ZONE_RIGHT_HALF,
    SNAP_ZONE_TOP_HALF,
    SNAP_ZONE_BOTTOM_HALF,
    SNAP_ZONE_QUARTER_TL,
    SNAP_ZONE_QUARTER_TR,
    SNAP_ZONE_QUARTER_BL,
    SNAP_ZONE_QUARTER_BR,
    SNAP_ZONE_CENTER_FLOAT,
    SNAP_ZONE_FULLSCREEN
} sigma_snap_zone_t;

/* --- Window Snapping Primitives --- */
void snap_init(void);
void snap_window_to_zone(uint32_t window_id, sigma_snap_zone_t zone);
void snap_auto_arrange(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SNAP_H */
