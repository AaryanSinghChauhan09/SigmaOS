/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN EMOTION UX (S-EMOTION)
 * =========================================================================
 * Mission: A hyper-personalized layer that analyzes subtle user cues
 * (voice stress, facial tension) to dynamically adjust OS responsiveness,
 * color psychology, and notification priority.
 * =========================================================================
 */

#ifndef SIGMA_EMOTION_H
#define SIGMA_EMOTION_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    EMOTION_STATE_NEUTRAL,
    EMOTION_STATE_FOCUSED,
    EMOTION_STATE_FRUSTRATED,
    EMOTION_STATE_FATIGUED
} sigma_emotion_state_t;

/* --- Emotion UX Primitives --- */
void emotion_init(void);
void emotion_update_state(sigma_emotion_state_t new_state);
sigma_emotion_state_t emotion_get_current_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_EMOTION_H */
