// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_staged_update.cpp — karma-gated staged update rollout
 * (Fedora Bodhi-inspired)
 */

#include "sigma_staged_update.h"
#include "sigma_log.h"

extern "C" void sigma_strncpy(char* d, const char* s, sigma_size_t n);

static const char* stage_name(sigma_update_stage_t s) {
    switch (s) {
    case UPDATE_STAGE_PENDING:  return "PENDING";
    case UPDATE_STAGE_CANARY:   return "CANARY";
    case UPDATE_STAGE_TESTING:  return "TESTING";
    case UPDATE_STAGE_STABLE:   return "STABLE";
    case UPDATE_STAGE_REVERTED: return "REVERTED";
    }
    return "UNKNOWN";
}

void sigma_staged_update_init(sigma_staged_update_t* upd,
                               const char*  pkg_name,
                               const char*  version,
                               const char*  prev_version,
                               sigma_u32    nodes_total,
                               bool         auto_revert) {
    sigma_strncpy(upd->pkg_name,     pkg_name,     sizeof(upd->pkg_name)    - 1);
    sigma_strncpy(upd->version,      version,      sizeof(upd->version)     - 1);
    sigma_strncpy(upd->prev_version, prev_version, sizeof(upd->prev_version)- 1);
    upd->karma            = 0;
    upd->karma_threshold  = 3;
    upd->karma_revert     = -2;
    upd->nodes_deployed   = 0;
    upd->nodes_total      = nodes_total;
    upd->stage            = UPDATE_STAGE_PENDING;
    upd->auto_revert      = auto_revert;
}

void sigma_update_apply_karma(sigma_staged_update_t* upd, int delta) {
    if (upd->stage == UPDATE_STAGE_STABLE ||
        upd->stage == UPDATE_STAGE_REVERTED) {
        return;  /* final states — no further karma processing */
    }

    upd->karma += delta;
    sigma_log_info("[sigma-bodhi] %s v%s karma=%d (delta=%+d) stage=%s\n",
                   upd->pkg_name, upd->version,
                   upd->karma, delta, stage_name(upd->stage));

    /* Check revert threshold first */
    if (upd->karma <= upd->karma_revert && upd->auto_revert) {
        sigma_update_revert(upd, "karma below threshold");
        return;
    }

    /* Check promotion threshold */
    if (upd->karma >= upd->karma_threshold &&
        upd->stage == UPDATE_STAGE_TESTING) {
        upd->stage = UPDATE_STAGE_STABLE;
        sigma_log_info("[sigma-bodhi] %s v%s PROMOTED to STABLE (karma=%d)\n",
                       upd->pkg_name, upd->version, upd->karma);
    }
}

void sigma_update_advance_stage(sigma_staged_update_t* upd) {
    switch (upd->stage) {
    case UPDATE_STAGE_PENDING:
        upd->stage = UPDATE_STAGE_CANARY;
        upd->nodes_deployed = upd->nodes_total / 100 + 1;  /* ~1% */
        sigma_log_info("[sigma-bodhi] %s v%s → CANARY (%u nodes)\n",
                       upd->pkg_name, upd->version, upd->nodes_deployed);
        break;
    case UPDATE_STAGE_CANARY:
        upd->stage = UPDATE_STAGE_TESTING;
        upd->nodes_deployed = upd->nodes_total / 10 + 1;   /* ~10% */
        sigma_log_info("[sigma-bodhi] %s v%s → TESTING (%u nodes)\n",
                       upd->pkg_name, upd->version, upd->nodes_deployed);
        break;
    default:
        sigma_log_warn("[sigma-bodhi] advance_stage: already in %s\n",
                       stage_name(upd->stage));
    }
}

void sigma_update_revert(sigma_staged_update_t* upd, const char* reason) {
    sigma_log_err("[sigma-bodhi] REVERTING %s v%s → v%s: %s (karma=%d)\n",
                  upd->pkg_name, upd->version, upd->prev_version,
                  reason, upd->karma);
    upd->stage = UPDATE_STAGE_REVERTED;
    sigma_rollback_package(upd->pkg_name, upd->prev_version);
}

void sigma_staged_update_print(const sigma_staged_update_t* upd) {
    sigma_log_info(
        "[sigma-bodhi] pkg=%-20s ver=%-10s prev=%-10s "
        "stage=%-10s karma=%+3d deployed=%u/%u\n",
        upd->pkg_name, upd->version, upd->prev_version,
        stage_name(upd->stage), upd->karma,
        upd->nodes_deployed, upd->nodes_total);
}
