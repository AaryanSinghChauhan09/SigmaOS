#include "Lattice.h"
#include "sigma_hal.h"
#include "sigma_libc.h"
#include "sigma_proc.h"

/**
 * SigmaOS Sovereign Process Manager
 * Implements a Priority-Aware Task Switching (PATS) algorithm.
 */

static sigma_process_t process_table[64];
static uint32_t current_pid = 0;
static uint32_t active_processes = 0;

extern "C" void proc_init() {
    sigma_log("[PROC] Initializing Sovereign Process Lattice...");
    
    // Spawn PID 0: Sovereign Genesis Task
    proc_spawn("SovereignGenesis", 0);
}

extern "C" uint32_t proc_spawn(const char* name, uint32_t priority) {
    if (active_processes >= 64) return 0;
    
    sigma_process_t* proc = &process_table[active_processes++];
    proc->pid = active_processes;
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
    // Selects the next READY process with the highest priority (lowest value).
    
    // Task Watchdog: Check for runaway processes
    sigma_process_t* current = (current_pid > 0) ? &process_table[current_pid - 1] : SIGMA_NULL;
    if (current && current->state == SIGMA_PROC_RUNNING) {
        current->cpu_time++;
        if (current->cpu_time > 1000) { // Quota exceeded
            sigma_printf("[PROC] [WATCHDOG] PID %d ('%s') exceeded silicon quota. Throttling.\n", 
                         current->pid, current->name);
            current->priority++; // Deprioritize
            current->cpu_time = 0;
        }
        current->state = SIGMA_PROC_READY;
    }

    sigma_process_t* next = SIGMA_NULL;
    uint32_t highest_priority = 0xFFFFFFFF;
    
    for (uint32_t i = 0; i < active_processes; i++) {
        if (process_table[i].state == SIGMA_PROC_READY && process_table[i].priority < highest_priority) {
            highest_priority = process_table[i].priority;
            next = &process_table[i];
        }
    }
    
    if (next) {
        sigma_printf("[PROC] PATS Context Switch: PID %d -> PID %d (%s)\n", current_pid, next->pid, next->name);
        current_pid = next->pid;
        next->state = SIGMA_PROC_RUNNING;
    }
}

extern "C" sigma_process_t* proc_get_current() {
    return &process_table[current_pid - 1];
}
