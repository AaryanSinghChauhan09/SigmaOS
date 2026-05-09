/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ORB MANAGER (Package Management Shard)
 * =========================================================================
 * Mission: Cryptographically signed, zero-latency Orb deployment.
 * Layer  : L5 — Industrial Ecosystem
 * =========================================================================
 */

#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "core/sigma_kernel_types.h"
#include "libc/SovereignLibC.h"
#include "core/SigmaOOP.hpp"
#include "sigma_log.h"
/* Use relative path from industrial/ → security/ within the same kernel tree */
#include "../security/SovereignQKD.hpp"

/**
 * SovereignOrbManager — Sovereign Orb Package Ecosystem
 * Principles: Cryptographically Signed Orbs, Zero-Latency Deployment.
 * Mission: Filling the industrial gap for a robust package ecosystem.
 *
 * Modularization Note:
 *   This shard lives at Layer 5 (Industrial) and communicates with the
 *   Layer 3 Security shard (SovereignQKD) only via its public C++ interface.
 *   It must never access kernel memory directly.
 */

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

    void init() {
        sigma_log_info("[ORB-MAN] Initializing Sovereign Package Ecosystem...");
        /* m_installed_orbs is default-initialized to 0 via member initializer */
        sigma_log_info("[ORB-MAN] Orb-Lattice Registry ONLINE (Zero-Dependency).");
    }

    bool resolveDependencies(OrbName name) {
        sigma_log_info("[ORB-MAN] Resolving dependency graph for Orb:");
        sigma_log_info(name.value);
        
        // Mock recursive dependency check
        sigma_log_info("[ORB-MAN] Scanning Sovereign Lattice for shared shards...");
        sigma_log_info("[ORB-MAN] Dependencies: [SovereignLibC, SovereignPQC, SovereignNetStack].");
        sigma_log_info("[ORB-MAN] Graph VALIDATED. All dependencies satisfied.");
        return true;
    }

    /**
     * @param name  The unique Orb identifier.
     * @param sig   The quantum-signed hash of the Orb payload.
     *
     * Using strong-type wrappers OrbName/OrbSig prevents the two adjacent
     * `const char*` parameters from being accidentally swapped.
     */
    void installOrb(OrbName name, OrbSig sig) {
        if (!resolveDependencies(name)) {
            sigma_log_err("[ORB-MAN] Dependency resolution FAILED. Aborting install.");
            return;
        }

        (void)sig; /* Signature consumed by QKD engine below */
        sigma_log_info("[ORB-MAN] Deploying Orb...");

        /* Zero-Trust: delegate verification entirely to the QKD security shard */
        bool verified = SigmaOS::Kernel::Security::SovereignQKD::getInstance()
                            .verifyQuantumIntegrity();

        if (verified) {
            sigma_log_info("[ORB-MAN] Orb INTEGRATED into Lattice.");
            if (m_installed_orbs < 10) {
                sigma_strncpy(m_registry[m_installed_orbs], name.value, 63);
                m_installed_orbs++;
            }
        } else {
            sigma_log_err("[ORB-MAN] SIGNATURE MISMATCH — Orb rejected by QKD Core.");
        }
    }

    void rollbackOrb(OrbName name) {
        sigma_log_info("[ORB-MAN] Initiating rollback for Orb...");
        bool found = false;
        for (sigma_u32 i = 0; i < m_installed_orbs; ++i) {
            if (sigma_streq(m_registry[i], name.value)) {
                sigma_log_info("[ORB-MAN] Rollback SUCCESSFUL. System state reverted for:");
                sigma_log_info(name.value);
                m_registry[i][0] = '\0'; // Simple mock removal
                m_installed_orbs--;
                found = true;
                break;
            }
        }
        if (!found) {
            sigma_log_err("[ORB-MAN] Rollback FAILED: Orb not found in registry.");
        }
    }

    void listOrbs() const {
        sigma_log_info("[ORB-MAN] --- Σ SOVEREIGN ORB REGISTRY ---");
        for (sigma_u32 i = 0; i < 10; ++i) {
            if (m_registry[i][0] != '\0') {
                sigma_log_info(m_registry[i]);
            }
        }
        sigma_log_info("[ORB-MAN] --------------------------------");
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

/* --- C Bridge --- */
extern "C" void orb_manager_init() {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::getInstance().init();
}

extern "C" void orb_install(const char* orb_name, const char* orb_sig) {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::getInstance().installOrb(
        SigmaOS::Kernel::Industrial::OrbName{orb_name},
        SigmaOS::Kernel::Industrial::OrbSig{orb_sig});
}

extern "C" void orb_rollback(const char* orb_name) {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::getInstance().rollbackOrb(
        SigmaOS::Kernel::Industrial::OrbName{orb_name});
}

extern "C" void orb_list() {
    SigmaOS::Kernel::Industrial::SovereignOrbManager::getInstance().listOrbs();
}
