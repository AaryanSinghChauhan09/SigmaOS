/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ZEN SCHEDULER HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_ZEN_SCHEDULER_H
#define SOVEREIGN_ZEN_SCHEDULER_H

#include "sigma_types.h"

void sigma_sched_add_task        (const char* name, sigma_u32 prio, sigma_u32 policy);
void sigma_sched_balance         (void);
void SovereignZenScheduler_Init  (void);
void SovereignZenScheduler_Audit (void);

#endif /* SOVEREIGN_ZEN_SCHEDULER_H */
