// SPDX-License-Identifier: GPL-2.0-only
// sigma_continuous_auth.h — SigmaOS Continuous Authentication
// Purpose: Never log in again — identity verified every second passively.
//          Typing rhythm + mouse patterns + face + Bluetooth proximity +
//          Wi-Fi device presence. Zero friction, maximum assurance.
//          RBI step-up auth for transactions > ₹5000.

#pragma once
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

// ---------------------------------------------------------------------------
// Auth Signal Sources
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_AUTH_SIG_TYPING_RHYTHM    = 1,  // Keystroke dynamics (98% accuracy)
    SIGMA_AUTH_SIG_MOUSE_PATTERN    = 2,  // Mouse movement biometrics
    SIGMA_AUTH_SIG_FACE             = 3,  // Webcam liveness detection
    SIGMA_AUTH_SIG_BLUETOOTH        = 4,  // Paired watch/earbuds proximity
    SIGMA_AUTH_SIG_WIFI_DEVICE      = 5,  // Phone MAC near gateway
    SIGMA_AUTH_SIG_SEAT_PRESSURE    = 6,  // Smart chair (if connected)
    SIGMA_AUTH_SIG_TOUCH_PATTERN    = 7,  // Touchscreen pressure/velocity
} sigma_auth_signal_type_t;

typedef struct {
    sigma_auth_signal_type_t type;
    double   confidence;              // 0.0 – 1.0
    time_t   last_seen;
    bool     active;                  // Signal currently present
    double   weight;                  // Weight in overall confidence score
} sigma_auth_signal_t;

// ---------------------------------------------------------------------------
// Authentication State
// ---------------------------------------------------------------------------

typedef enum {
    SIGMA_AUTH_STATE_VERIFIED        = 0,  // Full access — all signals match
    SIGMA_AUTH_STATE_RESTRICTED      = 1,  // 1 signal dropped → minor restrictions
    SIGMA_AUTH_STATE_LOCKED          = 2,  // Multiple signals dropped → lock screen
    SIGMA_AUTH_STATE_SUSPENDED       = 3,  // Security event → full suspend
    SIGMA_AUTH_STATE_ABSENT          = 4,  // Face gone >5 min → auto-lock
} sigma_auth_state_t;

typedef struct {
    char     user_did[128];           // Current user's DID
    sigma_auth_state_t state;
    double   overall_confidence;      // Weighted sum of all active signals (0.0-1.0)
    time_t   state_since;
    time_t   last_full_auth;
    // Signal array
    sigma_auth_signal_t signals[8];
    int      signal_count;
    // Restrictions in RESTRICTED state
    bool     can_transfer_money;      // Blocked if confidence < 0.85
    bool     can_access_health_records;
    bool     can_admin_system;
    bool     can_sign_documents;      // DID signing requires ≥ 0.90
    // RBI compliance
    double   rbi_stepup_threshold;    // 0.95 required for transactions > ₹5000
    bool     stepup_triggered;        // OTP requested for this transaction
} sigma_auth_session_t;

// ---------------------------------------------------------------------------
// Behavioral Biometrics
// ---------------------------------------------------------------------------

typedef struct {
    // Typing rhythm features (per key)
    double   dwell_time_ms_mean;      // Average key hold time
    double   dwell_time_ms_stddev;
    double   flight_time_ms_mean;     // Average time between keys
    double   flight_time_ms_stddev;
    double   error_rate;              // Backspace frequency
    double   typing_speed_wpm;
    double   bigram_timing[256];      // Timing for common 2-key combos
    uint32_t training_samples;        // How many sessions trained on
    bool     model_ready;             // Need ≥ 50 sessions to be reliable
} sigma_auth_typing_profile_t;

typedef struct {
    // Mouse movement features
    double   avg_velocity;
    double   avg_acceleration;
    double   curvature_index;         // How curved paths are (per-person trait)
    double   pause_duration_ms_mean;
    double   click_duration_ms_mean;
    double   scroll_velocity_mean;
    uint32_t training_samples;
    bool     model_ready;             // Need ≥ 100 sessions
} sigma_auth_mouse_profile_t;

// ---------------------------------------------------------------------------
// Face Liveness Detection
// ---------------------------------------------------------------------------

typedef struct {
    bool     face_present;
    double   face_match_confidence;   // 0.0-1.0 (vs enrolled photo)
    bool     liveness_confirmed;      // Anti-spoofing: not a photo/video
    double   attention_score;         // Looking at screen (for medical records)
    time_t   face_last_seen;
    uint32_t blink_count_per_min;    // <3 blinks/min → may be video replay
    bool     eyes_open;
    bool     mask_detected;           // Lower weight if masked
} sigma_auth_face_status_t;

// ---------------------------------------------------------------------------
// Transaction Step-Up Auth (RBI Compliance)
// ---------------------------------------------------------------------------

typedef struct {
    double   amount;                  // Transaction amount ₹
    char     merchant[64];
    char     type[32];                // "upi", "neft", "rtgs", "imps"
    double   current_confidence;
    bool     stepup_required;         // true if amount > 5000 AND confidence < 0.95
    bool     otp_sent;
    bool     otp_verified;
    char     otp_ref[32];
    time_t   stepup_at;
    time_t   stepup_expires;          // OTP valid for 10 minutes
} sigma_auth_stepup_t;

// RBI mandate: step-up authentication for transactions > ₹5,000
// SigmaOS: if continuous auth confidence ≥ 0.95 → no OTP needed (seamless)
//          if confidence < 0.95 → OTP to registered mobile (mandatory)
int sigma_auth_stepup_check(double amount, sigma_auth_stepup_t *out);
int sigma_auth_stepup_verify_otp(const char *ref, const char *otp,
                                  bool *ok);

// ---------------------------------------------------------------------------
// Audit Log
// ---------------------------------------------------------------------------

typedef struct {
    time_t   access_time;
    char     user_did[128];
    char     resource[256];           // What was accessed
    char     action[64];              // "read", "write", "sign", "execute"
    double   auth_confidence;         // Confidence at time of access
    bool     stepup_used;             // Was OTP/step-up required?
    bool     access_granted;
    char     deny_reason[128];        // If denied
} sigma_auth_access_log_t;

int sigma_auth_log_access(const char *resource, const char *action,
                           bool granted, const char *deny_reason);
int sigma_auth_log_query(time_t from, time_t to,
                          sigma_auth_access_log_t *entries, int *count);

// ---------------------------------------------------------------------------
// API
// ---------------------------------------------------------------------------

int sigma_auth_init(const char *user_did);
int sigma_auth_get_session(sigma_auth_session_t *out);
double sigma_auth_get_confidence(void);
sigma_auth_state_t sigma_auth_get_state(void);

// Signal updates (called by signal sources)
int sigma_auth_update_typing(sigma_auth_typing_profile_t *observed);
int sigma_auth_update_mouse(sigma_auth_mouse_profile_t *observed);
int sigma_auth_update_face(sigma_auth_face_status_t *status);
int sigma_auth_update_bluetooth(const char *device_mac, bool present);
int sigma_auth_update_wifi_device(const char *phone_mac, bool present);

// Profile training
int sigma_auth_train_typing(void);   // Runs background capture for 50 sessions
int sigma_auth_train_mouse(void);    // Runs background capture for 100 sessions
int sigma_auth_enroll_face(void);    // Capture reference face (liveness check)

// CLI:
// sigma-auth continuous enable
// sigma-auth continuous status        # current confidence: 97%
// sigma-auth continuous log           # who accessed what, when
// sigma-auth train --signal typing
