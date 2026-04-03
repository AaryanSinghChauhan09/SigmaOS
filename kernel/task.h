/* 
 Σ SIGMAOS ZENITH: PROCESS CONTROL BLOCK (v1600.0)
 Mission: Task State Management & Scheduling Structures.
*/

#ifndef SIGMA_TASK_H
#define SIGMA_TASK_H

#include <stdint.h>
#include <stdbool.h>

// Σ PROCESS STATE
typedef enum {
    TASK_RUNNING,
    TASK_READY,
    TASK_SLEEPING,
    TASK_ZOMBIE,
    TASK_KILLED
} sigma_task_state;

// Σ TASK CONTEXT (Registers)
struct sigma_task_context {
    uint64_t rax, rbx, rcx, rdx, rsi, rdi, rbp, rsp;
    uint64_t r8, r9, r10, r11, r12, r13, r14, r15;
    uint64_t rip, rflags;
} __attribute__((packed));

// Σ PROCESS CONTROL BLOCK (PCB)
typedef struct {
    uint32_t pid;
    char name[32];
    sigma_task_state state;
    struct sigma_task_context context;
    uint64_t cr3; // Page Table Base
    uint64_t* stack_base;
    uint32_t priority;
} sigma_task;

// Σ SCHEDULER ENTRY
void sigma_task_init();
void sigma_task_yield();
void sigma_task_exit(uint32_t code);

#endif
