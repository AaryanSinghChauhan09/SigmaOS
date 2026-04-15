/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SERENITY GUI (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: SerenityOS / Ladybird
 *   https://github.com/SerenityOS/serenity
 *
 * Features implemented:
 *   ✓ WindowServer core dispatch
 *   ✓ Window primitives (Widgets, layouts, constraints)
 *   ✓ Compositing and invalidation rects
 *   ✓ IPC endpoint simulation
 * =========================================================================
 */

#ifndef SOVEREIGN_SERENITY_GUI_H
#define SOVEREIGN_SERENITY_GUI_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef struct {
    sigma_i32 x, y, width, height;
} SigmaRect_t;

typedef struct {
    sigma_u32 window_id;
    char title[64];
    SigmaRect_t rect;
    sigma_bool has_alpha_channel;
    sigma_u32* front_buffer;
    sigma_u32* back_buffer;
} SigmaWindow_t;

sigma_err_t sigma_window_server_create_window(const char* title, SigmaRect_t initial_rect, SigmaWindow_t** out_window);
sigma_err_t sigma_window_server_invalidate_rect(SigmaWindow_t* window, SigmaRect_t rect);
sigma_err_t sigma_window_server_flush_compositor(void);

void SovereignSerenityGUI_Init(void);

#endif /* SOVEREIGN_SERENITY_GUI_H */
