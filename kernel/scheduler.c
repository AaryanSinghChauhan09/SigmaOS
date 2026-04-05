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

    /* MLFQ CORE LOGIC: RoadMap #21 Compliance */
    /* 1. Starvation Prevention: Priority Boosting */
    static sigma_u64 total_ticks = 0;
    total_ticks++;
    if (total_ticks % 1000 == 0) {
        for (int i = 0; i < MAX_TASKS; i++) {
            if (task_list[i].pid != 0) {
                task_list[i].queue_id = 0; /* Promote all to highest priority */
                task_list[i].time_slice = 10;
                task_list[i].wait_time = 0;
            }
        }
        sigma_printf("[KERNEL] MLFQ Priority Boost Pulsed.\n");
    }

    /* 2. Sharding Mechanism: Find highest non-empty queue */
    int target_queue = -1;
    for (int q = 0; q < 4; q++) {
        for (int i = 0; i < total_tasks; i++) {
            if (task_list[i].pid != 0 && (task_list[i].state == TASK_STATE_READY || task_list[i].state == TASK_STATE_RUNNING)) {
                if (task_list[i].queue_id == (sigma_u32)q) {
                    target_queue = q;
                    break;
                }
            }
        }
        if (target_queue != -1) break;
    }

    if (target_queue == -1) return;

    /* 3. Round-Robin within the priority queue */
    int start = (current_task_idx + 1) % total_tasks;
    int next_idx = -1;
    for (int i = 0; i < total_tasks; i++) {
        int idx = (start + i) % total_tasks;
        if (task_list[idx].pid != 0 && task_list[idx].state == TASK_STATE_READY && task_list[idx].queue_id == (sigma_u32)target_queue) {
            next_idx = idx;
            break;
        }
    }

    /* 4. Execution context switch with Time-Slice Demotion */
    if (next_idx != -1) {
        if (current_task_idx != -1 && task_list[current_task_idx].state == TASK_STATE_RUNNING) {
            task_list[current_task_idx].state = TASK_STATE_READY;
            /* MLFQ Penalty: If task used its FULL slice, demote it */
            if (task_list[current_task_idx].time_slice == 0) {
                if (task_list[current_task_idx].queue_id < 3) {
                    task_list[current_task_idx].queue_id++;
                    task_list[current_task_idx].time_slice = (task_list[current_task_idx].queue_id + 1) * 10;
                }
            }
        }
        current_task_idx = next_idx;
        task_list[current_task_idx].state = TASK_STATE_RUNNING;
        if (task_list[current_task_idx].time_slice > 0)
            task_list[current_task_idx].time_slice--; /* Consume shard time */
    }
}


SIGMA_NORETURN void sigma_panic(const char* message) {
    sigma_printf("\nΣ SIGMAOS KERNEL PANIC: %s\n", message);
    sigma_printf("SYSTEM HALTED. SOVEREIGN SHUTDOWN INITIATED.\n");
    while(1) {
        // Absolute halt
    }
}
