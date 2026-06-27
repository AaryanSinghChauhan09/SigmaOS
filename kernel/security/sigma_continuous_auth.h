// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_continuous_auth.h — Continuous authentication (never log in again)
 *
 * Traditional login: verify identity ONCE at boot, then trust forever.
 * Problem: if someone walks away from an unlocked screen, attacker gets in.
 *
 * Continuous Authentication: verify identity CONTINUOUSLY using passive signals:
 *   - Typing cadence (keystroke dynamics — unique per person)
 *   - Touch/swipe pressure + timing on touchscreens
 *   - Facial recognition (front camera, opt-in, local ML)
 *   - Voice verification (speaker ID, local model)
 *   - Behavioural: app usage patterns, time-of-day, location (NavIC)
 *
 * When confidence drops below threshold:
 *   - LOW:  subtle notification — "Are you still there?"
 *   - MED:  screen dims, secondary factor requested
 *   - HIGH: session locked, suspicious activity logged
 *
 * All processing is LOCAL — no biometrics leave the device.
 * This is NOT surveillance — it's ANTI-surveillance (detects imposters).
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Auth signals ────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_AUTH_KEYSTROKE   = (1 << 0),
    SIGMA_AUTH_TOUCH       = (1 << 1),
    SIGMA_AUTH_FACE        = (1 << 2),
    SIGMA_AUTH_VOICE       = (1 << 3),
    SIGMA_AUTH_BEHAVIOUR   = (1 << 4),
    SIGMA_AUTH_LOCATION    = (1 << 5),   /* unusual location = lower score  */
    SIGMA_AUTH_TIME_OF_DAY = (1 << 6),   /* 3am usage = suspicious          */
} sigma_auth_signal_t;

/* ── Confidence score ────────────────────────────────────────────────────── */
typedef struct {
    float    overall;              /* 0.0–1.0 combined confidence           */
    float    keystroke;
    float    touch;
    float    face;
    float    voice;
    float    behaviour;
    sigma_u64 last_update_ns;
    sigma_u32 signals_active;      /* bitmask of active signals             */
} sigma_auth_score_t;

/* ── Risk thresholds ─────────────────────────────────────────────────────── */
#define SIGMA_AUTH_THRESHOLD_HIGH    0.85f  /* fully trusted                */
#define SIGMA_AUTH_THRESHOLD_MEDIUM  0.60f  /* prompt secondary factor      */
#define SIGMA_AUTH_THRESHOLD_LOW     0.40f  /* lock screen                  */
#define SIGMA_AUTH_THRESHOLD_ALARM   0.20f  /* log intrusion + alert        */

/* ── Keystroke dynamics sample ───────────────────────────────────────────── */
typedef struct {
    sigma_u32 keycode;
    sigma_u64 press_ns;
    sigma_u64 release_ns;
    sigma_u64 dwell_ns;     /* press_to_release                            */
    sigma_u64 flight_ns;    /* release_to_next_press                       */
} sigma_keystroke_sample_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Initialise continuous auth with desired signals bitmask. */
int sigma_cauth_init(sigma_u32 signals, float lock_threshold,
                      void (*on_lock)(float score, void *ctx), void *ctx);

/* Feed a keystroke event. */
int sigma_cauth_keystroke(const sigma_keystroke_sample_t *sample);

/* Feed a raw audio frame (16kHz mono float32) for voice verification. */
int sigma_cauth_voice(const float *pcm, size_t n_samples);

/* Feed a camera frame for facial recognition. */
int sigma_cauth_face(const sigma_u8 *rgba_frame,
                      sigma_u32 width, sigma_u32 height);

/* Get current confidence score. */
int sigma_cauth_score(sigma_auth_score_t *out);

/* Manually re-authenticate (after lock). */
int sigma_cauth_reauth_pin(const char *pin);
int sigma_cauth_reauth_did(const sigma_u8 *challenge, size_t clen,
                             const sigma_u8 *response, size_t rlen);

/* Enroll the owner's biometric profile (first boot / reset). */
int sigma_cauth_enroll_keystroke(const sigma_keystroke_sample_t *samples,
                                   int n_samples);
int sigma_cauth_enroll_face(const sigma_u8 *frames[], int n_frames,
                              sigma_u32 width, sigma_u32 height);

void sigma_cauth_shutdown(void);
