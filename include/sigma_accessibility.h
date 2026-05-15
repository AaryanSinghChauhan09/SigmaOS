/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ACCESSIBILITY SERVICE (S-ACCESS)
 * =========================================================================
 * Mission: Universal access — screen reader, magnifier, colour-correct,
 *          input-assist — at the silicon kernel layer.
 * Competitor parity: GNOME Orca / Windows Narrator / macOS VoiceOver.
 * ZERO-DEPENDENCY: No HLL runtime. Bare-metal silicon accessibility.
 * =========================================================================
 */

#ifndef SIGMA_ACCESSIBILITY_H
#define SIGMA_ACCESSIBILITY_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Accessibility Feature Flags --- */
#define SIGMA_ACCESS_SCREEN_READER   (1u << 0)
#define SIGMA_ACCESS_MAGNIFIER       (1u << 1)
#define SIGMA_ACCESS_HIGH_CONTRAST   (1u << 2)
#define SIGMA_ACCESS_STICKY_KEYS     (1u << 3)
#define SIGMA_ACCESS_SLOW_KEYS       (1u << 4)
#define SIGMA_ACCESS_COLOUR_CORRECT  (1u << 5)
#define SIGMA_ACCESS_HAPTIC_ASSIST   (1u << 6)
#define SIGMA_ACCESS_VOICE_INPUT     (1u << 7)

typedef struct {
    sigma_u32 features_active;   /* Bitmask of SIGMA_ACCESS_* flags  */
    sigma_u32 magnifier_percent; /* 100 = 1x, 200 = 2x               */
    sigma_u32 speech_rate_wpm;   /* Words-per-minute for screen reader*/
    sigma_u32 colour_mode;       /* 0 = Normal, 1 = Deuteranopia, …   */
} sigma_accessibility_config_t;

/* --- Accessibility Primitives --- */
void accessibility_init(void);
void accessibility_enable(sigma_u32 feature_flags);
void accessibility_disable(sigma_u32 feature_flags);
void accessibility_set_speech_rate(sigma_u32 wpm);
void accessibility_announce(const char* msg);
sigma_u32 accessibility_get_active_features(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ACCESSIBILITY_H */
