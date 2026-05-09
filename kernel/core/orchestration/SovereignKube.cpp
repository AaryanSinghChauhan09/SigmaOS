#include "sigma_log.h"
#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "sigma_kube.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

SovereignKubeEngine& SovereignKubeEngine::getInstance() {
    static SovereignKubeEngine instance;
    return instance;
}

void SovereignKubeEngine::init() {
    sigma_log("[KUBE] Initializing Sovereign Lattice Reconciliation Loop (LRL) Orchestrator...");
    this->initialized = 1u;
    sigma_log("[KUBE] LRL: Orchestration nexus ONLINE. Shard scaling ARMED.");
}

sigma_u32 SovereignKubeEngine::createDeployment(const char* name, sigma_u32 container_id,
                                          sigma_u32 pattern, sigma_u32 replicas) {
    if (this->state.active_deployments >= SIGMA_KUBE_DEPLOYMENT_MAX) {
        sigma_log("[KUBE] LRL: [WARN] Deployment registry FULL.");
        return 0u;
    }

    sigma_kube_deployment_t* dep = 
        &this->state.deployments[this->state.active_deployments++];
    
    dep->deployment_id = this->next_deployment_id++;
    dep->pattern       = pattern;
    dep->replicas      = replicas;
    dep->current_count = 0u;
    dep->container_id  = container_id;

    sigma_u32 i = 0u;
    while (i < SIGMA_KUBE_NAME_LEN - 1u && name && name[i]) {
        dep->name[i] = name[i];
        i++;
    }
    dep->name[i] = '\0';

    sigma_log("[KUBE] LRL: Deployment '%s' (ID=%u) CREATED. Desired replicas: %u.\n",
                 dep->name, dep->deployment_id, replicas);
    
    this->reconcileLattice();
    return dep->deployment_id;
}

void SovereignKubeEngine::scaleDeployment(sigma_u32 deployment_id, sigma_u32 replicas) {
    sigma_kube_deployment_t* dep = this->findDeployment(deployment_id);
    if (!dep) return;

    dep->replicas = replicas;
    sigma_log("[KUBE] LRL: Scaling deployment '%s' to %u replicas.\n", dep->name, replicas);
    this->reconcileLattice();
}

void SovereignKubeEngine::deleteDeployment(sigma_u32 deployment_id) {
    sigma_kube_deployment_t* dep = this->findDeployment(deployment_id);
    if (!dep) return;

    sigma_log("[KUBE] LRL: Deleting deployment '%s'. Terminating shards...\n", dep->name);
    dep->replicas = 0u;
    this->reconcileLattice();
}

void SovereignKubeEngine::reconcileLattice() {
    sigma_log("[KUBE] LRL: Commencing lattice reconciliation sweep...");
    
    sigma_u32 new_total_pods = 0u;
    for (sigma_u32 i = 0u; i < this->state.active_deployments; i++) {
        sigma_kube_deployment_t* dep = &this->state.deployments[i];
        
        if (dep->current_count < dep->replicas) {
            sigma_u32 diff = dep->replicas - dep->current_count;
            sigma_log("[KUBE] LRL: Scaling UP '%s'. Spawning %u instances.\n", dep->name, diff);
            dep->current_count = dep->replicas; /* Simulated immediate spawn */
        } else if (dep->current_count > dep->replicas) {
            sigma_u32 diff = dep->current_count - dep->replicas;
            sigma_log("[KUBE] LRL: Scaling DOWN '%s'. Terminating %u instances.\n", dep->name, diff);
            dep->current_count = dep->replicas; /* Simulated immediate term */
        }
        new_total_pods += dep->current_count;
    }
    
    this->state.total_pods = new_total_pods;
    sigma_log("[KUBE] LRL: Reconciliation COMPLETE. Lattice is stable.");
}

const sigma_kube_state_t* SovereignKubeEngine::getState() const {
    return &this->state;
}

sigma_kube_deployment_t* SovereignKubeEngine::findDeployment(sigma_u32 id) {
    for (sigma_u32 i = 0u; i < this->state.active_deployments; i++) {
        if (this->state.deployments[i].deployment_id == id)
            return &this->state.deployments[i];
    }
    return SIGMA_NULL;
}

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void kube_init() {
    SigmaOS::Kernel::Orchestration::SovereignKubeEngine::init();
}

extern "C" sigma_u32 kube_create_deployment(const char* name, sigma_u32 container_id,
                                            sigma_u32 pattern, sigma_u32 replicas) {
    return SigmaOS::Kernel::Orchestration::SovereignKubeEngine::createDeployment(name, container_id, pattern, replicas);
}

extern "C" void kube_scale_deployment(sigma_u32 deployment_id, sigma_u32 replicas) {
    SigmaOS::Kernel::Orchestration::SovereignKubeEngine::scaleDeployment(deployment_id, replicas);
}

extern "C" void kube_delete_deployment(sigma_u32 deployment_id) {
    SigmaOS::Kernel::Orchestration::SovereignKubeEngine::deleteDeployment(deployment_id);
}

extern "C" void kube_reconcile_lattice() {
    SigmaOS::Kernel::Orchestration::SovereignKubeEngine::reconcileLattice();
}

extern "C" const sigma_kube_state_t* kube_get_state() {
    return SigmaOS::Kernel::Orchestration::SovereignKubeEngine::getState();
}




