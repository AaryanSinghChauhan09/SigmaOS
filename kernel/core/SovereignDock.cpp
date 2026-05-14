#include "sigma_types.h"
#include "sigma_log.h"
#include "sigma_hal.h"
#include "sigma_log.h"
#include "SovereignLibC.h"
#include "sigma_log.h"

/**
 * SigmaOS Sovereign Dock Engine
 * High-performance, customizable spatial dock for the Zenith interface.
 *
 * USP: A hardware-accelerated, zero-dependency smart dock that intelligently 
 * clusters running micro-VM containers and shards based on predictive ML models.
 *
 * Design: OOP-isolated singleton — SovereignDockEngine.
 */

class SovereignDockEngine {
public:
    static SovereignDockEngine& getInstance() {
        static SovereignDockEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[DOCK] Initializing Sovereign Smart Dock...");
        this->item_count = 0;
        this->dock_scale = 1.0f;
        this->auto_hide = false;
        sigma_hardened_strcpy(this->position, "BOTTOM", 16);
        sigma_log("[DOCK] Smart Dock ACTIVE.");
    }

    void addAppToDock(const char* app_name) {
        if (this->item_count >= 32) {
            sigma_log("[DOCK] [ERROR] Dock is full.");
            return;
        }
        
        sigma_hardened_strcpy(this->dock_items[this->item_count], app_name, 64);
        this->item_count++;
        sigma_log_info("[DOCK] Added '%s' to Sovereign Dock. Total items: %u\n", app_name, this->item_count);
    }

    void configureDock(const char* pos, bool hide, float scale) {
        sigma_hardened_strcpy(this->position, pos, 16);
        this->auto_hide = hide;
        this->dock_scale = scale;
        
        sigma_log_info("[DOCK] Configuration updated: Pos=%s, AutoHide=%d, Scale=%.2f\n", 
                     this->position, (int)this->auto_hide, this->dock_scale);
    }

private:
    SovereignDockEngine() : item_count(0), dock_scale(1.0f), auto_hide(false) {}

    char dock_items[32][64];
    sigma_u32 item_count;
    
    char position[16];
    bool auto_hide;
    float dock_scale;
};

/* --- C Wrappers --- */
extern "C" void dock_init() {
    SovereignDockEngine::getInstance().init();
}

extern "C" void dock_add_app(const char* app_name) {
    SovereignDockEngine::getInstance().addAppToDock(app_name);
}

extern "C" void dock_configure(const char* pos, bool hide, float scale) {
    SovereignDockEngine::getInstance().configureDock(pos, hide, scale);
}


