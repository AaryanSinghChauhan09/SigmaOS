/*
 * =============================================================================
 * Σ SIGMAOS: ZENITH DESKTOP ENVIRONMENT
 * =============================================================================
 * Mission: A sleek, modern, spatial desktop environment running natively on top
 *          of the Zenith Window Manager (sigma-wm).
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef ZENITH_DESKTOP_H
#define ZENITH_DESKTOP_H

#include "../sigma_kernel_types.h"

typedef enum {
    THEME_DARK = 0,
    THEME_LIGHT,
    THEME_HIGH_CONTRAST
} zenith_theme_t;

typedef struct {
    sigma_u32  dock_win_id;
    sigma_u32  panel_win_id;
    sigma_bool is_blur_enabled;
    zenith_theme_t current_theme;
} zenith_desktop_state_t;

#ifdef __cplusplus
extern "C" {
#endif

void zenith_init(void);
void zenith_draw_dock(void);
void zenith_draw_top_panel(void);
void zenith_set_theme(zenith_theme_t theme);
void zenith_handle_click(int x, int y);

#ifdef __cplusplus
}
#endif

#endif /* ZENITH_DESKTOP_H */
