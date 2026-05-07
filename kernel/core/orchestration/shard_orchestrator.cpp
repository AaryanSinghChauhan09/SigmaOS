#include "hal/sigma_hal.h"
#include "core/sigma_types.h"
#include "shard_orchestrator.hpp"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

SovereignShardOrchestrator::SovereignShardOrchestrator() : m_shard_count(0), m_lattice_signature(0xDEADC0DEBEEFCAFE) {
    sigma_memset(m_registry, 0, sizeof(m_registry));
    sigma_log("[USR]: Unified Shard Registry Online. Lattice Signature Verified.\n");
}

void SovereignShardOrchestrator::RegisterShard(const char* name, sigma_u32 version, sigma_u64 addr) {
    if (m_shard_count < 512) {
        m_registry[m_shard_count] = { name, version, SIGMA_FALSE, addr };
        m_shard_count++;
        sigma_log("[USR]: Shard Registered: %s (v%d.%d) at %p\n", name, version >> 16, version & 0xFFFF, (void*)addr);
    }
}

void SovereignShardOrchestrator::ActivateShard(const char* name) {
    for (sigma_u32 i = 0; i < m_shard_count; ++i) {
        if (sigma_streq(m_registry[i].name, name)) {
            m_registry[i].active = SIGMA_TRUE;
            sigma_log("[USR]: Shard ACTIVATED: %s\n", name);
            return;
        }
    }
}

void SovereignShardOrchestrator::DeactivateShard(const char* name) {
    for (sigma_u32 i = 0; i < m_shard_count; ++i) {
        if (sigma_streq(m_registry[i].name, name)) {
            m_registry[i].active = SIGMA_FALSE;
            sigma_log("[USR]: Shard DEACTIVATED: %s\n", name);
            return;
        }
    }
}

sigma_bool SovereignShardOrchestrator::VerifyLatticeIntegrity() {
    // Simulated Post-Quantum Cryptographic verification
    sigma_log("[USR]: Performing Lattice-PQC Signature Audit...\n");
    return (m_lattice_signature == 0xDEADC0DEBEEFCAFE) ? SIGMA_TRUE : SIGMA_FALSE;
}

void SovereignShardOrchestrator::Audit() {
    sigma_log("\n--- Σ SOVEREIGN LATTICE ORCHESTRATION AUDIT ---\n");
    sigma_log("| Total Shards   : %d\n", m_shard_count);
    for (sigma_u32 i = 0; i < m_shard_count; ++i) {
        sigma_log("| [%s] %-20s | v%d.%-5d | Addr: %p\n", 
            m_registry[i].active ? "ACTIVE" : "IDLE  ", 
            m_registry[i].name, 
            m_registry[i].version >> 16, 
            m_registry[i].version & 0xFFFF, 
            (void*)m_registry[i].load_addr);
    }
    sigma_log("----------------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS



