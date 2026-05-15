#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignGov : public SigmaOS::SigmaObject {
public:
    static SovereignGov& getInstance() {
        static SovereignGov instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignGov";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereigngov_init() {
    SigmaOS::Kernel::SovereignGov::getInstance().init();
}

} // extern "C"
