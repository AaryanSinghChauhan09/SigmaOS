/*
 * =============================================================================
 * Σ SIGMAOS: ZENITH WINDOW MANAGER (sigma-wm v1.0)
 * =============================================================================
 * Mission: The kernel/userland boundary for graphics compositing.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_WM_H
#define SIGMA_WM_H

#include "../sigma_kernel_types.h"

#define WM_MAX_WINDOWS      32
#define WM_TITLE_LEN        32
#define WM_SCREEN_WIDTH   1920
#define WM_SCREEN_HEIGHT  1080

typedef struct {
    sigma_u8 r, g, b, a;
} sigma_color_t;

typedef struct {
    sigma_u32      win_id;
    sigma_u32      owner_pid;
    char           title[WM_TITLE_LEN];
    int            x, y;
    int            width, height;
    int            z_index;          /* 0 = background, higher = foreground */
    sigma_bool     is_visible;
    sigma_vaddr_t  framebuffer;      /* Virtual address of the window's backing store */
} sigma_window_t;

#ifdef __cplusplus
extern "C" {
#endif

void      wm_init(void);
sigma_u32 wm_create_window(sigma_u32 pid, const char* title, int x, int y, int w, int h);
int       wm_destroy_window(sigma_u32 win_id);

int       wm_move_window(sigma_u32 win_id, int new_x, int new_y);
int       wm_set_z_index(sigma_u32 win_id, int z_index);
int       wm_set_visibility(sigma_u32 win_id, sigma_bool visible);

void      wm_composite(void); /* Draws all windows to the master framebuffer */

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_WM_H */
