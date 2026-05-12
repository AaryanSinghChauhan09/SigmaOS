#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignCreative : public SigmaOS::SigmaObject {
public:
    static SovereignCreative& getInstance() {
        static SovereignCreative instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignCreative";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sovereigncreative_init() {
    SigmaOS::Kernel::SovereignCreative::getInstance().init();
}
