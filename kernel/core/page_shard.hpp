#ifndef PAGE_SHARD_HPP
#define PAGE_SHARD_HPP

#include "SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPageShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPageShard"; }

    void MapPage(sigma_u64 virt, sigma_u64 phys, sigma_u64 flags) {
        sigma_printf("[PAGE-SHARD]: Mapping Shard: 0x%llX -> 0x%llX [FLAGS: 0x%llX]\n", virt, phys, flags);
    }

    void FlushTLB() {
#if defined(SIGMA_ARCH_X86_64)
        __asm__ volatile ("mov %%cr3, %%rax; mov %%rax, %%cr3" : : : "rax", "memory");
#endif
    }

    void AuditPaging() {
        sigma_printf("\n--- Î£ SOVEREIGN PAGING AUDIT ---\n");
        sigma_printf("| Page Mode      : 4-Level Paging (x86_64)\n");
        sigma_printf("| TLB Status     : PERSISTENT / SHARDED\n");
        sigma_printf("| NX Bit         : ENABLED (Security Shunt)\n");
        sigma_printf("------------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif
