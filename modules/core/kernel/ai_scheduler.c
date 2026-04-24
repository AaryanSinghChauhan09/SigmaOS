#include "sigma_libc.h"
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS AI-Assisted Adaptive Scheduler
// Predicts workload patterns and pre-allocates CPU/memory
// ---------------------------------------------------------

#define MAX_PROCESSES 64
#define HISTORY_WINDOW 16   // Number of ticks to observe per process

typedef struct {
    uint32_t pid;
    uint32_t cpu_ticks_history[HISTORY_WINDOW]; // Rolling window of CPU usage
    uint32_t mem_pages_history[HISTORY_WINDOW];
    uint8_t  history_index;
    uint32_t predicted_cpu;   // AI-predicted CPU demand for next window
    uint32_t predicted_mem;   // AI-predicted memory demand
    uint32_t dynamic_priority; // Adjusted by AI prediction
} ai_pcb_t;

static ai_pcb_t ai_procs[MAX_PROCESSES];
static uint32_t ai_proc_count = 0;
static uint32_t current_ai_pid = 0;

// Simple Exponential Moving Average (EMA) — lightweight AI primitive
static uint32_t ema(const uint32_t* history, uint8_t len, uint32_t alpha_pct) {
    if (len == 0) return 0;
    uint32_t result = history[0];
    for (uint8_t i = 1; i < len; i++) {
        // EMA = alpha * new + (1 - alpha) * prev
        result = (alpha_pct * history[i] + (100 - alpha_pct) * result) / 100;
    }
    return result;
}

// Register a process with the AI scheduler
int ai_sched_register(uint32_t pid) {
    if (ai_proc_count >= MAX_PROCESSES) return -1;
    ai_pcb_t* p = &ai_procs[ai_proc_count++];
    p->pid = pid;
    p->history_index = 0;
    p->dynamic_priority = 50; // Neutral start
    for (int i = 0; i < HISTORY_WINDOW; i++) {
        p->cpu_ticks_history[i] = 0;
        p->mem_pages_history[i] = 0;
    }
    return 0;
}

// Update a process's resource usage metrics each tick
void ai_sched_update(uint32_t pid, uint32_t cpu_ticks, uint32_t mem_pages) {
    for (uint32_t i = 0; i < ai_proc_count; i++) {
        if (ai_procs[i].pid != pid) continue;
        ai_pcb_t* p = &ai_procs[i];
        uint8_t idx = p->history_index % HISTORY_WINDOW;
        p->cpu_ticks_history[idx] = cpu_ticks;
        p->mem_pages_history[idx] = mem_pages;
        p->history_index++;

        // Re-compute EMA predictions
        p->predicted_cpu = ema(p->cpu_ticks_history, HISTORY_WINDOW, 30);
        p->predicted_mem = ema(p->mem_pages_history, HISTORY_WINDOW, 30);

        // Raise priority for CPU-hungry processes, lower for idle
        if (p->predicted_cpu > 80) p->dynamic_priority = 90;  // Boost interactive
        else if (p->predicted_cpu < 10) p->dynamic_priority = 20; // Reduce background
        else p->dynamic_priority = 50; // Normal
        return;
    }
}

// AI-driven scheduling tick: pick highest priority predicted-needy process
uint32_t ai_sched_tick() {
    uint32_t best_pid = 0;
    uint32_t best_priority = 0;
    for (uint32_t i = 0; i < ai_proc_count; i++) {
        if (ai_procs[i].dynamic_priority > best_priority) {
            best_priority = ai_procs[i].dynamic_priority;
            best_pid = ai_procs[i].pid;
        }
    }
    current_ai_pid = best_pid;
    return best_pid;
}
