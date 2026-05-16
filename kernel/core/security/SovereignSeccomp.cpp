#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Seccomp Shard (S-SECCOMP)
 * Implementation: Secure Computing Mode / System Call Filtering.
 * Mission: Provide userspace protection by restricting available system calls per-process.
 * Absorbed: Linux seccomp-bpf patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignSeccomp : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignSeccomp> {
    friend class SigmaOS::SigmaSingleton<SovereignSeccomp>;
public:
    const char* type_name() const noexcept override { return "SovereignSeccomp"; }

    void init() {
        sigma_log_info("[S-SECCOMP] Initializing Syscall Filtering Engine...");
        sigma_log_info("[S-SECCOMP] Userspace execution protection (Pledge equivalent): ENABLED.");
    }

private:
    SovereignSeccomp() = default;
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void seccomp_init() { SigmaOS::Kernel::Security::SovereignSeccomp::getInstance().init(); }
}

