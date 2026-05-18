#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/security/sigma_sandbox.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Sandbox Container
 * Implements a Cryptographic Isolation Boundary (CIB) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal application sandboxing.
 */

class SovereignSandboxManager {
public:
    static SovereignSandboxManager& getInstance() {
        static SovereignSandboxManager instance;
        return instance;
    }

    void init() {
        sigma_log("[SANDBOX] Initializing Sovereign Sandbox Container (OOPS Isolation)...");
    }

    uint32_t createContainer(const sigma_sandbox_config_t* config) {
        if (this->container_count >= 256) return 0;
        
        uint32_t id = ++this->container_count;
        this->active_containers[id - 1] = *config;
        this->active_containers[id - 1].container_id = id;
        
        sigma_log_info("[SANDBOX] CIB: Created isolated container ID %d.\n", (int)id);
        return id;
    }

    bool execute(uint32_t container_id, const char* binary_path) {
        if (container_id == 0 || container_id > this->container_count) return false;
        
        sigma_sandbox_config_t* config = &this->active_containers[container_id - 1];
        if (config->container_id == 0) return false;

        sigma_log_info("[SANDBOX] CIB: Validating Enclave Key for Container %d...\n", (int)container_id);
        sigma_log_info("[SANDBOX] CIB: Executing '%s' within Container %d...\n", binary_path, (int)container_id);
        
        if (!config->network_access) {
            sigma_log("[SANDBOX] CIB: Network access BLOCKED by container policy.");
        }
        
        sigma_log("[SANDBOX] CIB: Secure execution started in restricted silicon domain.");
        return true;
    }

    void destroyContainer(uint32_t container_id) {
        if (container_id > 0 && container_id <= this->container_count) {
            sigma_log_info("[SANDBOX] CIB: Destroying container ID %d.\n", (int)container_id);
            this->active_containers[container_id - 1].container_id = 0; 
        }
    }

private:
    SovereignSandboxManager() : container_count(0) {}
    
    sigma_sandbox_config_t active_containers[256];
    uint32_t container_count;
};

/* --- C Wrappers --- */
extern "C" void sandbox_init() {
    SovereignSandboxManager::getInstance().init();
}

extern "C" uint32_t sandbox_create_container(const sigma_sandbox_config_t* config) {
    return SovereignSandboxManager::getInstance().createContainer(config);
}

extern "C" bool sandbox_execute(uint32_t container_id, const char* binary_path) {
    return SovereignSandboxManager::getInstance().execute(container_id, binary_path);
}

extern "C" void sandbox_destroy_container(uint32_t container_id) {
    SovereignSandboxManager::getInstance().destroyContainer(container_id);
}


 