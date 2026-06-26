/*
 * Σ SigmaOS Zenith — VGA Framebuffer & Text Mode Display Driver
 * Absorbs: Linux fbdev, VGA text mode standard (0xB8000 BIOS convention)
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;

/* ─────────────── VGA Text Mode Constants ─────────────── */
#define VGA_WIDTH     80
#define VGA_HEIGHT    25
#define VGA_MEMORY    ((volatile u16*)0xB8000ULL)

/* ─────────────── VGA Color Palette ─────────────── */
typedef enum {
    VGA_COLOR_BLACK         = 0,
    VGA_COLOR_BLUE          = 1,
    VGA_COLOR_GREEN         = 2,
    VGA_COLOR_CYAN          = 3,
    VGA_COLOR_RED           = 4,
    VGA_COLOR_MAGENTA       = 5,
    VGA_COLOR_BROWN         = 6,
    VGA_COLOR_LIGHT_GREY    = 7,
    VGA_COLOR_DARK_GREY     = 8,
    VGA_COLOR_LIGHT_BLUE    = 9,
    VGA_COLOR_LIGHT_GREEN   = 10,
    VGA_COLOR_LIGHT_CYAN    = 11,
    VGA_COLOR_LIGHT_RED     = 12,
    VGA_COLOR_LIGHT_MAGENTA = 13,
    VGA_COLOR_YELLOW        = 14,
    VGA_COLOR_WHITE         = 15,
} VGAColor;

static inline u8 sigma_vga_entry_color(VGAColor fg, VGAColor bg) {
    return (u8)fg | ((u8)bg << 4);
}

static inline u16 sigma_vga_entry(u8 ch, u8 color) {
    return (u16)ch | ((u16)color << 8);
}

/* ─────────────── Driver State ─────────────── */
static u32  vga_row;
static u32  vga_col;
static u8   vga_color;
static volatile u16* vga_buffer;

/* ─────────────── x86 Port I/O for Cursor ─────────────── */
static inline void sigma_outb_vga(u16 port, u8 val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

/* ─────────────── API: Initialize VGA Driver ─────────────── */
extern "C" void sigma_vga_init() {
    vga_row    = 0;
    vga_col    = 0;
    vga_color  = sigma_vga_entry_color(VGA_COLOR_LIGHT_GREY, VGA_COLOR_BLACK);
    vga_buffer = VGA_MEMORY;

    /* Clear screen */
    for (u32 y = 0; y < VGA_HEIGHT; y++)
        for (u32 x = 0; x < VGA_WIDTH; x++)
            vga_buffer[y * VGA_WIDTH + x] = sigma_vga_entry(' ', vga_color);
}

/* ─────────────── API: Set Foreground Color ─────────────── */
extern "C" void sigma_vga_set_color(VGAColor fg, VGAColor bg) {
    vga_color = sigma_vga_entry_color(fg, bg);
}

/* ─────────────── Internal: Scroll screen one line ─────────────── */
static void vga_scroll() {
    /* Move all lines up by one */
    for (u32 y = 1; y < VGA_HEIGHT; y++)
        for (u32 x = 0; x < VGA_WIDTH; x++)
            vga_buffer[(y - 1) * VGA_WIDTH + x] = vga_buffer[y * VGA_WIDTH + x];

    /* Clear last line */
    for (u32 x = 0; x < VGA_WIDTH; x++)
        vga_buffer[(VGA_HEIGHT - 1) * VGA_WIDTH + x] = sigma_vga_entry(' ', vga_color);

    vga_row = VGA_HEIGHT - 1;
}

/* ─────────────── Internal: Hardware cursor update ─────────────── */
static void vga_update_cursor() {
    u32 pos = vga_row * VGA_WIDTH + vga_col;
    sigma_outb_vga(0x3D4, 0x0F);
    sigma_outb_vga(0x3D5, (u8)(pos & 0xFF));
    sigma_outb_vga(0x3D4, 0x0E);
    sigma_outb_vga(0x3D5, (u8)((pos >> 8) & 0xFF));
}

/* ─────────────── API: Put single character ─────────────── */
extern "C" void sigma_vga_putchar(char c) {
    if (c == '\n') {
        vga_col = 0;
        vga_row++;
        if (vga_row >= VGA_HEIGHT) vga_scroll();
        vga_update_cursor();
        return;
    }

    if (c == '\r') {
        vga_col = 0;
        vga_update_cursor();
        return;
    }

    if (c == '\b') {
        if (vga_col > 0) {
            vga_col--;
            vga_buffer[vga_row * VGA_WIDTH + vga_col] = sigma_vga_entry(' ', vga_color);
        }
        vga_update_cursor();
        return;
    }

    if (c == '\t') {
        u32 spaces = 8 - (vga_col % 8);
        for (u32 i = 0; i < spaces; i++) sigma_vga_putchar(' ');
        return;
    }

    vga_buffer[vga_row * VGA_WIDTH + vga_col] = sigma_vga_entry((u8)c, vga_color);
    vga_col++;

    if (vga_col >= VGA_WIDTH) {
        vga_col = 0;
        vga_row++;
        if (vga_row >= VGA_HEIGHT) vga_scroll();
    }

    vga_update_cursor();
}

/* ─────────────── API: Print null-terminated string ─────────────── */
extern "C" void sigma_vga_puts(const char* str) {
    while (*str) sigma_vga_putchar(*str++);
}

/* ─────────────── API: Sovereign printf (No libc) ─────────────── */
static void vga_print_hex(u64 val) {
    const char hex_chars[] = "0123456789ABCDEF";
    char buf[19];
    buf[0] = '0'; buf[1] = 'x';
    for (int i = 17; i >= 2; i--) {
        buf[i] = hex_chars[val & 0xF];
        val >>= 4;
    }
    buf[18] = '\0';
    sigma_vga_puts(buf);
}

static void vga_print_uint(u64 val) {
    if (val == 0) { sigma_vga_putchar('0'); return; }
    char buf[21];
    u32 i = 20;
    buf[20] = '\0';
    while (val > 0) {
        buf[--i] = '0' + (val % 10);
        val /= 10;
    }
    sigma_vga_puts(buf + i);
}

extern "C" void sigma_vga_printf(const char* fmt, ...) {
    /* Sovereign variadic argument parsing via GCC built-ins */
    __builtin_va_list args;
    __builtin_va_start(args, fmt);

    while (*fmt) {
        if (*fmt == '%') {
            fmt++;
            switch (*fmt) {
                case 's': sigma_vga_puts(__builtin_va_arg(args, const char*)); break;
                case 'c': sigma_vga_putchar((char)__builtin_va_arg(args, int)); break;
                case 'd': {
                    int v = __builtin_va_arg(args, int);
                    if (v < 0) { sigma_vga_putchar('-'); vga_print_uint((u64)(-v)); }
                    else vga_print_uint((u64)v);
                    break;
                }
                case 'u': vga_print_uint((u64)__builtin_va_arg(args, u32)); break;
                case 'x': case 'X': vga_print_hex((u64)__builtin_va_arg(args, u32)); break;
                case 'l': {
                    fmt++;
                    if (*fmt == 'x' || *fmt == 'X')
                        vga_print_hex(__builtin_va_arg(args, u64));
                    else if (*fmt == 'u')
                        vga_print_uint(__builtin_va_arg(args, u64));
                    break;
                }
                case '%': sigma_vga_putchar('%'); break;
            }
        } else {
            sigma_vga_putchar(*fmt);
        }
        fmt++;
    }

    __builtin_va_end(args);
}
