/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN PROCESS MANAGER (v1.0)
 * =============================================================================
 * Mission: Full process lifecycle management with PID allocation, PCB tracking,
 *          parent/child relationships, and state machine transitions.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_PROCESS_MANAGER_H
#define SIGMA_PROCESS_MANAGER_H

#include "../sigma_kernel_types.h"

#define SIGMA_MAX_PROCESSES   4096
#define SIGMA_PROC_NAME_LEN   64
#define SIGMA_PROC_INVALID_PID 0

typedef enum {
    PROC_STATE_CREATED    = 0,
    PROC_STATE_READY      = 1,
    PROC_STATE_RUNNING    = 2,
    PROC_STATE_BLOCKED    = 3,
    PROC_STATE_TERMINATED = 4
} sigma_proc_state_t;

typedef struct {
    sigma_u32         pid;
    sigma_u32         parent_pid;
    sigma_proc_state_t state;
    sigma_u8          priority;        /* 0 = highest, 255 = lowest */
    char              name[SIGMA_PROC_NAME_LEN];
    sigma_u64         mem_allocated;   /* bytes */
    sigma_u64         cpu_time_us;     /* microseconds of CPU time consumed */
    sigma_u64         start_tsc;       /* TSC at creation */
    sigma_vaddr_t     stack_base;
    sigma_usize       stack_size;
    sigma_vaddr_t     page_table_root; /* CR3 value for this process */
    sigma_bool        is_kernel;       /* 1 = kernel thread, 0 = userland */
} sigma_pcb_t;

#ifdef __cplusplus
extern "C" {
#endif

void      process_manager_init(void);
sigma_u32 process_create(const char* name, sigma_u8 priority, sigma_bool is_kernel);
int       process_kill(sigma_u32 pid);
int       process_set_state(sigma_u32 pid, sigma_proc_state_t new_state);
int       process_set_priority(sigma_u32 pid, sigma_u8 priority);
const sigma_pcb_t* process_getinfo(sigma_u32 pid);
void      process_list(void);
sigma_u32 process_get_count(void);
sigma_u32 process_fork(sigma_u32 parent_pid);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PROCESS_MANAGER_H */
