/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UNIVERSAL UI (S-UNIVERSALUI)
 * =========================================================================
 * Mission: Silicon-accelerated rendering and universal human-machine interface
 * logic, enabling next-generation user experiences directly from the kernel.
 * =========================================================================
 */

#ifndef SIGMA_UNIVERSALUI_H
#define SIGMA_UNIVERSALUI_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    UI_THEME_DARK_NEON,
    UI_THEME_LIGHT_GLASS,
    UI_THEME_HOLO_HUD
} sigma_ui_theme_t;

/* --- Universal UI Primitives --- */
void universalui_init(void);
void universalui_set_theme(sigma_ui_theme_t theme);
void universalui_render_widget(uint32_t widget_id, uint32_t x, uint32_t y);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_UNIVERSALUI_H */
