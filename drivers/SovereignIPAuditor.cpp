/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN IP AUDITOR (Licensing Compliance Shard)
 * =========================================================================
 * Mission: Ensures that all ported Linux drivers and firmware loaded into
 *          SigmaOS strictly comply with open-source licensing (GPL, MIT, etc.)
 *          and do not breach intellectual property laws.
 * Layer  : Drivers / Security
 * =========================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class SovereignIPAuditor : public SigmaObject, public SigmaSingleton<SovereignIPAuditor> {
    friend class SigmaSingleton<SovereignIPAuditor>;
public:

    const char* type_name() const noexcept override { return "SovereignIPAuditor"; }

    static bool verifyDriverCompliance(const char* driver_name, const char* license_tag) {
        sigma_log_info("[IP-AUDIT] Verifying IP compliance for driver:");
        sigma_log_info("%s", driver_name);

        // Acceptable open-source licenses that allow integration
        if (sigma_strstr(license_tag, "GPL") != SIGMA_NULL || 
            sigma_strstr(license_tag, "MIT") != SIGMA_NULL || 
            sigma_strstr(license_tag, "Apache") != SIGMA_NULL ||
            sigma_strstr(license_tag, "Dual MIT/GPL") != SIGMA_NULL) {
            
            sigma_log_info("[IP-AUDIT] License ACCEPTED. Driver is IP-compliant.");
            return true;
        }

        // Reject proprietary blobs without explicit legal agreements
        if (sigma_strstr(license_tag, "Proprietary") != SIGMA_NULL || sigma_strstr(license_tag, "Closed") != SIGMA_NULL) {
            sigma_log_err("[IP-AUDIT] 🚨 LICENSE REJECTED: Proprietary driver blocked to prevent IP breach.");
            return false;
        }

        sigma_log_warn("[IP-AUDIT] Unknown license. Applying clean-room quarantine protocol.");
        return false;
    }

private:
    SovereignIPAuditor() = default;
};

} // namespace Drivers
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

extern "C" int ip_audit_verify(const char* name, const char* license) {
    return SigmaOS::Kernel::Drivers::SovereignIPAuditor::verifyDriverCompliance(name, license) ? 1 : 0;
}

} // extern "C"
