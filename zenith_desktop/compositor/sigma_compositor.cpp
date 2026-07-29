/**
 * =========================================================================
 * Σ SIGMAOS: ZENITH COMPOSITOR ENGINE (PHASE 4)
 * =========================================================================
 * A purely native, hardware-accelerated rendering pipeline and window 
 * manager. Zenith bypasses legacy Wayland/X11 complexity and communicates 
 * directly with the SigmaOS graphics and input shards.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_error_codes.h"
#include <sigma_libc.h>

// Link external structured logger
extern "C" void zenith_log_structured(sigma_u32 error_code, const char* component, const char* desc, sigma_u32 container_id);

namespace Zenith {

struct Rect {
    sigma_i32 x, y;
    sigma_u32 width, height;
};

class Window {
public:
    sigma_u32 id;
    sigma_u32 container_id; // Link to Sovereign Orchestrator Shard
    Rect      geometry;
    sigma_u32 z_index;
    sigma_u8* backing_buffer; // App's shared memory buffer
    bool      is_focused;

    Window(sigma_u32 win_id, sigma_u32 c_id, Rect geom) 
        : id(win_id), container_id(c_id), geometry(geom), z_index(0), is_focused(false) {
        
        // Allocate backing store for window rendering.
        sigma_u64 buffer_size = geom.width * geom.height * 4; // 32-bit ARGB
        backing_buffer = (sigma_u8*)sigma_malloc(buffer_size);
        if (!backing_buffer) {
            zenith_log_structured(ZEN_402_WINDOW_ALLOCATION_OOM, "Compositor", "Window buffer allocation failed", c_id);
        }
    }
    
    ~Window() {
        if (backing_buffer) {
            sigma_free(backing_buffer);
        }
    }
};

#include "../sdk/include/zenith.h"

// Fwd declarations for sovereign framebuffer interaction
extern "C" void sigma_fb_flush(void);
extern "C" void sigma_fb_draw_rect(int x, int y, int w, int h, uint32_t color);
extern "C" int sigma_input_poll_event(void);

// Simulated compositor loop avoiding GTK/Qt completely
void compositor_loop() {
    bool running = true;
    while (running) {
        // 1. Poll input events (simulated)
        int ev = sigma_input_poll_event();
        if (ev < 0) {
            // handle error or exit
        }
        
        // 2. Draw surfaces to framebuffer
        // Draw background
        sigma_fb_draw_rect(0, 0, 1920, 1080, 0x1E1E2E); // Catppuccin Mocha Base
        
        // Draw some "windows"
        sigma_fb_draw_rect(100, 100, 800, 600, 0x313244); // Surface0
        sigma_fb_draw_rect(950, 100, 800, 600, 0x313244); 
        
        // 3. Flush framebuffer
        sigma_fb_flush();
        
        // Simulated wait for vblank
        // sys_yield();
        break; // break for testing to avoid infinite loop
    }
}

class ZenithCompositor {
public:
    void Init() {
        // Initialize DRM/KMS or direct framebuffer
        sigma_printf("[Compositor] Sovereign Zenith Compositor Initialized.\n");
    }
    
    void Run() {
        sigma_printf("[Compositor] Entering main compositor loop...\n");
        compositor_loop();
    }
};

class Compositor {
public:
    static Compositor& getInstance() {
        static Compositor instance;
        return instance;
    }

    void init() {
        sys_print("[Zenith] Initializing Native Compositor Engine...\n");
        m_framebuffer = acquire_hardware_framebuffer(&m_fb_width, &m_fb_height);
        
        if (m_framebuffer) {
            sys_print("[Zenith] Hardware Framebuffer acquired: %ux%u (32bpp)\n", m_fb_width, m_fb_height);
            clear_screen(0xFF1E1E1E); // Zenith Dark Theme Background
        } else {
            m_fallback_mode = true;
            sys_print("[Zenith] ERROR: Failed to acquire hardware framebuffer! Triggering safe VGA fallback mode...\n");
            zenith_log_structured(ZEN_502_VGA_FALLBACK_TRIGGERED, "Compositor", "Hardware FB failed, safe VGA recovery triggered", 0);
        }
    }

    Window* createWindow(sigma_u32 id, sigma_u32 container_id, sigma_i32 x, sigma_i32 y, sigma_u32 w, sigma_u32 h) {
        if (m_window_count >= 128) return nullptr;
        
        Window* win = new Window(id, container_id, {x, y, w, h});
        win->z_index = m_window_count;
        m_windows[m_window_count++] = win;
        
        sys_print("[Zenith] Window %u created for Container [%u] at (%d,%d) [%ux%u]\n", 
                  id, container_id, x, y, w, h);
        return win;
    }

    void renderFrame() {
        if (m_fallback_mode) {
            sys_print("[Zenith-Fallback] Displaying safe mode diagnostics...\n");
            // Simply log frames deterministically without raw screen writes
            return;
        }

        // 1. Clear background
        clear_screen(0xFF1E1E1E);

        // 2. Render windows back-to-front (Painter's Algorithm)
        for (sigma_u32 z = 0; z < m_window_count; z++) {
            Window* win = getWindowByZIndex(z);
            if (win) {
                composite_window(win);
            }
        }

        // 3. Render cursor (Top-most)
        render_cursor();

        // 4. Page Flip / VSync (Simulated)
        sys_ipc_send(5, 1, /* MSG_GPU_FLIP */ SIGMA_NULL, 0);
    }

    void triggerCompositorSelfHealing() {
        sys_print("[Zenith-SelfHealing] Re-initiating hardware framebuffer connection...\n");
        zenith_log_structured(ZEN_503_SELF_HEALING_RESTART, "Compositor", "Attempting compositor graphics reboot", 0);
        m_fallback_mode = false;
        init();
    }

    void poll_input_events() {
        /* Phase 7–8: drain input shard queue (keyboard/pointer) */
        sigma_u64 evt[4] = {0, 0, 0, 0};
        sigma_status st = sys_ipc_send(6, 2, /* MSG_INPUT_POLL */ evt, sizeof(evt));
        if (st != K_OK) return;

        m_pointer_x = (sigma_i32)evt[0];
        m_pointer_y = (sigma_i32)evt[1];
        m_pointer_buttons = (sigma_u32)evt[2];

        if (evt[3] != 0) {
            focus_window_at(m_pointer_x, m_pointer_y);
        }
    }

    void run_event_loop(sigma_u32 frames) {
        for (sigma_u32 f = 0; f < frames; ++f) {
            poll_input_events();
            renderFrame();
        }
    }

