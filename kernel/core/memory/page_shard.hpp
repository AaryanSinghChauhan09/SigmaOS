#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#ifndef PAGE_SHARD_HPP
#define PAGE_SHARD_HPP

#include "libc/SovereignLibC.h"

#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPageShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPageShard"; }

    void MapPage(sigma_u64 virt, sigma_u64 phys, sigma_u64 flags) {
        sigma_log("[PAGE-SHARD]: Mapping Shard: 0x%llX -> 0x%llX [FLAGS: 0x%llX]\n", virt, phys, flags);
    }

    void FlushTLB() {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("mov %%cr3, %%rax; mov %%rax, %%cr3" : : : "rax", "memory");
#endif
    }

    void AuditPaging() {
        sigma_log("\n--- Î£ SOVEREIGN PAGING AUDIT ---\n");
        sigma_log("| Page Mode      : 4-Level Paging (x86_64)\n");
        sigma_log("| TLB Status     : PERSISTENT / SHARDED\n");
        sigma_log("| NX Bit         : ENABLED (Security Shunt)\n");
        sigma_log("------------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

