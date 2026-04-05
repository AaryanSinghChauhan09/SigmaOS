/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SCHEDULER SHARD TYPES
 * =========================================================================
 * Mission: Absolute task management and processor sharding.
 * =========================================================================
 */

#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#include "../libc/sigma_types.h"

#define MAX_TASKS 64
#define STACK_SIZE 16384

typedef enum {
    TASK_STATE_READY,
    TASK_STATE_RUNNING,
    TASK_STATE_SLEEPING,
    TASK_STATE_BLOCKED,
    TASK_STATE_DEAD
} task_state_t;

typedef struct {
    pid_t pid;
    task_state_t state;
    virt_addr_t stack_ptr;
    virt_addr_t entry_point;
    sigma_u64 cpu_time;
    sigma_u32 priority;
    sigma_u32 queue_id;    /* MLFQ Queue Level (0-3) */
    sigma_u32 time_slice;  /* Remaining ticks in current queue */
    sigma_u32 wait_time;   /* Ticks spent waiting (for Boosting) */
} task_control_block_t;

/* Global Shard Interface */
void sigma_scheduler_init(void);
sigma_err_t sigma_task_create(virt_addr_t entry, sigma_u32 priority);
void sigma_schedule(void);
SIGMA_NORETURN void sigma_panic(const char* message);

#endif /* SIGMA_SCHEDULER_H */
