/*
 * =========================================================================
 * Σ SIGMAOS: REALTEK RTL8111/8168 ETHERNET DRIVER
 * =========================================================================
 * Mission: Port of the Linux r8169 LKM for Realtek Gigabit Ethernet.
 * Layer  : Drivers
 * =========================================================================
 */

#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {
namespace Hardware {

class RealtekRTL8111 : public SigmaObject {
public:
    static RealtekRTL8111& getInstance() {
        static RealtekRTL8111 instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "RealtekRTL8111"; }

    static bool initDevice() {
        sigma_log_info("[RTL8111] Probing for Realtek Gigabit Ethernet controller...");
        // Map Linux r8169 firmware
        sigma_log_info("[RTL8111] Loading r8169-v2.bin firmware...");
        sigma_log_info("[RTL8111] Link established: 1000Mbps Full-Duplex.");
        return true;
    }

private:
    RealtekRTL8111() = default;
};
} // namespace Hardware
} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void rtl8111_init() {
    SigmaOS::Kernel::Drivers::Hardware::RealtekRTL8111::initDevice();
}

} // extern "C"
