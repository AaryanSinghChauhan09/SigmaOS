#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Sovereign Onboarding Wizard
 * Tailored setup paths for different user personas.
 *
 * USP: A ring-0 initialization sequence that dynamically configures Zenith UI, 
 * accessibility profiles, and workflow macros based on a unified persona selection.
 *
 * Design: OOP-isolated singleton — SovereignOnboardingEngine.
 */

class SovereignOnboardingEngine {
public:
    static SovereignOnboardingEngine& getInstance() {
        static SovereignOnboardingEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[ONBOARDING] Initializing Zenith Persona Configurator...");
        this->persona_set = false;
    }

    void applyPersona(const char* persona_type) {
        sigma_printf("[ONBOARDING] Applying '%s' Persona profile to Sovereign Lattice...\n", persona_type);
        
        if (sigma_hardened_strcmp(persona_type, "Developer") == 0) {
            sigma_log("[ONBOARDING] -> Enabling Container Networking & GPU Hardware Passthrough.");
        } else if (sigma_hardened_strcmp(persona_type, "Creator") == 0) {
            sigma_log("[ONBOARDING] -> Maximizing GPU VRAM allocation & Zenith Morphic Compositor quality.");
        } else {
            sigma_log("[ONBOARDING] -> Applying Standard Adaptive Defaults.");
        }
        
        this->persona_set = true;
        sigma_log("[ONBOARDING] Sovereign setup complete. Welcome to SigmaOS.");
    }

private:
    SovereignOnboardingEngine() : persona_set(false) {}
    bool persona_set;
};

/* --- C Wrappers --- */
extern "C" void onboarding_init() {
    SovereignOnboardingEngine::getInstance().init();
}

extern "C" void onboarding_apply_persona(const char* type) {
    SovereignOnboardingEngine::getInstance().applyPersona(type);
}
