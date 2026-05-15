/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN AMBIENT AUDIO (S-AMBIENTAUDIO)
 * =========================================================================
 * Mission: Generative background audioscapes native to the kernel to 
 * improve focus, flow states, and drown out distracting background noise.
 * =========================================================================
 */

#ifndef SIGMA_AMBIENTAUDIO_H
#define SIGMA_AMBIENTAUDIO_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    AMBIENT_THEME_BROWN_NOISE,
    AMBIENT_THEME_CAFE,
    AMBIENT_THEME_COSMIC,
    AMBIENT_THEME_RAIN
} sigma_ambient_theme_t;

/* --- Ambient Audio Primitives --- */
void ambientaudio_init(void);
void ambientaudio_set_theme(sigma_ambient_theme_t theme);
void ambientaudio_adjust_intensity(float intensity);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_AMBIENTAUDIO_H */
