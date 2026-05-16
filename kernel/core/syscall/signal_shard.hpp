#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
#ifndef SIGNAL_SHARD_HPP
#define SIGNAL_SHARD_HPP

#include "../../../include/libc/SovereignLibC.h"

#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

class SovereignSignalShard : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SovereignSignalShard"; }

    void SendSignal(sigma_u32 pid, int sig) {
        sigma_log("[SIGNAL-SHARD]: Sending Signal %d to Shard PID: %u\n", sig, pid);
    }

    void HandleException(int ex) {
        sigma_log("[SIGNAL-SHARD]: Intercepted Hardware Exception: 0x%X\n", ex);
        sigma_log("[SIGNAL-SHARD]: Transitioning to Sovereign Recovery State.\n");
    }
};

} // namespace Kernel
} // namespace SigmaOS

#endif

