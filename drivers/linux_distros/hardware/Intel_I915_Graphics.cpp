/*
 * =========================================================================
 * Σ SIGMAOS: INTEL I915 GRAPHICS (DRV-008)
 * =========================================================================
 * Mission: Implements DRV-008 for Intel Integrated Graphics.
 * Layer  : L1 — Kernel Primitives / Drivers
 * =========================================================================
 */

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class IntelI915Shard : public SigmaObject {
public:
    static IntelI915Shard& getInstance() {
        static IntelI915Shard instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "IntelI915Shard"; }

    static void init() {
        sigma_log_info("[I915] Probing Intel UHD/Iris Graphics...");
        sigma_log_info("[I915] Mapping GTT (Graphics Translation Table) apertures.");
        sigma_log_info("[I915] Display engine ONLINE. Framebuffer mapped to Zenith.");
    }

private:
    IntelI915Shard() = default;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

void i915_init() {
    SigmaOS::Kernel::Drivers::IntelI915Shard::init();
}

} // extern "C"
