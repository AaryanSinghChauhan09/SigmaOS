/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UNIFIED PACKAGE SYSTEM (S-UNIFIEDPKG)
 * =========================================================================
 * Mission: A single, cryptographically verified package manager handling
 * both system and application software, avoiding fragmentation.
 * =========================================================================
 */

#ifndef SIGMA_UNIFIEDPKG_H
#define SIGMA_UNIFIEDPKG_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char package_name[64];
    uint32_t version_hi;
    uint32_t version_lo;
    uint8_t cryptographic_signature[64];
    bool is_system_critical;
} sigma_unified_pkg_t;

/* --- Unified Package Primitives --- */
void unifiedpkg_init(void);
bool unifiedpkg_install(const char* package_url, bool system_level);
bool unifiedpkg_verify_signature(const sigma_unified_pkg_t* pkg);
void unifiedpkg_list_installed(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_UNIFIEDPKG_H */
