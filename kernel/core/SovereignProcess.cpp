#include "sigma_proc.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Process Manager
 * Implements a Priority-Aware Task Switching (PATS) algorithm.
 */

#include "Lattice.h"
#include "sigma_proc.h"

/**
 * SigmaOS Sovereign Process Manager
 * Implements a Priority-Aware Task Switching (PATS) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal process orchestration.
 *
 * Design: OOP-isolated singleton — SovereignProcessEngine.
 */

/* --- Sovereign Process Engine (OOP Isolation) --- */
static struct {
    sigma_process_t table[64];
    sigma_u32 active_count;
    sigma_u32 current_pid;
    sigma_u32 quantum_limit;
    sigma_u64 switches_performed;
    sigma_u32 initialized;
} SovereignProcessEngine = {
    .active_count = 0u,
    .current_pid = 0u,
    .quantum_limit = 1000u,
    .switches_performed = 0u,
    .initialized = 0u
};

extern "C" void proc_init() {
    sigma_log("[PROC] Initializing Sovereign Scheduler (PATS Algorithm)...");
    
    // Spawn PID 0: Sovereign Genesis Task
    proc_spawn("SovereignGenesis", 0u);
    SovereignProcessEngine.initialized = 1u;
}

extern "C" sigma_u32 proc_spawn(const char* name, sigma_u32 priority) {
    if (SovereignProcessEngine.active_count >= 64u) return 0u;
    
    sigma_process_t* proc = &SovereignProcessEngine.table[SovereignProcessEngine.active_count++];
    proc->pid = SovereignProcessEngine.active_count;
    sigma_hardened_strcpy(proc->name, name, 32);
    proc->state = SIGMA_PROC_READY;
    proc->priority = priority;
    proc->cpu_time = 0u;
    proc->capability_mask = 0xFFFFFFFFu; // Full sovereignty by default
    
    sigma_printf("[PROC] Spawned Task: %s (PID: %u, Priority: %u)\n", name, (unsigned)proc->pid, (unsigned)priority);
    return proc->pid;
}

extern "C" void proc_yield() {
    /* PATS (Priority-Aware Task Switching) Algorithm */
    
    if (SovereignProcessEngine.current_pid > 0u) {
        sigma_process_t* current = &SovereignProcessEngine.table[SovereignProcessEngine.current_pid - 1u];
        if (current->state == SIGMA_PROC_RUNNING) {
            current->cpu_time++;
            if (current->cpu_time > SovereignProcessEngine.quantum_limit) {
                sigma_printf("[PROC] [WATCHDOG] PID %u ('%s') exceeded quota. Deprioritizing.\n", 
                             (unsigned)current->pid, current->name);
                current->priority++;
                current->cpu_time = 0u;
            }
            current->state = SIGMA_PROC_READY;
        }
    }

    sigma_process_t* next = (sigma_process_t*)SIGMA_NULL;
    sigma_u32 highest_priority = 0xFFFFFFFFu;
    
    uint32_t start_idx = SovereignProcessEngine.current_pid; 
    for (uint32_t offset = 0u; offset < SovereignProcessEngine.active_count; offset++) {
        uint32_t i = (start_idx + offset) % SovereignProcessEngine.active_count;
        
        if (SovereignProcessEngine.table[i].state == SIGMA_PROC_READY && 
            SovereignProcessEngine.table[i].priority < highest_priority) {
            highest_priority = SovereignProcessEngine.table[i].priority;
            next = &SovereignProcessEngine.table[i];
        }
    }
    
    if (next) {
        sigma_printf("[PROC] PATS Context Switch: PID %u -> PID %u (%s)\n", 
                     (unsigned)SovereignProcessEngine.current_pid, (unsigned)next->pid, next->name);
        SovereignProcessEngine.current_pid = next->pid;
        next->state = SIGMA_PROC_RUNNING;
        SovereignProcessEngine.switches_performed++;
    }
}

extern "C" sigma_process_t* proc_get_current() {
    if (SovereignProcessEngine.current_pid == 0u) return (sigma_process_t*)SIGMA_NULL;
    return &SovereignProcessEngine.table[SovereignProcessEngine.current_pid - 1u];
}

extern "C" sigma_u64 proc_get_switch_count() {
    return SovereignProcessEngine.switches_performed;
}
