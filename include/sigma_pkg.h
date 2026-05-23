/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN PACKAGE MANAGEMENT (S-PKG)
 * =========================================================================
 * Mission: Zero-dependency shard distribution and dependency resolution.
 * =========================================================================
 */

#ifndef SIGMA_PKG_H
#define SIGMA_PKG_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    char name[64];
    unsigned int version;
    unsigned int dependency_shard_ids[16];
    unsigned int dep_count;
} sigma_package_t;

/* --- Package Primitives --- */
void pkg_init(void);
bool pkg_install_shard(const char* name, unsigned int shard_id);
void pkg_resolve_dependencies(unsigned int shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PKG_H */
