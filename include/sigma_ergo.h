/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ADAPTIVE ERGONOMICS (S-ERGO)
 * =========================================================================
 * Mission: A built-in OS layer that monitors user strain, automatically
 * adjusting color temperatures, brightness, and enforcing micro-breaks.
 * =========================================================================
 */

#ifndef SIGMA_ERGO_H
#define SIGMA_ERGO_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Adaptive Ergonomics Primitives --- */
void ergo_init(void);
void ergo_update_screen_temperature(uint32_t kelvin);
void ergo_evaluate_fatigue(uint32_t active_minutes);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ERGO_H */
