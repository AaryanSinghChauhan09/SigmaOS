#include "libc/sigma_libc.h"
#include "libc/sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Microkernel Core Prototype
// ---------------------------------------------------------

// Process States
typedef enum {
    STATE_READY,
    STATE_RUNNING,
    STATE_BLOCKED,
    STATE_TERMINATED
} process_state_t;

// Minimal Process Control Block (PCB)
typedef struct {
    uint32_t pid;
    process_state_t state;
    void* stack_pointer;
    void* page_table;
    uint32_t priority;
} pcb_t;

#define MAX_PROCESSES 256
static pcb_t process_table[MAX_PROCESSES];
static uint32_t current_process = 0;

// Initialize the kernel structures
void init_core_kernel() {
    for (int i = 0; i < MAX_PROCESSES; i++) {
        process_table[i].pid = i;
        process_table[i].state = STATE_TERMINATED;
    }
}

// Simple Round-Robin Scheduler Prototype
void schedule() {
    uint32_t next_process = (current_process + 1) % MAX_PROCESSES;
    
    // Find next ready process
    while (process_table[next_process].state != STATE_READY && next_process != current_process) {
        next_process = (next_process + 1) % MAX_PROCESSES;
    }
    
    // Perform Context Switch (architecture dependent, handled by HAL)
    // context_switch(&process_table[current_process].stack_pointer, process_table[next_process].stack_pointer);
    current_process = next_process;
}

// Inter-Process Communication (IPC)
// Minimal synchronous message passing
typedef struct {
    uint32_t sender_pid;
    uint32_t message_type;
    void* payload;
} message_t;

int send_message(uint32_t target_pid, message_t* msg) {
    if (target_pid >= MAX_PROCESSES || process_table[target_pid].state == STATE_TERMINATED) {
        return -1; // Target invalid
    }
    // In a real microkernel, we would copy the message to target's queue or map memory
    return 0; // Success
}
