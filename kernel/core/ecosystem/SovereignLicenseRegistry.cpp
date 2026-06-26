/**
 * ===========================================================================
 * Σ SIGMAOS: SOVEREIGN LICENSE REGISTRY (S-LICENSE) v1.0
 * ===========================================================================
 * Mission: Enforce licensing clarity and compliance with the Indian Copyright
 *          Act 1957. Tracks OSS licenses and blocks unauthorized proprietary
 *          blobs to maintain ecosystem transparency.
 * ===========================================================================
 */

#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"

#define MAX_LICENSED_PACKAGES 1024

namespace SigmaOS {
namespace Kernel {
namespace Ecosystem {

enum LicenseType {
    LICENSE_GPLV2,
    LICENSE_GPLV3,
    LICENSE_MIT,
    LICENSE_APACHE2,
    LICENSE_PROPRIETARY,
    LICENSE_UNKNOWN
};

struct PackageLicense {
    char        package_name[64];
    LicenseType type;
    bool        is_compliant; // False if proprietary and unapproved
};

static PackageLicense s_registry[MAX_LICENSED_PACKAGES];
static sigma_u32      s_pkg_count = 0;

class SovereignLicenseRegistry {
public:
    static SovereignLicenseRegistry& getInstance() {
        static SovereignLicenseRegistry instance;
        return instance;
    }

    void init() {
        sigma_log("[LICENSE]: ═══════════════════════════════════════════════\n");
        sigma_log("[LICENSE]: Σ SOVEREIGN LICENSE REGISTRY v1.0\n");
        sigma_log("[LICENSE]: ═══════════════════════════════════════════════\n");
        s_pkg_count = 0;
        
        // Seed core OS packages
        registerPackage("sigma-kernel", LICENSE_GPLV2, true);
        registerPackage("omnipkg", LICENSE_MIT, true);
        registerPackage("zenith-desktop", LICENSE_GPLV3, true);
    }

    bool registerPackage(const char* pkg, LicenseType type, bool is_approved) {
        if (s_pkg_count >= MAX_LICENSED_PACKAGES) return false;
        
        PackageLicense* p = &s_registry[s_pkg_count];
        sigma_strncpy(p->package_name, pkg, 64);
        p->type = type;
        
        // Enforce compliance policy: Proprietary requires explicit approval
        if (type == LICENSE_PROPRIETARY && !is_approved) {
            p->is_compliant = false;
            sigma_log_warn("[LICENSE]: WARNING - Package '%s' flagged as non-compliant proprietary software.\n", pkg);
        } else {
            p->is_compliant = true;
        }

        s_pkg_count++;
        return true;
    }

    bool verifyCompliance(const char* pkg) {
        for (sigma_u32 i = 0; i < s_pkg_count; i++) {
            if (sigma_strcmp(s_registry[i].package_name, pkg) == 0) {
                return s_registry[i].is_compliant;
            }
        }
        return false; // Unknown packages are implicitly non-compliant
    }

private:
    SovereignLicenseRegistry() = default;
};

} // namespace Ecosystem
} // namespace Kernel
} // namespace SigmaOS

/* ---- C Wrappers ---- */
extern "C" void license_registry_init() {
    SigmaOS::Kernel::Ecosystem::SovereignLicenseRegistry::getInstance().init();
}
extern "C" bool license_verify(const char* pkg) {
    return SigmaOS::Kernel::Ecosystem::SovereignLicenseRegistry::getInstance().verifyCompliance(pkg);
}
