#include "sigma_log.h"
#include "Lattice.h"
#include "libc/SovereignLibC.h"
/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN HARDWARE AUDIT (v128.0 - ZERO-STD NATIVE)
 * =========================================================================
 * Refactored into modular hardware shards for industrial silicon mapping.
 * =========================================================================
 */

#include "kernel/diagnostics/hardware_audit.hpp"

void _start(void) {
    sigma_log("--- Σ SIGMA OS SOVEREIGN HARDWARE AUDIT (ZENITH) ---\n");
    SigmaOS::Diagnostics::SovereignHardwareAudit audit;
    audit.AuditProcessors();
    audit.AuditMemory();
    
    sigma_log("[SUCCESS]: All Hardware Shards mapped via Silicon-Direct APEX-API.\n");
    sigma_exit(0);
}

} // extern "C"
