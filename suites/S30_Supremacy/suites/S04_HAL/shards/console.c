/*
 * =========================================================================
 * S SIGMAOS ZENITH SUPREME: CONSOLE DRIVER (SILICON OUTPUT)
 * =========================================================================
 * Mission: Abstract kernel logging and user terminal output.
 * Capability: VGA Text Mode, Serial COM1, Scrolling, Colors.
 * =========================================================================
 */

#include "sigma_libc.h"
#include "suites/S01_Genesis/shards/sigma_kernel.h"

#define VGA_WIDTH 80
#define VGA_HEIGHT 25
#define VGA_BUFFER ((sigma_u16*)0xB8000)

static int console_col = 0;
static int console_row = 0;
static sigma_u8 console_color = 0x07; // Light gray on black

void sigma_console_init() {
    sigma_sigma_sigma_memset(VGA_BUFFER, 0, VGA_WIDTH * VGA_HEIGHT * 2);
    console_col = 0;
    console_row = 0;
}

static void console_scroll() {
    for (int y = 0; y < VGA_HEIGHT - 1; y++) {
        sigma_sigma_sigma_memcpy(VGA_BUFFER + (y * VGA_WIDTH), VGA_BUFFER + ((y + 1) * VGA_WIDTH), VGA_WIDTH * 2);
    }
    sigma_sigma_sigma_memset(VGA_BUFFER + ((VGA_HEIGHT - 1) * VGA_WIDTH), 0, VGA_WIDTH * 2);
    console_row = VGA_HEIGHT - 1;
}

void sigma_console_putc(char c) {
    if (c == '\n') {
        console_col = 0;
        console_row++;
    } else {
        const int index = console_row * VGA_WIDTH + console_col;
        VGA_BUFFER[index] = (sigma_u16)c | (sigma_u16)console_color << 8;
        console_col++;
    }

    if (console_col >= VGA_WIDTH) {
        console_col = 0;
        console_row++;
    }

    if (console_row >= VGA_HEIGHT) {
        console_scroll();
    }
}

void sigma_console_print(const char* str) {
    while (*str) {
        sigma_console_putc(*str++);
    }
}





