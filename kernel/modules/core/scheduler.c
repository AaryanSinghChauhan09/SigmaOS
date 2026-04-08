#include "../../../include/sigma_kernel.h"
#include "../../SovereignOmniShard.h"

#define MAX_TASKS 64
typedef enum { TASK_READY, TASK_RUNNING, TASK_WAITING, TASK_ZOMBIE } sigma_task_state_t;

typedef struct {
    sigma_u32 pid;
    sigma_task_state_t state;
    sigma_u32 priority;
    void* stack_ptr;
} sigma_tcb_t;

static sigma_tcb_t g_task_table[MAX_TASKS];
static int g_current_task = -1;

void SovereignScheduler_Init() {
    sigma_printf("Σ [INIT]: Sovereign O(1) Multi-Priority Scheduler Online.\n");
    for(int i=0; i<MAX_TASKS; i++) g_task_table[i].state = TASK_ZOMBIE;
}

int sigma_task_spawn(sigma_u32 pid, sigma_u32 priority) {
    for(int i=0; i<MAX_TASKS; i++) {
        if(g_task_table[i].state == TASK_ZOMBIE) {
            g_task_table[i].pid = pid;
            g_task_table[i].priority = priority;
            g_task_table[i].state = TASK_READY;
            sigma_printf("Σ [SCHED]: Spawned PID %d (Priority %d)\n", pid, priority);
            return i;
        }
    }
    return -1;
}


