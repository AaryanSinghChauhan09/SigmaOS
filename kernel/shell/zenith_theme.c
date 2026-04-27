/*
 * =============================================================================
 * Σ SIGMAOS SHELL: ZENITH GUI THEME ENGINE (v1.0)
 * =============================================================================
 * Principles: Glassmorphism & High-Contrast Aesthetics.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

typedef struct Theme {
    u8      primary_color;
    u8      secondary_color;
    u8      accent_color;
    bool_t  glassmorphism;
} theme_t;

static theme_t zenith_current_theme;

void zenith_theme_init() {
    /* Default: Sovereign Dark */
    zenith_current_theme.primary_color   = 0x00; /* Black */
    zenith_current_theme.secondary_color = 0x08; /* Dark Grey */
    zenith_current_theme.accent_color    = 0x01; /* Blue */
    zenith_current_theme.glassmorphism   = TRUE;
    
    kprintf("Σ [ZENITH-THEME]: Sovereign Dark aesthetics applied.\n");
}

/* Update the GUI theme at runtime */
void zenith_theme_update(u8 primary, u8 accent) {
    zenith_current_theme.primary_color = primary;
    zenith_current_theme.accent_color = accent;
    kprintf("Σ [ZENITH-THEME]: Theme hot-swapped.\n");
}

theme_t zenith_get_theme() {
    return zenith_current_theme;
}
