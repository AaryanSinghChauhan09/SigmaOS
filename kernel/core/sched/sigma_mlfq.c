#include "sigma_mlfq.h"
#include <stddef.h>

static sigma_task_t task_table[MAX_TASKS];
static sigma_task_t* run_queues[NUM_QUEUES];
static sigma_task_t* current_task = NULL;

// Quanta per priority level: 1, 2, 4, 8 ticks
static const uint32_t quanta[NUM_QUEUES] = {1, 2, 4, 8};

static uint32_t next_task_id = 1;

extern void task_switch(uint64_t* old_rsp, uint64_t new_rsp);

void sigma_sched_init(void) {
    for (int i = 0; i < MAX_TASKS; i++) {
        task_table[i].state = TASK_FREE;
    }
    for (int i = 0; i < NUM_QUEUES; i++) {
        run_queues[i] = NULL;
    }
    current_task = NULL;
}

static void enqueue_task(sigma_task_t* task) {
    if (!task || task->priority >= NUM_QUEUES) return;
    
    int p = task->priority;
    task->next = NULL;
    
    if (run_queues[p] == NULL) {
        run_queues[p] = task;
    } else {
        sigma_task_t* tail = run_queues[p];
        while (tail->next != NULL) {
            tail = tail->next;
        }
        tail->next = task;
    }
    task->state = TASK_READY;
}

static sigma_task_t* dequeue_task(void) {
    for (int p = 0; p < NUM_QUEUES; p++) {
        if (run_queues[p] != NULL) {
            sigma_task_t* task = run_queues[p];
            run_queues[p] = task->next;
            task->next = NULL;
            return task;
        }
    }
    return NULL;
}

uint32_t sigma_sched_add_task(void* entry_point, void* stack_top) {
    // Find free task
    sigma_task_t* t = NULL;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (task_table[i].state == TASK_FREE) {
            t = &task_table[i];
            break;
        }
    }
    if (!t) return 0;
    
    t->id = next_task_id++;
    t->priority = 0;
    t->time_slice = quanta[0];
    
    // Set up initial stack frame (simplification for proof-of-concept)
    uint64_t* stack = (uint64_t*)stack_top;
    *(--stack) = (uint64_t)entry_point; // RIP
    // Need to push other registers (R15-R8, RBP, RDI, RSI, RDX, RCX, RBX, RAX)
    for(int i=0; i<15; i++) *(--stack) = 0;
    
    t->rsp = (uint64_t)stack;
    
    enqueue_task(t);
    return t->id;
}

void sigma_sched_tick(void) {
    if (!current_task) {
        // Find a task to run
        sigma_task_t* next = dequeue_task();
        if (next) {
            current_task = next;
            current_task->state = TASK_RUNNING;
        }
        return;
    }
    
    current_task->time_slice--;
    
    if (current_task->time_slice == 0) {
        // Demote priority
        if (current_task->priority < NUM_QUEUES - 1) {
            current_task->priority++;
        }
        sigma_sched_yield();
    }
}

void sigma_sched_yield(void) {
    if (!current_task) return;
    
    sigma_task_t* prev = current_task;
    
    // Reset time slice for current queue
    prev->time_slice = quanta[prev->priority];
    
    if (prev->state == TASK_RUNNING) {
        enqueue_task(prev);
    }
    
    sigma_task_t* next = dequeue_task();
    if (!next) {
        // Nothing else to run, just continue if possible
        if (prev->state == TASK_READY) {
            current_task = dequeue_task();
            current_task->state = TASK_RUNNING;
        } else {
            current_task = NULL;
        }
        return;
    }
    
    current_task = next;
    current_task->state = TASK_RUNNING;
    
    // Perform context switch
    // Note: task_switch is assembly that saves old_rsp, loads new_rsp
    task_switch(&prev->rsp, current_task->rsp);
}
