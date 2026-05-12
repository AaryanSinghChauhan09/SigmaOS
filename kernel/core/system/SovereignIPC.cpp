#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignIPC : public SigmaOS::SigmaObject {
public:
    static SovereignIPC& getInstance() {
        static SovereignIPC instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignIPC";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sovereignipc_init() {
    SigmaOS::Kernel::SovereignIPC::getInstance().init();
}
