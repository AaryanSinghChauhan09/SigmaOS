/*
 * SigmaOS: Sovereign Driver Loader (HAL Shard)
 * Layer: L1 - Kernel Primitives / HAL
 */
#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/* Forward-declare all driver entry points at file scope */
extern "C" void gpu_init();
extern "C" void nvme_init();
extern "C" void nic_init();
extern "C" void usb_init();
extern "C" void wifi_init();

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
        sigma_log_info("[DRIVER-LOADER] Loading hardware drivers...");
        gpu_init();
        nvme_init();
        nic_init();
        usb_init();
        wifi_init();
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
