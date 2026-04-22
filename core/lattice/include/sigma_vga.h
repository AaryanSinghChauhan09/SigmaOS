#ifndef SIGMA_VGA_H
#define SIGMA_VGA_H

#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_types.h"

/* =========================================================================
 * SIGMA OS: BARE METAL VGA DEVICE DRIVER (SYSTEM-LEVEL HEADER)
 * Handles hardware-level Text Mode rendering mapped at memory 0xB8000.
 * ========================================================================= */

#define VGA_WIDTH 80
#define VGA_HEIGHT 25

// Hardware color constants mapped to VGA controller
typedef enum {
    VGA_COLOR_BLACK = 0,
    VGA_COLOR_BLUE = 1,
    VGA_COLOR_GREEN = 2,
    VGA_COLOR_CYAN = 3,
    VGA_COLOR_RED = 4,
    VGA_COLOR_MAGENTA = 5,
    VGA_COLOR_BROWN = 6,
    VGA_COLOR_LIGHT_GREY = 7,
    VGA_COLOR_DARK_GREY = 8,
    VGA_COLOR_LIGHT_BLUE = 9,
    VGA_COLOR_LIGHT_GREEN = 10,
    VGA_COLOR_LIGHT_CYAN = 11,
    VGA_COLOR_LIGHT_RED = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_LIGHT_BROWN = 14,
    VGA_COLOR_WHITE = 15,
} vga_color_t;

void sigma_vga_init(void);
void sigma_vga_set_color(vga_color_t fg, vga_color_t bg);
void sigma_vga_put_char(char c);
void sigma_vga_print(const char* str);
void sigma_vga_clear_screen(void);

#endif
