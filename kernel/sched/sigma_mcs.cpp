// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_mcs.cpp — MCS budget accounting (seL4 MCS checkBudgetRestart-inspired)
 */
#include "sigma_mcs.h"
#include "sigma_log.h"

extern "C" sigma_u64 sigma_clock_monotonic_ns(void);

#define MAX_MCS_THREADS 64
static sigma_sched_ctx_t* g_contexts[MAX_MCS_THREADS];
static int g_ctx_count = 0;

void sigma_mcs_register(sigma_sched_ctx_t* sc) {
    if (g_ctx_count < MAX_MCS_THREADS) {
        g_contexts[g_ctx_count++] = sc;
        sigma_log_info("[sigma-mcs] registered %s budget=%llu ns period=%llu ns\n",
                       sc->name,
                       (unsigned long long)sc->budget_ns,
                       (unsigned long long)sc->period_ns);
    }
}

bool sigma_mcs_check_budget(sigma_sched_ctx_t* sc) {
    if (sc->remaining_ns > 0) return true;  /* still has budget */

    sigma_log_warn("[sigma-mcs] %s budget exhausted — preempting (crit=%d)\n",
                   sc->name, sc->criticality);
    return false;
}

void sigma_mcs_replenish_all(sigma_u64 now_ns) {
    for (int i = 0; i < g_ctx_count; i++) {
        sigma_sched_ctx_t* sc = g_contexts[i];
        if (now_ns >= sc->next_replenish_ns) {
            sc->remaining_ns      = sc->budget_ns;
            sc->next_replenish_ns = now_ns + sc->period_ns;
        }
    }
}
