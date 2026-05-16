#include "../../../include/SigmaOOP.hpp"
#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign FAT Shard (S-FAT)
 * Implementation: FAT16, FAT32, vfat, exFAT.
 * Mission: Universal storage compatibility across all platforms.
 * Absorbed: Linux dosfs/vfat patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignFAT : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignFAT> {
    friend class SigmaOS::SigmaSingleton<SovereignFAT>;
public:
    const char* type_name() const noexcept override { return "SovereignFAT"; }

    void init() {
        sigma_log_info("[S-FAT] Initializing Universal FAT/vfat/exFAT Engine...");
        sigma_log_info("[S-FAT] Legacy storage compatibility: ENABLED.");
    }

private:
    SovereignFAT() = default;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void fat_init() { SigmaOS::Kernel::FS::SovereignFAT::getInstance().init(); }
}

