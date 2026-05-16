/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN UX & PERSONALIZATION ENGINE
 * =========================================================================
 * Mission: High-fidelity aesthetics and automated personalization.
 * =========================================================================
 */

#ifndef SIGMA_UX_H
#define SIGMA_UX_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 primary_color;
    sigma_u32 secondary_color;
    sigma_u32 transparency_level;
    bool blur_enabled;
    const char* font_shard_id;
} sigma_theme_t;

/* --- UX Primitives --- */
void ux_init(void);
void ux_apply_theme(sigma_theme_t* theme);
void ux_render_dashboard(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_UX_H */
