#include "../../../include/SigmaOOP.hpp"
#include "core/SovereignArch.hpp"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign IA-64 Architecture Shard (S-IA64)
 * Implementation: Intel Itanium EPIC (Explicitly Parallel Instruction Computing) orchestration.
 * Mission: Support legacy mission-critical high-end industrial servers.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

class SovereignArchIA64 : public SovereignArch, public SigmaOS::SigmaSingleton<SovereignArchIA64> {
    friend class SigmaOS::SigmaSingleton<SovereignArchIA64>;
public:
    const char* type_name() const noexcept override { return "SovereignArchIA64"; }
    const char* arch_name() const noexcept override { return "ia64"; }

    void halt() override {
        sigma_log_info("[IA64] Halting EPIC execution bundles...");
        while(1) __asm__ volatile("hint @pause");
    }

    void reboot() override {
        sigma_log_info("[IA64] System reset via SAL (System Abstraction Layer) call...");
    }

    void setupPaging(sigma_u64 phys_base) override {
        sigma_log_info("[IA64] Initializing Region Registers & Protection Keys @ 0x%016llX", phys_base);
    }

    void enableInterrupts() override { __asm__ volatile("ssm 1 << 14"); } // Set psr.i
    void disableInterrupts() override { __asm__ volatile("rsm 1 << 14"); } // Reset psr.i

    sigma_u32 getCpuCount() override { return 4096; } // High-end IA64 clusters
    sigma_u32 getCurrentCpuId() override {
        return 0; // Simplified for IA64 bootstrap
    }

private:
    SovereignArchIA64() = default;
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void arch_init_ia64() { SigmaOS::Kernel::Arch::SovereignArchIA64::getInstance().setupPaging(0); }
}

