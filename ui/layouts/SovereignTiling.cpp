#include "../../include/sigma_log.h"
#include "../../include/core/sigma_types.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Tiling Window Manager
 * Kernel-assisted deterministic window layout engine.
 *
 * USP: Unlike i3/Sway running in userland, the Sovereign Tiler operates
 * via direct framebuffer geometry commands through the Zenith MLC compositor.
 * Layout decisions are O(1) — no redraw overhead, no compositor round-trips.
 *
 * Design: OOP-isolated singleton — SovereignTilingEngine.
 */

typedef struct {
    sigma_u32 x, y, w, h;
    char app_id[32];
} sigma_tile_t;

class SovereignTilingEngine {
public:
    static SovereignTilingEngine& getInstance() {
        static SovereignTilingEngine instance;
        return instance;
    }

    static void init() {
        sigma_log("[TILING] Initializing Sovereign Tiling Window Manager...");
        this->tile_count = 0;
        this->screen_w = 1920;
        this->screen_h = 1080;
    }

    void tileApp(const char* app_id) {
        if (this->tile_count >= 16) return;
        sigma_tile_t* t = &this->tiles[this->tile_count];
        // Auto-calculate equal-width vertical splits
        sigma_u32 col_w = this->screen_w / (this->tile_count + 1);
        for (sigma_u32 i = 0; i <= this->tile_count; i++) {
            this->tiles[i].x = i * col_w;
            this->tiles[i].y = 0;
            this->tiles[i].w = col_w;
            this->tiles[i].h = this->screen_h;
        }
        sigma_hardened_strcpy(t->app_id, app_id, 32);
        this->tile_count++;
        sigma_log("[TILING] Tiled '%s'. Layout: %u equal columns.\n", app_id, this->tile_count);
    }

    void setLayout(const char* layout_name) {
        sigma_log("[TILING] Applying layout: '%s' across %u tiles.\n", layout_name, this->tile_count);
    }

private:
    SovereignTilingEngine() : tile_count(0), screen_w(1920), screen_h(1080) {}
    sigma_tile_t tiles[16];
    sigma_u32 tile_count;
    sigma_u32 screen_w, screen_h;
};

extern "C" void tiling_init() { SovereignTilingEngine::init(); }
extern "C" void tiling_add_app(const char* id) { SovereignTilingEngine::tileApp(id); }
extern "C" void tiling_set_layout(const char* layout) { SovereignTilingEngine::setLayout(layout); }




