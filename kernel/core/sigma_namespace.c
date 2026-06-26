/*
 * =========================================================================
 * Σ SIGMAOS: NAMESPACE & CGROUP PRIMITIVES (Phase 17)
 * =========================================================================
 * Native kernel implementation of container isolation.
 * No POSIX dependencies (no clone(), no unshare()).
 * =========================================================================
 */

#include "../../include/sigma_container.h"
#include "../../include/sigma_zenithd_log.h"
#include "../../include/sigma_libc.h"
#include "../../include/sigma_context.h"

static sigma_u32 next_namespace_id = 1;
static sigma_container_registry_t container_registry;

/*
 * sigma_sys_clone: Creates a new scheduling entity isolated by the specified flags.
 * Returns the global PID of the new entity.
 */
sigma_u32 sigma_sys_clone(void (*entry_point)(void*), void* arg, sigma_u32 iso_flags) {
    ZENITH_INFO("core_ns", "sigma_sys_clone invoked with isolation flags");
    
    sigma_namespace_t new_ns;
    sigma_memset(&new_ns, 0, sizeof(new_ns));
    
    if (iso_flags & SIGMA_CTR_ISO_PID) new_ns.pid_ns_id = next_namespace_id++;
    if (iso_flags & SIGMA_CTR_ISO_NET) new_ns.net_ns_id = next_namespace_id++;
    if (iso_flags & SIGMA_CTR_ISO_MNT) new_ns.mnt_ns_id = next_namespace_id++;
    
    /* TODO: Allocate new stack and initialize MLFQ context with sigma_context_init */
    (void)entry_point;
    (void)arg;
    
    sigma_u32 new_pid = 9999; /* Simulated global PID */
    
    ZENITH_INFO("core_ns", "Process cloned into new namespaces");
    return new_pid;
}

/* Userland API Implementation */
sigma_u32 sys_container_create(const char* name, sigma_u32 isolation_flags,
                               sigma_u32 cpu_shares, sigma_u32 mem_limit_mb) {
    /* Find empty slot */
    for (sigma_u32 i = 0; i < SIGMA_CTR_MAX; i++) {
        if (container_registry.containers[i].state == SIGMA_CTR_DEAD || 
            container_registry.containers[i].state == 0) {
            
            sigma_container_t* c = &container_registry.containers[i];
            c->id = i + 1;
            sigma_strncpy(c->name, name, SIGMA_CTR_NAME_LEN);
            c->state = SIGMA_CTR_CREATED;
            c->mem_limit_mb = mem_limit_mb;
            c->cpu_shares = cpu_shares;
            
            ZENITH_INFO("core_ns", "Container created in registry");
            return c->id;
        }
    }
    return 0;
}

void sys_container_start(sigma_u32 id) {
    if (id == 0 || id > SIGMA_CTR_MAX) return;
    container_registry.containers[id - 1].state = SIGMA_CTR_RUNNING;
    ZENITH_INFO("core_ns", "Container started");
}

void sys_container_stop(sigma_u32 id) {
    if (id == 0 || id > SIGMA_CTR_MAX) return;
    container_registry.containers[id - 1].state = SIGMA_CTR_STOPPED;
    ZENITH_INFO("core_ns", "Container stopped");
}

void sys_container_destroy(sigma_u32 id) {
    if (id == 0 || id > SIGMA_CTR_MAX) return;
    container_registry.containers[id - 1].state = SIGMA_CTR_DEAD;
    ZENITH_INFO("core_ns", "Container destroyed");
}

const sigma_container_registry_t* sys_container_get_registry(void) {
    return &container_registry;
}
