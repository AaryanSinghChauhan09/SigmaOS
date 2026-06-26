// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_mcs.h — Mixed Criticality Scheduling (seL4 MCS-inspired)
 * Each thread has a budget (CPU time per period). High-criticality threads
 * (zerotrust, crypto) always preempt low-criticality threads (AI, browser).
 */
#include <sigma_kernel_types.h>
#include <stdbool.h>

#define SIGMA_CRIT_HIGH  0   /* zerotrust, trustd, init — preempt everything */
#define SIGMA_CRIT_LOW   1   /* AI engine, browser, package installs          */

typedef struct {
    sigma_u64 budget_ns;          /* CPU time per period                     */
    sigma_u64 period_ns;          /* replenishment period                    */
    sigma_u64 remaining_ns;       /* budget remaining this period            */
    sigma_u64 next_replenish_ns;  /* monotonic time of next replenishment    */
    sigma_u32 priority;           /* 0 = highest                             */
    sigma_u8  criticality;        /* SIGMA_CRIT_HIGH or SIGMA_CRIT_LOW       */
    const char* name;
} sigma_sched_ctx_t;

/* Check if thread has remaining budget; if not, block until replenishment */
bool sigma_mcs_check_budget(sigma_sched_ctx_t* sc);

/* Called by scheduler tick — replenish budgets for elapsed periods */
void sigma_mcs_replenish_all(sigma_u64 now_ns);

/* Well-known budget presets */
#define SIGMA_MCS_ZEROTRUST { .budget_ns=500000, .period_ns=1000000,   .priority=0, .criticality=SIGMA_CRIT_HIGH, .name="sigma-zerotrust" }
#define SIGMA_MCS_TRUSTD    { .budget_ns=300000, .period_ns=1000000,   .priority=1, .criticality=SIGMA_CRIT_HIGH, .name="sigma-trustd"    }
#define SIGMA_MCS_NETWORK   { .budget_ns=400000, .period_ns=1000000,   .priority=2, .criticality=SIGMA_CRIT_HIGH, .name="sigma-net"       }
#define SIGMA_MCS_AI_ENGINE { .budget_ns=2000000,.period_ns=100000000, .priority=10,.criticality=SIGMA_CRIT_LOW,  .name="sigma-ai"        }
#define SIGMA_MCS_BROWSER   { .budget_ns=5000000,.period_ns=16666667,  .priority=8, .criticality=SIGMA_CRIT_LOW,  .name="zenith-browser"  }
