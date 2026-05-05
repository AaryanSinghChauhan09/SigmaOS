#include "../../../include/sigma_types.h"
#include "sigma_hal.h"
#include "../../../include/SovereignLibC.h"

/**
 * SigmaOS Kernel-Level I/O Orchestrator (Step 3: Debugging)
 * Redirects all sigma_write calls to COM1 serial output for QEMU/Bochs tracing.
 */

extern "C" sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count) {
    const char* data = (const char*)buf;
    (void)fd;
    
    // We ignore fd for now and send everything to COM1
    for (sigma_size_t i = 0; i < count; i++) {
        // Step 3: Serial Debugging
        serial_putc(data[i]);
    }
    
    return (sigma_ssize_t)count;
}

// Ensure serial is initialized before first print
extern "C" void kernel_io_init() {
    serial_init();
    sigma_log("[KERNEL-IO] COM1 Serial Shard Active (115200 8N1).");
}

