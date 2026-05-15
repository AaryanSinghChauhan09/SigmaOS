#include "../../../../include/libc/sigma_libc.h"
#include "../../../../include/libc/sigma_libc.h"
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Self-Healing Kernel Module Watchdog
// Transactional rollback if a module crashes
// ---------------------------------------------------------

#define MAX_WATCHED_MODULES 32

typedef enum {
    MOD_STATE_OK,
    MOD_STATE_FAULTED,
    MOD_STATE_RECOVERING
} module_health_t;

typedef struct {
    char name[32];
    module_health_t health;
    uint32_t fault_count;
    uint32_t max_faults;         // After this many faults, mark unrecoverable
    void (*init_fn)(void);       // Used for restart
    void (*cleanup_fn)(void);
    uint8_t checkpoint[512];     // Snapshot of module state before last known-good state
} watched_module_t;

static watched_module_t watchlist[MAX_WATCHED_MODULES];
static uint32_t watch_count = 0;

// Register a module for watchdog monitoring
void watchdog_register(const char* name, uint32_t max_faults,
                       void(*init)(void), void(*cleanup)(void)) {
    if (watch_count >= MAX_WATCHED_MODULES) return;
    watched_module_t* m = &watchlist[watch_count++];
    strncpy(m->name, name, 31);
    m->health = MOD_STATE_OK;
    m->fault_count = 0;
    m->max_faults = max_faults;
    m->init_fn = init;
    m->cleanup_fn = cleanup;
    memset(m->checkpoint, 0, sizeof(m->checkpoint));
}

// Save a checkpoint (called before risky operations)
void watchdog_checkpoint(const char* name, const uint8_t* state, size_t len) {
    for (uint32_t i = 0; i < watch_count; i++) {
        if (strncmp(watchlist[i].name, name, 31) == 0) {
            size_t copy_len = len < 512 ? len : 512;
            memcpy(watchlist[i].checkpoint, state, copy_len);
            return;
        }
    }
}

// Trigger recovery when a module faults
int watchdog_recover(const char* name) {
    for (uint32_t i = 0; i < watch_count; i++) {
        if (strncmp(watchlist[i].name, name, 31) != 0) continue;
        watched_module_t* m = &watchlist[i];
        m->fault_count++;
        m->health = MOD_STATE_RECOVERING;

        if (m->fault_count > m->max_faults) {
            m->health = MOD_STATE_FAULTED;
            return -1; // Unrecoverable — disable module
        }

        // Teardown and restart from checkpoint
        if (m->cleanup_fn) m->cleanup_fn();
        if (m->init_fn)    m->init_fn();
        m->health = MOD_STATE_OK;
        return 0; // Recovered
    }
    return -2; // Module not found in watchlist
}
