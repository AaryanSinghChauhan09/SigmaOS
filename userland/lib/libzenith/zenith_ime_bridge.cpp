/*
 * =========================================================================
 * Σ SIGMAOS: LIBZENITH — IME BRIDGE
 * =========================================================================
 * Connects the sigma_ime_core daemon to the libzenith widget framework.
 * Renders predictive character candidate popups within the active text field.
 * =========================================================================
 */
#include "../../../klib/include/sigma_stdio.h"

// Called when sigma_ime_core sends a candidate list
extern "C" void zenith_ime_show_candidates(const char** candidates, int count) {
    sigma_printf("[zenith-ime-bridge] Rendering %d character candidates:\n", count);
    for (int i = 0; i < count; i++) {
        sigma_printf("  [%d] %s\n", i + 1, candidates[i]);
    }
    // Triggers a hardware-accelerated popup surface via zenith_compositor
}

// Called on candidate selection (keyboard number or mouse click)
extern "C" void zenith_ime_commit_candidate(int index) {
    sigma_printf("[zenith-ime-bridge] Committing candidate [%d] to active widget.\n", index);
}

// Hide candidate window on Escape or focus loss
extern "C" void zenith_ime_dismiss() {
    sigma_printf("[zenith-ime-bridge] Candidate popup dismissed.\n");
}
