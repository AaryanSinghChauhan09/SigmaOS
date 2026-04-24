/*
 * =============================================================================
 * Σ SIGMAOS HAL: VGA TEXT/GRAPHICS DRIVER (Contract Implementation)
 * =============================================================================
 * Implements SigmaDisplayOps for legacy VGA (mode 0x13 / text mode 0x03).
 * Selected when SIGMA_DRIVER_VGA is defined (see sigma_features.h).
 * =============================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma/hal_contract.h"
#include "sigma/sigma_features.h"

#if defined(SIGMA_DRIVER_VGA) || !defined(SIGMA_DRIVER_FRAMEBUFFER)

/* ── VGA hardware constants ─────────────────────────────────────────────── */

#define VGA_FRAMEBUFFER   0xA0000
#define VGA_TEXT_BUFFER    0xB8000
#define VGA_WIDTH          320
#define VGA_HEIGHT         200
#define VGA_TEXT_COLS       80
#define VGA_TEXT_ROWS       25

static u32 vga_cur_w = VGA_WIDTH;
static u32 vga_cur_h = VGA_HEIGHT;
static u32 vga_cur_bpp = 8;

/* ── Port I/O (ONLY place that touches hardware ports) ──────────────────── */

static inline void outb(u16 port, u8 val) {
    __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline u8 inb(u16 port) {
    u8 ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* ── Contract implementation ────────────────────────────────────────────── */

static k_status vga_init(u32 width, u32 height, u32 bpp) {
    (void)width; (void)height; (void)bpp;
    /* Set VGA mode 0x13 (320×200, 256 colors) via BIOS int 10h
     * In a real kernel this would program VGA registers directly. */
    vga_cur_w   = VGA_WIDTH;
    vga_cur_h   = VGA_HEIGHT;
    vga_cur_bpp = 8;
    return K_OK;
}

static void vga_put_pixel(u32 x, u32 y, u32 argb) {
    if (x >= vga_cur_w || y >= vga_cur_h) return;
    volatile u8* fb = (volatile u8*)(usize)VGA_FRAMEBUFFER;
    /* Map 32-bit ARGB → 8-bit VGA palette index (simplified) */
    fb[y * vga_cur_w + x] = (u8)(argb & 0xFF);
}

static void vga_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 argb) {
    u32 row, col;
    for (row = y; row < y + h && row < vga_cur_h; row++) {
        for (col = x; col < x + w && col < vga_cur_w; col++) {
            vga_put_pixel(col, row, argb);
        }
    }
}

static void vga_blit(const void* src, u32 x, u32 y, u32 w, u32 h, u32 stride) {
    const u8* s = (const u8*)src;
    u32 row;
    for (row = 0; row < h && (y + row) < vga_cur_h; row++) {
        volatile u8* fb = (volatile u8*)(usize)VGA_FRAMEBUFFER;
        u32 off = (y + row) * vga_cur_w + x;
        u32 col;
        for (col = 0; col < w && (x + col) < vga_cur_w; col++) {
            fb[off + col] = s[row * stride + col];
        }
    }
}

static void vga_swap_buffers(void) {
    /* VGA mode 0x13 is single-buffered; no-op */
}

static void vga_get_resolution(u32* out_w, u32* out_h, u32* out_bpp) {
    if (out_w)   *out_w   = vga_cur_w;
    if (out_h)   *out_h   = vga_cur_h;
    if (out_bpp) *out_bpp = vga_cur_bpp;
}

static k_status vga_set_mode(u32 width, u32 height, u32 bpp) {
    (void)width; (void)height; (void)bpp;
    /* Mode switching would reprogram VGA CRTC registers here */
    return K_OK;
}

/* ── Vtable singleton ───────────────────────────────────────────────────── */

static const SigmaDisplayOps vga_display_ops = {
    .init           = vga_init,
    .put_pixel      = vga_put_pixel,
    .fill_rect      = vga_fill_rect,
    .blit           = vga_blit,
    .swap_buffers   = vga_swap_buffers,
    .get_resolution = vga_get_resolution,
    .set_mode       = vga_set_mode,
};

/* ── Auto-registration (called from hal_init_drivers) ───────────────────── */

void hal_vga_register(void) {
    hal_register_display(&vga_display_ops);
}

#endif /* SIGMA_DRIVER_VGA */
