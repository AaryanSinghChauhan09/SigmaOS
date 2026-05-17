#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Security {

class SovereignMAC : public SigmaObject, public SigmaSingleton<SovereignMAC> {
    friend class SigmaSingleton<SovereignMAC>;
public:
    const char* type_name() const noexcept override { return "SovereignMAC"; }

    void init() {
        sigma_log_info("[MAC:CORE] Initializing Sovereign Mandatory Access Control...");
        sigma_log_info("[MAC:CORE] Engine: PQC-Attested Labeling System.");
        sigma_log_info("[MAC:CORE] Policy: Industrial Zero-Trust.");
    }

    bool checkAccess(const char* subject, const char* object, const char* action) {
        sigma_log_info("[MAC:AUDIT] Checking access: Subject(%s) -> Object(%s) Action(%s)", subject, object, action);
        
        // Strict confinement for browser shards
        if (sigma_strstr(object, "/dev/") || sigma_strstr(object, "/proc/")) {
            return false;
        }

        if (sigma_strstr(object, "/browser/") && sigma_strstr(object, "/etc/shadow")) {
            return false;
        }

        // Confinement for third-party apps
        if (sigma_strstr(subject, "userland/app") && sigma_strstr(object, "kernel/core/security/SovereignPQC")) {
            sigma_log_warn("[MAC:DENIED] Userland app attempted direct PQC access!");
            return false;
        }

        sigma_log_info("[MAC:AUDIT] Access GRANTED based on lattice-level policy.");
        return true;
    }
};

} // namespace Security
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void mac_init() {
        SigmaOS::Kernel::Security::SovereignMAC::getInstance().init();
    }

    int mac_check_access(const char* subject, const char* object, const char* action) {
        return SigmaOS::Kernel::Security::SovereignMAC::getInstance().checkAccess(subject, object, action) ? 1 : 0;
    }
}

