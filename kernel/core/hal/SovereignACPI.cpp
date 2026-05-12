#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignACPI : public SigmaOS::SigmaObject {
public:
    static SovereignACPI& getInstance() {
        static SovereignACPI instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignACPI";
    }

    void init() {
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sovereignacpi_init() {
    SigmaOS::Kernel::SovereignACPI::getInstance().init();
}
