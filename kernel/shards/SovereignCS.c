/*
 * Σ SHARD: SOVEREIGN-CS — Computer Science Fundamentals v2.0
 * Doctrine: Pure C11. No stdlib. All UDF.
 * Provides: Context switch, Scheduling, Banker deadlock detection.
 */
#include "../sigma_kernel_types.h"

/* ---- Round-Robin Scheduler (bare-metal timeslice) ---- */
#define SIGMA_SCHED_MAX 16
typedef struct {
    u32 pid;
    u32 priority;   /* 0=highest */
    u32 burst_ms;   /* simulated burst in ms */
    u32 remaining;
    bool_t alive;
} SigmaTask;

typedef struct {
    SigmaTask tasks[SIGMA_SCHED_MAX];
    u32 n;
    u32 current;
} SigmaScheduler;

static inline void sched_init(SigmaScheduler* s) { s->n = 0; s->current = 0; }

static inline bool_t sched_add(SigmaScheduler* s, u32 pid, u32 prio, u32 burst) {
    if (s->n >= SIGMA_SCHED_MAX) return FALSE;
    s->tasks[s->n++] = (SigmaTask){ pid, prio, burst, burst, TRUE };
    return TRUE;
}

/* Returns PID of next running task (Round-Robin pass) */
static u32 sched_tick(SigmaScheduler* s, u32 quantum_ms) {
    for (u32 i = 0; i < s->n; i++) {
        u32 idx = (s->current + i) % s->n;
        SigmaTask* t = &s->tasks[idx];
        if (!t->alive) continue;
        t->remaining = (t->remaining > quantum_ms) ? t->remaining - quantum_ms : 0;
        if (t->remaining == 0) t->alive = FALSE;
        s->current = (idx + 1) % s->n;
        return t->pid;
    }
    return 0; /* all done */
}

/* ---- Context Switch (saves/restores register state in PCB) ---- */
#define SIGMA_PCB_REGS 16
typedef struct { u64 regs[SIGMA_PCB_REGS]; u64 pc; u64 sp; u32 pid; } SigmaPCB;

/* Pure C mock — in hardware build this wraps a naked ASM routine */
static inline void sigma_ctx_switch(SigmaPCB* old_ctx, SigmaPCB* new_ctx) {
    /* Save current (caller-save regs are stacked by C ABI already) */
    sigma_memcpy(old_ctx->regs, new_ctx->regs, SIGMA_PCB_REGS * sizeof(u64)); /* placeholder */
    old_ctx->pc = new_ctx->pc;
    old_ctx->sp = new_ctx->sp;
}

/* ---- Banker's Deadlock Safety (pure UDF) ---- */
#define BANKER_P 5
#define BANKER_R 3
typedef struct {
    u32 alloc[BANKER_P][BANKER_R];
    u32 max_need[BANKER_P][BANKER_R];
    u32 avail[BANKER_R];
} SigmaBanker;

static bool_t sigma_banker_safe(SigmaBanker* b) {
    u32 work[BANKER_R];
    bool_t finish[BANKER_P];
    sigma_memcpy(work, b->avail, BANKER_R * sizeof(u32));
    for (u32 i = 0; i < BANKER_P; i++) finish[i] = FALSE;
    bool_t changed = TRUE;
    while (changed) {
        changed = FALSE;
        for (u32 i = 0; i < BANKER_P; i++) {
            if (finish[i]) continue;
            bool_t ok = TRUE;
            for (u32 r = 0; r < BANKER_R; r++) {
                u32 need = b->max_need[i][r] - b->alloc[i][r];
                if (need > work[r]) { ok = FALSE; break; }
            }
            if (ok) {
                for (u32 r = 0; r < BANKER_R; r++) work[r] += b->alloc[i][r];
                finish[i] = TRUE; changed = TRUE;
            }
        }
    }
    for (u32 i = 0; i < BANKER_P; i++) if (!finish[i]) return FALSE;
    return TRUE;
}
