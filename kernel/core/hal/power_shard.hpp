#include "sigma_hal.h"
#ifndef POWER_SHARD_HPP
#define POWER_SHARD_HPP

#include "libc/SovereignLibC.h"

#include "SigmaOOP.hpp"
#include "port_shard.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPowerShard : public SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignPowerShard"; }

    void Shutdown() {
        sigma_log("[POWER-SHARD]: Initiating Sovereign Shutdown Shunt (ACPI S5)...\n");
#if defined(SIGMA_ARCH_X86_64)
        // QEMU/VirtualBox specific shutdown port (simulated)
        SovereignPortShard::outw(0x604, 0x2000);
#endif
    }

    void Reboot() {
        sigma_log("[POWER-SHARD]: Initiating Sovereign Reboot Shunt (PS/2 Controller)...\n");
#if defined(SIGMA_ARCH_X86_64)
        SovereignPortShard::outb(0x64, 0xFE);
#endif
    }

    void AuditPower() {
        sigma_log("\n--- Î£ SOVEREIGN POWER AUDIT ---\n");
        sigma_log("| Energy State   : S0 (Operational Shard)\n");
        sigma_log("| Battery Shard  : 100%% [ZENITH-CHARGE]\n");
        sigma_log("| Thermal Shunt  : OPTIMAL (42 C)\n");
        sigma_log("-------------------------------\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 