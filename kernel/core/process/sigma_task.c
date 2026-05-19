/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: TASK & PROCESS CONTROL BLOCK (PCB) MANAGEMENT
 * =============================================================================
 * Inspired by: Linux kernel kernel/fork.c (task_struct)
 *              FreeBSD sys/kern/kern_proc.c (proc structure)
 * =============================================================================
 * Manages the lifecycle, states, and scheduling entities of isolated processes.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_TASKS       1024
#define TASK_NAME_LEN   32

/* Task States */
#define TASK_STATE_UNALLOCATED  0
#define TASK_STATE_READY        1
#define TASK_STATE_RUNNING      2
#define TASK_STATE_BLOCKED      3
#define TASK_STATE_ZOMBIE       4

typedef struct {
    sigma_u32  pid;
    sigma_u32  ppid;          /* Parent PID */
    sigma_u32  state;
    char       name[TASK_NAME_LEN];
    
    /* CPU Context (Registers) */
    sigma_u64  rip;
    sigma_u64  rsp;
    sigma_u64  cr3;           /* Page Directory Base Register */
    
    /* Accounting */
    sigma_u64  cpu_time_ms;
    sigma_u32  priority;
    
    sigma_bool active;
} sigma_task_t;

static sigma_task_t task_table[MAX_TASKS];
static sigma_u32 next_pid = 1;
static sigma_u32 current_task_idx = 0;

void task_init_subsystem(void) {
    sigma_memset(task_table, 0, sizeof(task_table));
    
    /* Create kernel idle task */
    task_table[0].pid      = 0;
    task_table[0].state    = TASK_STATE_READY;
    task_table[0].priority = 255; /* Lowest priority */
    task_table[0].active   = SIGMA_TRUE;
    sigma_strcpy(task_table[0].name, "kernel_idle", TASK_NAME_LEN);
    
    sigma_printf("[task] Process subsystem initialized (Max Tasks: %d)\n", MAX_TASKS);
}

int task_create(const char* name, sigma_u64 entry_point, sigma_u32 priority) {
    for (sigma_u32 i = 1; i < MAX_TASKS; i++) {
        if (!task_table[i].active) {
            task_table[i].pid      = next_pid++;
            task_table[i].ppid     = task_table[current_task_idx].pid;
            task_table[i].state    = TASK_STATE_READY;
            task_table[i].priority = priority;
            task_table[i].rip      = entry_point;
            task_table[i].cpu_time_ms = 0;
            task_table[i].active   = SIGMA_TRUE;
            
            sigma_u32 j = 0;
            while (j < TASK_NAME_LEN - 1 && name[j]) {
                task_table[i].name[j] = name[j];
                j++;
            }
            task_table[i].name[j] = '\0';
            
            sigma_printf("[task] Created PID %u ('%s') Priority: %u\n", 
                         task_table[i].pid, task_table[i].name, priority);
            return (int)task_table[i].pid;
        }
    }
    sigma_printf("[task] ERR: Task table full\n");
    return -1;
}

void task_exit(sigma_u32 exit_code) {
    sigma_task_t* current = &task_table[current_task_idx];
    if (current->pid == 0) {
        sigma_printf("[task] ERR: Kernel idle task cannot exit!\n");
        return;
    }
    
    current->state = TASK_STATE_ZOMBIE;
    sigma_printf("[task] PID %u ('%s') exited with code %u\n", 
                 current->pid, current->name, exit_code);
                 
    /* Trigger scheduler to yield CPU (simulated) */
}

void task_dump_list(void) {
    sigma_printf("\n--- Σ PROCESS LIST ---\n");
    sigma_printf("PID   PPID  STATE     PRIO  TIME(ms)  NAME\n");
    sigma_printf("--------------------------------------------\n");
    for (sigma_u32 i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].active) {
            const char* state_str = "UNK";
            switch (task_table[i].state) {
                case TASK_STATE_READY:   state_str = "READY"; break;
                case TASK_STATE_RUNNING: state_str = "RUN  "; break;
                case TASK_STATE_BLOCKED: state_str = "BLOCK"; break;
                case TASK_STATE_ZOMBIE:  state_str = "ZOMB "; break;
            }
            
            sigma_printf("%-4u  %-4u  %s  %-4u  %-8llu  %s\n",
                         task_table[i].pid,
                         task_table[i].ppid,
                         state_str,
                         task_table[i].priority,
                         task_table[i].cpu_time_ms,
                         task_table[i].name);
        }
    }
    sigma_printf("--------------------------------------------\n");
}
