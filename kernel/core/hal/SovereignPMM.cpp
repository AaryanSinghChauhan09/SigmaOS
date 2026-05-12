#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPMM : public SigmaOS::SigmaObject {
public:
    static SovereignPMM& getInstance() {
        static SovereignPMM instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignPMM";
    }

    void init() {
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignpmm_init() {
    SigmaOS::Kernel::SovereignPMM::getInstance().init();
}

} // extern "C"
