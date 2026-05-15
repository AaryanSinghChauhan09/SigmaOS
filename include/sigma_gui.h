/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GRAPHICS INTERFACE (SGI)
 * =========================================================================
 * Mission: Silicon-native, zero-latency frame-buffer orchestration.
 * =========================================================================
 */

#ifndef SIGMA_GUI_H
#define SIGMA_GUI_H

#include "../include/core/sigma_types.h"

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

class SovereignGUIEngine {
public:
    static SovereignGUIEngine& getInstance() {
        static SovereignGUIEngine instance;
        return instance;
    }

    void init(const sigma_fb_config_t* config);
    void drawPixel(sigma_u32 x, sigma_u32 y, sigma_u32 color);
    void flush();

private:
    SovereignGUIEngine() : initialized(0) {
        active_fb.width = 0;
        active_fb.height = 0;
        active_fb.bpp = 0;
        active_fb.frame_buffer = (void*)0;
    }
    
    sigma_fb_config_t active_fb;
    sigma_u32 initialized;
};
#endif

#endif /* SIGMA_GUI_H */
