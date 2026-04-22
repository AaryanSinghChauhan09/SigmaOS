#include "../../../include/sigma_vga.h"

/* =========================================================================
 * SIGMA OS: PHYSICAL VGA DISPLAY DRIVER
 * Bypasses system libraries, communicating directly with MMIO hardware.
 * ========================================================================= */

// Physical VGA text buffer mapped in hardware memory
static uint16_t* const VGA_BUFFER = (uint16_t*) 0xB8000;

static size_t vga_row;
static size_t vga_column;
static uint8_t vga_color;

// Combines foreground/background into a hardware color byte
static inline uint8_t vga_entry_color(vga_color_t fg, vga_color_t bg) {
    return fg | (bg << 4);
}

// Combines standard character and color byte into VGA hardware word
static inline uint16_t vga_entry(unsigned char uc, uint8_t color) {
    return (uint16_t) uc | (uint16_t) color << 8;
}

void sigma_vga_init(void) {
    vga_row = 0;
    vga_column = 0;
    vga_color = vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    sigma_vga_clear_screen();
}

void sigma_vga_set_color(vga_color_t fg, vga_color_t bg) {
    vga_color = vga_entry_color(fg, bg);
}

void sigma_vga_clear_screen(void) {
    for (size_t y = 0; y < VGA_HEIGHT; y++) {
        for (size_t x = 0; x < VGA_WIDTH; x++) {
            const size_t index = y * VGA_WIDTH + x;
            VGA_BUFFER[index] = vga_entry(' ', vga_color);
        }
    }
}

void sigma_vga_put_char(char c) {
    if (c == '\n') {
        vga_column = 0;
        if (++vga_row == VGA_HEIGHT) {
            vga_row = 0; // In a production OS this would map to a scrolling render algorithm
        }
        return;
    }

    const size_t index = vga_row * VGA_WIDTH + vga_column;
    VGA_BUFFER[index] = vga_entry(c, vga_color);

    if (++vga_column == VGA_WIDTH) {
        vga_column = 0;
        if (++vga_row == VGA_HEIGHT) {
            vga_row = 0; // Handle screen overflow
        }
    }
}

void sigma_vga_print(const char* str) {
    for (size_t i = 0; str[i] != '\0'; i++) {
        sigma_vga_put_char(str[i]);
    }
}
