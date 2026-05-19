/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: FRAMEBUFFER DEVICE (FBDEV)
 * =============================================================================
 * Inspired by: Linux kernel drivers/video/fbdev/core/fbmem.c
 *              FreeBSD sys/dev/fb/fbd.c
 * =============================================================================
 * Provides a standardized pixel plotting interface over VESA/UEFI GOP LFB.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define FB_MAX_DEVICES 4

typedef struct {
    sigma_u32  xres;
    sigma_u32  yres;
    sigma_u32  bpp;         /* Bits per pixel */
    sigma_u32  pitch;       /* Bytes per scanline */
    sigma_u64  phys_addr;
    sigma_u8*  virt_addr;
    sigma_bool active;
    char       id[16];
} sigma_fb_info_t;

static sigma_fb_info_t fb_devices[FB_MAX_DEVICES];
static sigma_u8 dummy_lfb[1024 * 768 * 4]; /* Simulated 1024x768x32 framebuffer */

void fbdev_init(void) {
    sigma_memset(fb_devices, 0, sizeof(fb_devices));
    sigma_printf("[fbdev] Framebuffer subsystem initialized\n");
}

int fbdev_register(const char* id, sigma_u32 xres, sigma_u32 yres, sigma_u32 bpp, sigma_u32 pitch, sigma_u64 phys_addr) {
    for (sigma_u32 i = 0; i < FB_MAX_DEVICES; i++) {
        if (!fb_devices[i].active) {
            sigma_strcpy(fb_devices[i].id, id, 16);
            fb_devices[i].xres      = xres;
            fb_devices[i].yres      = yres;
            fb_devices[i].bpp       = bpp;
            fb_devices[i].pitch     = pitch;
            fb_devices[i].phys_addr = phys_addr;
            
            /* Simulate memory mapping the physical LFB */
            if (phys_addr == 0xDEADBEEF) {
                fb_devices[i].virt_addr = dummy_lfb;
            } else {
                fb_devices[i].virt_addr = SIGMA_NULL; /* Requires real mmap */
            }
            
            fb_devices[i].active = SIGMA_TRUE;
            
            sigma_printf("[fbdev] Registered /dev/fb%d: %s (%dx%dx%d)\n", 
                         i, id, xres, yres, bpp);
            return (int)i;
        }
    }
    sigma_printf("[fbdev] ERR: Max framebuffer devices reached\n");
    return -1;
}

void fbdev_put_pixel(sigma_u32 fb_id, sigma_u32 x, sigma_u32 y, sigma_u32 color) {
    if (fb_id >= FB_MAX_DEVICES || !fb_devices[fb_id].active) return;
    
    sigma_fb_info_t* fb = &fb_devices[fb_id];
    if (x >= fb->xres || y >= fb->yres || !fb->virt_addr) return;
    
    if (fb->bpp == 32) {
        sigma_u32* pixel = (sigma_u32*)(fb->virt_addr + (y * fb->pitch) + (x * 4));
        *pixel = color;
    } else if (fb->bpp == 16) {
        sigma_u16* pixel = (sigma_u16*)(fb->virt_addr + (y * fb->pitch) + (x * 2));
        *pixel = (sigma_u16)color; /* Requires 565 conversion in reality */
    }
}

void fbdev_fill_rect(sigma_u32 fb_id, sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h, sigma_u32 color) {
    if (fb_id >= FB_MAX_DEVICES || !fb_devices[fb_id].active) return;
    sigma_fb_info_t* fb = &fb_devices[fb_id];
    
    if (x >= fb->xres || y >= fb->yres || !fb->virt_addr) return;
    if (x + w > fb->xres) w = fb->xres - x;
    if (y + h > fb->yres) h = fb->yres - y;
    
    /* Optimized fill for 32bpp */
    if (fb->bpp == 32) {
        for (sigma_u32 cur_y = y; cur_y < y + h; cur_y++) {
            sigma_u32* row = (sigma_u32*)(fb->virt_addr + (cur_y * fb->pitch) + (x * 4));
            for (sigma_u32 cur_x = 0; cur_x < w; cur_x++) {
                row[cur_x] = color;
            }
        }
    }
}

void fbdev_clear(sigma_u32 fb_id, sigma_u32 color) {
    if (fb_id >= FB_MAX_DEVICES || !fb_devices[fb_id].active) return;
    sigma_fb_info_t* fb = &fb_devices[fb_id];
    fbdev_fill_rect(fb_id, 0, 0, fb->xres, fb->yres, color);
}
