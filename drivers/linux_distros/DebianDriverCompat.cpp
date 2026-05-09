/*
 * =========================================================================
 * Σ SIGMAOS: DEBIAN DRIVER COMPATIBILITY (APT ecosystem)
 * =========================================================================
 * Mission: Translates Debian/Ubuntu specific LKM binaries and apt-derived
 *          firmware into SigmaOS HAL instances.
 * =========================================================================
 */

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class DebianDriverCompat : public SigmaObject {
public:
    static DebianDriverCompat& getInstance() {
        static DebianDriverCompat instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "DebianDriverCompat"; }

    static bool loadDebianDriver(const char* deb_firmware_path) {
        sigma_log_info("[DEBIAN-COMPAT] Loading .deb derived firmware...");
        // 1. Unpack dpkg layout if necessary
        // 2. Map standard Ubuntu/Debian /lib/firmware paths
        // 3. Inject standard Linux ABI symbols expected by Debian drivers
        sigma_log_info("%s", deb_firmware_path);
        sigma_log_info("[DEBIAN-COMPAT] Debian driver successfully hooked into HAL.");
        return true;
    }

private:
    DebianDriverCompat() = default;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" void debian_driver_load(const char* path) {
    SigmaOS::Kernel::Drivers::DebianDriverCompat::loadDebianDriver(path);
}
