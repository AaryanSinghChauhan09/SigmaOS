/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN HAPTIC ENGINE (S-HAPTIC)
 * =========================================================================
 * Mission: A dedicated kernel-level API for precise, immersive haptic 
 * feedback on supported hardware (trackpads, controllers, wearables).
 * =========================================================================
 */

#ifndef SIGMA_HAPTIC_H
#define SIGMA_HAPTIC_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    HAPTIC_PATTERN_CLICK,
    HAPTIC_PATTERN_BUZZ,
    HAPTIC_PATTERN_HEARTBEAT,
    HAPTIC_PATTERN_ERROR_THUD
} sigma_haptic_pattern_t;

/* --- Haptic Primitives --- */
void haptic_init(void);
void haptic_play_pattern(sigma_haptic_pattern_t pattern, float intensity);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_HAPTIC_H */
