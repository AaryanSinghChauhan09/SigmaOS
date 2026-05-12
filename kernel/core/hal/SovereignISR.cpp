#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignISR : public SigmaOS::SigmaObject {
public:
    static SovereignISR& getInstance() {
        static SovereignISR instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignISR";
    }

    void init() {
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sovereignisr_init() {
    SigmaOS::Kernel::SovereignISR::getInstance().init();
}
