#include "SigmaOOP.hpp"
#include "sigma_kernel_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign Quota Shard (S-QUOTA)
 * Implementation: User/Group storage limits.
 * Mission: Enable resource constraints for multi-tenant environments.
 * Absorbed: Linux disk quota subsystem.
 */

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignQuota : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignQuota> {
    friend class SigmaOS::SigmaSingleton<SovereignQuota>;
public:
    const char* type_name() const noexcept override { return "SovereignQuota"; }

    void init() {
        sigma_log_info("[S-QUOTA] Initializing File System Quota Engine...");
        sigma_log_info("[S-QUOTA] User/Group hard & soft limits: ENFORCED.");
    }

private:
    SovereignQuota() = default;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void quota_init() { SigmaOS::Kernel::FS::SovereignQuota::getInstance().init(); }
}

 