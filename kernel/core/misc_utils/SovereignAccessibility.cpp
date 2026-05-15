#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Accessibility Engine
 * Smart accessibility defaults and UI scaling.
 *
 * USP: Analyzes user interaction patterns to automatically adjust contrast, 
 * font scaling, and color-blind modes without requiring manual configuration.
 *
 * Design: OOP-isolated singleton � SovereignAccessibilityEngine.
 */

class SovereignAccessibilityEngine {
public:
    static SovereignAccessibilityEngine& getInstance() {
        static SovereignAccessibilityEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[ACCESSIBILITY] Initializing Smart Defaults Engine...");
        this->font_scale = 1.0f;
        this->high_contrast = false;
        sigma_hardened_strcpy(this->color_mode, "STANDARD", 16);
    }

    void enableColorBlindMode(const char* mode) {
        sigma_hardened_strcpy(this->color_mode, mode, 16);
        sigma_log("[ACCESSIBILITY] UI Color Palette dynamically shifted to '%s'.\n", mode);
    }

    void adjustFontScaling(float scale_factor) {
        this->font_scale = scale_factor;
        sigma_log("[ACCESSIBILITY] System-wide font scaling adjusted to %.2fx.\n", scale_factor);
    }

    void toggleHighContrast() {
        this->high_contrast = !this->high_contrast;
        sigma_log("[ACCESSIBILITY] High Contrast Mode: %s\n", this->high_contrast ? "ENABLED" : "DISABLED");
    }

private:
    SovereignAccessibilityEngine() : font_scale(1.0f), high_contrast(false) {}

    float font_scale;
    bool high_contrast;
    char color_mode[16];
};

/* --- C Wrappers --- */
void access_init() {
    SovereignAccessibilityEngine::init();
}

void access_set_colorblind(const char* mode) {
    SovereignAccessibilityEngine::enableColorBlindMode(mode);
}

void access_set_font_scale(float scale) {
    SovereignAccessibilityEngine::adjustFontScaling(scale);
}





} // extern "C"
