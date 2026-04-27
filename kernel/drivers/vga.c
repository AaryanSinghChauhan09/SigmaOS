/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VGA TEXT DRIVER (v1.0)
 * =============================================================================
 * Principles: Zero-Abstract Visualization.
 * =============================================================================
 */
#include "../../include/sigma_kernel_types.h"

#define VGA_ADDR 0xB8000
#define COLS     80
#define ROWS     25

static u16* vga_buffer = (u16*)VGA_ADDR;
static u8  cursor_x = 0;
static u8  cursor_y = 0;

void vga_clear() {
    for (int i = 0; i < COLS * ROWS; i++) {
        vga_buffer[i] = (u16)0x0720; /* Space, White on Black */
    }
    cursor_x = 0;
    cursor_y = 0;
}

void vga_putc(char c, u8 color) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
    } else {
        u32 idx = cursor_y * COLS + cursor_x;
        vga_buffer[idx] = (u16)c | ((u16)color << 8);
        cursor_x++;
    }

    if (cursor_x >= COLS) {
        cursor_x = 0;
        cursor_y++;
    }

    /* Simple scroll check */
    if (cursor_y >= ROWS) {
        sigma_memcpy(vga_buffer, vga_buffer + COLS, (ROWS - 1) * COLS * 2);
        sigma_memset(vga_buffer + (ROWS - 1) * COLS, 0, COLS * 2);
        cursor_y = ROWS - 1;
    }
}

void vga_puts(const char* s, u8 color) {
    while (*s) {
        vga_putc(*s++, color);
    }
}
