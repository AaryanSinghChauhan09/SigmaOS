/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTUAL TERMINAL CONSOLE (TTY)
 * =============================================================================
 * Inspired by: Linux kernel drivers/tty/vt/vt.c
 *              FreeBSD sys/dev/vt/vt_core.c
 * =============================================================================
 * Emulates an ANSI-compatible VT100 terminal buffer.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define TTY_COLS 80
#define TTY_ROWS 25
#define TTY_ATTR_DEFAULT 0x07 /* Light gray on black */

typedef struct {
    sigma_u16 buffer[TTY_ROWS][TTY_COLS];
    sigma_u32 cursor_x;
    sigma_u32 cursor_y;
    sigma_u8  current_attr;
    sigma_bool cursor_visible;
} sigma_tty_t;

static sigma_tty_t console;

static void tty_scroll(void) {
    for (sigma_u32 y = 1; y < TTY_ROWS; y++) {
        for (sigma_u32 x = 0; x < TTY_COLS; x++) {
            console.buffer[y - 1][x] = console.buffer[y][x];
        }
    }
    for (sigma_u32 x = 0; x < TTY_COLS; x++) {
        console.buffer[TTY_ROWS - 1][x] = (console.current_attr << 8) | ' ';
    }
    console.cursor_y--;
}

void tty_init(void) {
    sigma_memset(&console, 0, sizeof(console));
    console.current_attr = TTY_ATTR_DEFAULT;
    console.cursor_visible = SIGMA_TRUE;
    
    for (sigma_u32 y = 0; y < TTY_ROWS; y++) {
        for (sigma_u32 x = 0; x < TTY_COLS; x++) {
            console.buffer[y][x] = (TTY_ATTR_DEFAULT << 8) | ' ';
        }
    }
    sigma_printf("[tty] VT100 Console initialized (%dx%d)\n", TTY_COLS, TTY_ROWS);
}

void tty_putc(char c) {
    if (c == '\n') {
        console.cursor_x = 0;
        console.cursor_y++;
    } else if (c == '\r') {
        console.cursor_x = 0;
    } else if (c == '\b') {
        if (console.cursor_x > 0) {
            console.cursor_x--;
            console.buffer[console.cursor_y][console.cursor_x] = (console.current_attr << 8) | ' ';
        }
    } else if (c == '\t') {
        console.cursor_x = (console.cursor_x + 8) & ~7;
    } else {
        console.buffer[console.cursor_y][console.cursor_x] = (console.current_attr << 8) | c;
        console.cursor_x++;
    }

    if (console.cursor_x >= TTY_COLS) {
        console.cursor_x = 0;
        console.cursor_y++;
    }
    
    if (console.cursor_y >= TTY_ROWS) {
        tty_scroll();
        console.cursor_y = TTY_ROWS - 1;
    }
}

void tty_write(const char* str) {
    while (*str) {
        tty_putc(*str++);
    }
}

void tty_clear(void) {
    for (sigma_u32 y = 0; y < TTY_ROWS; y++) {
        for (sigma_u32 x = 0; x < TTY_COLS; x++) {
            console.buffer[y][x] = (console.current_attr << 8) | ' ';
        }
    }
    console.cursor_x = 0;
    console.cursor_y = 0;
}

void tty_set_attr(sigma_u8 fg, sigma_u8 bg) {
    console.current_attr = (bg << 4) | (fg & 0x0F);
}

void tty_move_cursor(sigma_u32 x, sigma_u32 y) {
    if (x < TTY_COLS && y < TTY_ROWS) {
        console.cursor_x = x;
        console.cursor_y = y;
    }
}
