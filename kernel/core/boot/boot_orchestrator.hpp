#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#ifndef BOOT_ORCHESTRATOR_HPP
#define BOOT_ORCHESTRATOR_HPP



#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignBootOrchestrator : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignBootOrchestrator"; }

    void Ignite(const char* profile_path);
    void ApplyPolicy(const char* policy);
    void Finalize();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

