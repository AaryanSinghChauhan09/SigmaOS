#include "core/sigma_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: OPTIMIZED VGA TEXT DRIVER (v1.2)
 * =============================================================================
 * Principles: Zero-Abstract Visualization & High-Performance Scrolling.
 * =============================================================================
 */
#include "core/sigma_kernel_types.h"

#define VGA_ADDR 0xB8000
#define COLS     80
#define ROWS     25

static sigma_u16* vga_buffer = (sigma_u16*)VGA_ADDR;
static sigma_u8  cursor_x = 0;
static sigma_u8  cursor_y = 0;

/* Fast Scroll using sigma_memcpy (Industrial Optimization) */
static void vga_scroll() {
    sigma_memcpy(vga_buffer, vga_buffer + COLS, (ROWS - 1) * COLS * 2);
    sigma_memset(vga_buffer + (ROWS - 1) * COLS, 0, COLS * 2);
    cursor_y = ROWS - 1;
}

void vga_putc(char c, sigma_u8 color) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
    } else if (c == '\r') {
        cursor_x = 0;
    } else if (c == '\b') {
        if (cursor_x > 0) cursor_x--;
    } else {
        sigma_u32 idx = cursor_y * COLS + cursor_x;
        vga_buffer[idx] = (sigma_u16)c | ((sigma_u16)color << 8);
        cursor_x++;
    }

    if (cursor_x >= COLS) {
        cursor_x = 0;
        cursor_y++;
    }

    if (cursor_y >= ROWS) {
        vga_scroll();
    }
}
