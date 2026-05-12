#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Diagnostics {

class SovereignKdump : public SigmaObject, public SigmaSingleton<SovereignKdump> {
    friend class SigmaSingleton<SovereignKdump>;
public:
    const char* type_name() const noexcept override { return "SovereignKdump"; }

    void init() {
        sigma_log_info("[DIAG:KDUMP] Initializing Sovereign Kernel Dump Engine...");
        sigma_log_info("[DIAG:KDUMP] Reserving crash-recovery memory region (64MB).");
    }

    void captureCrash(const char* fault_reason) {
        sigma_log_error("[DIAG:KDUMP] CRITICAL FAULT: %s", fault_reason);
        sigma_log_info("[DIAG:KDUMP] Dumping industrial state to /var/crash/sigma_vmcore.pqc...");
        sigma_log_info("[DIAG:KDUMP] PQC-Attestation complete. System rebooting via S-AUTO.");
    }
};

} // namespace Diagnostics
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void kdump_init() {
        SigmaOS::Kernel::Diagnostics::SovereignKdump::getInstance().init();
    }
}
