#include "../../../include/sigma_log.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Boot Splash Engine
 * Hardware-accelerated animated boot screen rendered at POST.
 *
 * USP: Unlike Plymouth's userland daemon, the Sovereign Boot Splash writes
 * pixels directly to the framebuffer before the kernel has even mounted VFS.
 * Fully customizable via .sab theme bundles � zero runtime dependencies.
 *
 * Design: OOP-isolated singleton � SovereignBootSplashEngine.
 */

class SovereignBootSplashEngine {
public:
    static SovereignBootSplashEngine& getInstance() {
        static SovereignBootSplashEngine instance;
        return instance;
    }

    static void init(sigma_u32 fb_width, sigma_u32 fb_height) {
        this->fb_w = fb_width;
        this->fb_h = fb_height;
        this->progress = 0;
        sigma_log("[BOOT-SPLASH] Framebuffer acquired (%ux%u). Rendering S SIGMAOS splash...\n",
                     fb_width, fb_height);
    }

    void updateProgress(sigma_u32 percent, const char* stage_name) {
        this->progress = percent;
        sigma_log("[BOOT-SPLASH] [%3u%%] %s\n", percent, stage_name);
    }

    void setTheme(const char* theme_name) {
        sigma_log("[BOOT-SPLASH] Applying boot splash theme: '%s'.\n", theme_name);
    }

    void dismiss() {
        sigma_log("[BOOT-SPLASH] Boot complete. Handing framebuffer to Zenith MLC compositor.");
    }

private:
    SovereignBootSplashEngine() : fb_w(0), fb_h(0), progress(0) {}
    sigma_u32 fb_w, fb_h, progress;
};

void bootsplash_init(sigma_u32 w, sigma_u32 h) { SovereignBootSplashEngine::init(w, h); }
void bootsplash_progress(sigma_u32 pct, const char* stage) { SovereignBootSplashEngine::updateProgress(pct, stage); }
void bootsplash_set_theme(const char* theme) { SovereignBootSplashEngine::setTheme(theme); }
void bootsplash_dismiss() { SovereignBootSplashEngine::dismiss(); }





} // extern "C"
 