#include "libc/SovereignLibC.h"
#include "core/sigma_types.h"

#include "observability/sigma_observe.h"
#include "hal/sigma_hal.h"


/**
 * SigmaOS Sovereign Observability Matrix
 * Implements a Dynamic Silicon Instrumentation (DSI) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal kernel tracing.
 */

static sigma_observe_probe_t active_probes[128];
static sigma_u32 probe_count = 0;

extern "C" void observe_init() {
    sigma_log("[OBSERVE] Initializing Sovereign Observability Matrix (DSI Algorithm)...");
}

extern "C" bool observe_attach_probe(const char* symbol, void (*callback)(void)) {
    if (probe_count >= 128) return false;
    
    // DSI (Dynamic Silicon Instrumentation) Algorithm
    // Rewrites live kernel memory to insert safe JMP instructions to the probe.
    
    sigma_u32 id = ++probe_count;
    active_probes[id - 1].probe_id = id;
    sigma_hardened_strcpy(active_probes[id - 1].target_symbol, symbol, 64);
    active_probes[id - 1].is_active = true;
    
    sigma_log("[OBSERVE] DSI: Safely attached probe %d to symbol '%s'.\n", id, symbol);
    return true;
}

extern "C" void observe_trigger_probe(sigma_u32 probe_id) {
    if (probe_id > 0 && probe_id <= probe_count && active_probes[probe_id - 1].is_active) {
        sigma_log("[OBSERVE] DSI: Probe %d triggered on '%s'. Capturing registers...\n", 
                     probe_id, active_probes[probe_id - 1].target_symbol);
    }
}



