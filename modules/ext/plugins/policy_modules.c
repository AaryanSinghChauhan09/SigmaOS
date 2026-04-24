#include <stdint.h>
#include "sigma_libc.h"

// ---------------------------------------------------------
// SigmaOS Policy Module System
// Security, scheduling and resource policies are loadable
// modules — swap the OS's behavior without rebooting
// ---------------------------------------------------------

#define MAX_POLICIES    16
#define POLICY_NAME_LEN 32

typedef enum {
    POLICY_SECURITY,
    POLICY_SCHEDULING,
    POLICY_MEMORY,
    POLICY_NETWORK
} policy_type_t;

typedef struct {
    uint32_t     policy_id;
    char         name[POLICY_NAME_LEN];
    policy_type_t type;
    uint8_t      active;

    // Security policy hooks
    int  (*can_access)(uint32_t pid, uint32_t resource_id, uint8_t rights);
    void (*on_violation)(uint32_t pid, const char* reason);

    // Scheduling policy hooks
    uint32_t (*pick_next)(uint32_t* runqueue, uint32_t queue_len);
    uint32_t (*timeslice_ms)(uint32_t pid, uint8_t priority);

    // Memory quota hook
    uint32_t (*max_pages)(uint32_t pid, uint8_t priority);
} policy_module_t;

static policy_module_t policies[MAX_POLICIES];
static uint32_t policy_count = 0;
static uint32_t active_policy[4] = {0}; // one active policy per type

// --- Built-in: Strict security policy (deny all by default) ---
static int strict_can_access(uint32_t pid, uint32_t resource_id, uint8_t rights) {
    (void)pid; (void)resource_id; (void)rights;
    return 0; // Deny everything unless explicitly granted
}
static void strict_on_violation(uint32_t pid, const char* reason) {
    // Immediately suspend process on violation
    // cap_registry_revoke_pid(pid);
}

// --- Built-in: Permissive security policy (dev mode) ---
static int permissive_can_access(uint32_t pid, uint32_t resource_id, uint8_t rights) {
    (void)pid; (void)resource_id; (void)rights;
    return 1; // Allow everything (dev/debug use only)
}
static void permissive_on_violation(uint32_t pid, const char* reason) {
    // Log only, don't kill
}

// --- Built-in: Round-robin scheduler policy ---
static uint32_t rr_pick_next(uint32_t* runqueue, uint32_t len) {
    static uint32_t idx = 0;
    if (len == 0) return UINT32_MAX;
    return runqueue[(idx++) % len];
}
static uint32_t rr_timeslice_ms(uint32_t pid, uint8_t priority) { (void)pid; return 10; }

// --- Built-in: Priority scheduler policy ---
static uint32_t prio_pick_next(uint32_t* runqueue, uint32_t len) {
    // In real impl: sort by priority, pick highest
    return len > 0 ? runqueue[0] : UINT32_MAX;
}
static uint32_t prio_timeslice_ms(uint32_t pid, uint8_t priority) {
    return (uint32_t)(5 + priority * 2); // Higher priority = longer slice
}

// Register a policy module
uint32_t policy_register(const char* name, policy_type_t type,
                          int(*can_access)(uint32_t,uint32_t,uint8_t),
                          void(*on_violation)(uint32_t,const char*),
                          uint32_t(*pick_next)(uint32_t*,uint32_t),
                          uint32_t(*timeslice_ms)(uint32_t,uint8_t),
                          uint32_t(*max_pages)(uint32_t,uint8_t)) {
    if (policy_count >= MAX_POLICIES) return UINT32_MAX;
    policy_module_t* p = &policies[policy_count];
    p->policy_id = policy_count++;
    __builtin_strncpy(p->name, name, POLICY_NAME_LEN - 1);
    p->type = type;
    p->active = 0;
    p->can_access = can_access;
    p->on_violation = on_violation;
    p->pick_next = pick_next;
    p->timeslice_ms = timeslice_ms;
    p->max_pages = max_pages;
    return p->policy_id;
}

// Hot-swap the active policy for a given type
int policy_activate(uint32_t policy_id) {
    if (policy_id >= policy_count) return -1;
    policy_type_t type = policies[policy_id].type;

    // Deactivate current
    if (active_policy[type] < policy_count)
        policies[active_policy[type]].active = 0;

    policies[policy_id].active = 1;
    active_policy[type] = policy_id;
    return 0;
}

// Dispatch: check access via active security policy
int policy_check_access(uint32_t pid, uint32_t resource_id, uint8_t rights) {
    policy_module_t* p = &policies[active_policy[POLICY_SECURITY]];
    if (!p->active || !p->can_access) return 0;
    return p->can_access(pid, resource_id, rights);
}

// Dispatch: pick next scheduled process via active scheduling policy
uint32_t policy_schedule_next(uint32_t* runqueue, uint32_t len) {
    policy_module_t* p = &policies[active_policy[POLICY_SCHEDULING]];
    if (!p->active || !p->pick_next) return UINT32_MAX;
    return p->pick_next(runqueue, len);
}

// Boot-time registration of built-in policies
void policy_init(void) {
    uint32_t strict_id = policy_register("strict_security",  POLICY_SECURITY,
        strict_can_access, strict_on_violation, NULL, NULL, NULL);
    policy_register("permissive_security", POLICY_SECURITY,
        permissive_can_access, permissive_on_violation, NULL, NULL, NULL);
    uint32_t rr_id = policy_register("round_robin_scheduler", POLICY_SCHEDULING,
        NULL, NULL, rr_pick_next, rr_timeslice_ms, NULL);
    policy_register("priority_scheduler", POLICY_SCHEDULING,
        NULL, NULL, prio_pick_next, prio_timeslice_ms, NULL);

    // Activate defaults
    policy_activate(strict_id);
    policy_activate(rr_id);
}
