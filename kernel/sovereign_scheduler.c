/*
 * Cosmos AI-OS: Sovereign Task Scheduler (C Layer)
 * ================================================
 * Mission: High-speed, predictive CPU thread management.
 */

#include <stddef.h>
#include <stdint.h>


#define MAX_TASKS 256

// Architecture-dependent CPU Context for x86_64
typedef struct {
  uint64_t r15, r14, r13, r12, r11, r10, r9, r8;
  uint64_t rbp, rdi, rsi, rdx, rcx, rbx, rax;
  uint64_t rip;
  uint64_t cs;
  uint64_t rflags;
  uint64_t rsp;
  uint64_t ss;
} cpu_context_t;

typedef enum {
  TASK_FREE = 0,
  TASK_READY,
  TASK_RUNNING,
  TASK_BLOCKED,
  TASK_SLEEPING
} task_state_t;

typedef struct {
  uint32_t pid;
  task_state_t state;
  cpu_context_t *context;
  uint64_t *stack_base;
  uint64_t *stack_ptr;
  int priority;
  char name[32];
} tcb_t;

static tcb_t task_table[MAX_TASKS];
static tcb_t *current_task = NULL;
static uint32_t next_pid = 1;

// Extern the assembly context switcher
extern void cosmos_switch_tasks(uint64_t **old_sp, uint64_t *new_sp);

void cosmos_scheduler_init() {
  for (int i = 0; i < MAX_TASKS; i++) {
    task_table[i].state = TASK_FREE;
  }
}

int cosmos_create_task(void (*entry_point)(), uint64_t *stack_space,
                       const char *name) {
  for (int i = 0; i < MAX_TASKS; i++) {
    if (task_table[i].state == TASK_FREE) {
      tcb_t *task = &task_table[i];
      task->pid = next_pid++;
      task->state = TASK_READY;
      task->priority = 10;

      // Setup stack pointer to point to the top of the allocated space
      task->stack_base = stack_space;
      task->stack_ptr =
          stack_space; // Assuming stack grows down, initialized by caller

      // Name copy
      int n = 0;
      while (name[n] && n < 31) {
        task->name[n] = name[n];
        n++;
      }
      task->name[n] = '\0';

      return task->pid;
    }
  }
  return -1; // Out of task slots
}

// Round Robin / Priority Dispatcher
void cosmos_schedule() {
  if (!current_task) {
    current_task = &task_table[0]; // Idle task usually
    return;
  }

  tcb_t *next_task = NULL;
  for (int i = 0; i < MAX_TASKS; i++) {
    int idx = (current_task->pid + i) % MAX_TASKS;
    if (task_table[idx].state == TASK_READY) {
      next_task = &task_table[idx];
      break;
    }
  }

  if (next_task && next_task != current_task) {
    tcb_t *old = current_task;
    current_task = next_task;
    old->state = TASK_READY;
    current_task->state = TASK_RUNNING;

    // ASM call to swap stacks instantly
    cosmos_switch_tasks(&old->stack_ptr, current_task->stack_ptr);
  }
}
