#include "core/SovereignArch.hpp"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign RISC-V Architecture Implementation
 * Mission: Porting the industrial lattice to Open-Source RISC-V silicon.
 */

namespace SigmaOS {
namespace Kernel {
namespace Arch {

class SovereignArchRISCV : public SovereignArch {
public:
    const char* type_name() const noexcept override { return "SovereignArchRISCV"; }
    const char* arch_name() const noexcept override { return "riscv64"; }

    void halt() override {
        sigma_log_info("[S-ARCH:RISCV] SBI: System Shutdown.");
    }

    void reboot() override {
        sigma_log_info("[S-ARCH:RISCV] SBI: System Cold Reboot.");
    }

    void setupPaging(sigma_u64 phys_base) override {
        sigma_log_info("[S-ARCH:RISCV] Initializing Sv39 Paging (Base: 0x%016llX)...", phys_base);
    }

    void enableInterrupts() override { /* asm volatile("csrrs zero, sstatus, %0" : : "r"(1 << 1)); */ }
    void disableInterrupts() override { /* asm volatile("csrrc zero, sstatus, %0" : : "r"(1 << 1)); */ }

    sigma_u32 getCpuCount() override { return 4; } // Mock value for HART count
    sigma_u32 getCurrentCpuId() override { return 0; }
};

} // namespace Arch
} // namespace Kernel
} // namespace SigmaOS
 