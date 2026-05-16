#include "../../../../../../../include/libc/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN I/O SHARD (v1.0 - PURE C11)
 * =========================================================================
 */

#include "../../../../../../../include/libc/sigma_libc.h"

void sigma_print(const char* str) {
    if (!str) return;
    sigma_write(1, str, sigma_sigma_strlen(str));
}

void sigma_sigma_printf(const char* format, ...) {
    // Mission: A truly zero-dependency printf (Industrial Grade Simplified)
    // For now, it prints the literal format. 
    // In Zenith Supreme, this would include a custom va_list parser.
    sigma_print(format);
}
