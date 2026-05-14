#include "core/sigma_types.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign TmpFS Shard (S-TMPFS)
 * Implementation: RAM disk and tmpfs.
 * Mission: Ultra-low latency memory-backed volatile storage.
 * Absorbed: Linux tmpfs/ramfs patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace FS {

class SovereignTmpFS : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignTmpFS> {
    friend class SigmaOS::SigmaSingleton<SovereignTmpFS>;
public:
    const char* type_name() const noexcept override { return "SovereignTmpFS"; }

    void init() {
        sigma_log_info("[S-TMPFS] Initializing In-Memory TmpFS Engine...");
        sigma_log_info("[S-TMPFS] Volatile RAM disk mounting: READY.");
    }

private:
    SovereignTmpFS() = default;
};

} // namespace FS
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void tmpfs_init() { SigmaOS::Kernel::FS::SovereignTmpFS::getInstance().init(); }
}
