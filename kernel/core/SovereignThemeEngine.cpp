#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Sovereign Theme Engine
 * High-performance, silicon-native personalization for the Zenith interface.
 *
 * Design: OOP-isolated singleton — SovereignThemeEngine.
 */

class SovereignThemeEngine {
public:
    static SovereignThemeEngine& getInstance() {
        static SovereignThemeEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[THEME] Initializing Sovereign Silicon Theme Engine...");
        
        this->active_theme.accent_color = 0x00A0FF; // Sigma Blue
        this->active_theme.background_blur_sigma = 20;
        this->active_theme.dark_mode = true;
        sigma_hardened_strcpy(this->active_theme.font_family, "Outfit", 32);
    }

    void applyAccent(sigma_u32 color) {
        this->active_theme.accent_color = color;
        sigma_printf("[THEME] Silicon Accent Color updated to 0x%06X.\n", (unsigned)color);
    }

    void toggleDarkMode() {
        this->active_theme.dark_mode = !this->active_theme.dark_mode;
        sigma_log(this->active_theme.dark_mode ? "[THEME] Dark Mode: ENABLED." : "[THEME] Dark Mode: DISABLED.");
    }

private:
    SovereignThemeEngine() {
        active_theme.accent_color = 0;
        active_theme.background_blur_sigma = 0;
        active_theme.dark_mode = false;
    }
    
    struct {
        sigma_u32 accent_color;
        sigma_u32 background_blur_sigma;
        bool      dark_mode;
        char      font_family[32];
    } active_theme;
};

/* --- C Wrappers --- */
extern "C" void theme_init() {
    SovereignThemeEngine::getInstance().init();
}

extern "C" void theme_apply_accent(sigma_u32 color) {
    SovereignThemeEngine::getInstance().applyAccent(color);
}

extern "C" void theme_toggle_dark_mode() {
    SovereignThemeEngine::getInstance().toggleDarkMode();
}
