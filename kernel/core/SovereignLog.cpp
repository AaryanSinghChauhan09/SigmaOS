#include "sigma_log.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign Log Implementation (v28.0 Zenith)
 * Implements a Wait-Free Circular Shard Logging (WFCSL) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal machine-state tracing.
 *
 * Design: OOP-isolated singleton — SovereignLogEngine.
 */


#define LOG_BUFFER_SIZE 256u

/* --- Sovereign Log Engine (OOP Isolation) --- */
static struct {
    sigma_log_entry_t circular_buffer[LOG_BUFFER_SIZE];
    sigma_u32         write_ptr;
    sigma_u64         total_emitted;
    sigma_u32         initialized;
} SovereignLogEngine = {
    .write_ptr = 0u,
    .total_emitted = 0u,
    .initialized = 0u
};

extern "C" void log_init() {
    sigma_log("[LOG] Initializing Sovereign System Logging Nexus...");
    SovereignLogEngine.initialized = 1u;
}

extern "C" void log_emit(sigma_u32 severity, const char* message) {
    /* WFCSL (Wait-Free Circular Shard Logging) Algorithm
     * Uses atomic pointer increments to allow non-blocking log emission. */
    
    sigma_u32 current_ptr = __atomic_fetch_add(&SovereignLogEngine.write_ptr, 1u, __ATOMIC_SEQ_CST);
    sigma_log_entry_t* entry = &SovereignLogEngine.circular_buffer[current_ptr % LOG_BUFFER_SIZE];
    
    entry->timestamp = 0u; // Simulated timestamp
    entry->severity = severity;
    sigma_hardened_strcpy(entry->message, message, 128);
    
    SovereignLogEngine.total_emitted++;
    
    const char* tag = "INFO";
    if (severity >= 3u) tag = "CRITICAL";
    else if (severity == 2u) tag = "WARN";
    else if (severity == 0u) tag = "DEBUG";

    sigma_printf("[LOG] [%s] %s\n", tag, message);
}

extern "C" void log_dump_lattice() {
    sigma_log("[LOG] WFCSL: Dumping machine-state trace...");
    sigma_u32 limit = (SovereignLogEngine.write_ptr > LOG_BUFFER_SIZE) ? LOG_BUFFER_SIZE : SovereignLogEngine.write_ptr;
    for(sigma_u32 i=0; i < limit; i++) {
        sigma_log_entry_t* entry = &SovereignLogEngine.circular_buffer[i];
        sigma_printf("[TRACE] S%u: %s\n", entry->severity, entry->message);
    }
}

extern "C" sigma_u64 log_get_total_emitted() {
    return SovereignLogEngine.total_emitted;
}
