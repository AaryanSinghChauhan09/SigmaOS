/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PROCESS MANAGEMENT
 * =========================================================================
 * Mission: Zero-overhead context switching and capability-gated tasks.
 * =========================================================================
 */

#ifndef SIGMA_PROC_H
#define SIGMA_PROC_H

#include <sigma_types.h>

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
    uint32_t pid;
    char name[32];
    sigma_proc_state_t state;
    uint32_t priority;
    uint64_t cpu_time;
    uint32_t capability_mask;
} sigma_process_t;

/* --- Process Primitives --- */
void proc_init(void);
uint32_t proc_spawn(const char* name, uint32_t priority);
void proc_yield(void);
sigma_process_t* proc_get_current(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PROC_H */
