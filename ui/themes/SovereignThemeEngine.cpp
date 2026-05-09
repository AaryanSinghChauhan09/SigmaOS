#include "../../include/sigma_log.h"
#include "../../include/core/sigma_types.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"
#include "sigma_time.h"

/**
 * SigmaOS Sovereign Adaptive Theme Engine
 * High-performance, silicon-native personalization for the Zenith interface.
 *
 * USP: Hardware-accelerated glassmorphism with Adaptive auto-dark switching 
 * natively embedded in the OS kernel.
 *
 * Design: OOP-isolated singleton — SovereignThemeEngine.
 */

class SovereignThemeEngine {
public:
    static SovereignThemeEngine& getInstance() {
        static SovereignThemeEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[THEME] Initializing Sovereign Adaptive Theme Engine...");
        
        this->active_theme.accent_color = 0x00A0FF; // Sigma Blue
        this->active_theme.background_blur_sigma = 20;
        this->active_theme.dark_mode = true;
        this->adaptive_switch_enabled = true;
        sigma_hardened_strcpy(this->active_theme.font_family, "Outfit", 32);
        sigma_log("[THEME] Adaptive Engine initialized. Awaiting ambient sensor data.");
    }

    void applyAccent(sigma_u32 color) {
        this->active_theme.accent_color = color;
        sigma_log("[THEME] Silicon Accent Color updated to 0x%06X.\n", (unsigned)color);
    }

    void toggleDarkMode() {
        this->active_theme.dark_mode = !this->active_theme.dark_mode;
        sigma_log(this->active_theme.dark_mode ? "[THEME] Dark Mode: ENABLED." : "[THEME] Dark Mode: DISABLED.");
    }

    void evaluateAdaptiveTheme() {
        if (!this->adaptive_switch_enabled) return;

        // Scaffolded Adaptive UI Theme logic: 
        // Queries real-time clock to switch Dark/Light mode dynamically.
        // In reality, this would also query an ambient light sensor.
        sigma_time_t now = time_now();
        bool should_be_dark = (now.hour >= 18 || now.hour <= 6);
        
        if (should_be_dark != this->active_theme.dark_mode) {
            sigma_log("[THEME] Adaptive Trigger: Auto-switching theme due to time (%02d:%02d).\n", now.hour, now.minute);
            this->toggleDarkMode();
        }
    }

private:
    SovereignThemeEngine() {
        active_theme.accent_color = 0;
        active_theme.background_blur_sigma = 0;
        active_theme.dark_mode = false;
        adaptive_switch_enabled = false;
    }
    
    struct {
        sigma_u32 accent_color;
        sigma_u32 background_blur_sigma;
        bool      dark_mode;
        char      font_family[32];
    } active_theme;

    bool adaptive_switch_enabled;
};

/* --- C Wrappers --- */
extern "C" void theme_init() {
    SovereignThemeEngine::init();
}

extern "C" void theme_apply_accent(sigma_u32 color) {
    SovereignThemeEngine::applyAccent(color);
}

extern "C" void theme_toggle_dark_mode() {
    SovereignThemeEngine::toggleDarkMode();
}

extern "C" void theme_evaluate_adaptive() {
    SovereignThemeEngine::evaluateAdaptiveTheme();
}




