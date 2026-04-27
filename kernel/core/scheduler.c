/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-SCHEDULER (v1.1)
 * =============================================================================
 * Principles: Preemptive Round-Robin, Zero-Latency Context Switching.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Task {
    u64     rsp;            /* Stack Pointer */
    u64     id;             /* Task ID */
    u32     priority;       /* Execution Priority */
    u32     state;          /* 0: Running, 1: Ready, 2: Blocked */
    struct Task* next;      /* Circular List */
} Task;

static Task* current_task = 0;
static Task* task_list = 0;
static u64   next_task_id = 1;

extern void switch_to_task(u64* old_rsp, u64 new_rsp);
extern void kprintf(const char* fmt, ...);
extern void* slab_alloc(u32 size);

void scheduler_init() {
    /* Initialize kernel task (Main) */
    current_task = (Task*)slab_alloc(sizeof(Task));
    current_task->id = 0;
    current_task->state = 0;
    current_task->next = current_task;
    task_list = current_task;
    
    kprintf("Σ [SCHEDULER]: Sovereign kernel task active.\n");
}

void yield() {
    if (!current_task || current_task->next == current_task) return;
    
    Task* old = current_task;
    current_task = current_task->next;
    
    /* Perform context switch */
    switch_to_task(&old->rsp, current_task->rsp);
}

void schedule_task(void (*entry)(void)) {
    Task* new_task = (Task*)slab_alloc(sizeof(Task));
    void* stack = slab_alloc(4096); /* Allocate 4KB stack */
    
    u64* stack_ptr = (u64*)((u8*)stack + 4096);
    
    /* Setup initial stack for switch_to_task (x86_64 ABI) */
    *(--stack_ptr) = (u64)entry; /* Return address */
    *(--stack_ptr) = 0;         /* RBP */
    *(--stack_ptr) = 0;         /* RBX */
    *(--stack_ptr) = 0;         /* R12 */
    *(--stack_ptr) = 0;         /* R13 */
    *(--stack_ptr) = 0;         /* R14 */
    *(--stack_ptr) = 0;         /* R15 */
    
    new_task->rsp = (u64)stack_ptr;
    new_task->id = next_task_id++;
    new_task->state = 1; /* Ready */
    
    /* Add to circular list */
    new_task->next = current_task->next;
    current_task->next = new_task;
    
    kprintf("Σ [SCHEDULER]: Shard task %d spawned.\n", new_task->id);
}
