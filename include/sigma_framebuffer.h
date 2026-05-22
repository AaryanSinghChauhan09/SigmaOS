#ifndef SIGMA_FRAMEBUFFER_H
#define SIGMA_FRAMEBUFFER_H

#include <stdint.h>

#define SIGMA_IOCTL_FB_GET_INFO 0x4601
#define SIGMA_IOCTL_FB_SET_RES  0x4602

struct sigma_fb_info {
    uint32_t width;
    uint32_t height;
    uint32_t bpp;
    uint32_t pitch;
    uint64_t phys_addr;
};

// Framebuffer driver API
int sigma_fb_init(void);
struct sigma_fb_info* sigma_fb_get_info(void);
void* sigma_fb_mmap(void);

#endif // SIGMA_FRAMEBUFFER_H
