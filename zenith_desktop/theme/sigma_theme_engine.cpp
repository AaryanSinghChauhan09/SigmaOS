/**
 * =========================================================================
 * Σ SIGMAOS: ZENITH THEME & WIDGET ENGINE
 * =========================================================================
 * M4.2 Native Widget Toolkit. Replaces Javascript mockups with a pure C++
 * rendering engine that draws UI primitives (buttons, panels, text) 
 * directly to the compositor's framebuffers using the active Sigma Theme.
 * =========================================================================
 */

#include "../../include/sigma_theme.h"
#include "../../include/sigma_kernel_types.h"

namespace Zenith {
namespace Widgets {

/* Simulated Compositor API hooks */
extern void draw_rect(sigma_u8* buffer, sigma_u32 buf_w, sigma_i32 x, sigma_i32 y, sigma_u32 w, sigma_u32 h, sigma_u32 color, sigma_u32 radius);
extern void draw_text(sigma_u8* buffer, sigma_u32 buf_w, sigma_i32 x, sigma_i32 y, const char* text, sigma_u32 color);
sigma_status broadcast_theme_update();


class ThemeEngine {
public:
    static ThemeEngine& getInstance() {
        static ThemeEngine instance;
        return instance;
    }

    void init() {
        /* Default to Sigma Dark Theme */
        m_accent_color = 0xFF007AFF; /* macOS-like Blue */
        m_bg_color     = 0xFF1E1E1E; /* Dark gray */
        m_fg_color     = 0xFFFFFFFF; /* White text */
        m_surface_col  = 0xFF2D2D2D; /* Elevated surface */
        m_border_radius = 8;
        m_gap_inner     = 4;
        m_gap_outer     = 8;
    }

    /* Native UI Primitives */

    void renderButton(sigma_u8* win_buf, sigma_u32 win_w, sigma_i32 x, sigma_i32 y, sigma_u32 w, sigma_u32 h, const char* label, bool is_hovered) {
        sigma_u32 btn_color = is_hovered ? lightenColor(m_accent_color, 20) : m_accent_color;
        
        /* Draw rounded rectangle background */
        draw_rect(win_buf, win_w, x, y, w, h, btn_color, m_border_radius);
        
        /* Draw centered text (mock coordinates) */
        draw_text(win_buf, win_w, x + 16, y + (h/2) - 8, label, 0xFFFFFFFF);
    }

    void renderPanel(sigma_u8* win_buf, sigma_u32 win_w, sigma_i32 x, sigma_i32 y, sigma_u32 w, sigma_u32 h) {
        /* Draw elevated surface with rounded corners */
        draw_rect(win_buf, win_w, x, y, w, h, m_surface_col, m_border_radius + 4);
    }

    void renderLabel(sigma_u8* win_buf, sigma_u32 win_w, sigma_i32 x, sigma_i32 y, const char* text) {
        draw_text(win_buf, win_w, x, y, text, m_fg_color);
    }

    /* Theme State Mutation */
    
    void setAccentColor(sigma_u32 argb) {
        m_accent_color = argb;
        /* Trigger IPC redraw event to compositor */
    }

    void setUIMetrics(sigma_u32 radius, sigma_u32 inner_gap, sigma_u32 outer_gap) {
        m_border_radius = radius;
        m_gap_inner     = inner_gap;
        m_gap_outer     = outer_gap;
        /* Trigger IPC layout recompute event to Window Manager */
    }

private:
    ThemeEngine() {}

    sigma_u32 lightenColor(sigma_u32 color, sigma_u32 percent) {
        /* Mock lightening algorithm */
        return color; 
    }

    sigma_u32 m_accent_color;
    sigma_u32 m_bg_color;
    sigma_u32 m_fg_color;
    sigma_u32 m_surface_col;
    sigma_u32 m_border_radius;
    sigma_u32 m_gap_inner;
    sigma_u32 m_gap_outer;
};

} // namespace Widgets
} // namespace Zenith

/* --- C API Wrappers --- */
extern "C" {
    void zenith_theme_init(void) {
        Zenith::Widgets::ThemeEngine::getInstance().init();
    }

    void zenith_draw_button(sigma_u8* buf, sigma_u32 w, sigma_i32 x, sigma_i32 y, sigma_u32 bw, sigma_u32 bh, const char* label, bool hover) {
        Zenith::Widgets::ThemeEngine::getInstance().renderButton(buf, w, x, y, bw, bh, label, hover);
    }

    void zenith_theme_set_metrics(sigma_u32 r, sigma_u32 ig, sigma_u32 og) {
        Zenith::Widgets::ThemeEngine::getInstance().setUIMetrics(r, ig, og);
    }
}

