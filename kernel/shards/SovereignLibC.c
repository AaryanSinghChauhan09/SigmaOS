/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN LIBC MASTER SHARD (v20.0)
 * =========================================================================
 * This is the unified entry point for the modularized Sovereign LibC.
 * Modularized for industrial-grade maintainability.
 * =========================================================================
 */

#include "../../include/SovereignLibC.h"

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

/* Hardened Primitives */
void* sigma_secure_memset(void* s, int c, sigma_size_t n) {
    volatile unsigned char *p = (volatile unsigned char *)s;
    while (n--) *p++ = c;
    return s;
}

void sigma_hardened_strcpy(char* dest, const char* src, sigma_size_t dest_size) {
    if (dest_size == 0) return;
    sigma_size_t i;
    for (i = 0; i < dest_size - 1 && src[i] != '\0'; i++) {
        dest[i] = src[i];
    }
    dest[i] = '\0';
}
