#include "SovereignLibC.h"
/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN HARDWARE AUDIT (v128.0 - ZERO-STD NATIVE)
 * =========================================================================
 * Refactored into modular hardware shards for industrial silicon mapping.
 * =========================================================================
 */

#include "kernel/diagnostics/hardware_audit.hpp"

extern "C" void _start(void) {
    sigma_printf("--- Î£ SIGMA OS SOVEREIGN HARDWARE AUDIT (ZENITH) ---\n");
    SigmaOS::Diagnostics::SovereignHardwareAudit audit;
    audit.AuditProcessors();
    audit.AuditMemory();
    
    sigma_printf("[SUCCESS]: All Hardware Shards mapped via Silicon-Direct APEX-API.\n");
    sigma_exit(0);
}
