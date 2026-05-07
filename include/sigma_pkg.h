/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PACKAGE MANAGEMENT (S-PKG)
 * =========================================================================
 * Mission: Zero-dependency shard distribution and dependency resolution.
 * =========================================================================
 */

#ifndef SIGMA_PKG_H
#define SIGMA_PKG_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char name[64];
    uint32_t version;
    uint32_t dependency_shard_ids[16];
    uint32_t dep_count;
} sigma_package_t;

/* --- Package Primitives --- */
void pkg_init(void);
bool pkg_install_shard(const char* name, uint32_t shard_id);
void pkg_resolve_dependencies(uint32_t shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PKG_H */
