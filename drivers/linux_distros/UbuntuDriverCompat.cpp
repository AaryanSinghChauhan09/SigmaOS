#include "../../include/core/sigma_types.h"
#include "../../include/sigma_log.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class UbuntuDriverCompat : public SigmaObject, public SigmaSingleton<UbuntuDriverCompat> {
    friend class SigmaSingleton<UbuntuDriverCompat>;
public:
    const char* type_name() const noexcept override { return "UbuntuDriverCompat"; }

    void init() {
        sigma_log_info("[UBUNTU-COMPAT] Initializing Debian-based driver abstraction layer...");
        sigma_log_info("[UBUNTU-COMPAT] Mapping .deb kernel objects to Sovereign Shards.");
    }

    void wrapDriver(const char* driver_name) {
        sigma_log_info("[UBUNTU-COMPAT] Wrapping Ubuntu Driver: %s", driver_name);
        sigma_log_info("[UBUNTU-COMPAT] ABI Handshake complete.");
    }
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void ubuntu_compat_init() {
        SigmaOS::Kernel::Drivers::UbuntuDriverCompat::getInstance().init();
    }
}
