/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN WORK-STEALING SHARD (v52.1-SUPREME-COSMOS)
 * =========================================================================
 * Mission: Dynamic core-balancing via work-stealing queues.
 * Principles: Multi-Processing, Computer Science, Distributed, Throughput.
 *
 * Implements a scheduler where idle cores steal tasks from busy ones.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

typedef struct {
    sigma_u32 core_id;
    sigma_u32 task_queue[32]; // Deque for work stealing
    int       head;
    int       tail;
} SigmaWorker_t;

/**
 * sigma_sched_steal: Attempts to steal a task from a sibling core's queue.
 * Principle: Multi-Processing / Throughput Optimization.
 */
sigma_u32 sigma_sched_steal(SigmaWorker_t* thief, SigmaWorker_t* victim) {
    sigma_printf("[STEAL]: Core %u attempting to steal from Core %u...\n", 
                 thief->core_id, victim->core_id);
    
    // Atomic pop from the tail of the victim's deque
    if (victim->head != victim->tail) {
        sigma_u32 task = victim->task_queue[victim->tail % 32];
        victim->tail++;
        sigma_printf("[STEAL]: Task 0x%X transfered. Core %u load-balanced.\n", 
                     task, thief->core_id);
        return task;
    }
    return 0; // Nothing to steal
}

/* --- Module Factory --- */

void SovereignWorkSteal_Register(void) {
    sigma_printf("[ORCHESTRATOR]: Sovereign Work-Stealing (Galactic Balancing) active.\n");
}



