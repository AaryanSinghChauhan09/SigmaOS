/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SCHEDULER SHARDING INFRASTRUCTURE
 * =========================================================================
 * Mission: Absolute task lifecycle management.
 * =========================================================================
 */

#include "scheduler.h"
#include "../libc/sigma_libc.h"

extern task_control_block_t task_list[MAX_TASKS];
extern int current_task_idx;
extern int total_tasks;

void sigma_scheduler_init(void) {
    sigma_memset(task_list, 0, sizeof(task_list));
    current_task_idx = -1;
    total_tasks = 0;
    sigma_printf("[KERNEL] Scheduler initialized (MAX_TASKS: %d)\n", MAX_TASKS);
}

sigma_err_t sigma_task_create(virt_addr_t entry, sigma_u32 priority) {
    /* Search for a free slot in the shard grid */
    int slot = -1;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_list[i].pid == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) return SIGMA_ENOMEM;
    
    task_list[slot].pid = (pid_t)(slot + 1);
    task_list[slot].state = TASK_STATE_READY;
    task_list[slot].entry_point = entry;
    task_list[slot].priority = priority;
    task_list[slot].queue_id = 0;   /* Initial MLFQ Queue (Roadmap 21) */
    task_list[slot].time_slice = 10; /* Starting Quantum */
    task_list[slot].cpu_time = 0;
    task_list[slot].wait_time = 0;
    
    if (slot >= total_tasks) total_tasks = slot + 1;
    
    sigma_printf("[KERNEL] MLFQ Task created (PID: %d, Queue: %d, Entry: 0x%llx)\n", task_list[slot].pid, task_list[slot].queue_id, entry);
    return SIGMA_OK;
}
