#include <stdint.h>
#include "sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Service Capsule System
// Every subsystem is an independent hot-swappable capsule
// ---------------------------------------------------------

#define MAX_CAPSULES    64
#define MAX_DEPS        8
#define CAPSULE_NAME    32

typedef enum {
    CAPSULE_UNLOADED,
    CAPSULE_LOADING,
    CAPSULE_ACTIVE,
    CAPSULE_SUSPENDED,
    CAPSULE_FAULTED
} capsule_state_t;

typedef struct {
    uint32_t       capsule_id;
    char           name[CAPSULE_NAME];
    capsule_state_t state;
    uint32_t       version;          // Semantic version packed: major<<16|minor<<8|patch
    uint32_t       deps[MAX_DEPS];   // Dependency capsule IDs (0 = none)
    uint8_t        dep_count;
    uint32_t       required_caps;    // Bitmask of capability types needed
    void (*init)(void);
    void (*suspend)(void);
    void (*resume)(void);
    void (*teardown)(void);
} capsule_t;

static capsule_t capsules[MAX_CAPSULES];
static uint32_t capsule_count = 0;

// Register a capsule (done at boot or via module store)
uint32_t capsule_register(const char* name, uint32_t version,
                          void(*init)(void), void(*suspend)(void),
                          void(*resume)(void), void(*teardown)(void)) {
    if (capsule_count >= MAX_CAPSULES) return UINT32_MAX;
    capsule_t* c = &capsules[capsule_count];
    c->capsule_id = capsule_count++;
    strncpy(c->name, name, CAPSULE_NAME - 1);
    c->version = version;
    c->state = CAPSULE_UNLOADED;
    c->dep_count = 0;
    c->init = init;
    c->suspend = suspend;
    c->resume = resume;
    c->teardown = teardown;
    return c->capsule_id;
}

// Declare a dependency (capsule_id depends on dep_id being active first)
int capsule_add_dep(uint32_t capsule_id, uint32_t dep_id) {
    capsule_t* c = &capsules[capsule_id];
    if (c->dep_count >= MAX_DEPS) return -1;
    c->deps[c->dep_count++] = dep_id;
    return 0;
}

// Resolve and load a capsule (respects dependency order)
int capsule_load(uint32_t capsule_id) {
    if (capsule_id >= capsule_count) return -1;
    capsule_t* c = &capsules[capsule_id];
    if (c->state == CAPSULE_ACTIVE) return 0; // Already running

    // Load all dependencies first (recursive)
    for (uint8_t i = 0; i < c->dep_count; i++) {
        int r = capsule_load(c->deps[i]);
        if (r != 0) return r; // Dependency failed
    }

    c->state = CAPSULE_LOADING;
    if (c->init) c->init();
    c->state = CAPSULE_ACTIVE;
    return 0;
}

// Hot-swap: suspend old capsule, activate new version
int capsule_hotswap(uint32_t old_id, uint32_t new_id) {
    if (old_id >= capsule_count || new_id >= capsule_count) return -1;
    capsule_t* old = &capsules[old_id];
    capsule_t* nw  = &capsules[new_id];

    // Suspend old
    old->state = CAPSULE_SUSPENDED;
    if (old->suspend) old->suspend();

    // Activate new
    int r = capsule_load(new_id);
    if (r != 0) {
        // Rollback: resume old if new fails
        if (old->resume) old->resume();
        old->state = CAPSULE_ACTIVE;
        return -2;
    }

    // Teardown old (now safe since new is active)
    if (old->teardown) old->teardown();
    old->state = CAPSULE_UNLOADED;
    return 0;
}

// Unload a capsule (with capability cleanup hook)
int capsule_unload(uint32_t capsule_id) {
    if (capsule_id >= capsule_count) return -1;
    capsule_t* c = &capsules[capsule_id];
    if (c->teardown) c->teardown();
    c->state = CAPSULE_UNLOADED;
    return 0;
}
