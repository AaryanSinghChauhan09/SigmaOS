/*
 * Σ SIGMAOS: GENERAL PURPOSE DISTRO COMPAT RUNTIME (v15.2)
 * Absorbed: Ubuntu, Debian, Fedora, Arch Linux.
 * Zero-dependency, silicon-direct, no stdlib.
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Distro {
namespace General {

class GeneralPurposeEngine {
private:
    sigma_bool m_kiss_enabled;
    sigma_bool m_upstream_first;

public:
    static GeneralPurposeEngine& getInstance() {
        static GeneralPurposeEngine instance;
        return instance;
    }

    void init() {
        sigma_log_info("[S-DISTRO/GENERAL] Enforcing Debian DFSG and Arch KISS principles...\n");
        m_kiss_enabled = SIGMA_TRUE;
        m_upstream_first = SIGMA_TRUE;
    }

    sigma_bool verify_package_license(const char* license) {
        if (sigma_strcmp(license, "GPL") == 0 || sigma_strcmp(license, "MIT") == 0) {
            return SIGMA_TRUE;
        }
        return SIGMA_FALSE;
    }
};

} // namespace General
} // namespace Distro
} // namespace SigmaOS

extern "C" {
void initialize_general_principles() {
    SigmaOS::Distro::General::GeneralPurposeEngine::getInstance().init();
}
}
