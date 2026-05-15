#include "SovereignOnboarding.hpp"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"

SovereignOnboardingEngine& SovereignOnboardingEngine::getInstance() {
    static SovereignOnboardingEngine instance;
    return instance;
}

void SovereignOnboardingEngine::startWizard() {
    sigma_log("[ONBOARDING] Initiating Sovereign Setup Wizard...");
    sigma_log_info("Welcome to Σ SigmaOS. Please select your operational persona:\n");
    sigma_log_info("1. Sovereign (Standard)\n2. Amnesic (Zero-Persistence)\n3. Industrial (Production)\n");
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
    SovereignOnboardingEngine::getInstance().startWizard();
}


