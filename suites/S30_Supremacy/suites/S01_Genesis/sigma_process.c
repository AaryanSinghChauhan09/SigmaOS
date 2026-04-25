#include "../../../include/sigma_process.h"
#include "../../../include/sigma_pmm.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

/* =========================================================================
 * SIGMA OS: SCHEDULER & CONTEXT SWITCH IMPLEMENTATION (Step 6)
 * Pre-emptive Multitasking via Hardware Timers (Round-Robin).
 * ========================================================================= */

static sigma_process_t process_table[MAX_PROCESSES];
static int current_pid = -1;
static uint32_t next_pid = 1;

// Internal routine: Selects the next viable process queue index
static int get_next_ready_process() {
    int start = (current_pid >= 0) ? current_pid : 0;
    
    for (int i = 1; i <= MAX_PROCESSES; i++) {
        int idx = (start + i) % MAX_PROCESSES;
        if (process_table[idx].state == PROCESS_READY) {
            return idx;
        }
    }
    return -1; // No viable processes (Kernel Idle state)
}

void sigma_scheduler_init() {
    for (int i = 0; i < MAX_PROCESSES; i++) {
        process_table[i].state = PROCESS_NULL;
    }
}

int sigma_process_spawn(void (*entry_point)(), const char* name, uint32_t priority) {
    // Find an empty PCB slot
    int free_idx = -1;
    for (int i = 0; i < MAX_PROCESSES; i++) {
        if (process_table[i].state == PROCESS_NULL || process_table[i].state == PROCESS_TERMINATED) {
            free_idx = i;
            break;
        }
    }
    
    if (free_idx == -1) return -1; // Max threads reached

    // Allocate physical memory for the process stack safely mapping via PMM
    void* process_stack = sigma_pmm_allocate_block();
    if (!process_stack) return -1; // OOM state

    sigma_process_t* p = &process_table[free_idx];
    p->pid = next_pid++;
    
    // Hard-copy process name bounds 
    int name_len = 0;
    while(name_len < 31 && name[name_len] != '\0') {
        p->name[name_len] = name[name_len];
        name_len++;
    }
    p->name[name_len] = '\0';
    
    p->priority = priority;
    p->ticks_executed = 0;

    // Simulate pushing an EIP standard context pointer
    p->context.eip = (uint32_t)entry_point;
    
    // Stack grows downwards in x86 architecture. Set ESP to top of the 4KB allocated page.
    p->context.esp = (uint32_t)process_stack + SIGMA_PAGE_SIZE; 
    p->context.eflags = 0x202; // IF flag enabled -> Interrupts globally enabled during exec
    
    p->state = PROCESS_READY;
    return p->pid;
}

void sigma_scheduler_tick() {
    // Fired by the native hardware Programmable Interval Timer (PIT).
    if (current_pid >= 0) {
        sigma_process_t* current = &process_table[current_pid];
        if (current->state == PROCESS_RUNNING) {
            current->ticks_executed++;
            current->state = PROCESS_READY; // Pre-empt current thread and force it to yield
        }
    }

    int next_idx = get_next_ready_process();
    if (next_idx != -1) {
        current_pid = next_idx;
        process_table[next_idx].state = PROCESS_RUNNING;
        
        // In reality, this function now returns into Assembly (sigma_idt)
        // Which seamlessly `iret` Pops the newly restored process_table[next_idx].context back into the live CPU registers.
    }
}
