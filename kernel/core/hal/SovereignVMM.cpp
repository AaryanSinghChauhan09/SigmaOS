#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignVMM : public SigmaOS::SigmaObject {
public:
    static SovereignVMM& getInstance() {
        static SovereignVMM instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignVMM";
    }

    void init() {
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignvmm_init() {
    SigmaOS::Kernel::SovereignVMM::getInstance().init();
}

} // extern "C"
