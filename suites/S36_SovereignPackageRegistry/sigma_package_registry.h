#ifndef SIGMA_PACKAGE_REGISTRY_H
#define SIGMA_PACKAGE_REGISTRY_H

#include "../../include/libc/sigma_libc.h"

/* SigmaOS Sovereign Package Registry - Phase 7 Sovereign Intelligence
 * Implements a content-addressed, decentralized package distribution
 * overlay for the Lattice Mesh.
 */

#define SIGMA_PKG_MAX_NAME 64
#define SIGMA_PKG_MAX_DEPS 16
#define SIGMA_PKG_REGISTRY_CAPACITY 1024

typedef struct {
    uint8_t hash[32]; // SHA-256 equivalent content address
    char name[SIGMA_PKG_MAX_NAME];
    uint32_t version_major;
    uint32_t version_minor;
    uint32_t size_bytes;
    uint32_t dependencies[SIGMA_PKG_MAX_DEPS];
    uint32_t dep_count;
    uint8_t is_verified;
} sigma_package_manifest_t;

typedef struct {
    sigma_package_manifest_t entries[SIGMA_PKG_REGISTRY_CAPACITY];
    uint32_t total_packages;
    uint8_t registry_root_hash[32]; // Merkle root representation
} sigma_package_registry_t;

/* Initialize the decentralized package registry */
void sigma_registry_init(sigma_package_registry_t* reg);

/* Register a new package manifest into the local lattice node */
int sigma_registry_add_package(sigma_package_registry_t* reg, const sigma_package_manifest_t* pkg);

/* Resolve a package by its content hash. Returns index or -1 if not found */
int sigma_registry_resolve_by_hash(const sigma_package_registry_t* reg, const uint8_t* hash);

/* Calculate a simplistic Merkle root hash for the registry state */
void sigma_registry_compute_root(sigma_package_registry_t* reg);

#endif
