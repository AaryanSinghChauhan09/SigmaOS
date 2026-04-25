// SigmaOS — sigma-sec-audit: Automated Security Auditor
// Module: sigma-sec-audit
// USP: Non-bypassable silicon-level event logger, integrates with CI/CD

#ifndef SIGMA_SEC_AUDIT_H
#define SIGMA_SEC_AUDIT_H

#define SIGMA_AUDIT_MAX_LOGS 1024

typedef enum SigmaAuditLevel {
    AUDIT_INFO     = 0,
    AUDIT_WARNING  = 1,
    AUDIT_CRITICAL = 2
} SigmaAuditLevel;

typedef struct SigmaAuditEvent {
    unsigned long   timestamp_rdtsc;
    SigmaAuditLevel level;
    unsigned int    actor_id; // PID or capability token ID
    unsigned int    target_id;
    char            action[32];
} SigmaAuditEvent;

typedef struct SigmaAuditLog {
    SigmaAuditEvent events[SIGMA_AUDIT_MAX_LOGS];
    unsigned int    count;
    unsigned int    dropped; // Logs dropped due to full buffer
} SigmaAuditLog;

static inline unsigned long audit_rdtsc(void) {
#if defined(__x86_64__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void audit_init(SigmaAuditLog* l) {
    l->count = 0;
    l->dropped = 0;
}

static inline void audit_log(SigmaAuditLog* l, SigmaAuditLevel level, 
                             unsigned int actor, unsigned int target, const char* action) {
    if (l->count >= SIGMA_AUDIT_MAX_LOGS) {
        l->dropped++;
        return;
    }
    SigmaAuditEvent* e = &l->events[l->count++];
    e->timestamp_rdtsc = audit_rdtsc();
    e->level = level;
    e->actor_id = actor;
    e->target_id = target;
    
    int i = 0;
    while(i < 31 && action[i]) {
        e->action[i] = action[i];
        i++;
    }
    e->action[i] = '\0';
}

#endif /* SIGMA_SEC_AUDIT_H */
