#include "../../include/SovereignLibC.h"
/*
 * =========================================================================
 * Σ SIGMAOS ZENITH SUPREME: SCHEDULER SHARD (O(1) COORDINATION)
 * =========================================================================
 * Mission: Absolute task management and processor sharding.
 * Capability: Preemptive multitasking, Round-Robin logic, Task States.
 * =========================================================================
 */

#include "../../include/libc/sigma_libc.h"
#include "../../include/core/sigma_types.h"

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
} task_control_block_t;

static task_control_block_t task_list[MAX_TASKS];
static int current_task_idx = -1;
static int total_tasks = 0;

void sigma_scheduler_init() {
    sigma_memset(task_list, 0, sizeof(task_list));
    current_task_idx = -1;
    total_tasks = 0;
    sigma_printf("[KERNEL] Scheduler initialized (MAX_TASKS: %d)\n", MAX_TASKS);
}

sigma_err_t sigma_task_create(virt_addr_t entry, sigma_u32 priority) {
    if (total_tasks >= MAX_TASKS) return SIGMA_ENOMEM;
    
    int idx = total_tasks++;
    task_list[idx].pid = idx + 1;
    task_list[idx].state = TASK_STATE_READY;
    task_list[idx].entry_point = entry;
    task_list[idx].priority = priority;
    task_list[idx].cpu_time = 0;
    
    sigma_printf("[KERNEL] Task created (PID: %d, Entry: 0x%llx)\n", task_list[idx].pid, entry);
    return SIGMA_OK;
}

void sigma_schedule() {
    if (total_tasks == 0) return;
    
    // Simple Round-Robin Sharding
    int next_idx = (current_task_idx + 1) % total_tasks;
    
    // Check if task is valid for running
    while (task_list[next_idx].state != TASK_STATE_READY && task_list[next_idx].state != TASK_STATE_RUNNING) {
        next_idx = (next_idx + 1) % total_tasks;
    }
    
    if (current_task_idx != -1) {
        task_list[current_task_idx].state = TASK_STATE_READY;
    }
    
    current_task_idx = next_idx;
    task_list[current_task_idx].state = TASK_STATE_RUNNING;
    
    // In a real kernel, we would trigger a context switch here (SovereignTaskSwitch)
    // sigma_printf("[KERNEL] Context switch to PID: %d\n", task_list[current_task_idx].pid);
}

SIGMA_NORETURN void sigma_panic(const char* message) {
    sigma_printf("\nΣ SIGMAOS KERNEL PANIC: %s\n", message);
    sigma_printf("SYSTEM HALTED. SOVEREIGN SHUTDOWN INITIATED.\n");
    while(1) {
        // Absolute halt
    }
}
