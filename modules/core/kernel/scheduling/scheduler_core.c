#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Core Scheduler Engine
// USP: Policy-agnostic, hot-swappable, capability-aware
// ---------------------------------------------------------

#define MAX_PROCESSES 256

typedef enum {
    PROC_STATE_EMPTY,
    PROC_STATE_READY,
    PROC_STATE_RUNNING,
    PROC_STATE_BLOCKED_IPC,
    PROC_STATE_BLOCKED_MEM
} proc_state_t;

typedef struct {
    uint32_t pid;
    proc_state_t state;
    uint8_t priority;
    uint64_t cpu_time_used;
    uint32_t timeslice_remaining;
    
    // Capability context linked to the process
    uint64_t root_capability_token; 
} process_control_block_t;

static process_control_block_t ptable[MAX_PROCESSES];
static uint32_t current_pid = 0;
static uint32_t runqueue[MAX_PROCESSES];
static uint32_t runqueue_len = 0;

// External Policy Hooks (from policy_modules.c)
extern uint32_t policy_schedule_next(uint32_t* runqueue, uint32_t len);

// External Resource Economy Hooks
extern void token_enforce_expiries(uint64_t current_tick);
extern void mem_contract_enforce_expiries(uint64_t current_tick);

// The core context switch dispatcher
static void context_switch(uint32_t old_pid, uint32_t new_pid) {
    if (old_pid == new_pid) return;
    
    // Assembly stub would save registers for old_pid
    // and load registers + CR3 (page table) for new_pid
}

// Rebuild the runqueue based on READY processes
static void rebuild_runqueue(void) {
    runqueue_len = 0;
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (ptable[i].state == PROC_STATE_READY) {
            runqueue[runqueue_len++] = ptable[i].pid;
        }
    }
}

// Master Scheduler Tick (Called by hardware timer IRQ)
void scheduler_tick(uint64_t current_tick) {
    // 1. USP: Enforce Sovereign Tokens and Memory Contracts every tick
    // If a contract expires, the capability is stripped instantly.
    token_enforce_expiries(current_tick);
    mem_contract_enforce_expiries(current_tick);

    // 2. Decrement timeslice for running process
    if (ptable[current_pid].state == PROC_STATE_RUNNING) {
        if (ptable[current_pid].timeslice_remaining > 0) {
            ptable[current_pid].timeslice_remaining--;
            ptable[current_pid].cpu_time_used++;
            return; // Continue executing current process
        }
        // Timeslice expired, return to READY queue
        ptable[current_pid].state = PROC_STATE_READY;
    }

    // 3. Rebuild queue and pick next process
    rebuild_runqueue();
    
    if (runqueue_len == 0) {
        // Idle loop (or halt CPU to save power)
        return;
    }

    // USP: Hot-Swappable Scheduling Logic
    // We don't hardcode the algorithm (e.g. CFS). We ask the active Policy Module.
    uint32_t next_pid = policy_schedule_next(runqueue, runqueue_len);
    
    if (next_pid != UINT32_MAX && next_pid != current_pid) {
        ptable[next_pid].state = PROC_STATE_RUNNING;
        // In a real implementation, we'd query the policy for the new timeslice length
        ptable[next_pid].timeslice_remaining = 10; 
        
        uint32_t old_pid = current_pid;
        current_pid = next_pid;
        
        context_switch(old_pid, new_pid);
    }
}
