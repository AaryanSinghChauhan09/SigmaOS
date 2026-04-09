/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SERENITY GUI — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignSerenityGUI.h"

static SigmaWindow_t s_windows[32];
static sigma_u32 s_window_count = 0;

sigma_err_t sigma_window_server_create_window(const char* title, SigmaRect_t initial_rect, SigmaWindow_t** out_window) {
    if (s_window_count >= 32) return SIGMA_ENOSPC;
    
    SigmaWindow_t *win = &s_windows[s_window_count++];
    win->window_id = s_window_count;
    sigma_strcpy(win->title, title, 64);
    win->rect = initial_rect;
    win->has_alpha_channel = SIGMA_TRUE;
    win->front_buffer = SIGMA_NULL; /* Deferred allocation in true OS */
    win->back_buffer = SIGMA_NULL;
    
    *out_window = win;
    sigma_printf("Σ [WINDOWSERVER]: Created Window #%u '%s' at [%d,%d %dx%d]\n", 
                 win->window_id, win->title, win->rect.x, win->rect.y, win->rect.width, win->rect.height);
    return SIGMA_OK;
}

sigma_err_t sigma_window_server_invalidate_rect(SigmaWindow_t* window, SigmaRect_t rect) {
    sigma_printf("Σ [WINDOWSERVER]: Invalidation rect [%d,%d %dx%d] registered for Window #%u.\n",
                 rect.x, rect.y, rect.width, rect.height, window->window_id);
    return SIGMA_OK;
}

sigma_err_t sigma_window_server_flush_compositor(void) {
    sigma_printf("Σ [WINDOWSERVER]: Compositing front/back buffers via hardware acceleration (simulated).\n");
    return SIGMA_OK;
}

void SovereignSerenityGUI_Init(void) {
    sigma_printf("Σ [WINDOWSERVER]: Initialising Sovereign Serenity GUI Matrix...\n");
    SigmaWindow_t *win = SIGMA_NULL;
    SigmaRect_t rect = { 100, 100, 800, 600 };
    sigma_window_server_create_window("Serenity-Style App", rect, &win);
    
    SigmaRect_t dirty = { 100, 100, 50, 50 };
    sigma_window_server_invalidate_rect(win, dirty);
    sigma_window_server_flush_compositor();
}
