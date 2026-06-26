/*
 * Σ SigmaOS Zenith — Zenith Window Manager Stub
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

struct sigma_window {
    u32 id;
    u32 x, y, width, height;
    u32 bg_color;
    bool active;
};

#define MAX_WINDOWS 32
static struct sigma_window windows[MAX_WINDOWS];
static u32 window_count = 0;

extern "C" void sigma_compositor_draw_rect(u32 x, u32 y, u32 width, u32 height, u32 color);
extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" u32 sigma_wm_create_window(u32 x, u32 y, u32 width, u32 height, u32 color) {
    if (window_count >= MAX_WINDOWS) return 0;
    
    u32 id = window_count + 1;
    windows[window_count].id = id;
    windows[window_count].x = x;
    windows[window_count].y = y;
    windows[window_count].width = width;
    windows[window_count].height = height;
    windows[window_count].bg_color = color;
    windows[window_count].active = true;
    
    window_count++;
    sigma_vga_printf("WM: Created window %u\n", id);
    return id;
}

extern "C" void sigma_wm_render_all() {
    for (u32 i = 0; i < window_count; i++) {
        if (windows[i].id != 0) {
            sigma_compositor_draw_rect(
                windows[i].x, 
                windows[i].y, 
                windows[i].width, 
                windows[i].height, 
                windows[i].bg_color
            );
        }
    }
}
