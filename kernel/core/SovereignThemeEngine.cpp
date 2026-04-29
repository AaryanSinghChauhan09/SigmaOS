#include "Lattice.h"
#include "sigma_hal.h"
#include "sigma_libc.h"

/**
 * SigmaOS Sovereign Theme Engine
 * High-performance, silicon-native personalization for the Zenith interface.
 */

typedef struct {
    uint32_t accent_color;
    uint32_t background_blur_sigma;
    bool dark_mode;
    char font_family[32];
} theme_config_t;

static theme_config_t active_theme;

extern "C" void theme_init() {
    sigma_log("[THEME] Initializing Sovereign Silicon Theme Engine...");
    
    active_theme.accent_color = 0x00A0FF; // Sigma Blue
    active_theme.background_blur_sigma = 20;
    active_theme.dark_mode = true;
    sigma_hardened_strcpy(active_theme.font_family, "Outfit", 32);
}

extern "C" void theme_apply_accent(uint32_t color) {
    active_theme.accent_color = color;
    sigma_log("[THEME] Silicon Accent Color updated to 0x%06X.", color);
    // Directly push to GPU registers for real-time update
}

extern "C" void theme_toggle_dark_mode() {
    active_theme.dark_mode = !active_theme.dark_mode;
    sigma_log("[THEME] Dark Mode: %s.", active_theme.dark_mode ? "ENABLED" : "DISABLED");
}
