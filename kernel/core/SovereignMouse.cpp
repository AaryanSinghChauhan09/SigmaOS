#include "sigma_usb.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign USB Mouse Shard (v28.0 Zenith)
 * Implements a HID Mouse Orchestration (HMO) algorithm.
 * ZERO-DEPENDENCY: Direct HID report parsing from S-USB.
 *
 * Design: OOP-isolated singleton — SovereignMouseEngine.
 */

/* --- Sovereign Mouse Engine (OOP Isolation) --- */
static struct {
    sigma_u32 x;
    sigma_u32 y;
    sigma_u32 buttons;
    sigma_u32 initialized;
} SovereignMouseEngine = {
    .x = 0u, .y = 0u, .buttons = 0u,
    .initialized = 0u
};

extern "C" void mouse_init() {
    sigma_log("[MOUSE] Initializing Sovereign HID Mouse Orchestration (HMO)...");
    SovereignMouseEngine.initialized = 1u;
}

extern "C" void mouse_handle_report(const sigma_u8* report) {
    if (!report) return;
    /* HMO Algorithm: Decodes USB HID mouse reports into lattice coordinates */
    SovereignMouseEngine.x += (sigma_u32)report[1];
    SovereignMouseEngine.y += (sigma_u32)report[2];
    SovereignMouseEngine.buttons = report[0];
    
    sigma_printf("[MOUSE] HMO: Position (%u, %u) | Buttons: 0x%02X\n", 
                 SovereignMouseEngine.x, SovereignMouseEngine.y, SovereignMouseEngine.buttons);
}
