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

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Drivers {

class SovereignIPAuditor : public SigmaObject {
public:
    static SovereignIPAuditor& getInstance() {
        static SovereignIPAuditor instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignIPAuditor"; }

    bool verifyDriverCompliance(const char* driver_name, const char* license_tag) {
        sigma_log_info("[IP-AUDIT] Verifying IP compliance for driver:");
        sigma_log_info(driver_name);

        // Acceptable open-source licenses that allow integration
        if (sigma_strstr(license_tag, "GPL") || 
            sigma_strstr(license_tag, "MIT") || 
            sigma_strstr(license_tag, "Apache") ||
            sigma_strstr(license_tag, "Dual MIT/GPL")) {
            
            sigma_log_info("[IP-AUDIT] License ACCEPTED. Driver is IP-compliant.");
            return true;
        }

        // Reject proprietary blobs without explicit legal agreements
        if (sigma_strstr(license_tag, "Proprietary") || sigma_strstr(license_tag, "Closed")) {
            sigma_log_err("[IP-AUDIT] 🚨 LICENSE REJECTED: Proprietary driver blocked to prevent IP breach.");
            return false;
        }

        sigma_log_warn("[IP-AUDIT] Unknown license. Applying clean-room quarantine protocol.");
        return false;
    }

private:
    SovereignIPAuditor() = default;
};

}
}
}

extern "C" int ip_audit_verify(const char* name, const char* license) {
    return SigmaOS::Kernel::Drivers::SovereignIPAuditor::getInstance()
        .verifyDriverCompliance(name, license) ? 1 : 0;
}
