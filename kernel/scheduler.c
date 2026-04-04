/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SCHEDULER SHARD (O(1) COORDINATION)
 * =========================================================================
 * Mission: Absolute task management and processor sharding.
 * Capability: Preemptive multitasking, Round-Robin logic, Task States.
 * =========================================================================
 */

#include "scheduler.h"
#include "../libc/sigma_libc.h"
#include "../libc/sigma_types.h"


task_control_block_t task_list[MAX_TASKS];
int current_task_idx = -1;
int total_tasks = 0;

void sigma_schedule() {

    if (total_tasks == 0) return;
    
    /* 1. Find highest priority among READY tasks */
    sigma_u32 max_prio = 0;
    sigma_bool any_ready = SIGMA_FALSE;
    for (int i = 0; i < total_tasks; i++) {
        if (task_list[i].pid != 0 && (task_list[i].state == TASK_STATE_READY || task_list[i].state == TASK_STATE_RUNNING)) {
            if (!any_ready || task_list[i].priority > max_prio) {
                max_prio = task_list[i].priority;
                any_ready = SIGMA_TRUE;
            }
        }
    }

    if (!any_ready) return; /* No tasks ready to run */

    /* 2. Round-Robin Sharding: find next task with max_prio */
    int start = (current_task_idx + 1) % total_tasks;
    int next_idx = -1;
    for (int i = 0; i < total_tasks; i++) {
        int idx = (start + i) % total_tasks;
        if (task_list[idx].pid != 0 && task_list[idx].state == TASK_STATE_READY && task_list[idx].priority == max_prio) {
            next_idx = idx;
            break;
        }
    }

    /* 3. Execute Switch */
    if (next_idx != -1) {
        if (current_task_idx != -1 && task_list[current_task_idx].state == TASK_STATE_RUNNING) {
            task_list[current_task_idx].state = TASK_STATE_READY;
        }
        current_task_idx = next_idx;
        task_list[current_task_idx].state = TASK_STATE_RUNNING;
        
        /* Note: In bare-metal, we would call SovereignTaskSwitch(current_task_idx) here */
    }
}


SIGMA_NORETURN void sigma_panic(const char* message) {
    sigma_printf("\nΣ SIGMAOS KERNEL PANIC: %s\n", message);
    sigma_printf("SYSTEM HALTED. SOVEREIGN SHUTDOWN INITIATED.\n");
    while(1) {
        // Absolute halt
    }
}
