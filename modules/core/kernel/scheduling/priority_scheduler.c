#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Priority-Based Preemptive Scheduler
// ---------------------------------------------------------

#define MAX_TASKS 256
#define NUM_PRIORITIES 4 // 0: Real-time, 1: High, 2: Normal, 3: Background

typedef struct {
    int pid;
    int state; // 0: Empty, 1: Ready, 2: Running, 3: Blocked, 4: Dead
    int priority;
    uint64_t rip; // Instruction pointer
    uint64_t rsp; // Stack pointer
    uint64_t cr3; // Page directory
    int time_slice; // Remaining time slice
} task_t;

typedef struct {
    task_t* head;
    task_t* tail;
} task_queue_t;

static task_t task_table[MAX_TASKS];
static task_queue_t ready_queues[NUM_PRIORITIES];
static task_t* current_task = 0;

void scheduler_init() {
    for (int i = 0; i < MAX_TASKS; i++) {
        task_table[i].state = 0; // Empty
    }
    for (int i = 0; i < NUM_PRIORITIES; i++) {
        ready_queues[i].head = 0;
        ready_queues[i].tail = 0;
    }
}

int create_task(void* entry_point, int priority) {
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state == 0) {
            task_table[i].pid = i;
            task_table[i].state = 1; // Ready
            task_table[i].priority = priority;
            task_table[i].rip = (uint64_t)entry_point;
            task_table[i].time_slice = (4 - priority) * 10; // Higher priority gets more time
            // Add to ready queue (mocked list insertion)
            return i;
        }
    }
    return -1; // Process table full
}

// Called on timer interrupt
void schedule() {
    if (current_task) {
        current_task->time_slice--;
        if (current_task->time_slice > 0) {
            return; // Continue running
        }
        // Time slice expired, put back to ready queue
        current_task->state = 1;
        // current_task->time_slice = reset;
    }
    
    // Find next task
    for (int i = 0; i < NUM_PRIORITIES; i++) {
        // If queue not empty, pop head and run
        // current_task = pop_task(&ready_queues[i]);
        // current_task->state = 2; // Running
        // switch_context(current_task->rsp, current_task->cr3);
        // break;
    }
}
