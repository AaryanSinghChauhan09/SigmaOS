#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

/* --- External Shard Endpoints --- */
extern "C" {
    void hal_init();
    void allocator_init();
    void pmm_init_shard(sigma_u64 mem_size);
    void vmm_init_shard();
    void vfs_init();
    void scheduler_init();
    void vault_init();
    void sel_init();
    void qkd_init();
    void mesh_init();
    void silicon_init_transpiler();
    void neural_automator_init();
    void market_init_shard();
    void orb_manager_init();
    void governance_init_shard();
    void persistence_init();
    void dna_init();
    void time_init();
    void diag_init();
    void ipc_init();
    void hyper_init();
    void neural_init();
    void pqc_init();
    void monitor_init();
    void sandbox_init();
    void ai_persona_init();
}

/**
 * SigmaOS Sovereign Orchestrator Shard
 * Principles: Deterministic Startup, Shard Dependency Resolution.
 * Mission: Bootstrapping the 600-shard lattice in zero-latency sequence.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignOrchestrator : public SigmaObject {
public:
    static SovereignOrchestrator& getInstance() {
        static SovereignOrchestrator instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignOrchestrator"; }

    void onShardEvent(const char* s) {
        (void)s;
        sigma_log("Σ [ORCHESTRATOR]: Processing Shard State Event...");
    }

    void bootstrap() {
        sigma_log("Σ [ORCHESTRATOR]: Initiating Lattice Phase 1 (Foundation)...");
        
        // FOUNDATION
        hal_init();
        time_init(); // Initialize timekeeping early
        diag_init(); // Initialize diagnostics early
        allocator_init();
        silicon_init_transpiler(); // Initialize native ISA translation
        pmm_init_shard(128 * 1024 * 1024); 
        vmm_init_shard();
        
        sigma_log("Σ [ORCHESTRATOR]: Initiating Lattice Phase 2 (Interaction)...");
        vfs_init();
        scheduler_init();
        dna_init();
        persistence_init();
        ipc_init(); // Initialize IPC before neural systems
        hyper_init(); // Initialize Hypervisor for shard isolation
        neural_init(); // Initialize Neural Nexus (S-NPU)
        monitor_init(); // Initialize System Monitor
        neural_automator_init(); // Activate Cognitive Task Queue
        
        sigma_log("Σ [ORCHESTRATOR]: Initiating Lattice Phase 3 (Security)...");
        vault_init();
        sel_init();
        pqc_init(); // Post-Quantum Cryptography active
        sandbox_init(); // Secure container isolation active
        qkd_init(); // Quantum-Key Distribution active
        mesh_init(); // Mesh-First Protocol active
        
        sigma_log("Σ [ORCHESTRATOR]: Initiating Lattice Phase 4 (Ecosystem)...");
        market_init_shard(); // Decentralized Orb Exchange active
        orb_manager_init(); // Local Orb verification and execution active
        ai_persona_init(); // Neural Adaptation active
        governance_init_shard(); // Community Contributor Registry active
        
        sigma_log("Σ [ORCHESTRATOR]: Lattice Singularity Achieved. System LIVE.");
    }

private:
    SovereignOrchestrator() {}
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS


/* --- C Bridge --- */
extern "C" void sigma_bootstrap_lattice() {
    SigmaOS::Kernel::System::SovereignOrchestrator::getInstance().bootstrap();
}
