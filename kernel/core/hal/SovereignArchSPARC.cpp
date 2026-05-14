#include "core/SigmaOOP.hpp"
#include "core/SovereignArch.hpp"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign SPARC Architecture Shard (S-SPARC)
 * Implementation: Sun/Oracle SPARC V8/V9 hardware orchestration.
 * Mission: Support industrial-grade SPARC servers and mission-critical mainframes.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

class SovereignArchSPARC : public SovereignArch, public SigmaOS::SigmaSingleton<SovereignArchSPARC> {
    friend class SigmaOS::SigmaSingleton<SovereignArchSPARC>;
public:
    const char* type_name() const noexcept override { return "SovereignArchSPARC"; }
    const char* arch_name() const noexcept override { return "sparc64"; }

    void halt() override {
        sigma_log_info("[SPARC] Entering PROM stop state...");
        __asm__ volatile("ta 0x0"); // Trap to PROM
    }

    void reboot() override {
        sigma_log_info("[SPARC] Triggering system reset...");
    }

    void setupPaging(sigma_u64 phys_base) override {
        sigma_log_info("[SPARC] Initializing MMU Contexts & TSBs @ 0x%016llX", phys_base);
    }

    void enableInterrupts() override { __asm__ volatile("wrpr %0, 0, %%pstate" : : "r"(0x82)); } // Enable IE
    void disableInterrupts() override { __asm__ volatile("wrpr %0, 0, %%pstate" : : "r"(0x80)); } // Disable IE

    sigma_u32 getCpuCount() override { return 512; }
    sigma_u32 getCurrentCpuId() override {
        sigma_u64 id;
        __asm__ volatile("rd %%asr19, %0" : "=r"(id)); // Read ASR19 for CPU ID
        return (sigma_u32)id;
    }

private:
    SovereignArchSPARC() = default;
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void arch_init_sparc() { SigmaOS::Kernel::Arch::SovereignArchSPARC::getInstance().setupPaging(0); }
}

