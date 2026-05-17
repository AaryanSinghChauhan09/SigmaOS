#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"
#ifndef SOVEREIGN_ONBOARDING_HPP
#define SOVEREIGN_ONBOARDING_HPP

#include "../../../include/sigma_kernel_types.h"

class SovereignOnboardingEngine {
public:
    static SovereignOnboardingEngine& getInstance();
    void startWizard();
    void setPersona(const char* persona_name);

private:
    SovereignOnboardingEngine() : completed(false) {}
    bool completed;
    char active_persona[32];
};

extern "C" {
    void onboarding_start();
}

#endif

 