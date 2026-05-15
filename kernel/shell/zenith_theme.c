/*
 * =============================================================================
 * Î£ SIGMAOS SHELL: ZENITH GUI THEME ENGINE (v1.0)
 * =============================================================================
 * Principles: Glassmorphism & High-Contrast Aesthetics.
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

typedef struct Theme {
    sigma_u8      primary_color;
    sigma_u8      secondary_color;
    sigma_u8      accent_color;
    sigma_bool  glassmorphism;
} theme_t;

static theme_t zenith_current_theme;

void zenith_theme_init() {
    /* Default: Sovereign Dark */
    zenith_current_theme.primary_color   = 0x00; /* Black */
    zenith_current_theme.secondary_color = 0x08; /* Dark Grey */
    zenith_current_theme.accent_color    = 0x01; /* Blue */
    zenith_current_theme.glassmorphism   = SIGMA_TRUE;
    
    kprintf("Î£ [ZENITH-THEME]: Sovereign Dark aesthetics applied.\n");
}

/* Update the GUI theme at runtime */
void zenith_theme_update(sigma_u8 primary, sigma_u8 accent) {
    zenith_current_theme.primary_color = primary;
    zenith_current_theme.accent_color = accent;
    kprintf("Î£ [ZENITH-THEME]: Theme hot-swapped.\n");
}

theme_t zenith_get_theme() {
    return zenith_current_theme;
}
