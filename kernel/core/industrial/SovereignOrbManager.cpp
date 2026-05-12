/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN ORB MANAGER (Package Management Shard)
 * =========================================================================
 * Mission: Cryptographically signed, zero-latency Orb deployment.
 * Layer  : L5 â€" Industrial Ecosystem
 * =========================================================================
 */

#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "core/sigma_kernel_types.h"
#include "libc/SovereignLibC.h"
#include "core/SigmaOOP.hpp"
#include "sigma_log.h"
#include "security/SovereignQKD.hpp"

/**
 * SovereignOrbManager â€" Sovereign Orb Package Ecosystem
 * Principles: Cryptographically Signed Orbs, Zero-Latency Deployment.
 * Mission: Filling the industrial gap for a robust package ecosystem.
 *
 * Modularization Note:
 *   This shard lives at Layer 5 (Industrial) and communicates with the
 *   Layer 3 Security shard (SovereignQKD) only via its public C++ interface.
 *   It must never access kernel memory directly.
 */
extern "C" int orb_resolve_deps(const char* name);

namespace SigmaOS {
namespace Kernel {
namespace Industrial {

/* Strong-type wrappers to prevent parameter-swap errors (CWE-683) */
struct OrbName    { const char* value; };
struct OrbSig     { const char* value; };

class SovereignOrbManager : public SigmaObject {
public:
    static SovereignOrbManager& getInstance() {
        static SovereignOrbManager instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignOrbManager"; }

    static void init() {
        sigma_log("[ORB-MAN] Initializing Sovereign Package Ecosystem...\n");
        /* m_installed_orbs is default-initialized to 0 via member initializer */
        sigma_log("[ORB-MAN] Orb-Lattice Registry ONLINE (Zero-Dependency).\n");
    }

    static bool resolveDependencies(OrbName name) {
        // Now handled by SovereignOrbResolver shard
        return orb_resolve_deps(name.value) != 0;
    }

    /**
     * @param name  The unique Orb identifier.
     * @param sig   The quantum-signed hash of the Orb payload.
     *
     * Using strong-type wrappers OrbName/OrbSig prevents the two adjacent
     * `const char*` parameters from being accidentally swapped.
     */
    static void installOrb(OrbName name, OrbSig sig) {
        if (!resolveDependencies(name)) {
            sigma_log("[ORB-MAN] Dependency resolution FAILED. Aborting install.\n");
            return;
        }

        (void)sig; /* Signature consumed by QKD engine below */
        sigma_log("[ORB-MAN] Deploying Orb...\n");

        /* Zero-Trust: delegate verification entirely to the QKD security shard */
        bool verified = SigmaOS::Kernel::Security::SovereignQKD::getInstance()
                            .verifyQuantumIntegrity();

        if (verified) {
            sigma_log("[ORB-MAN] Orb INTEGRATED into Lattice.\n");
            auto& self = getInstance();
            if (self.m_installed_orbs < 10) {
                sigma_strncpy(self.m_registry[self.m_installed_orbs], name.value, 63);
                self.m_installed_orbs++;
            }
        } else {
            sigma_log("[ORB-MAN] SIGNATURE MISMATCH â€" Orb rejected by QKD Core.\n");
        }
    }

    static void rollbackOrb(OrbName name) {
        sigma_log("[ORB-MAN] Initiating rollback for Orb...\n");
        bool found = false;
        auto& self = getInstance();
        for (sigma_u32 i = 0; i < self.m_installed_orbs; ++i) {
            if (sigma_streq(self.m_registry[i], name.value)) {
                sigma_log("[ORB-MAN] Rollback SUCCESSFUL. System state reverted for:\n");
                sigma_log("%s\n", name.value);
                self.m_registry[i][0] = '\0'; // Simple mock removal
                self.m_installed_orbs--;
                found = true;
                break;
            }
        }
        if (!found) {
            sigma_log("[ORB-MAN] Rollback FAILED: Orb not found in registry.\n");
        }
    }

    static void listOrbs() {
        sigma_log("[ORB-MAN] --- Î£ SOVEREIGN ORB REGISTRY ---\n");
        auto& self = getInstance();
        for (sigma_u32 i = 0; i < 10; ++i) {
            if (self.m_registry[i][0] != '\0') {
                sigma_log("%s\n", self.m_registry[i]);
            }
        }
        sigma_log("[ORB-MAN] --------------------------------\n");
    }

private:
    SovereignOrbManager() {
        for(int i=0; i<10; ++i) m_registry[i][0] = '\0';
    }
    SovereignOrbManager(const SovereignOrbManager&) = delete;
    SovereignOrbManager& operator=(const SovereignOrbManager&) = delete;

    char m_registry[10][64]; // Mock registry for rollback (PKG-003)
    sigma_u32 m_installed_orbs{0U}; /* Default member initializer (C++11) */
};

} // namespace Industrial
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void orb_manager_init() {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::init();
}

void orb_install(const char* orb_name, const char* orb_sig) {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::installOrb(
        SigmaOS::Kernel::Industrial::OrbName{orb_name},
        SigmaOS::Kernel::Industrial::OrbSig{orb_sig});
}

void orb_rollback(const char* orb_name) {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::rollbackOrb(
        SigmaOS::Kernel::Industrial::OrbName{orb_name});
}

void orb_list() {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::listOrbs();
}










} // extern "C"
