/**
 * =========================================================================
 * Σ SIGMAOS: ZENITH WINDOW MANAGER (sigma-wm v1.0)
 * =========================================================================
 * Basic compositing window manager handling backbuffers and z-indexing.
 * =========================================================================
 */

#include "../include/sigma_kernel_types.h"
#include "../include/sigma_log.h"
#include "../include/ui/sigma_wm.h"

namespace SigmaOS {
namespace UI {

class SovereignWindowManager {
public:
    static SovereignWindowManager& getInstance() {
        static SovereignWindowManager instance;
        return instance;
    }

    void init() {
        m_window_count = 0;
        for (sigma_u32 i = 0; i < WM_MAX_WINDOWS; i++) {
            m_windows[i].win_id = 0;
        }

        /* Fake framebuffer allocation for master display */
        m_master_fb = 0x80000000ULL; 

        sigma_log("[WM] Zenith Window Manager initialized.");
        sigma_log_info("[WM] Screen resolution: %dx%d (32-bit ARGB)\n", WM_SCREEN_WIDTH, WM_SCREEN_HEIGHT);
    }

    sigma_u32 createWindow(sigma_u32 pid, const char* title, int x, int y, int w, int h) {
        if (m_window_count >= WM_MAX_WINDOWS) return 0;

        sigma_u32 id = m_window_count + 1;
        sigma_window_t& win = m_windows[id - 1];
        win.win_id = id;
        win.owner_pid = pid;
        sigma_strncpy(win.title, title, WM_TITLE_LEN);
        win.x = x;
        win.y = y;
        win.width = w;
        win.height = h;
        win.z_index = m_window_count; /* Place on top */
        win.is_visible = SIGMA_TRUE;
        win.framebuffer = 0x90000000ULL + (id * 1024 * 1024); /* Fake memory address */

        m_window_count++;
        sigma_log_info("[WM] Window %u created: '%s' [%dx%d] at (%d,%d)\n", id, title, w, h, x, y);
        return id;
    }

    int destroyWindow(sigma_u32 win_id) {
        sigma_window_t* win = findWindow(win_id);
        if (!win) return K_ERR_NOTFOUND;

        sigma_log_info("[WM] Window %u destroyed.\n", win_id);
        win->win_id = 0;
        return K_OK;
    }

    int moveWindow(sigma_u32 win_id, int new_x, int new_y) {
        sigma_window_t* win = findWindow(win_id);
        if (!win) return K_ERR_NOTFOUND;

        win->x = new_x;
        win->y = new_y;
        return K_OK;
    }

    int setZIndex(sigma_u32 win_id, int z_index) {
        sigma_window_t* win = findWindow(win_id);
        if (!win) return K_ERR_NOTFOUND;

        win->z_index = z_index;
        return K_OK;
    }

    int setVisibility(sigma_u32 win_id, sigma_bool visible) {
        sigma_window_t* win = findWindow(win_id);
        if (!win) return K_ERR_NOTFOUND;

        win->is_visible = visible;
        return K_OK;
    }

    void composite() {
        sigma_window_t* sorted[WM_MAX_WINDOWS];
        sigma_u32 visible_count = 0;
        
        for (sigma_u32 i = 0; i < m_window_count; i++) {
            if (m_windows[i].win_id != 0 && m_windows[i].is_visible) {
                sorted[visible_count++] = &m_windows[i];
            }
        }
        
        /* Bubble sort by z_index */
        for (sigma_u32 i = 0; i < visible_count; i++) {
            for (sigma_u32 j = 0; j < visible_count - i - 1; j++) {
                if (sorted[j]->z_index > sorted[j+1]->z_index) {
                    sigma_window_t* temp = sorted[j];
                    sorted[j] = sorted[j+1];
                    sorted[j+1] = temp;
                }
            }
        }
        
        /* Simulated Blitting */
        sigma_log_info("[WM] Compositing %u visible windows to master framebuffer (0x%llX)...\n", 
                       visible_count, (unsigned long long)m_master_fb);
        for (sigma_u32 i = 0; i < visible_count; i++) {
            sigma_window_t* win = sorted[i];
            /* In reality: sigma_memcpy(m_master_fb + offset, win->framebuffer, size) */
            sigma_log_info("  -> Blitting window '%s' [z:%d] from backbuffer (0x%llX)\n", 
                           win->title, win->z_index, (unsigned long long)win->framebuffer);
        }
    }

private:
    SovereignWindowManager() : m_window_count(0), m_master_fb(0) {}

    sigma_window_t* findWindow(sigma_u32 id) {
        if (id == 0 || id > m_window_count) return SIGMA_NULL;
        return &m_windows[id - 1];
    }

    sigma_window_t m_windows[WM_MAX_WINDOWS];
    sigma_u32      m_window_count;
    sigma_vaddr_t  m_master_fb;
};

} // namespace UI
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void wm_init(void) { SigmaOS::UI::SovereignWindowManager::getInstance().init(); }

sigma_u32 wm_create_window(sigma_u32 pid, const char* title, int x, int y, int w, int h) {
    return SigmaOS::UI::SovereignWindowManager::getInstance().createWindow(pid, title, x, y, w, h);
}

int wm_destroy_window(sigma_u32 win_id) {
    return SigmaOS::UI::SovereignWindowManager::getInstance().destroyWindow(win_id);
}

int wm_move_window(sigma_u32 win_id, int new_x, int new_y) {
    return SigmaOS::UI::SovereignWindowManager::getInstance().moveWindow(win_id, new_x, new_y);
}

int wm_set_z_index(sigma_u32 win_id, int z_index) {
    return SigmaOS::UI::SovereignWindowManager::getInstance().setZIndex(win_id, z_index);
}

int wm_set_visibility(sigma_u32 win_id, sigma_bool visible) {
    return SigmaOS::UI::SovereignWindowManager::getInstance().setVisibility(win_id, visible);
}

void wm_composite(void) {
    SigmaOS::UI::SovereignWindowManager::getInstance().composite();
}

} // extern "C"
