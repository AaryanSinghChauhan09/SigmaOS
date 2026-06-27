// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_kiosk.h — Locked-down single-app kiosk mode
 *
 * Target markets:
 *   - Bank ATMs (RBI mandates India-made software by 2027)
 *   - Government CSC kiosks (5 lakh Common Service Centres)
 *   - Hospital patient check-in terminals
 *   - Railway/bus ticket booking terminals
 *   - School computer labs
 *
 * Security model:
 *   - Single app fills the screen; no taskbar, no desktop
 *   - All keyboard shortcuts disabled (Ctrl+Alt+Del, Super, Alt+Tab)
 *   - App auto-restarts on crash (< 500 ms recovery)
 *   - Admin unlock requires DID signature (not a PIN)
 *   - Each user session wiped on exit (tmpfs per session)
 *   - Remote management via sigma-fleet
 *
 * CLI:
 *   sigma-kiosk enable --app sigma-pos
 *   sigma-kiosk enable --url https://irctc.in
 *   sigma-kiosk disable --admin-did <did:sigma:admin>
 *   sigma-kiosk status
 *   sigma-kiosk session-wipe    # manual wipe between users
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

/* ── Kiosk configuration ─────────────────────────────────────────────────── */
typedef enum {
    SIGMA_KIOSK_MODE_APP = 0,   /* Run a sigma-app fullscreen              */
    SIGMA_KIOSK_MODE_URL = 1,   /* Run a URL in locked-down Chromium       */
    SIGMA_KIOSK_MODE_POS = 2,   /* sigma-pos retail mode (special layout)  */
} sigma_kiosk_mode_t;

typedef struct {
    sigma_kiosk_mode_t mode;

    /* App mode */
    char app_id[64];            /* sigma app ID, e.g. "sigma-pos"          */
    char app_args[256];         /* command-line args for the app           */

    /* URL mode */
    char url[512];              /* URL to open in Chromium kiosk mode      */
    bool url_allow_navigation;  /* false = locked to single URL            */

    /* Session management */
    bool session_wipe_on_exit;  /* wipe all session data after each user   */
    sigma_u32 idle_timeout_s;   /* wipe session after idle (0=never)       */
    bool auto_restart_app;      /* restart app if it crashes               */
    sigma_u32 restart_delay_ms; /* delay before restart (default 500ms)    */

    /* Admin unlock */
    char admin_did[128];        /* DID URI for admin — e.g. did:sigma:xxx  */
    char admin_pubkey[64];      /* Ed25519 public key (hex) for DID verify */

    /* Display */
    bool hide_cursor;           /* hide mouse cursor (touch screens)       */
    bool disable_screensaver;
    sigma_u32 screen_timeout_s; /* blank screen after idle (0=never)       */

    /* Input restrictions */
    bool block_keyboard_shortcuts; /* Ctrl+Alt+Del, Super, Alt+Tab, etc.   */
    bool block_right_click;
    bool touch_only;            /* reject mouse input on touch devices     */

    /* Networking */
    bool restrict_network;      /* allow only URLs in allowlist            */
    char allowlist_domains[16][128]; /* allowed domains in URL mode        */
    int  n_allowlist;

    /* Fleet integration */
    char fleet_group[64];       /* sigma-fleet group for remote mgmt       */
} sigma_kiosk_config_t;

/* ── Runtime state ───────────────────────────────────────────────────────── */
typedef struct {
    bool               active;
    sigma_kiosk_config_t config;
    sigma_u32          app_pid;
    sigma_u32          restart_count;
    sigma_u64          session_start_ns;
    sigma_u64          last_activity_ns;
    bool               admin_unlocked;   /* true during admin session       */
} sigma_kiosk_state_t;

/* ── API ─────────────────────────────────────────────────────────────────── */

/* Enable kiosk mode with the given configuration. */
int sigma_kiosk_enable(const sigma_kiosk_config_t *config);

/* Disable kiosk mode. Requires a valid admin DID challenge-response. */
int sigma_kiosk_disable(const sigma_u8 *admin_challenge,
                         sigma_size_t   challenge_len,
                         const sigma_u8 *admin_signature,
                         sigma_size_t   sig_len);

/* Query current kiosk state. */
int sigma_kiosk_status(sigma_kiosk_state_t *out);

/* Manually wipe the current user's session (for multi-user kiosks). */
int sigma_kiosk_session_wipe(void);

/* Restart the kiosk app (e.g. after config update). */
int sigma_kiosk_restart_app(void);

/* Update config on a running kiosk (pushed by sigma-fleet). */
int sigma_kiosk_update_config(const sigma_kiosk_config_t *new_config);

/* Default configurations for common use cases. */
sigma_kiosk_config_t sigma_kiosk_config_atm(const char *admin_did);
sigma_kiosk_config_t sigma_kiosk_config_csc(const char *admin_did);
sigma_kiosk_config_t sigma_kiosk_config_pos(const char *admin_did);
sigma_kiosk_config_t sigma_kiosk_config_school(const char *admin_did);
