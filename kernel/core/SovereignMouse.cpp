#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Sovereign USB Mouse Shard (v28.0 Zenith)
 * Implements a HID Mouse Orchestration (HMO) algorithm.
 * ZERO-DEPENDENCY: Direct HID report parsing from S-USB.
 *
 * Design: OOP-isolated singleton — SovereignMouseEngine.
 */

class SovereignMouseEngine {
public:
    static SovereignMouseEngine& getInstance() {
        static SovereignMouseEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[MOUSE] Initializing Sovereign HID Mouse Orchestration (HMO)...");
        this->initialized = 1u;
    }

    void handleReport(const sigma_u8* report) {
        if (!report) return;
        /* HMO Algorithm: Decodes USB HID mouse reports into lattice coordinates */
        this->x += (sigma_u32)report[1];
        this->y += (sigma_u32)report[2];
        this->buttons = report[0];
        
        sigma_log_info("[MOUSE] HMO: Position (%u, %u) | Buttons: 0x%02X\n", 
                     this->x, this->y, this->buttons);
    }

private:
    SovereignMouseEngine() : x(0), y(0), buttons(0), initialized(0) {}
    
    sigma_u32 x;
    sigma_u32 y;
    sigma_u32 buttons;
    sigma_u32 initialized;
};

/* --- C Wrappers --- */
extern "C" void mouse_init() {
    SovereignMouseEngine::getInstance().init();
}

extern "C" void mouse_handle_report(const sigma_u8* report) {
    SovereignMouseEngine::getInstance().handleReport(report);
}


 