// SigmaOS — Cooperative Round-Robin Scheduler
// Module: sigma-sys-scheduler
// Single responsibility: maintain a run queue and context-switch between tasks
// Uses Inline Assembly (RDTSC) for time-slice tracking

#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#define SIGMA_MAX_TASKS  32
#define SIGMA_TASK_NAME_LEN 16

typedef enum SigmaTaskState {
    TASK_READY   = 0,
    TASK_RUNNING = 1,
    TASK_BLOCKED = 2,
    TASK_DEAD    = 3
} SigmaTaskState;

typedef void (*sigma_task_fn)(void*);

typedef struct SigmaTask {
    unsigned int      pid;
    char              name[SIGMA_TASK_NAME_LEN];
    SigmaTaskState    state;
    sigma_task_fn     entry;
    void*             arg;
    unsigned long     slice_start; // RDTSC cycle count
} SigmaTask;

typedef struct SigmaScheduler {
    SigmaTask tasks[SIGMA_MAX_TASKS];
    unsigned int count;
    unsigned int current;
} SigmaScheduler;

/* Read hardware cycle counter */
static inline unsigned long rdtsc_now(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void sched_init(SigmaScheduler* s) {
    s->count   = 0;
    s->current = 0;
}

static inline int sched_add(SigmaScheduler* s, const char* name,
                              sigma_task_fn fn, void* arg) {
    if (s->count >= SIGMA_MAX_TASKS) return -1;
    SigmaTask* t = &s->tasks[s->count];
    t->pid   = s->count + 1;
    t->state = TASK_READY;
    t->entry = fn;
    t->arg   = arg;
    t->slice_start = 0;
    // Copy name manually — no strncpy
    for (int i = 0; i < SIGMA_TASK_NAME_LEN - 1 && name[i]; i++)
        t->name[i] = name[i];
    t->name[SIGMA_TASK_NAME_LEN - 1] = '\0';
    s->count++;
    return (int)t->pid;
}

/* Run one scheduling tick: pick next READY task and execute it */
static inline void sched_tick(SigmaScheduler* s) {
    if (s->count == 0) return;
    for (unsigned int i = 0; i < s->count; i++) {
        unsigned int idx = (s->current + i) % s->count;
        SigmaTask* t = &s->tasks[idx];
        if (t->state == TASK_READY) {
            t->state = TASK_RUNNING;
            t->slice_start = rdtsc_now();
            if (t->entry) t->entry(t->arg);
            t->state = TASK_READY;
            s->current = (idx + 1) % s->count;
            return;
        }
    }
}

static inline void sched_block(SigmaScheduler* s, unsigned int pid) {
    for (unsigned int i = 0; i < s->count; i++)
        if (s->tasks[i].pid == pid) { s->tasks[i].state = TASK_BLOCKED; return; }
}

static inline void sched_unblock(SigmaScheduler* s, unsigned int pid) {
    for (unsigned int i = 0; i < s->count; i++)
        if (s->tasks[i].pid == pid) { s->tasks[i].state = TASK_READY; return; }
}

#endif /* SIGMA_SCHEDULER_H */
