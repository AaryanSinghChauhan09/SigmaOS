/**
 * SigmaOS: Sovereign GPU/VBE Reference Driver
 * USP: Direct silicon pixel sharding for Zenith UI.
 */

#include "libc/sigma_libc.h"

typedef struct {
    uint16_t attributes;
    uint8_t win_a, win_b;
    uint16_t granularity;
    uint16_t winsize;
    uint16_t segment_a, segment_b;
    uint32_t real_mode_ptr;
    uint16_t pitch;
    uint16_t width, height;
    uint8_t w_char, y_char, planes, bpp, banks;
    uint8_t memory_model, bank_size, image_pages;
    uint8_t reserved0;
    uint8_t red_mask, red_position;
    uint8_t green_mask, green_position;
    uint8_t blue_mask, blue_position;
    uint8_t rsv_mask, rsv_position;
    uint8_t direct_color_attributes;
    uint32_t framebuffer;
} __attribute__((packed)) vbe_mode_info_t;

void sigma_gpu_draw_pixel(vbe_mode_info_t *info, int x, int y, uint32_t color) {
    uint32_t *fb = (uint32_t *)(uintptr_t)info->framebuffer;
    fb[y * info->width + x] = color;
}
