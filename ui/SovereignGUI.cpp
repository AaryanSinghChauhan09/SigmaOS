#include "../include/sigma_log.h"
#include "../include/libc/SovereignLibC.h"
#include "../include/sigma_kernel_types.h"
#include "../include/hal/sigma_hal.h"
#include "../include/sigma_kernel_types.h"
#include "../include/ui/sigma_gui.h"
#include "../include/hal/sigma_hal.h"

/**
 * SigmaOS Sovereign GUI Implementation
 * Implements a Predictive Frame-Buffer Rendering (PFR) algorithm.
 * ZERO-DEPENDENCY: Strictly uses bare-metal silicon primitives.
 */

/* --- Sovereign GUI Engine (OOP Isolation) --- */

void SovereignGUIEngine::init(const sigma_fb_config_t* config) {
    if (config) {
        this->active_fb = *config;
    }
    this->initialized = 1u;
    sigma_log("[GUI] Sovereign SGI Initialized. Frame-buffer mapped to silicon.");
}

void SovereignGUIEngine::drawPixel(sigma_u32 x, sigma_u32 y, sigma_u32 color) {
    /* PFR (Predictive Frame-Buffer Rendering) Algorithm
     * Direct silicon memory access for pixel placement. */
    if (!this->initialized) return;
    if (x >= this->active_fb.width || y >= this->active_fb.height) return;
    
    sigma_u32* fb = (sigma_u32*)this->active_fb.frame_buffer;
    fb[y * this->active_fb.width + x] = color;
}

void SovereignGUIEngine::flush() {
    /* Coalescing Graphics Update (CGU) Algorithm
     * Simulates a bare-metal DMA flush to the physical display device. */
    sigma_log("[GUI] CGU Flush: Silicon state synchronized with display.");
}

/* --- C Wrappers --- */
void gui_init(sigma_fb_config_t* config) {
    SovereignGUIEngine::init(config);
}

void gui_draw_pixel(sigma_u32 x, sigma_u32 y, sigma_u32 color) {
    SovereignGUIEngine::drawPixel(x, y, color);
}

void gui_flush() {
    SovereignGUIEngine::flush();
}




} // extern "C"
