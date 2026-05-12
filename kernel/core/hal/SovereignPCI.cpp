#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPCI : public SigmaOS::SigmaObject {
public:
    static SovereignPCI& getInstance() {
        static SovereignPCI instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignPCI";
    }

    void init() {
        sigma_log_info("[HAL] Initializing Hardware Abstraction Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void sovereignpci_init() {
    SigmaOS::Kernel::SovereignPCI::getInstance().init();
}

} // extern "C"
