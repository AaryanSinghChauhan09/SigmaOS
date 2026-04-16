#ifndef SIGMA_PROCESS_H
#define SIGMA_PROCESS_H

#include <stdint.h>

/* =========================================================================
 * SIGMA OS: SCHEDULER & MULTITASKING SHARD (SYSTEM-LEVEL HEADER)
 * Defines strictly packed Process Control Blocks (PCBs) for Ring-0 logic.
 * ========================================================================= */

#define MAX_PROCESSES 256

typedef enum {
    PROCESS_NULL = 0,
    PROCESS_READY,
    PROCESS_RUNNING,
    PROCESS_BLOCKED,
    PROCESS_TERMINATED
} sigma_process_state_t;

// Context switch register state saved to the CPU stack upon hardware tick
typedef struct {
    uint32_t eax, ebx, ecx, edx, esi, edi, ebp, esp;
    uint32_t eip, eflags;
    uint32_t cr3; // Physical page directory root
} sigma_cpu_context_t;

typedef struct {
    uint32_t pid;
    sigma_process_state_t state;
    sigma_cpu_context_t context;
    char name[32];
    uint32_t priority;
    uint64_t ticks_executed;
} sigma_process_t;

void sigma_scheduler_init();
int sigma_process_spawn(void (*entry_point)(), const char* name, uint32_t priority);
void sigma_scheduler_yield();
void sigma_scheduler_tick();

#endif
