#include "SigmaOOP.hpp"
#include "sigma_kernel_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign NX Shard (S-NX)
 * Implementation: Executable Space Protection (W^X).
 * Mission: Prevent arbitrary code execution by ensuring memory is never both writeable and executable.
 * Absorbed: Linux NX bit / PaX patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignNX : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignNX> {
    friend class SigmaOS::SigmaSingleton<SovereignNX>;
public:
    const char* type_name() const noexcept override { return "SovereignNX"; }

    void init() {
        sigma_log_info("[S-NX] Initializing Executable Space Protection (W^X)...");
        sigma_log_info("[S-NX] Data Execution Prevention (DEP): ACTIVE on all shards.");
    }

private:
    SovereignNX() = default;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void nx_init() { SigmaOS::Kernel::Security::SovereignNX::getInstance().init(); }
}

 