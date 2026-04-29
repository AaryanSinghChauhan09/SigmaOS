/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN NANO-LIBRARY CORE (S-NANOLIB)
 * =========================================================================
 * Mission: Provide an extreme minimal footprint standard library,
 * prioritizing speed and size over legacy POSIX bloat.
 * Inspired by Alpine Linux's musl libc.
 * =========================================================================
 */

#ifndef SIGMA_NANOLIB_H
#define SIGMA_NANOLIB_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- NanoLib Primitives --- */
void nanolib_init(void);
uint32_t nanolib_strlen(const char* str);
void* nanolib_memcpy(void* dest, const void* src, uint32_t n);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NANOLIB_H */
