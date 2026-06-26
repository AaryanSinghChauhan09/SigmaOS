// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_staged_update.h — karma-gated staged rollout (Fedora Bodhi-inspired)
 *
 * Updates progress through stages: CANARY (1%) → TESTING (10%) → STABLE (100%).
 * Karma accumulates from test results (+1 success / -1 regression).
 * Negative karma automatically triggers a rollback to the previous version.
 *
 * Usage:
 *   sigma_staged_update_t upd = {};
 *   sigma_staged_update_init(&upd, "zenith-browser", "0.2.0", 100, true);
 *   // ... after testing ...
 *   sigma_update_apply_karma(&upd, +1);  // test passed
 *   sigma_update_apply_karma(&upd, -1);  // regression reported
 */

#include <sigma_kernel_types.h>

typedef enum {
    UPDATE_STAGE_PENDING,  /* built, not yet deployed to any node        */
    UPDATE_STAGE_CANARY,   /* deployed to ~1% of fleet                   */
    UPDATE_STAGE_TESTING,  /* deployed to ~10%, collecting karma votes   */
    UPDATE_STAGE_STABLE,   /* promoted to 100% of fleet                  */
    UPDATE_STAGE_REVERTED, /* rolled back — previous version restored    */
} sigma_update_stage_t;

typedef struct {
    char     pkg_name[64];
    char     version[32];
    char     prev_version[32];    /* version to restore on revert         */
    int      karma;               /* accumulator                          */
    int      karma_threshold;     /* votes needed to promote (default: 3) */
    int      karma_revert;        /* votes to trigger revert (default: -2)*/
    sigma_u32 nodes_deployed;
    sigma_u32 nodes_total;
    sigma_update_stage_t stage;
    bool     auto_revert;
} sigma_staged_update_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

void sigma_staged_update_init(sigma_staged_update_t* upd,
                               const char*  pkg_name,
                               const char*  version,
                               const char*  prev_version,
                               sigma_u32    nodes_total,
                               bool         auto_revert);

/*
 * Apply a karma delta (+1 = success, -1 = regression).
 * Automatically promotes to STABLE or reverts based on thresholds.
 */
void sigma_update_apply_karma(sigma_staged_update_t* upd, int delta);

/* Advance to the next deployment stage (PENDING → CANARY → TESTING). */
void sigma_update_advance_stage(sigma_staged_update_t* upd);

/* Manually trigger a rollback regardless of karma. */
void sigma_update_revert(sigma_staged_update_t* upd, const char* reason);

/* Print current status to the audit log. */
void sigma_staged_update_print(const sigma_staged_update_t* upd);

/* Kernel hook — restores pkg_name to prev_version in the package database. */
extern void sigma_rollback_package(const char* pkg_name,
                                   const char* target_version);