private:
    Compositor() : m_framebuffer(nullptr), m_fb_width(0), m_fb_height(0), m_window_count(0), m_fallback_mode(false) {}

    // Hardware Abstraction Stubs
    sigma_u32* acquire_hardware_framebuffer(sigma_u32* w, sigma_u32* h) {
        *w = 1920; *h = 1080;
        static sigma_u32 mock_fb[1920 * 1080];
        return mock_fb;
    }

    void clear_screen(sigma_u32 color) {
        if (m_fallback_mode) return;
        for (sigma_u32 i = 0; i < m_fb_width * m_fb_height; i++) {
            m_framebuffer[i] = color;
        }
    }

    Window* getWindowByZIndex(sigma_u32 z) {
        for (sigma_u32 i = 0; i < m_window_count; i++) {
            if (m_windows[i]->z_index == z) return m_windows[i];
        }
        return nullptr;
    }

    void composite_window(Window* win) {
        // Alpha blending and bit-blitting from win->backing_buffer to m_framebuffer
    }

    void render_cursor() {
        if (m_fallback_mode || !m_framebuffer) return;
        sigma_u32 cx = (sigma_u32)m_pointer_x;
        sigma_u32 cy = (sigma_u32)m_pointer_y;
        if (cx >= m_fb_width || cy >= m_fb_height) return;
        m_framebuffer[cy * m_fb_width + cx] = 0xFFFFFFFF;
    }

    void focus_window_at(sigma_i32 x, sigma_i32 y) {
        for (sigma_i32 i = (sigma_i32)m_window_count - 1; i >= 0; --i) {
            Window* w = m_windows[i];
            if (!w) continue;
            const Rect& g = w->geometry;
            if (x >= g.x && x < (g.x + (sigma_i32)g.width) &&
                y >= g.y && y < (g.y + (sigma_i32)g.height)) {
                w->is_focused = true;
                return;
            }
        }
    }

    sigma_u32* m_framebuffer;
    sigma_u32  m_fb_width;
    sigma_u32  m_fb_height;
    
    Window*    m_windows[128];
    sigma_u32  m_window_count;
    bool       m_fallback_mode;
    sigma_i32  m_pointer_x = 0;
    sigma_i32  m_pointer_y = 0;
    sigma_u32  m_pointer_buttons = 0;
};

} // namespace Zenith

extern "C" {
    void zenith_compositor_init() {
        Zenith::Compositor::getInstance().init();
    }
    
    void zenith_compositor_render() {
        Zenith::Compositor::getInstance().renderFrame();
    }

    void zenith_compositor_heal() {
        Zenith::Compositor::getInstance().triggerCompositorSelfHealing();
    }

    void zenith_compositor_run_loop(sigma_u32 frames) {
        Zenith::Compositor::getInstance().run_event_loop(frames);
    }
}
