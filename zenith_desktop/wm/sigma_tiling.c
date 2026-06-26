/*
 * sigma_tiling.c — C ABI shim for Zenith tiling WM (sigma_tiling_wm.cpp).
 */
#include "sigma_tiling.h"

extern sigma_status sigma_wm_init(sigma_u32 w, sigma_u32 h);
extern sigma_status sigma_wm_auto_tile(void);
extern sigma_status sigma_wm_layout(sigma_u32 m);
extern sigma_status sigma_wm_gaps(sigma_u32 inner, sigma_u32 outer);

sigma_status sigma_tiling_init(sigma_u32 screen_w, sigma_u32 screen_h) {
    return sigma_wm_init(screen_w, screen_h);
}

sigma_status sigma_tiling_auto_tile(void) {
    return sigma_wm_auto_tile();
}

sigma_status sigma_tiling_set_layout(sigma_u32 mode) {
    return sigma_wm_layout(mode);
}

sigma_status sigma_tiling_set_gaps(sigma_u32 inner, sigma_u32 outer) {
    return sigma_wm_gaps(inner, outer);
}
