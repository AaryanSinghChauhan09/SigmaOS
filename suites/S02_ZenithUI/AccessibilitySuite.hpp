#pragma once
#include <stdint.h>
#include "../../include/libc/sigma_libc.h"

namespace SigmaOS {
namespace UI {

// Sprint 16: Accessibility Suite Expansion
class AccessibilitySuite {
private:
    bool screen_reader;
    bool voice_control;
    bool high_contrast;

public:
    AccessibilitySuite() : screen_reader(false), voice_control(false), high_contrast(false) {
        sigma_log("[ACCESSIBILITY] Zenith Accessibility Suite Online.");
    }

    void enable_profile(const char* profile_name) {
        sigma_print("[ACCESSIBILITY] Activating Profile: ");
        sigma_print(profile_name);
        sigma_print("\n");

        if (sigma_strcmp(profile_name, "Low Vision") == 0) {
            high_contrast = true;
            screen_reader = true;
            sigma_log("[ACCESSIBILITY] High-contrast UI and Screen Reader activated.");
        } else if (sigma_strcmp(profile_name, "Motor Assistance") == 0) {
            voice_control = true;
            sigma_log("[ACCESSIBILITY] Offline Voice Control and Switch Control activated.");
        }
    }
};

} // namespace UI
} // namespace SigmaOS
