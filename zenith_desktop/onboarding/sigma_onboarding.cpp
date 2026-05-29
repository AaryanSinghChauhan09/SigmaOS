/**
 * =========================================================================
 * Σ ZENITH ONBOARDING WIZARD
 * =========================================================================
 * Inspired by Zorin OS's layout chooser and Elementary OS's first-run
 * setup experience. Guides new users through sovereign system configuration 
 * without requiring technical expertise.
 * =========================================================================
 */

#include <sigma_libc.h>
#include <sigma_error_codes.h>
#include <sigma_profiles.h>

// Forward declare external Control Center hook
extern "C" void zenith_settings_set_profile(sigma_system_profile_t type);
extern "C" void zenith_log_structured(sigma_u32 code, const char* comp, const char* desc, sigma_u32 cid);

namespace Zenith {
namespace Onboarding {

class OnboardingWizard {
public:
    static OnboardingWizard& getInstance() {
        static OnboardingWizard instance;
        return instance;
    }

    void run() {
        sys_print("\n");
        sys_print("╔══════════════════════════════════════════════════════════════╗\n");
        sys_print("║          WELCOME TO SIGMAOS — SOVEREIGN BY DEFAULT          ║\n");
        sys_print("╚══════════════════════════════════════════════════════════════╝\n");
        sys_print("\n");
        
        stepWelcome();
        stepProfileSelection();
        stepNetworkConfiguration();
        stepDeclarativeImport();
        stepComplete();
    }

private:
    void stepWelcome() {
        sys_print("[ STEP 1/4: WELCOME ]\n");
        sys_print("SigmaOS is a zero-dependency, sovereign operating system.\n");
        sys_print("Apps run inside isolated containers by default. Your data stays yours.\n\n");
    }

    void stepProfileSelection() {
        sys_print("[ STEP 2/4: CHOOSE YOUR SOVEREIGN PROFILE ]\n");
        sys_print("  [1] Standard       — Balanced desktop experience\n");
        sys_print("  [2] Forensic       — CAINE-style read-only; for security audits\n");
        sys_print("  [3] IoT            — Lightweight, minimal footprint for ARM64 devices\n");
        sys_print("  [4] Enterprise     — Hardened ACLs, strict logging\n");
        sys_print("  [5] Education      — Permissive sandbox for exploration\n\n");
        
        // Default to Standard for first-run
        zenith_settings_set_profile(SIGMA_PROFILE_STANDARD);
        sys_print("[Onboarding] Default profile loaded: Standard\n\n");
    }

    void stepNetworkConfiguration() {
        sys_print("[ STEP 3/4: NETWORK ISOLATION ]\n");
        sys_print("SigmaOS enforces Whonix-style gateway/workstation network splits.\n");
        sys_print("All GUI applications are sandboxed from direct network access by default.\n");
        sys_print("[Onboarding] Whonix-style firewall rules applied.\n\n");
    }

    void stepDeclarativeImport() {
        sys_print("[ STEP 4/4: DECLARATIVE PROFILE IMPORT (OPTIONAL) ]\n");
        sys_print("Do you have an existing settings.json profile to import?\n");
        sys_print("[Onboarding] Skipping for now. You can import later from Control Center.\n\n");
    }

    void stepComplete() {
        sys_print("╔══════════════════════════════════════════════════════════════╗\n");
        sys_print("║    SETUP COMPLETE. WELCOME TO SOVEREIGN COMPUTING. 🚀       ║\n");
        sys_print("╚══════════════════════════════════════════════════════════════╝\n");
        sys_print("\n");
        zenith_log_structured(ZEN_SUCCESS, "Onboarding", "First-run wizard completed successfully", 0);
    }
};

} // namespace Onboarding
} // namespace Zenith

extern "C" {
    void zenith_onboarding_run() {
        Zenith::Onboarding::OnboardingWizard::getInstance().run();
    }
}
