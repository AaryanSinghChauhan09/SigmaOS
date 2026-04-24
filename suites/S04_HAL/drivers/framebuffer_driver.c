/*
 * =============================================================================
 * Σ SIGMAOS HAL: UEFI GOP FRAMEBUFFER DRIVER (Contract Implementation)
 * =============================================================================
 * Implements SigmaDisplayOps for a linear UEFI framebuffer.
 * Selected when SIGMA_DRIVER_FRAMEBUFFER is defined.
 *
 * At boot the UEFI stub passes the GOP framebuffer address, width, height,
 * and pitch.  This driver uses that information for pixel-perfect rendering.
 * =============================================================================
 */

#include "sigma_kernel_types.h"
#include "sigma/hal_contract.h"
#include "sigma/sigma_features.h"

#ifdef SIGMA_DRIVER_FRAMEBUFFER

/* ── Framebuffer state (populated by UEFI bootloader) ───────────────────── */

static volatile u32* g_fb_base  = (void*)0;
static u32 g_fb_width   = 0;
static u32 g_fb_height  = 0;
static u32 g_fb_pitch   = 0;   /* bytes per scanline */
static u32 g_fb_bpp     = 32;

/* Back-buffer for double-buffering */
static u32* g_back_buffer = (void*)0;

/* ── External: set by UEFI boot stub ────────────────────────────────────── */

extern void* uefi_gop_framebuffer_base;
extern u32   uefi_gop_width;
extern u32   uefi_gop_height;
extern u32   uefi_gop_pitch;

/* ── Contract implementation ────────────────────────────────────────────── */

static k_status fb_init(u32 width, u32 height, u32 bpp) {
    (void)width; (void)height; (void)bpp;
    g_fb_base   = (volatile u32*)uefi_gop_framebuffer_base;
    g_fb_width  = uefi_gop_width;
    g_fb_height = uefi_gop_height;
    g_fb_pitch  = uefi_gop_pitch;
    g_fb_bpp    = 32;

    /* Allocate back-buffer (requires vmalloc from S05_Memory) */
    extern vaddr_t vmalloc(u64 npages);
    u64 fb_size  = (u64)g_fb_pitch * g_fb_height;
    u64 pages    = (fb_size + PAGE_SIZE - 1) / PAGE_SIZE;
    g_back_buffer = (u32*)vmalloc(pages);

    return g_fb_base ? K_OK : K_ERR_NODEV;
}

static void fb_put_pixel(u32 x, u32 y, u32 argb) {
    if (x >= g_fb_width || y >= g_fb_height) return;
    u32* target = g_back_buffer ? g_back_buffer : (u32*)g_fb_base;
    target[y * (g_fb_pitch / 4) + x] = argb;
}

static void fb_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 argb) {
    u32 row, col;
    u32* target = g_back_buffer ? g_back_buffer : (u32*)g_fb_base;
    u32 stride  = g_fb_pitch / 4;
    for (row = y; row < y + h && row < g_fb_height; row++) {
        for (col = x; col < x + w && col < g_fb_width; col++) {
            target[row * stride + col] = argb;
        }
    }
}

static void fb_blit(const void* src, u32 x, u32 y, u32 w, u32 h, u32 stride) {
    const u32* s = (const u32*)src;
    u32 dst_stride = g_fb_pitch / 4;
    u32* target = g_back_buffer ? g_back_buffer : (u32*)g_fb_base;
    u32 row, col;
    for (row = 0; row < h && (y + row) < g_fb_height; row++) {
        for (col = 0; col < w && (x + col) < g_fb_width; col++) {
            target[(y + row) * dst_stride + (x + col)] = s[row * (stride / 4) + col];
        }
    }
}

static void fb_swap_buffers(void) {
    if (!g_back_buffer || !g_fb_base) return;
    u64 fb_size = (u64)g_fb_pitch * g_fb_height;
    const u8* src = (const u8*)g_back_buffer;
    volatile u8* dst = (volatile u8*)g_fb_base;
    u64 i;
    for (i = 0; i < fb_size; i++) {
        dst[i] = src[i];
    }
}

static void fb_get_resolution(u32* out_w, u32* out_h, u32* out_bpp) {
    if (out_w)   *out_w   = g_fb_width;
    if (out_h)   *out_h   = g_fb_height;
    if (out_bpp) *out_bpp = g_fb_bpp;
}

static k_status fb_set_mode(u32 width, u32 height, u32 bpp) {
    /* UEFI GOP mode switching would go here */
    (void)width; (void)height; (void)bpp;
    return K_OK;
}

/* ── Vtable singleton ───────────────────────────────────────────────────── */

static const SigmaDisplayOps fb_display_ops = {
    .init           = fb_init,
    .put_pixel      = fb_put_pixel,
    .fill_rect      = fb_fill_rect,
    .blit           = fb_blit,
    .swap_buffers   = fb_swap_buffers,
    .get_resolution = fb_get_resolution,
    .set_mode       = fb_set_mode,
};

/* ── Auto-registration ──────────────────────────────────────────────────── */

void hal_framebuffer_register(void) {
    hal_register_display(&fb_display_ops);
}

#endif /* SIGMA_DRIVER_FRAMEBUFFER */
