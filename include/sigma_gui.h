/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GRAPHICS INTERFACE (SGI)
 * =========================================================================
 * Mission: Silicon-native, zero-latency frame-buffer orchestration.
 * =========================================================================
 */

#ifndef SIGMA_GUI_H
#define SIGMA_GUI_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 width;
    sigma_u32 height;
    sigma_u32 bpp;
    void* frame_buffer;
} sigma_fb_config_t;

typedef struct {
    sigma_u32 x;
    sigma_u32 y;
    sigma_u32 color;
} sigma_pixel_t;

/* --- Graphics Primitives --- */
void gui_init(sigma_fb_config_t* config);
void gui_draw_pixel(sigma_u32 x, sigma_u32 y, sigma_u32 color);
void gui_flush(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_GUI_H */
