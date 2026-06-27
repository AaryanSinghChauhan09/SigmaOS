// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_session.h — Session & login manager
 *
 * Handles: login screen, user authentication, session startup, screen lock.
 * Talks to sigma-trustd for credential validation and sigma-bus for events.
 *
 * Session lifecycle:
 *   sigma-session start
 *       │
 *       ▼
 *   show login screen (sigma_session_show_login)
 *       │
 *       ▼ user enters PIN / password
 *   sigma_session_authenticate(user, credential)
 *       │
 *       ▼ success
 *   launch Zenith desktop (fork zenith_unified_init)
 *       │
 *       ▼
 *   sigma_session_lock() on Ctrl+L / idle timeout
 *       │
 *       ▼ user re-authenticates
 *   resume session
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

typedef enum {
    SIGMA_SESSION_STATE_LOGGEDOUT = 0,
    SIGMA_SESSION_STATE_LOGGINGON,
    SIGMA_SESSION_STATE_ACTIVE,
    SIGMA_SESSION_STATE_LOCKED,
    SIGMA_SESSION_STATE_SWITCHING,
} sigma_session_state_t;

typedef struct {
    char     username[64];
    char     display[16];     /* ":0", ":1" etc.                           */
    sigma_u32 uid;
    sigma_u32 gid;
    char     home[256];
    char     shell[64];
    sigma_session_state_t state;
    sigma_u64 started_ns;     /* monotonic start time                      */
} sigma_session_t;

/* ── Authentication ──────────────────────────────────────────────────────── */

/* Authenticate with password or PIN. Returns 0 on success, -1 on failure. */
int  sigma_session_authenticate(const char* username, const char* credential);

/* PAM-style pluggable auth: checks /sigma/etc/shadow or sigma-trustd cert */
int  sigma_session_pam_check(const char* username, const char* password);

/* ── Session lifecycle ───────────────────────────────────────────────────── */

int  sigma_session_start(const char* username, sigma_session_t* out);
int  sigma_session_lock(sigma_session_t* session);
int  sigma_session_unlock(sigma_session_t* session, const char* credential);
void sigma_session_logout(sigma_session_t* session);

/* Get current session state */
sigma_session_state_t sigma_session_get_state(const sigma_session_t* session);

/* ── Screen lock policy ──────────────────────────────────────────────────── */

/* Lock after N seconds of idle */
void sigma_session_set_lock_timeout(sigma_u32 idle_seconds);

/* ── Privilege escalation (sudo equivalent) ──────────────────────────────── */

/* Request elevated privileges — prompts user if not already elevated */
int sigma_session_elevate(const char* reason, bool* granted_out);

/* ── Multi-user ───────────────────────────────────────────────────────────── */

int sigma_session_switch_user(const char* target_username);
int sigma_session_list_active(sigma_session_t* out, int max);
