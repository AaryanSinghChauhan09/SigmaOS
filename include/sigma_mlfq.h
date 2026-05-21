#ifndef SIGMA_MLFQ_H
#define SIGMA_MLFQ_H

#include <stdint.h>

#define MAX_TASKS 256
#define NUM_QUEUES 4

typedef enum {
    TASK_FREE = 0,
    TASK_READY,
    TASK_RUNNING,
    TASK_BLOCKED
} task_state_t;

typedef struct sigma_task {
    uint32_t id;
    task_state_t state;
    int priority;          // 0 (highest) to 3 (lowest)
    uint32_t time_slice;   // Remaining ticks in quantum
    uint64_t rsp;          // Saved stack pointer
    struct sigma_task* next; // Next in queue
} sigma_task_t;

// Initialize MLFQ scheduler
void sigma_sched_init(void);

// Called by PIT timer handler
void sigma_sched_tick(void);

// Yield CPU to next task
void sigma_sched_yield(void);

// Create a new task (adds to highest priority queue)
uint32_t sigma_sched_add_task(void* entry_point, void* stack_top);

// Block the current task
void sigma_sched_block_current(void);

// Unblock a task
void sigma_sched_unblock(uint32_t task_id);

#endif // SIGMA_MLFQ_H
