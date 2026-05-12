#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignFS : public SigmaOS::SigmaObject {
public:
    static SovereignFS& getInstance() {
        static SovereignFS instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignFS";
    }

    void init() {
        sigma_log_info("[STUB] Initializing Modular Shard...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" void sovereignfs_init() {
    SigmaOS::Kernel::SovereignFS::getInstance().init();
}
