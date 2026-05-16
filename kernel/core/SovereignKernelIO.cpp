#include "../../include/sigma_kernel_types.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_log.h"

/**
 * SigmaOS Kernel-Level I/O Orchestrator
 * Redirects all sigma_write calls to COM1 serial output.
 */

extern "C" {

sigma_ssize_t sigma_write(int fd, const void* buf, sigma_size_t count) {
    const char* data = (const char*)buf;
    (void)fd;
    
    for (sigma_size_t i = 0; i < count; i++) {
        serial_putc(data[i]);
    }
    
    return (sigma_ssize_t)count;
}

void kernel_io_init() {
    serial_init();
    sigma_log("[KERNEL-IO] COM1 Serial Shard Active (115200 8N1).");
}

} // extern "C"
