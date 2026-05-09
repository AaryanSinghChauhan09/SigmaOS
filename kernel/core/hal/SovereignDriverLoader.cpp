/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DRIVER LOADER (HAL Shard)
 * =========================================================================
 * Mission: Isolated shard for driver initialization and registry.
 * Layer  : L1 — Kernel Primitives / HAL
 * =========================================================================
 */

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignDriverLoader : public SigmaObject {
public:
    static SovereignDriverLoader& getInstance() {
        static SovereignDriverLoader instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignDriverLoader"; }

    static void loadAll() {
        sigma_log_info("[DRIVER-LOADER] Loading required hardware drivers...");
        extern "C" void gpu_init(); gpu_init();
        extern "C" void nvme_init(); nvme_init();
        extern "C" void nic_init(); nic_init();
        extern "C" void usb_init(); usb_init();
        extern "C" void wifi_init(); wifi_init();
    }

private:
    SovereignDriverLoader() = default;
};
} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS
extern "C" void hal_load_drivers() {
    SigmaOS::Kernel::HAL::SovereignDriverLoader::loadAll();
}
