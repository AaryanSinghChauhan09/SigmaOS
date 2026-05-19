/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: OUT-OF-MEMORY (OOM) KILLER
 * =============================================================================
 * Inspired by: Linux kernel mm/oom_kill.c
 *              FreeBSD sys/vm/vm_pageout.c
 * =============================================================================
 * Calculates task badness scores and terminates processes to free memory.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define MAX_TASKS 1024

/* Simulating access to the task list from sigma_task.c */
typedef struct {
    sigma_u32  pid;
    char       name[32];
    sigma_u64  memory_usage_bytes;
    sigma_u32  oom_score_adj; /* -1000 to +1000 */
    sigma_bool is_kernel_thread;
    sigma_bool active;
} sigma_oom_task_info_t;

/* Dummy task table for OOM killer simulation */
static sigma_oom_task_info_t oom_task_table[MAX_TASKS];

void oom_killer_init(void) {
    sigma_memset(oom_task_table, 0, sizeof(oom_task_table));
    sigma_printf("[oom] Out-Of-Memory (OOM) Killer initialized\n");
}

/* Register dummy task for testing */
void oom_register_dummy_task(sigma_u32 pid, const char* name, sigma_u64 mem_bytes, sigma_u32 adj, sigma_bool is_kernel) {
    if (pid < MAX_TASKS) {
        oom_task_table[pid].pid = pid;
        sigma_strcpy(oom_task_table[pid].name, name, 32);
        oom_task_table[pid].memory_usage_bytes = mem_bytes;
        oom_task_table[pid].oom_score_adj = adj;
        oom_task_table[pid].is_kernel_thread = is_kernel;
        oom_task_table[pid].active = SIGMA_TRUE;
    }
}

/* Calculate badness score. Higher is worse (more likely to be killed) */
static sigma_u64 calculate_badness(const sigma_oom_task_info_t* task, sigma_u64 total_ram) {
    if (task->is_kernel_thread) {
        return 0; /* Never kill kernel threads */
    }
    if (task->oom_score_adj == (sigma_u32)-1000) {
        return 0; /* OOM_SCORE_ADJ_MIN equivalent */
    }

    sigma_u64 score = task->memory_usage_bytes;
    
    /* Apply adjustment (scale it against total RAM) */
    if (task->oom_score_adj != 0) {
        sigma_u64 adj_amount = (total_ram * (task->oom_score_adj > 1000 ? 1000 : task->oom_score_adj)) / 1000;
        
        if ((sigma_s32)task->oom_score_adj < 0) {
            /* Negative adjustment reduces score */
            adj_amount = (total_ram * ((sigma_s32)task->oom_score_adj * -1)) / 1000;
            if (score > adj_amount) {
                score -= adj_amount;
            } else {
                score = 1;
            }
        } else {
            /* Positive adjustment increases score */
            score += adj_amount;
        }
    }
    return score;
}

void oom_trigger(sigma_u64 total_ram) {
    sigma_printf("\n[oom] *** OUT OF MEMORY CONDITION DETECTED ***\n");
    sigma_printf("[oom] Scanning task list for termination candidate...\n");
    
    sigma_oom_task_info_t* victim = SIGMA_NULL;
    sigma_u64 max_score = 0;
    
    for (sigma_u32 i = 0; i < MAX_TASKS; i++) {
        if (oom_task_table[i].active) {
            sigma_u64 score = calculate_badness(&oom_task_table[i], total_ram);
            
            sigma_printf("[oom] PID %4u (%-16s): Score %llu (Mem: %llu MB)\n", 
                         oom_task_table[i].pid, oom_task_table[i].name, 
                         score, oom_task_table[i].memory_usage_bytes / (1024 * 1024));
                         
            if (score > max_score) {
                max_score = score;
                victim = &oom_task_table[i];
            }
        }
    }
    
    if (victim) {
        sigma_printf("[oom] *** OOM KILLER: Terminating PID %u (%s) ***\n", 
                     victim->pid, victim->name);
        
        /* Simulated termination */
        victim->active = SIGMA_FALSE;
        
        sigma_printf("[oom] Recovered %llu MB of memory.\n", 
                     victim->memory_usage_bytes / (1024 * 1024));
    } else {
        sigma_printf("[oom] *** KERNEL PANIC: System out of memory and no killable tasks found! ***\n");
    }
}
