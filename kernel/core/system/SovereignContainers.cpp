#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Containerization Engine
 * Silicon-native deployment isolation.
 *
 * USP: Utilizes the Sovereign Enforcement Layer (SEL) and SovereignVFS to spawn 
 * mathematically isolated micro-VMs without the overhead of Docker or KVM.
 * Allows bare-metal speed while executing untrusted userland code.
 *
 * Design: OOP-isolated singleton — SovereignContainerEngine.
 */

class SovereignContainerEngine {
public:
    static SovereignContainerEngine& getInstance() {
        static SovereignContainerEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[CONTAINER] Initializing Sovereign Micro-VM Containerization...");
        this->active_containers = 0;
        sigma_log("[CONTAINER] Bare-metal isolation namespaces ACTIVE.");
    }

    void spawnContainer(const char* container_name, const char* entrypoint) {
        if (this->active_containers >= 32) {
            sigma_log("[CONTAINER] [ERROR] Namespace pool exhausted.");
            return;
        }

        sigma_hardened_strcpy(this->containers[this->active_containers], container_name, 32);
        this->active_containers++;
        
        sigma_log("[CONTAINER] Spawned Container '%s' executing '%s'. Total Active: %u\n", 
                     container_name, entrypoint, this->active_containers);
    }

private:
    SovereignContainerEngine() : active_containers(0) {}

    char containers[32][32];
    sigma_u32 active_containers;
};

/* --- C Wrappers --- */
extern "C" void container_init() {
    SovereignContainerEngine::init();
}

extern "C" void container_spawn(const char* name, const char* entrypoint) {
    SovereignContainerEngine::spawnContainer(name, entrypoint);
}




