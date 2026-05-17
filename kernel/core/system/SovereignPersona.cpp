#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignPersona : public SigmaOS::SigmaObject {
public:
    static SovereignPersona& getInstance() {
        static SovereignPersona instance;
        return instance;
    }

    const char* type_name() const noexcept override {
        return "SovereignPersona";
    }

    void init() {
        sigma_log_info("[S-PERSONA] Initializing Persona Engine...");
    }
};

} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void persona_init() {
    SigmaOS::Kernel::SovereignPersona::getInstance().init();
}

} // extern "C"
 