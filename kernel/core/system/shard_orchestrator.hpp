#include "../../../include/hal/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#ifndef SHARD_ORCHESTRATOR_HPP
#define SHARD_ORCHESTRATOR_HPP

#include "../../../include/core/sigma_types.h"
#include "../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

struct ShardMetadata {
    const char* name;
    sigma_u32 version;
    sigma_bool active;
    sigma_u64 load_addr;
};

/*
 * =========================================================================
 * SOVEREIGN SHARD ORCHESTRATOR (USR - Unified Shard Registry)
 * =========================================================================
 * Quantum-Safe Registry for dynamic shard discovery and lifecycle management.
 */
class SovereignShardOrchestrator : public SigmaObject {
private:
    ShardMetadata m_registry[512];
    sigma_u32 m_shard_count;
    sigma_u64 m_lattice_signature; // PQC Signature for lattice integrity

public:
    SovereignShardOrchestrator();
    const char* type_name() const noexcept override { return "SovereignShardOrchestrator"; }

    void RegisterShard(const char* name, sigma_u32 version, sigma_u64 addr);
    void ActivateShard(const char* name);
    void DeactivateShard(const char* name);
    
    // Quantum-Safe Lattice Audit
    sigma_bool VerifyLatticeIntegrity();
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

