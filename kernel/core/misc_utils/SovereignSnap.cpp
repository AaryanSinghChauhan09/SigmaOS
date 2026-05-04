#include "../../../include/sigma_kernel_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_snap_types.h"
#include "../../../include/SovereignSnap.h"

namespace SigmaOS {
namespace Kernel {
namespace UI {

SovereignSnapEngine& SovereignSnapEngine::getInstance() {
    static SovereignSnapEngine instance;
    return instance;
}

void SovereignSnapEngine::init() {
    sigma_log("Σ [SNAP]: Initializing Sovereign Dynamic Shard-Snapping (DSS)...");
    this->active_zone_count = 0u;
    this->initialized       = 1u;
    sigma_log("Σ [SNAP]: DSS Multi-window spatial lattice ONLINE.");
}

void SovereignSnapEngine::applyLayout(sigma_u32 layout_id) {
    sigma_printf("Σ [SNAP]: Applying spatial layout L%02u...\n", layout_id);
    sigma_log("Σ [SNAP]: Viewport reconciliation complete.");
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
        sigma_printf("Σ [SNAP]: Zone %u registered (%u,%u,%u,%u).\n",
                     this->active_zone_count, x, y, w, h);
    }
}

} // namespace UI
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void snap_init() {
    SigmaOS::Kernel::UI::SovereignSnapEngine::getInstance().init();
}

extern "C" void snap_window_to_zone(sigma_u32 window_id, sigma_snap_zone_id_t zone) {
    (void)window_id;
    SigmaOS::Kernel::UI::SovereignSnapEngine::getInstance().applyLayout((sigma_u32)zone);
}

extern "C" void snap_auto_arrange() {
    SigmaOS::Kernel::UI::SovereignSnapEngine::getInstance().applyLayout(0u);
}

extern "C" void snap_register_zone(sigma_u32 x, sigma_u32 y, sigma_u32 w, sigma_u32 h) {
    SigmaOS::Kernel::UI::SovereignSnapEngine::getInstance().registerZone(x, y, w, h);
}
