// SigmaOS — sigma-sys-cron: Sovereign Task Scheduler (cron-inspired)
// Inspired by: cron, anacron, systemd timers
// Module: sigma-auto-cron
// USP: No /etc/crontab parser — tasks registered as C function pointers
// Execution driven by RDTSC-based wall-clock approximation, not system time

#ifndef SIGMA_SYS_CRON_H
#define SIGMA_SYS_CRON_H

#define SIGMA_CRON_MAX_TASKS  32
#define SIGMA_CRON_NAME_LEN   32
// Approximate: 3 GHz CPU → 3e9 cycles per second
#define SIGMA_CYCLES_PER_SEC  3000000000UL

typedef void (*cron_task_fn)(void* ctx);

typedef enum SigmaCronPeriod {
    CRON_EVERY_SECOND  = 1,
    CRON_EVERY_MINUTE  = 60,
    CRON_EVERY_HOUR    = 3600,
    CRON_EVERY_DAY     = 86400
} SigmaCronPeriod;

typedef struct SigmaCronTask {
    char          name[SIGMA_CRON_NAME_LEN];
    cron_task_fn  fn;
    void*         ctx;
    unsigned long period_cycles;   // interval in CPU cycles
    unsigned long last_run;        // RDTSC at last execution
    unsigned long run_count;
    unsigned char enabled;
} SigmaCronTask;

typedef struct SigmaCron {
    SigmaCronTask tasks[SIGMA_CRON_MAX_TASKS];
    unsigned int  count;
} SigmaCron;

static inline unsigned long cron_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void cron_init(SigmaCron* c) { c->count = 0; }

static inline int cron_register(SigmaCron* c, const char* name,
                                  cron_task_fn fn, void* ctx,
                                  unsigned long period_secs) {
    if (c->count >= SIGMA_CRON_MAX_TASKS) return -1;
    SigmaCronTask* t = &c->tasks[c->count++];
    for (int i = 0; i < SIGMA_CRON_NAME_LEN - 1 && name[i]; i++) t->name[i] = name[i];
    t->fn             = fn;
    t->ctx            = ctx;
    t->period_cycles  = period_secs * SIGMA_CYCLES_PER_SEC;
    t->last_run       = cron_rdtsc();
    t->run_count      = 0;
    t->enabled        = 1;
    return 0;
}

// Called from timer IRQ or main loop — fires due tasks
static inline unsigned int cron_tick(SigmaCron* c) {
    unsigned int fired = 0;
    unsigned long now = cron_rdtsc();
    for (unsigned int i = 0; i < c->count; i++) {
        SigmaCronTask* t = &c->tasks[i];
        if (!t->enabled) continue;
        if (now - t->last_run >= t->period_cycles) {
            t->fn(t->ctx);
            t->last_run = now;
            t->run_count++;
            fired++;
        }
    }
    return fired;
}

static inline void cron_disable(SigmaCron* c, const char* name) {
    for (unsigned int i = 0; i < c->count; i++) {
        const char* n = c->tasks[i].name; const char* s = name;
        while (*n && *s && *n == *s) { n++; s++; }
        if (!*n && !*s) { c->tasks[i].enabled = 0; return; }
    }
}

static inline void cron_enable(SigmaCron* c, const char* name) {
    for (unsigned int i = 0; i < c->count; i++) {
        const char* n = c->tasks[i].name; const char* s = name;
        while (*n && *s && *n == *s) { n++; s++; }
        if (!*n && !*s) { c->tasks[i].enabled = 1; return; }
    }
}

#endif /* SIGMA_SYS_CRON_H */
