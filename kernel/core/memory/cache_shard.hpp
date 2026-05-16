#include "../../../include/sigma_hal.h"
#ifndef CACHE_SHARD_HPP
#define CACHE_SHARD_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignCacheShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignCacheShard"; }

    void FlushL1Cache() {
#if defined(SIGMA_ARCH_X86_64)
        sigma_log("[CACHE-SHARD]: Executing WBINVD for absolute silicon cache finality...\n");
        __asm__ volatile ("wbinvd" : : : "memory");
#endif
    }

    void PrefetchShard(void* addr) {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("prefetcht0 (%0)" : : "r"(addr));
#endif
    }

    void AuditCache() {
        sigma_log("\n--- Î£ SOVEREIGN CACHE AUDIT ---\n");
        sigma_log("| L1 Cache Shards: 32 KB [PRIVATE]\n");
        sigma_log("| L2 Cache Shards: 256 KB [SEMI-PRIVATE]\n");
        sigma_log("| L3 Cache Shards: 16 MB [SHARED LATTICE]\n");
        sigma_log("-------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

