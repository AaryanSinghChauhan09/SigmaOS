#include "sigma_proc.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Process Manager
 * Implements a Priority-Aware Task Switching (PATS) algorithm.
 */

/* --- Sovereign Scheduler Object (OOPS Isolation) --- */
static struct {
    sigma_process_t table[64];
    uint32_t active_count;
    uint32_t current_pid;
    uint32_t quantum_limit;
} SovereignScheduler = {
    .active_count = 0,
    .current_pid = 0,
    .quantum_limit = 1000
};

extern "C" void proc_init() {
    sigma_log("[PROC] Initializing Sovereign Scheduler (OOPS Isolation)...");
    
    // Spawn PID 0: Sovereign Genesis Task
    proc_spawn("SovereignGenesis", 0);
}

extern "C" uint32_t proc_spawn(const char* name, uint32_t priority) {
    if (SovereignScheduler.active_count >= 64) return 0;
    
    sigma_process_t* proc = &SovereignScheduler.table[SovereignScheduler.active_count++];
    proc->pid = SovereignScheduler.active_count;
    sigma_hardened_strcpy(proc->name, name, 32);
    proc->state = SIGMA_PROC_READY;
    proc->priority = priority;
    proc->cpu_time = 0;
    proc->capability_mask = 0xFFFFFFFF; // Full sovereignty by default
    
    sigma_printf("[PROC] Spawned Task: %s (PID: %d, Priority: %d)\n", name, proc->pid, priority);
    return proc->pid;
}

extern "C" void proc_yield() {
    // PATS (Priority-Aware Task Switching) Algorithm
    
    // Watchdog: Check current process
    if (SovereignScheduler.current_pid > 0) {
        sigma_process_t* current = &SovereignScheduler.table[SovereignScheduler.current_pid - 1];
        if (current->state == SIGMA_PROC_RUNNING) {
            current->cpu_time++;
            if (current->cpu_time > SovereignScheduler.quantum_limit) {
                sigma_printf("[PROC] [WATCHDOG] PID %d ('%s') exceeded quota. Deprioritizing.\n", 
                             current->pid, current->name);
                current->priority++;
                current->cpu_time = 0;
            }
            current->state = SIGMA_PROC_READY;
        }
    }

    sigma_process_t* next = (sigma_process_t*)SIGMA_NULL;
    uint32_t highest_priority = 0xFFFFFFFF;
    
    // Circular scan to prevent starvation (Round-Robin within same priority)
    uint32_t start_idx = SovereignScheduler.current_pid; 
    for (uint32_t offset = 0; offset < SovereignScheduler.active_count; offset++) {
        uint32_t i = (start_idx + offset) % SovereignScheduler.active_count;
        
        if (SovereignScheduler.table[i].state == SIGMA_PROC_READY && 
            SovereignScheduler.table[i].priority < highest_priority) {
            highest_priority = SovereignScheduler.table[i].priority;
            next = &SovereignScheduler.table[i];
        }
    }
    
    if (next) {
        sigma_printf("[PROC] PATS Context Switch: PID %d -> PID %d (%s)\n", 
                     (int)SovereignScheduler.current_pid, (int)next->pid, next->name);
        SovereignScheduler.current_pid = next->pid;
        next->state = SIGMA_PROC_RUNNING;
    }
}

extern "C" sigma_process_t* proc_get_current() {
    if (SovereignScheduler.current_pid == 0) return (sigma_process_t*)SIGMA_NULL;
    return &SovereignScheduler.table[SovereignScheduler.current_pid - 1];
}
