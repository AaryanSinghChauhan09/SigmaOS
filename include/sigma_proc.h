/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PROCESS MANAGEMENT
 * =========================================================================
 * Mission: Zero-overhead context switching and capability-gated tasks.
 * =========================================================================
 */

#ifndef SIGMA_PROC_H
#define SIGMA_PROC_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_PROC_READY,
    SIGMA_PROC_RUNNING,
    SIGMA_PROC_BLOCKED,
    SIGMA_PROC_ZOMBIE
} sigma_proc_state_t;

typedef struct {
    sigma_u32 pid;
    char      name[32];
    sigma_proc_state_t state;
    sigma_u32 priority;
    sigma_u64 cpu_time;
    sigma_u32 capability_mask;
} sigma_process_t;

/* --- Process Primitives --- */
void             proc_init(void);
sigma_u32        proc_spawn(const char* name, sigma_u32 priority);
void             proc_yield(void);
sigma_process_t* proc_get_current(void);
sigma_u64        proc_get_switch_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PROC_H */
