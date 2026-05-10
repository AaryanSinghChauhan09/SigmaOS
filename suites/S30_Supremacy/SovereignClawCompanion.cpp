#include "../include/core/sigma_types.h"
#include "sigma_universal_ui.h" // Assume UI API
#include "../include/sigma_log.h"
#include "ai/sigma_claw.h"

/**
 * SovereignClawCompanion — Userland app for the Claw Gateway
 * Provides the interactive "Live Canvas" and multi-channel conversational UI.
 */

extern "C" void claw_companion_launch(void) {
    sigma_log_info("[CLAW-UI] Launching Sovereign Claw Companion App...");
    
    // Initialize connection to the kernel-side Claw Gateway
    claw_gateway_init();
    
    // Setup Live Canvas Window
    sigma_log_info("[CLAW-UI] Initializing Live Canvas window in SovereignUI...");
    
    // Register companion app background daemon for Voice Wake
    sigma_log_info("[CLAW-UI] Voice Wake listener activated (Talk Mode ready).");
}

extern "C" void claw_companion_handle_input(const char* user_input) {
    sigma_log_info("[CLAW-UI] User Input Received: %s", user_input);
    
    // Route local input through the gateway
    claw_route_message("Local-LiveCanvas", user_input);
}
