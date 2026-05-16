#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignMedical : public SigmaOS::SigmaObject {
public:
    static SovereignMedical& getInstance() {
        static SovereignMedical instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignMedical";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignmedical_init() {
    SigmaOS::Kernel::SovereignMedical::getInstance().init();
}

} // extern "C"
