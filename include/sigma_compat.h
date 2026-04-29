/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN COMPATIBILITY LAYER (S-COMPAT)
 * =========================================================================
 * Mission: Native, zero-overhead execution of foreign binary formats (ELF/PE).
 * Inspired by WSL / Wine / FreeBSD Linuxulator.
 * =========================================================================
 */

#ifndef SIGMA_COMPAT_H
#define SIGMA_COMPAT_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    COMPAT_MODE_LINUX_ELF,
    COMPAT_MODE_WINDOWS_PE,
    COMPAT_MODE_WASM_SHARD
} sigma_compat_mode_t;

/* --- Compatibility Primitives --- */
void compat_init(void);
bool compat_load_binary(const char* path, sigma_compat_mode_t mode);
void compat_mediate_syscall(uint32_t foreign_id, void* args);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_COMPAT_H */
