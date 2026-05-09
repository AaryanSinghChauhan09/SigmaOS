#include "SovereignOnboarding.hpp"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"

SovereignOnboardingEngine& SovereignOnboardingEngine::getInstance() {
    static SovereignOnboardingEngine instance;
    return instance;
}

void SovereignOnboardingEngine::startWizard() {
    sigma_log("[ONBOARDING] Initiating Sovereign Setup Wizard...");
    sigma_log("Welcome to Σ SigmaOS. Please select your operational persona:\n");
    sigma_log("1. Sovereign (Standard)\n2. Amnesic (Zero-Persistence)\n3. Industrial (Production)\n");
}

void SovereignOnboardingEngine::setPersona(const char* persona_name) {
    sigma_hardened_strcpy(this->active_persona, persona_name, 32);
    if (sigma_strcmp(persona_name, "Amnesic") == 0) {
        sigma_log("[ONBOARDING] Amnesic mode enabled. Shredding initialization artifacts.");
    } else if (sigma_strcmp(persona_name, "Industrial") == 0) {
        sigma_log("[ONBOARDING] Industrial mode enabled. Hardening all lattice shards.");
    }
}

extern "C" void onboarding_start() {
    SovereignOnboardingEngine::startWizard();
}



