#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignNet : public SigmaOS::SigmaObject {
public:
    static SovereignNet& getInstance() {
        static SovereignNet instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignNet";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignnet_init() {
    SigmaOS::Kernel::SovereignNet::getInstance().init();
}

} // extern "C"
