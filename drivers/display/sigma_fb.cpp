/*
 * Σ SigmaOS — sigma_fb: Sovereign Linear Framebuffer Driver
 * Zero-Dependency: No X11, no Wayland, no Mesa.
 * Absorbs: Linux drivers/video/fbdev/ architecture, VESA VBE mode setting.
 * Implements: Pixel plotting, rect fill, basic font rendering on a linear framebuffer.
 */

typedef unsigned int   u32;
typedef unsigned short u16;
typedef unsigned char  u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

/* Framebuffer state (typically passed by bootloader via multiboot info) */
struct FramebufferInfo {
    u32* address;     /* Pointer to pixel memory */
    u32  width;
    u32  height;
    u32  pitch;       /* Bytes per scanline */
    u8   bpp;         /* Bits per pixel (32 expected) */
};

static FramebufferInfo fb;

/* Initialize with values from bootloader */
extern "C" void sigma_fb_init(u32* addr, u32 w, u32 h, u32 pitch, u8 bpp) {
    fb.address = addr;
    fb.width = w;
    fb.height = h;
    fb.pitch = pitch;
    fb.bpp = bpp;
    sigma_vga_printf("[FB] Framebuffer %ux%u @%ubpp initialized\n", w, h, bpp);
}

/* Plot a single pixel */
extern "C" void fb_put_pixel(u32 x, u32 y, u32 color) {
    if (x < fb.width && y < fb.height) {
        u32* row = (u32*)((u8*)fb.address + y * fb.pitch);
        row[x] = color;
    }
}

/* Fill a rectangle */
extern "C" void fb_draw_rect(u32 x, u32 y, u32 w, u32 h, u32 color) {
    for (u32 row = y; row < y + h && row < fb.height; row++)
        for (u32 col = x; col < x + w && col < fb.width; col++)
            fb_put_pixel(col, row, color);
}

/* Minimal 8x8 bitmap font (ASCII 32-127, only space and a few chars stubbed) */
static const u8 font_8x8_basic[96][8] = {
    /* ' ' (32) */ {0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00},
    /* '!' (33) */ {0x18,0x18,0x18,0x18,0x18,0x00,0x18,0x00},
    /* '"' (34) */ {0x6C,0x6C,0x00,0x00,0x00,0x00,0x00,0x00},
    /* '#' (35) */ {0x6C,0xFE,0x6C,0x6C,0xFE,0x6C,0x00,0x00},
    /* '$' (36) */ {0x18,0x7E,0x58,0x7E,0x1A,0x7E,0x18,0x00},
    /* '%' (37) */ {0x00,0x62,0x64,0x08,0x10,0x26,0x46,0x00},
    /* '&' (38) */ {0x38,0x6C,0x38,0x76,0xDC,0xCC,0x76,0x00}
};

/* Draw a single character at pixel position */
extern "C" void fb_draw_char(u32 x, u32 y, char c, u32 color) {
    if (c < 32 || c > 127) return;
    const u8* glyph = font_8x8_basic[c - 32];
    for (int row = 0; row < 8; row++)
        for (int col = 0; col < 8; col++)
            if (glyph[row] & (0x80 >> col))
                fb_put_pixel(x + col, y + row, color);
}

/* Draw a text string */
extern "C" void fb_draw_text(u32 x, u32 y, const char* text, u32 color) {
    u32 cx = x;
    while (*text) {
        if (*text == '\n') {
            y += 10; cx = x;
        } else {
            fb_draw_char(cx, y, *text, color);
            cx += 8;
        }
        text++;
    }
}
