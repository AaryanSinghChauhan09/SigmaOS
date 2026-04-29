/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ASYNCHRONOUS INIT (S-INIT)
 * =========================================================================
 * Mission: Wait-free shard ignition and parallel machine-state setup.
 * Inspired by Void Linux / runit.
 * =========================================================================
 */

#ifndef SIGMA_INIT_H
#define SIGMA_INIT_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t shard_id;
    const char* description;
    bool is_parallel;
} sigma_init_step_t;

/* --- Init Primitives --- */
void sinit_init(void);
void sinit_execute_plan(void);
void sinit_report_status(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_INIT_H */
