
#include "sigma_log.h"
#include "sigma_hal.h"


/**
 * SigmaOS Sovereign Log Implementation
 * Implements a Wait-Free Circular Shard Logging (WFCSL) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal machine-state tracing.
 */

#define LOG_BUFFER_SIZE 256
static sigma_log_entry_t circular_buffer[LOG_BUFFER_SIZE];
static uint32_t write_ptr = 0;

extern "C" void log_init() {
    sigma_log("[LOG] Initializing Sovereign System Logging Nexus...");
}

extern "C" void log_emit(uint32_t severity, const char* message) {
    // WFCSL (Wait-Free Circular Shard Logging) Algorithm
    // Uses atomic pointer increments to allow non-blocking log emission.
    
    // Atomically increment and fetch write pointer
    uint32_t current_ptr = __atomic_fetch_add(&write_ptr, 1, __ATOMIC_SEQ_CST);
    sigma_log_entry_t* entry = &circular_buffer[current_ptr % LOG_BUFFER_SIZE];
    
    entry->timestamp = 0; // Simulated timestamp
    entry->severity = severity;
    sigma_hardened_strcpy(entry->message, message, 128);
    
    // Tiered display logic
    const char* tag = "INFO";
    if (severity >= 3) tag = "CRITICAL";
    else if (severity == 2) tag = "WARN";
    else if (severity == 0) tag = "DEBUG";

    sigma_printf("[LOG] [%s] %s\n", tag, message);
}

extern "C" void log_dump_lattice() {
    sigma_log("[LOG] WFCSL: Dumping machine-state trace...");
    for(uint32_t i=0; i < (write_ptr > LOG_BUFFER_SIZE ? LOG_BUFFER_SIZE : write_ptr); i++) {
        sigma_log_entry_t* entry = &circular_buffer[i];
        sigma_printf("[TRACE] S%d: %s\n", entry->severity, entry->message);
    }
}
