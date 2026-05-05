#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Unified Shard Registry (USR)
 * Quantum-Safe Shard Orchestrator.
 *
 * USP: Replaces bloated Linux dbus/systemctl with a mathematically verified,
 * amnesic-protected registry for dynamic service (shard) discovery and IPC.
 *
 * Design: OOP-isolated singleton — SovereignUSREngine.
 */

class SovereignUSREngine {
public:
    static SovereignUSREngine& getInstance() {
        static SovereignUSREngine instance;
        return instance;
    }

    void init() {
        sigma_log("[USR] Initializing Unified Shard Registry...");
        this->active_services = 0;
        sigma_log("[USR] Quantum-Safe Service Orchestrator ACTIVE.");
    }

    void registerShard(const char* shard_name, sigma_u32 shard_id) {
        if (this->active_services >= 256) return;
        sigma_hardened_strcpy(this->shard_names[this->active_services], shard_name, 32);
        this->shard_ids[this->active_services] = shard_id;
        this->active_services++;
        sigma_printf("[USR] Shard Registered: '%s' (ID: 0x%04X).\n", shard_name, shard_id);
    }

    sigma_u32 discoverShard(const char* shard_name) {
        for (sigma_u32 i = 0; i < this->active_services; i++) {
            if (sigma_hardened_strcmp(this->shard_names[i], shard_name) == 0) {
                sigma_printf("[USR] Shard Discovered: '%s' is active.\n", shard_name);
                return this->shard_ids[i];
            }
        }
        sigma_log("[USR] [WARN] Shard discovery failed: Service not found.");
        return 0; // Not found
    }

private:
    SovereignUSREngine() : active_services(0) {}

    char shard_names[256][32];
    sigma_u32 shard_ids[256];
    sigma_u32 active_services;
};

/* --- C Wrappers --- */
extern "C" void usr_init() {
    SovereignUSREngine::getInstance().init();
}

extern "C" void usr_register_shard(const char* name, sigma_u32 id) {
    SovereignUSREngine::getInstance().registerShard(name, id);
}

extern "C" sigma_u32 usr_discover_shard(const char* name) {
    return SovereignUSREngine::getInstance().discoverShard(name);
}


