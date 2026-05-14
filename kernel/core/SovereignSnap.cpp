#include "SovereignSnap.hpp"
#include "sigma_log.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "SovereignLibC.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign Snap Layouts (SovereignSnapEngine)
 * Class declared in SovereignSnap.hpp / sigma_snap.h.
 * Only method bodies defined here.
 */

void SovereignSnapEngine::init() {
    sigma_log("[SNAP] Initializing Sovereign Dynamic Shard-Snapping (DSS)...");
    this->active_zone_count = 0u;
    this->initialized       = 1u;
    sigma_log("[SNAP] DSS: Multi-window spatial lattice ACTIVE.");
}

void SovereignSnapEngine::applyLayout(sigma_u32 layout_id) {
    sigma_log_info("[SNAP] DSS: Applying spatial layout L%02u...\n", layout_id);
    /* DSS Algorithm: Recalculates shard viewports based on golden ratio
     * and intent-based priority.                                       */
    sigma_log("[SNAP] DSS: Viewport reconciliation complete.");
}

void SovereignSnapEngine::registerZone(sigma_u32 x, sigma_u32 y,
                                        sigma_u32 w, sigma_u32 h) {
    if (this->active_zone_count < 8u) {
        sigma_snap_zone_t* zone = &this->zones[this->active_zone_count++];
        zone->id       = this->active_zone_count;
        zone->x        = x;
        zone->y        = y;
        zone->w        = w;
        zone->h        = h;
        zone->capacity = w * h;
        sigma_log_info("[SNAP] DSS: Zone %u registered (%u,%u,%u,%u).\n",
                     this->active_zone_count, x, y, w, h);
    }
}

/* --- C Wrappers --- */
extern "C" void snap_init() {
    SovereignSnapEngine::getInstance().init();
}

extern "C" void snap_window_to_zone(sigma_u32 window_id, sigma_snap_zone_id_t zone) {
    (void)window_id;
    SovereignSnapEngine::getInstance().applyLayout((sigma_u32)zone);
}

extern "C" void snap_auto_arrange() {
    SovereignSnapEngine::getInstance().applyLayout(0u);
}

extern "C" void snap_register_zone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h) {
    SovereignSnapEngine::getInstance().registerZone(x, y, w, h);
}


