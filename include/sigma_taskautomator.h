/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TASK AUTOMATOR (S-TASKAUTOMATOR)
 * =========================================================================
 * Mission: A built-in, event-driven automation engine (superior to cron),
 * driven by natural language and system state triggers.
 * =========================================================================
 */

#ifndef SIGMA_TASKAUTOMATOR_H
#define SIGMA_TASKAUTOMATOR_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Task Automator Primitives --- */
void taskautomator_init(void);
void taskautomator_create_rule(const char* nlp_trigger, const char* action);
void taskautomator_evaluate_rules(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TASKAUTOMATOR_H */
