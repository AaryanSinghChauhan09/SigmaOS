#include "libc/sigma_libc.h"
#include <stdint.h>

namespace SigmaOS {
namespace HAL {
namespace Video {

// Track 1: Hardware Abstraction Layer - GPU/Video
class VESAFramebuffer {
private:
    uint32_t* buffer;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;

public:
    VESAFramebuffer() : buffer(nullptr), width(0), height(0), pitch(0) {}

    void init(uint32_t* fb_addr, uint32_t w, uint32_t h, uint32_t p) {
        buffer = fb_addr;
        width = w;
        height = h;
        pitch = p;
        sigma_log("[HAL-VIDEO] VESA Framebuffer Initialized.");
        sigma_print("[HAL-VIDEO] Resolution: ");
        sigma_print_num(width);
        sigma_print("x");
        sigma_print_num(height);
        sigma_print("\n");
    }

    void put_pixel(uint32_t x, uint32_t y, uint32_t color) {
        if (x >= width || y >= height || !buffer) return;
        buffer[y * (pitch / 4) + x] = color;
    }

    void clear(uint32_t color) {
        if (!buffer) return;
        for (uint32_t y = 0; y < height; y++) {
            for (uint32_t x = 0; x < width; x++) {
                buffer[y * (pitch / 4) + x] = color;
            }
        }
    }
};

} // namespace Video
} // namespace HAL
} // namespace SigmaOS
