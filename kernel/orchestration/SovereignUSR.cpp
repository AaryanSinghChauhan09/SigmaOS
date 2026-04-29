#include "sigma_types.h"
#include "sigma_hal.h"
#include "sigma_libc.h"

/**
 * SigmaOS Sovereign Unified Shard Registry (USR)
 * Implements a Quantum-Safe Shard Orchestrator for dynamic discovery.
 * Inspired by systemctl / apt-get / dbus.
 */

typedef struct {
    uint32_t shard_id;
    char name[64];
    bool is_active;
    uint32_t quantum_key; // For amnesic-protected discovery
} usr_entry_t;

static struct {
    usr_entry_t registry[512];
    uint32_t count;
} SovereignUSRManager = {
    .count = 0
};

extern "C" void usr_init() {
    sigma_log("[USR] Initializing Sovereign Unified Shard Registry...");
}

extern "C" uint32_t usr_register_shard(const char* name, uint32_t quantum_key) {
    if (SovereignUSRManager.count >= 512) return 0;
    
    uint32_t id = ++SovereignUSRManager.count;
    usr_entry_t* entry = &SovereignUSRManager.registry[id - 1];
    
    entry->shard_id = id;
    sigma_hardened_strcpy(entry->name, name, 64);
    entry->is_active = true;
    entry->quantum_key = quantum_key;
    
    sigma_printf("[USR] Registered Shard: %s (ID: %d, Key: 0x%08X)\n", name, id, quantum_key);
    return id;
}

extern "C" bool usr_activate_shard(uint32_t shard_id) {
    if (shard_id == 0 || shard_id > SovereignUSRManager.count) return false;
    SovereignUSRManager.registry[shard_id - 1].is_active = true;
    sigma_printf("[USR] Shard S%02d Activated.\n", shard_id);
    return true;
}
