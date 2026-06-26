// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_staged_update — karma-gated staged rollout logic
 *
 * Verifies:
 *   1. PENDING → CANARY → TESTING stage progression.
 *   2. Positive karma promotes TESTING → STABLE at threshold.
 *   3. Negative karma triggers automatic revert.
 *   4. Reverted package stays REVERTED regardless of further karma.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include "sigma_staged_update.h"

/* Capture rollback calls for assertion */
static char last_rollback_pkg[64]  = {};
static char last_rollback_ver[32]  = {};

extern "C" void sigma_rollback_package(const char* pkg, const char* ver) {
    strncpy(last_rollback_pkg, pkg, sizeof(last_rollback_pkg) - 1);
    strncpy(last_rollback_ver, ver, sizeof(last_rollback_ver) - 1);
}

/* klib stubs */
extern "C" {
    void sigma_strncpy(char* d, const char* s, sigma_size_t n) {
        strncpy(d, s, n);
    }
    void sigma_log_info(const char*, ...) {}
    void sigma_log_warn(const char*, ...) {}
    void sigma_log_err(const char*, ...)  {}
}

int main(void) {
    sigma_staged_update_t upd;
    sigma_staged_update_init(&upd, "zenith-browser", "0.2.0", "0.1.0",
                             /*nodes_total=*/100, /*auto_revert=*/true);

    /* ── Test 1: initial state is PENDING ───────────────────────────── */
    assert(upd.stage == UPDATE_STAGE_PENDING && "must start PENDING");
    assert(upd.karma == 0);

    /* ── Test 2: advance to CANARY ───────────────────────────────────── */
    sigma_update_advance_stage(&upd);
    assert(upd.stage == UPDATE_STAGE_CANARY);
    assert(upd.nodes_deployed == 2 &&  /* 1% of 100 + 1 */
           "canary must deploy to ~1% of nodes");

    /* ── Test 3: advance to TESTING ──────────────────────────────────── */
    sigma_update_advance_stage(&upd);
    assert(upd.stage == UPDATE_STAGE_TESTING);
    assert(upd.nodes_deployed == 11 && /* 10% of 100 + 1 */
           "testing must deploy to ~10% of nodes");

    /* ── Test 4: karma accumulates — promote at threshold (3) ──────── */
    sigma_update_apply_karma(&upd, +1);
    assert(upd.stage == UPDATE_STAGE_TESTING && "not yet promoted at karma=1");
    sigma_update_apply_karma(&upd, +1);
    assert(upd.stage == UPDATE_STAGE_TESTING && "not yet promoted at karma=2");
    sigma_update_apply_karma(&upd, +1);
    assert(upd.stage == UPDATE_STAGE_STABLE  && "must promote at karma=3");

    /* ── Test 5: stable stage ignores further karma ───────────────────── */
    sigma_update_apply_karma(&upd, -10);
    assert(upd.stage == UPDATE_STAGE_STABLE  && "stable must ignore karma");

    /* ── Test 6: negative karma triggers auto-revert ──────────────────── */
    sigma_staged_update_t upd2;
    sigma_staged_update_init(&upd2, "sigma-net", "0.3.0", "0.2.9",
                             50, true);
    sigma_update_advance_stage(&upd2);  /* CANARY */
    sigma_update_advance_stage(&upd2);  /* TESTING */

    sigma_update_apply_karma(&upd2, -1);
    assert(upd2.stage != UPDATE_STAGE_REVERTED && "one -1 must not revert yet");
    sigma_update_apply_karma(&upd2, -1);
    assert(upd2.stage == UPDATE_STAGE_REVERTED && "must revert at karma=-2");

    /* ── Test 7: rollback hook was called with correct version ──────── */
    assert(strcmp(last_rollback_pkg, "sigma-net")  == 0);
    assert(strcmp(last_rollback_ver, "0.2.9") == 0);

    /* ── Test 8: reverted update ignores further karma ───────────────── */
    sigma_update_apply_karma(&upd2, +100);
    assert(upd2.stage == UPDATE_STAGE_REVERTED &&
           "reverted update must stay reverted");

    printf("test_staged_update: PASS\n");
    return 0;
}
