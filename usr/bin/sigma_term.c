/*
 * =============================================================================
 * Σ SIGMAOS USERSPACE: TERMINAL EMULATOR
 * =============================================================================
 * Inspired by: st (suckless terminal), Alacritty
 * =============================================================================
 * A fast, hardware-accelerated terminal emulator sitting on top of the
 * SigmaOS compositor and fbdev layers.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"
#include "../../kernel/core/graphics/sigma_compositor.c" /* Simulated syscall binding */

#define TERM_COLS 120
#define TERM_ROWS 40

typedef struct {
    char ch;
    sigma_u32 fg_color;
    sigma_u32 bg_color;
    sigma_bool bold;
} term_cell_t;

typedef struct {
    sigma_u32 window_id;
    term_cell_t grid[TERM_ROWS][TERM_COLS];
    sigma_u32 cx, cy; /* Cursor position */
    sigma_u32 current_fg;
    sigma_u32 current_bg;
    sigma_bool running;
} sigma_terminal_t;

static sigma_terminal_t main_term;

void term_init(void) {
    sigma_memset(&main_term, 0, sizeof(sigma_terminal_t));
    
    /* Create a window via the compositor */
    main_term.window_id = compositor_create_window(1001, 800, 600);
    main_term.current_fg = 0xFFFFFF; /* White */
    main_term.current_bg = 0x000000; /* Black */
    
    for (int y = 0; y < TERM_ROWS; y++) {
        for (int x = 0; x < TERM_COLS; x++) {
            main_term.grid[y][x].ch = ' ';
            main_term.grid[y][x].fg_color = main_term.current_fg;
            main_term.grid[y][x].bg_color = main_term.current_bg;
        }
    }
    
    main_term.running = SIGMA_TRUE;
    sigma_printf("[usr] SigmaTerm Initialized (Window ID: %u)\n", main_term.window_id);
}

static void term_scroll(void) {
    for (int y = 1; y < TERM_ROWS; y++) {
        for (int x = 0; x < TERM_COLS; x++) {
            main_term.grid[y-1][x] = main_term.grid[y][x];
        }
    }
    for (int x = 0; x < TERM_COLS; x++) {
        main_term.grid[TERM_ROWS-1][x].ch = ' ';
        main_term.grid[TERM_ROWS-1][x].bg_color = main_term.current_bg;
    }
    main_term.cy--;
    compositor_damage_rect(main_term.window_id, 0, 0, 800, 600);
}

void term_putchar(char c) {
    if (c == '\n') {
        main_term.cx = 0;
        main_term.cy++;
    } else if (c == '\r') {
        main_term.cx = 0;
    } else {
        main_term.grid[main_term.cy][main_term.cx].ch = c;
        main_term.grid[main_term.cy][main_term.cx].fg_color = main_term.current_fg;
        main_term.grid[main_term.cy][main_term.cx].bg_color = main_term.current_bg;
        main_term.cx++;
    }
    
    if (main_term.cx >= TERM_COLS) {
        main_term.cx = 0;
        main_term.cy++;
    }
    if (main_term.cy >= TERM_ROWS) {
        term_scroll();
        main_term.cy = TERM_ROWS - 1;
    }
    
    compositor_damage_rect(main_term.window_id, 0, 0, 800, 600);
}

void term_print(const char* str) {
    while (*str) {
        term_putchar(*str++);
    }
}
