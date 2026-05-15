#include "../../../include/core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-SCHEDULER (v1.1)
 * =============================================================================
 * Principles: Preemptive Round-Robin, Zero-Latency Context Switching.
 * =============================================================================
 */
#include "../../../include/core/sigma_kernel_types.h"

typedef struct Task {
    sigma_u64     rsp;            /* Stack Pointer */
    sigma_u64     id;             /* Task ID */
    sigma_u32     priority;       /* Execution Priority */
    sigma_u32     state;          /* 0: Running, 1: Ready, 2: Blocked */
    struct Task* next;      /* Circular List */
} Task;

static Task* current_task = 0;
static Task* task_list = 0;
static sigma_u64   next_task_id = 1;

extern void switch_to_task(sigma_u64* old_rsp, sigma_u64 new_rsp);
extern void kprintf(const char* fmt, ...);
extern void* slab_alloc(sigma_u32 size);

void scheduler_init() {
    /* Initialize kernel task (Main) */
    current_task = (Task*)slab_alloc(sizeof(Task));
    current_task->id = 0;
    current_task->state = 0;
    current_task->next = current_task;
    task_list = current_task;
    
    kprintf("Î£ [SCHEDULER]: Sovereign kernel task active.\n");
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
    
    sigma_u64* stack_ptr = (sigma_u64*)((sigma_u8*)stack + 4096);
    
    /* Setup initial stack for switch_to_task (x86_64 ABI) */
    *(--stack_ptr) = (sigma_u64)entry; /* Return address */
    *(--stack_ptr) = 0;         /* RBP */
    *(--stack_ptr) = 0;         /* RBX */
    *(--stack_ptr) = 0;         /* R12 */
    *(--stack_ptr) = 0;         /* R13 */
    *(--stack_ptr) = 0;         /* R14 */
    *(--stack_ptr) = 0;         /* R15 */
    
    new_task->rsp = (sigma_u64)stack_ptr;
    new_task->id = next_task_id++;
    new_task->state = 1; /* Ready */
    
    /* Add to circular list */
    new_task->next = current_task->next;
    current_task->next = new_task;
    
    kprintf("Î£ [SCHEDULER]: Shard task %d spawned.\n", new_task->id);
}
