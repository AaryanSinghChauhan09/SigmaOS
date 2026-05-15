#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Capability-Aware Process Scheduler Prototype
// ---------------------------------------------------------

typedef enum {
    STATE_READY,
    STATE_RUNNING,
    STATE_BLOCKED,
    STATE_PAGING_WAIT // Suspended waiting for a page to load
} process_state_t;

typedef enum {
    PRIO_REALTIME = 0,
    PRIO_NORMAL = 1,
    PRIO_BACKGROUND = 2
} process_prio_t;

typedef struct {
    uint32_t pid;
    process_state_t state;
    process_prio_t priority;
    uint32_t active_memory_cap_id; // Capability needed for current execution
    void* stack_pointer;
} pcb_t;

#define MAX_PROCESSES 64
static pcb_t process_table[MAX_PROCESSES];
static uint32_t current_pid = 0;

// External reference to the capability checker
extern int cap_check(uint32_t pid, uint32_t cap_id, uint8_t required_rights);

// External reference to memory paging subsystem
extern int is_page_resident(uint64_t virtual_addr);
extern void trigger_page_in(uint64_t virtual_addr);

// The Core Scheduler
void scheduler_tick() {
    uint32_t next_pid = (current_pid + 1) % MAX_PROCESSES;
    int found = 0;

    // 1. Adaptive Scheduling: Search for the highest priority Ready task
    // Real-time tasks take absolute precedence
    for (int p = 0; p < MAX_PROCESSES; p++) {
        if (process_table[p].state == STATE_READY && process_table[p].priority == PRIO_REALTIME) {
            next_pid = p;
            found = 1;
            break;
        }
    }

    // 2. Fallback to Round-Robin for Normal/Background
    if (!found) {
        for (int i = 0; i < MAX_PROCESSES; i++) {
            uint32_t p = (current_pid + 1 + i) % MAX_PROCESSES;
            if (process_table[p].state == STATE_READY) {
                next_pid = p;
                break;
            }
        }
    }

    pcb_t* next_process = &process_table[next_pid];

    // 3. Security Hook: Capability Verification
    // Does the process have the right to execute its current memory segment?
    if (!cap_check(next_process->pid, next_process->active_memory_cap_id, 0x04 /* CAP_EXECUTE */)) {
        // Capability violation! Panic or kill the process.
        next_process->state = STATE_BLOCKED; 
        return; // Skip dispatch
    }

    // 4. Paging Integration
    // If the process is waiting on a page, do not schedule it. CPU is never wasted.
    if (next_process->state == STATE_PAGING_WAIT) {
        // Wait for paging subsystem to mark it READY via an interrupt
        return;
    }

    // Dispatch the process (Context Switch)
    process_table[current_pid].state = STATE_READY;
    current_pid = next_pid;
    process_table[current_pid].state = STATE_RUNNING;
    
    // switch_context(...);
}
