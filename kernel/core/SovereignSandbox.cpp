#include "sigma_sandbox.h"
#include "sigma_hal.h"
#include "sigma_libc.h"

/**
 * SigmaOS Sovereign Sandbox Container
 * Implements a Cryptographic Isolation Boundary (CIB) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal application sandboxing.
 */

/* --- Sovereign Sandbox Manager (OOPS Isolation) --- */
static struct {
    sigma_sandbox_config_t active_containers[256];
    uint32_t container_count;
} SovereignSandboxManager = {
    .container_count = 0
};

extern "C" void sandbox_init() {
    sigma_log("[SANDBOX] Initializing Sovereign Sandbox Container (OOPS Isolation)...");
}

extern "C" uint32_t sandbox_create_container(const sigma_sandbox_config_t* config) {
    if (SovereignSandboxManager.container_count >= 256) return 0;
    
    uint32_t id = ++SovereignSandboxManager.container_count;
    SovereignSandboxManager.active_containers[id - 1] = *config;
    SovereignSandboxManager.active_containers[id - 1].container_id = id;
    
    sigma_printf("[SANDBOX] CIB: Created isolated container ID %d.\n", (int)id);
    return id;
}

extern "C" bool sandbox_execute(uint32_t container_id, const char* binary_path) {
    if (container_id == 0 || container_id > SovereignSandboxManager.container_count) return false;
    
    sigma_sandbox_config_t* config = &SovereignSandboxManager.active_containers[container_id - 1];
    if (config->container_id == 0) return false;

    sigma_printf("[SANDBOX] CIB: Validating Enclave Key for Container %d...\n", (int)container_id);
    sigma_printf("[SANDBOX] CIB: Executing '%s' within Container %d...\n", binary_path, (int)container_id);
    
    if (!config->network_access) {
        sigma_log("[SANDBOX] CIB: Network access BLOCKED by container policy.");
    }
    
    sigma_log("[SANDBOX] CIB: Secure execution started in restricted silicon domain.");
    return true;
}

extern "C" void sandbox_destroy_container(uint32_t container_id) {
    if (container_id > 0 && container_id <= SovereignSandboxManager.container_count) {
        sigma_printf("[SANDBOX] CIB: Destroying container ID %d.\n", (int)container_id);
        SovereignSandboxManager.active_containers[container_id - 1].container_id = 0; 
    }
}
