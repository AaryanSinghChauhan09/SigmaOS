/**
 * =========================================================================
 * Σ SIGMAOS: ZENITH DESKTOP ENVIRONMENT
 * =========================================================================
 * Implements the spatial UI dock, top panel, and theme engine.
 * =========================================================================
 */

#include "../include/sigma_kernel_types.h"
#include "../include/sigma_log.h"
#include "../include/ui/sigma_wm.h"
#include "../include/ui/zenith_desktop.h"

namespace SigmaOS {
namespace UI {

class ZenithDesktopEnvironment {
public:
    static ZenithDesktopEnvironment& getInstance() {
        static ZenithDesktopEnvironment instance;
        return instance;
    }

    void init() {
        m_state.is_blur_enabled = SIGMA_TRUE;
        m_state.current_theme = THEME_DARK;

        sigma_log("[Zenith] Desktop Environment initializing...");
        
        /* Create the Top Panel (Status bar) */
        m_state.panel_win_id = wm_create_window(0, "ZenithPanel", 0, 0, WM_SCREEN_WIDTH, 32);
        wm_set_z_index(m_state.panel_win_id, 999); /* Always on top */

        /* Create the Spatial Dock */
        int dock_w = 600;
        int dock_h = 64;
        int dock_x = (WM_SCREEN_WIDTH / 2) - (dock_w / 2);
        int dock_y = WM_SCREEN_HEIGHT - dock_h - 16; /* Floating above bottom edge */
        
        m_state.dock_win_id = wm_create_window(0, "ZenithDock", dock_x, dock_y, dock_w, dock_h);
        wm_set_z_index(m_state.dock_win_id, 1000); /* Above panel */

        sigma_log("[Zenith] Base UI components created successfully.");
        drawPanel();
        drawDock();
    }

    void setTheme(zenith_theme_t theme) {
        m_state.current_theme = theme;
        sigma_log_info("[Zenith] Theme updated to %d\n", (int)theme);
        drawPanel();
        drawDock();
    }

    void drawPanel() {
        /* Stub: Issue draw commands to sigma-wm for the panel */
        sigma_color_t bg_dark = {30, 30, 30, 240};
        sigma_color_t bg_light = {240, 240, 240, 240};
        sigma_color_t bg = (m_state.current_theme == THEME_DARK) ? bg_dark : bg_light;
        
        sigma_log_info("[Zenith] Drawing Top Panel with color rgba(%u, %u, %u, %u)\n", bg.r, bg.g, bg.b, bg.a);
    }

    void drawDock() {
        /* Stub: Issue draw commands for a frosted-glass spatial dock */
        sigma_color_t bg_dark = {50, 50, 50, 200};
        sigma_color_t bg_light = {255, 255, 255, 200};
        sigma_color_t bg = (m_state.current_theme == THEME_DARK) ? bg_dark : bg_light;
                           
        sigma_log_info("[Zenith] Drawing Spatial Dock (Blur: %s) with color rgba(%u, %u, %u, %u)\n", 
                       m_state.is_blur_enabled ? "ON" : "OFF", bg.r, bg.g, bg.b, bg.a);
    }

    void handleClick(int x, int y) {
        /* Simple hit-testing stub */
        sigma_log_info("[Zenith] Click registered at (%d, %d)\n", x, y);
    }

private:
    ZenithDesktopEnvironment() {}
    zenith_desktop_state_t m_state;
};

} // namespace UI
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {
void zenith_init(void) { SigmaOS::UI::ZenithDesktopEnvironment::getInstance().init(); }
void zenith_draw_dock(void) { SigmaOS::UI::ZenithDesktopEnvironment::getInstance().drawDock(); }
void zenith_draw_top_panel(void) { SigmaOS::UI::ZenithDesktopEnvironment::getInstance().drawPanel(); }
void zenith_set_theme(zenith_theme_t theme) { SigmaOS::UI::ZenithDesktopEnvironment::getInstance().setTheme(theme); }
void zenith_handle_click(int x, int y) { SigmaOS::UI::ZenithDesktopEnvironment::getInstance().handleClick(x, y); }
}
