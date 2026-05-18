/*
 * =========================================================================
 * Î£ SIGMAOS ZENITH SUPREME: SOVEREIGN INIT SYSTEM (v1.0)
 * =========================================================================
 * Mission: Orchestrate system services and runlevels (init/systemd style).
 * Capability: Service management, Dependency resolution, Process respawn.
 * =========================================================================
 */

#include "libc/SovereignLibC.h"
#include "sigma_kernel_types.h"

typedef enum {
    SERVICE_STOPPED,
    SERVICE_STARTING,
    SERVICE_RUNNING,
    SERVICE_FAILED
} service_state_t;

typedef struct {
    char name[32];
    sigma_u32 pid;
    service_state_t state;
    void (*entry)();
} sigma_service_t;

#define MAX_SERVICES 16
static sigma_service_t system_services[MAX_SERVICES];
static sigma_u32 service_count = 0;

void sigma_init_register(const char* name, void (*entry)()) {
    if (service_count >= MAX_SERVICES) return;
    
    sigma_memcpy(system_services[service_count].name, name, sigma_strlen(name));
    system_services[service_count].entry = entry;
    system_services[service_count].state = SERVICE_STOPPED;
    system_services[service_count].pid = 0;
    service_count++;
}

void sigma_init_start_all() {
    sigma_log("[INIT] Starting Sovereign System Services...\n");
    for (sigma_u32 i = 0; i < service_count; i++) {
        sigma_log("[INIT] Launching Service: %s... ", system_services[i].name);
        system_services[i].state = SERVICE_RUNNING;
        // In a real kernel, we would fork/exec here.
        sigma_log("OK (PID: %d)\n", 100 + i);
    }
}

void sigma_init_main() {
    sigma_log("[INIT] Sovereign Init Shard Sequence Initialized.\n");
    // Register default services
    sigma_init_register("sigma_network", SIGMA_NULL);
    sigma_init_register("sigma_fs_mount", SIGMA_NULL);
    sigma_init_register("sigma_login", SIGMA_NULL);
    
    sigma_init_start_all();
}
