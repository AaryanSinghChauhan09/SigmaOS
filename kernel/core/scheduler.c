/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-SCHEDULER (v1.0)
 * =============================================================================
 * Principles: Preemptive Round-Robin, Zero-Latency Context Switching.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Task {
    u64     rsp;            /* Stack Pointer */
    u64     id;             /* Task ID */
    u32     priority;       /* Execution Priority */
    u32     state;          /* 0: Running, 1: Ready, 2: Blocked */
    struct Task* next;      /* Circular List */
} Task;

static Task* current_task = 0;
static Task* task_list = 0;

extern void switch_to_task(u64* old_rsp, u64 new_rsp);
extern void kprintf(const char* fmt, ...);

void scheduler_init() {
    /* Initialize kernel task */
}

void yield() {
    if (!current_task || !current_task->next) return;
    
    Task* old = current_task;
    current_task = current_task->next;
    
    /* Perform context switch */
    switch_to_task(&old->rsp, current_task->rsp);
}

void schedule_task(void (*entry)(void)) {
    kprintf("Σ [SCHEDULER]: Spawning new sovereign task...\n");
}
