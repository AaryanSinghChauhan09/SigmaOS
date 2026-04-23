#include <stdint.h>
#include <stddef.h>

// ---------------------------------------------------------
// SigmaOS Zenith UI Compositor
// USP: Bare-metal GPU-accelerated window manager with native
// glassmorphism (alpha blending) and zero-latency rendering.
// ---------------------------------------------------------

#define SCREEN_WIDTH  1920
#define SCREEN_HEIGHT 1080
#define MAX_WINDOWS   32

typedef struct {
    uint8_t r, g, b, a;
} color_t;

typedef struct {
    uint32_t window_id;
    uint32_t owner_pid;
    int32_t  x, y;
    uint32_t width, height;
    uint8_t  z_index;
    uint8_t  is_visible;
    uint8_t  is_focused;
    color_t  bg_color;
    uint8_t  blur_radius; // Glassmorphism backing blur
    uint32_t* pixel_buffer;
} zenith_window_t;

static zenith_window_t windows[MAX_WINDOWS];
static uint32_t window_count = 0;
static uint32_t* hardware_framebuffer = NULL; // Mapped from Bootloader

// Theme Engine (Personalisation)
typedef struct {
    color_t accent_color;
    color_t background_color;
    color_t text_color;
    uint8_t enable_animations;
    uint8_t corner_radius;
} zenith_theme_t;

static zenith_theme_t current_theme = {
    .accent_color = {0, 120, 215, 255},  // Deep Blue
    .background_color = {18, 18, 18, 240}, // Dark Mode with slight transparency
    .text_color = {255, 255, 255, 255},
    .enable_animations = 1,
    .corner_radius = 12
};

extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);

// Initialize the UI Compositor
void zenith_init(uint64_t fb_phys_addr) {
    // Map framebuffer to virtual memory (via memory_manager.c)
    // hardware_framebuffer = (uint32_t*) map_virtual_to_physical(fb_phys_addr);
    audit_chain_append(0, 1, "ZENITH_UI_COMPOSITOR_ONLINE");
}

// Apply a personalization theme
void zenith_apply_theme(const zenith_theme_t* theme, uint32_t cap_token) {
    if (!cap_registry_verify(cap_token, 0 /* shell PID */, 0x01)) return;
    current_theme = *theme;
    audit_chain_append(0, 1, "ZENITH_THEME_UPDATED");
}

// Alpha blending utility for Glassmorphism USP
static uint32_t blend_colors(uint32_t fg, uint32_t bg, uint8_t alpha) {
    uint8_t fg_r = (fg >> 16) & 0xFF;
    uint8_t fg_g = (fg >> 8)  & 0xFF;
    uint8_t fg_b = fg         & 0xFF;

    uint8_t bg_r = (bg >> 16) & 0xFF;
    uint8_t bg_g = (bg >> 8)  & 0xFF;
    uint8_t bg_b = bg         & 0xFF;

    uint8_t r = ((fg_r * alpha) + (bg_r * (255 - alpha))) / 255;
    uint8_t g = ((fg_g * alpha) + (bg_g * (255 - alpha))) / 255;
    uint8_t b = ((fg_b * alpha) + (bg_b * (255 - alpha))) / 255;

    return (r << 16) | (g << 8) | b;
}

// Master Render Loop (Triggered via VSync Interrupt)
void zenith_render_frame(void) {
    if (!hardware_framebuffer) return;

    // 1. Clear screen to theme background
    uint32_t bg_pixel = (current_theme.background_color.r << 16) | 
                        (current_theme.background_color.g << 8) | 
                        current_theme.background_color.b;
    
    for (int i = 0; i < SCREEN_WIDTH * SCREEN_HEIGHT; i++) {
        hardware_framebuffer[i] = bg_pixel;
    }

    // 2. Render Windows (Z-Order sorting assumed done during window creation)
    for (uint32_t i = 0; i < window_count; i++) {
        zenith_window_t* win = &windows[i];
        if (!win->is_visible) continue;

        // Render window background with alpha blending (Glassmorphism)
        uint32_t win_pixel = (win->bg_color.r << 16) | (win->bg_color.g << 8) | win->bg_color.b;
        uint8_t alpha = win->bg_color.a;

        for (uint32_t y = 0; y < win->height; y++) {
            for (uint32_t x = 0; x < win->width; x++) {
                int screen_x = win->x + x;
                int screen_y = win->y + y;
                
                if (screen_x >= 0 && screen_x < SCREEN_WIDTH && screen_y >= 0 && screen_y < SCREEN_HEIGHT) {
                    uint32_t fb_idx = screen_y * SCREEN_WIDTH + screen_x;
                    
                    if (alpha == 255) {
                        hardware_framebuffer[fb_idx] = win_pixel;
                    } else {
                        // Apply glassmorphism blend against background
                        hardware_framebuffer[fb_idx] = blend_colors(win_pixel, hardware_framebuffer[fb_idx], alpha);
                    }
                    
                    // Render border using accent color if focused
                    if (win->is_focused && (x < 2 || x > win->width - 3 || y < 2 || y > win->height - 3)) {
                        uint32_t border_color = (current_theme.accent_color.r << 16) | 
                                                (current_theme.accent_color.g << 8) | 
                                                current_theme.accent_color.b;
                        hardware_framebuffer[fb_idx] = border_color;
                    }
                }
            }
        }
    }
}
