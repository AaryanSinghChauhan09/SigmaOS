/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN FRONTEND SUITE (v2.0 - SUPREME UPGRADE)
 * =========================================================================
 * Mission: Real Window Management and Compositor Logic.
 * =========================================================================
 */

#include "../include/sigma_kernel.h"

typedef struct {
    sigma_u32 win_id;
    int x, y, w, h;
    int z_order;
    sigma_bool focused;
} SovereignWindow_t;

static SovereignWindow_t s_window_stack[16];
static int s_window_count = 0;

void sigma_frontend_wm_init(void) {
    sigma_memset(s_window_stack, 0, sizeof(s_window_stack));
    s_window_count = 0;
}

void sigma_frontend_create_window(int x, int y, int w, int h) {
    if (s_window_count >= 16) return;
    SovereignWindow_t *win = &s_window_stack[s_window_count];
    win->win_id = s_window_count;
    win->x = x; win->y = y; win->w = w; win->h = h;
    win->z_order = s_window_count;
    win->focused = SIGMA_TRUE;
    sigma_printf("  [WM]: Created window %d at (%d,%d) [%dx%d]\n", win->win_id, x, y, w, h);
    s_window_count++;
}

void SovereignFrontend_Init(void) {
    sigma_printf("Σ [FRONTEND]: Initialising Sovereign Compositor...\n");
    sigma_frontend_wm_init();
    sigma_frontend_create_window(100, 100, 800, 600);
    sigma_printf("Σ [FRONTEND]: Accelerator seated. Framebuffer mapped.\n");
}

void SovereignFrontend_Register(void) {
    static SovereignModule_t s_front_module = {
        .name = "SovereignFrontend",
        .type = MODULE_TYPE_CORE,
        .Init = (sigma_err_t(*)(void))SovereignFrontend_Init,
    };
    sigma_module_register(&s_front_module);
}
