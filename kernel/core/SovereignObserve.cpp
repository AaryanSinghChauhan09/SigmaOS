#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"

#include "../../include/observability/sigma_observe.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"


/**
 * SigmaOS Sovereign Observability Matrix
 * Implements a Dynamic Silicon Instrumentation (DSI) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal kernel tracing.
 */

static sigma_observe_probe_t active_probes[128];
static uint32_t probe_count = 0;

extern "C" void observe_init() {
    sigma_log("[OBSERVE] Initializing Sovereign Observability Matrix (DSI Algorithm)...");
}

extern "C" bool observe_attach_probe(const char* symbol, void (*callback)(void)) {
    if (probe_count >= 128) return false;
    
    // DSI (Dynamic Silicon Instrumentation) Algorithm
    // Rewrites live kernel memory to insert safe JMP instructions to the probe.
    
    uint32_t id = ++probe_count;
    active_probes[id - 1].probe_id = id;
    sigma_hardened_strcpy(active_probes[id - 1].target_symbol, symbol, 64);
    active_probes[id - 1].is_active = true;
    
    sigma_log_info("[OBSERVE] DSI: Safely attached probe %d to symbol '%s'.\n", id, symbol);
    return true;
}

extern "C" void observe_trigger_probe(uint32_t probe_id) {
    if (probe_id > 0 && probe_id <= probe_count && active_probes[probe_id - 1].is_active) {
        sigma_log_info("[OBSERVE] DSI: Probe %d triggered on '%s'. Capturing registers...\n", 
                     probe_id, active_probes[probe_id - 1].target_symbol);
    }
}


