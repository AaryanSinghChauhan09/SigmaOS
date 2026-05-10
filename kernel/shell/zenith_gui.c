#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: ZENITH GUI COMPOSITOR (v1.0)
 * =============================================================================
 * Principles: Sharded Window Management & VGA Glassmorphism.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

#define COLS 80
#define ROWS 25

typedef struct Window {
    sigma_u8  x, y, w, h;
    char* title;
    sigma_bool active;
} window_t;

extern void vga_putc_at(sigma_u8 x, sigma_u8 y, char c, sigma_u8 color);

void zenith_draw_window(window_t* win) {
    if (!win->active) return;

    /* Draw Border */
    for (sigma_u8 i = 0; i < win->w; i++) {
        vga_putc_at(win->x + i, win->y, '-', 0x07);
        vga_putc_at(win->x + i, win->y + win->h - 1, '-', 0x07);
    }
    for (sigma_u8 j = 0; j < win->h; j++) {
        vga_putc_at(win->x, win->y + j, '|', 0x07);
        vga_putc_at(win->x + win->w - 1, win->y + j, '|', 0x07);
    }

    /* Draw Title */
    sigma_u8 title_len = sigma_strlen(win->title);
    for (sigma_u8 k = 0; k < title_len && k < win->w - 2; k++) {
        vga_putc_at(win->x + 1 + k, win->y, win->title[k], 0x0B);
    }
}
