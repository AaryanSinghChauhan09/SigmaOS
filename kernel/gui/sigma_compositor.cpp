/*
 * Σ SigmaOS Zenith — Zenith GUI Compositor Stub
 * Zero-Dependency: No libc, no Wayland/X11.
 */

typedef unsigned char  u8;
typedef unsigned short u16;
typedef unsigned int   u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Linear Framebuffer Base (assuming initialized by Bootloader/VBE)
static u32* lfb = (u32*)0xFD000000;
static u32  fb_width = 1024;
static u32  fb_height = 768;

extern "C" void sigma_compositor_init() {
    sigma_vga_printf("Zenith Compositor: Initializing hardware renderer...\n");
    // Switch from VGA text mode to VBE graphics mode if available
}

extern "C" void sigma_compositor_draw_rect(u32 x, u32 y, u32 width, u32 height, u32 color) {
    if (!lfb) return;
    for (u32 j = y; j < y + height && j < fb_height; j++) {
        for (u32 i = x; i < x + width && i < fb_width; i++) {
            lfb[j * fb_width + i] = color;
        }
    }
}

extern "C" void sigma_compositor_flip() {
    // Double buffering logic would go here
}
