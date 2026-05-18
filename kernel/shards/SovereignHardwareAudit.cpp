#include "Lattice.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"
#include "sigma_log.h"
/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN HARDWARE AUDIT (v128.0 - ZERO-STD NATIVE)
 * =========================================================================
 * Refactored into modular hardware shards for industrial silicon mapping.
 * =========================================================================
 */

#include "kernel/diagnostics/hardware_audit.hpp"
#include "sigma_log.h"

extern "C" void _start(void) {
    sigma_log_info("--- Î£ SIGMA OS SOVEREIGN HARDWARE AUDIT (ZENITH) ---\n");
    SigmaOS::Diagnostics::SovereignHardwareAudit audit;
    audit.AuditProcessors();
    audit.AuditMemory();
    
    sigma_log_info("[SUCCESS]: All Hardware Shards mapped via Silicon-Direct APEX-API.\n");
    sigma_exit(0);
}


 