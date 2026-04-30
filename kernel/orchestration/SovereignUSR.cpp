#include "sigma_hal.h"
#include "sigma_usr.h"

/**
 * SigmaOS Sovereign Unified Shard Registry (USR)
 * Implements a Quantum-Safe Shard Orchestrator for dynamic discovery.
 * Inspired by systemctl / apt-get / dbus.
 */

/* --- Sovereign USR Implementation --- */

void SovereignUSRManager::init() {
    sigma_log("[USR] Initializing Sovereign Unified Shard Registry...");
}

uint32_t SovereignUSRManager::registerShard(const char* name, uint32_t quantum_key) {
    if (this->count >= 512) return 0;
    
    uint32_t id = ++this->count;
    sigma_usr_entry_t* entry = &this->registry[id - 1];
    
    entry->shard_id = id;
    sigma_hardened_strcpy(entry->name, name, 64);
    entry->is_active = true;
    entry->quantum_key = quantum_key;
    
    sigma_printf("[USR] Registered Shard: %s (ID: %d, Key: 0x%08X)\n", name, id, quantum_key);
    return id;
}

bool SovereignUSRManager::activateShard(uint32_t shard_id) {
    if (shard_id == 0 || shard_id > this->count) return false;
    this->registry[shard_id - 1].is_active = true;
    sigma_printf("[USR] Shard S%02d Activated.\n", shard_id);
    return true;
}

/* --- C Wrappers --- */
extern "C" void usr_init() {
    SovereignUSRManager::getInstance().init();
}

extern "C" uint32_t usr_register_shard(const char* name, uint32_t quantum_key) {
    return SovereignUSRManager::getInstance().registerShard(name, quantum_key);
}

extern "C" bool usr_activate_shard(uint32_t shard_id) {
    return SovereignUSRManager::getInstance().activateShard(shard_id);
}
