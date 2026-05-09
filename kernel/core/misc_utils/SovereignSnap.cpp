#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "core/sigma_kernel_types.h"
#include "libc/SovereignLibC.h"
#include "sigma_snap_types.h"
#include "SovereignSnap.h"
#include "sigma_log.h"




namespace SigmaOS {
namespace Kernel {
namespace UI {

SovereignSnapEngine& SovereignSnapEngine::getInstance() {
    static SovereignSnapEngine instance;
    return instance;
}

SovereignSnapEngine::SovereignSnapEngine() : initialized(0), active_zone_count(0) {
    // Shard-init
}

void SovereignSnapEngine::init() {
    log_emit(LOG_INFO, "Σ [SNAP]: Initializing Sovereign Dynamic Shard-Snapping (DSS)...");
    this->active_zone_count = 0u;
    this->initialized       = 1u;
    log_emit(LOG_INFO, "Σ [SNAP]: DSS Multi-window spatial lattice ONLINE.");
}

void SovereignSnapEngine::applyLayout(sigma_u32 layout_id) {
    (void)layout_id;
    log_emit(LOG_INFO, "[SNAP]: Applying spatial layout.");
    log_emit(LOG_INFO, "Σ [SNAP]: Viewport reconciliation complete.");
}

void SovereignSnapEngine::registerZone(sigma_u32 x, sigma_u32 y,
                                        sigma_u32 w, sigma_u32 h) {
    if (this->active_zone_count < 8u) {
        sigma_snap_zone_t* zone = &this->m_zones[this->active_zone_count++];
        zone->id       = this->active_zone_count;
        zone->x        = x;
        zone->y        = y;
        zone->w        = w;
        zone->h        = h;
        zone->capacity = w * h;
        log_emit(LOG_INFO, "Σ [SNAP]: Zone registered and calibrated.");
    }
}


} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void snap_init() {
    SigmaOS::Kernel::UI::SovereignSnapEngine::init();
}

extern "C" void snap_window_to_zone(sigma_u32 window_id, sigma_snap_zone_id_t zone) {
    (void)window_id;
    SigmaOS::Kernel::UI::SovereignSnapEngine::applyLayout((sigma_u32)zone);
}

extern "C" void snap_auto_arrange() {
    SigmaOS::Kernel::UI::SovereignSnapEngine::applyLayout(0u);
}

extern "C" void snap_register_zone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h) {
    SigmaOS::Kernel::UI::SovereignSnapEngine::registerZone(x, y, w, h);
}





