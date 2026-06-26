/*
 * Σ SigmaOS — sigma_vga_driver: Sovereign VGA Framebuffer
 * Zero-Dependency: No libc. Hardware-direct MMIO.
 * Provides the baseline graphical/text output for the entire OS.
 */

typedef unsigned char  uint8_t;
typedef unsigned short uint16_t;

#define VGA_ADDRESS 0xB8000
#define VGA_WIDTH   80
#define VGA_HEIGHT  25

static uint16_t* vga_buffer = (uint16_t*) VGA_ADDRESS;
static int vga_cursor_x = 0;
static int vga_cursor_y = 0;

/* 
 * Plot a character directly to the VGA hardware buffer
 */
extern "C" void vga_putc(int x, int y, char c, uint8_t color) {
    if (x >= 0 && x < VGA_WIDTH && y >= 0 && y < VGA_HEIGHT) {
        vga_buffer[y * VGA_WIDTH + x] = ((uint16_t)color << 8) | c;
    }
}

/*
 * Clear the screen
 */
extern "C" void sigma_vga_clear() {
    for (int y = 0; y < VGA_HEIGHT; y++) {
        for (int x = 0; x < VGA_WIDTH; x++) {
            vga_putc(x, y, ' ', 0x07);
        }
    }
    vga_cursor_x = 0;
    vga_cursor_y = 0;
}

/*
 * Standard kernel putchar
 */
extern "C" void sigma_vga_putchar(char c) {
    if (c == '\n') {
        vga_cursor_x = 0;
        vga_cursor_y++;
    } else {
        vga_putc(vga_cursor_x, vga_cursor_y, c, 0x07);
        vga_cursor_x++;
    }

    if (vga_cursor_x >= VGA_WIDTH) {
        vga_cursor_x = 0;
        vga_cursor_y++;
    }

    if (vga_cursor_y >= VGA_HEIGHT) {
        /* Basic scroll logic (stubbed) */
        sigma_vga_clear(); 
    }
}

/*
 * Standard kernel puts
 */
extern "C" void sigma_vga_puts(const char* s) {
    while (*s) {
        sigma_vga_putchar(*s++);
    }
}
