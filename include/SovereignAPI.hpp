#ifndef SOVEREIGN_API_HPP
#define SOVEREIGN_API_HPP

#include "./core/sigma_types.h"
#include "./SigmaOOP.hpp"

namespace SigmaOS {
namespace API {

/*
 * =========================================================================
 * SOVEREIGN INDUSTRIAL API (The Developer Nexus)
 * =========================================================================
 * Clean, industrial-grade API for lattice shard development. Provides 
 * polymorphic interfaces to core system services with zero-latency 
 * silicon-native bindings.
 */
class SovereignAPI : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignAPI"; }

    // Core Lattice Primitives
    static void Log(const char* message);
    static void* AllocateShard(sigma_size_t size);
    static void ReleaseShard(void* ptr);

    // Distributed Consensus
    static sigma_bool ProposeState(const char* shard_id, const void* data, sigma_size_t size);

    // Security & Encryption
    static void EncryptPQC(const void* src, void* dst, sigma_size_t size);

    // Hardware Telemetry
    static sigma_u32 GetSiliconPressure();
};

} // namespace API
} // namespace SigmaOS

#endif
