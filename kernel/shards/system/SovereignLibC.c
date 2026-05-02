/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LIBC MASTER SHARD (v20.0)
 * =========================================================================
 * This is the unified entry point for the modularized Sovereign LibC.
 * Modularized for industrial-grade maintainability.
 * =========================================================================
 */

#include "../../../include/SovereignLibC.h"

/* Modular implementations */
#include "libc/sigma_io.c"
#include "libc/sigma_string.c"
#include "libc/sigma_mem.c"

/* Legacy / Core Syscall Shims */
unsigned int sigma_sleep(unsigned int seconds) {
    sigma_printf("[ZENITH-LIBC]: Pulse sleep for %u seconds...\n", seconds);
    sigma_i64 req[2] = { (sigma_i64)seconds, 0 };
    sigma_nanosleep(req, SIGMA_NULL);
    return 0;
}
