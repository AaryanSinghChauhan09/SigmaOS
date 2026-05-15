#include "../../include/Lattice.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"
/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN PERSONA MANAGER (v21.0)
 * =========================================================================
 * Refactored into modular personalization shards for ease of use.
 * =========================================================================
 */

#include "kernel/core/persona_manager.hpp"
#include "../../include/sigma_log.h"

extern "C" void _start(void) {
    SigmaOS::Core::SovereignPersonaManager persona;
    
    persona.ApplyProfile("ZENITH_ULTRA_PREMIUM");
    persona.TogglePersonalization();
    
    sigma_exit(0);
}

int main() {
    sigma_log_info("[SIGMA_PERSONA]: Initiating Sovereign Customization Shards...\n");
    _start();
    return 0;
}


