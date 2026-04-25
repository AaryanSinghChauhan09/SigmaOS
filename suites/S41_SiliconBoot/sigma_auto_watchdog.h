// SigmaOS — sigma-auto-watchdog: Service Watchdog & Self-Healing
// Inspired by: systemd watchdog, supervisord, s6-svscan
// Module: sigma-auto-watchdog
// USP: No DBus, no cgroups dependency — pure C health-check ring
// Watchdog tick driven by hardware timer IRQ, not userland polling

#ifndef SIGMA_AUTO_WATCHDOG_H
#define SIGMA_AUTO_WATCHDOG_H

#define SIGMA_WD_MAX_SERVICES  32
#define SIGMA_WD_NAME_LEN      32
#define SIGMA_WD_MAX_RESTARTS   5

typedef enum SigmaWDState {
    WD_HEALTHY   = 0,
    WD_STALE     = 1,
    WD_RESTARTING= 2,
    WD_DEAD      = 3
} SigmaWDState;

typedef void (*wd_restart_fn)(void* ctx);

typedef struct SigmaWDService {
    char          name[SIGMA_WD_NAME_LEN];
    SigmaWDState  state;
    unsigned long last_heartbeat;  // RDTSC cycle count
    unsigned long timeout_cycles;  // max silence before STALE
    unsigned int  restart_count;
    unsigned int  max_restarts;
    wd_restart_fn restart_fn;
    void*         restart_ctx;
} SigmaWDService;

typedef struct SigmaWatchdog {
    SigmaWDService services[SIGMA_WD_MAX_SERVICES];
    unsigned int   count;
} SigmaWatchdog;

static inline unsigned long wd_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void wd_init(SigmaWatchdog* w) { w->count = 0; }

static inline int wd_register(SigmaWatchdog* w, const char* name,
                                unsigned long timeout_cycles,
                                wd_restart_fn restart, void* ctx) {
    if (w->count >= SIGMA_WD_MAX_SERVICES) return -1;
    SigmaWDService* s = &w->services[w->count++];
    for (int i = 0; i < SIGMA_WD_NAME_LEN - 1 && name[i]; i++) s->name[i] = name[i];
    s->state          = WD_HEALTHY;
    s->last_heartbeat = wd_rdtsc();
    s->timeout_cycles = timeout_cycles;
    s->restart_count  = 0;
    s->max_restarts   = SIGMA_WD_MAX_RESTARTS;
    s->restart_fn     = restart;
    s->restart_ctx    = ctx;
    return 0;
}

// Service calls this to prove it is alive
static inline void wd_heartbeat(SigmaWatchdog* w, const char* name) {
    for (unsigned int i = 0; i < w->count; i++) {
        const char* n = w->services[i].name; const char* s = name;
        while (*n && *s && *n == *s) { n++; s++; }
        if (!*n && !*s) {
            w->services[i].last_heartbeat = wd_rdtsc();
            w->services[i].state = WD_HEALTHY;
            return;
        }
    }
}

// Called from timer IRQ — checks all services
static inline void wd_tick(SigmaWatchdog* w) {
    unsigned long now = wd_rdtsc();
    for (unsigned int i = 0; i < w->count; i++) {
        SigmaWDService* s = &w->services[i];
        if (s->state == WD_DEAD) continue;
        if (now - s->last_heartbeat > s->timeout_cycles) {
            if (s->restart_count < s->max_restarts) {
                s->state = WD_RESTARTING;
                s->restart_count++;
                if (s->restart_fn) s->restart_fn(s->restart_ctx);
                s->last_heartbeat = wd_rdtsc();
                s->state = WD_HEALTHY;
            } else {
                s->state = WD_DEAD;
            }
        }
    }
}

static inline unsigned int wd_dead_count(SigmaWatchdog* w) {
    unsigned int n = 0;
    for (unsigned int i = 0; i < w->count; i++)
        if (w->services[i].state == WD_DEAD) n++;
    return n;
}

#endif /* SIGMA_AUTO_WATCHDOG_H */
