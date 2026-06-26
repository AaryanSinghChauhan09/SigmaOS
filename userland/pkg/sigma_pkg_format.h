/*
 * =============================================================================
 * Σ SIGMAOS: OMNIPACKAGE FORMAT DEFINITION
 * =============================================================================
 * Definition of the .spkg binary archive format.
 * =============================================================================
 */

#ifndef SIGMA_PKG_FORMAT_H
#define SIGMA_PKG_FORMAT_H

#include "../include/sigma_kernel_types.h"

#define SPKG_MAGIC 0x53504B47 /* 'SPKG' */

/* Package Header (512 bytes fixed) */
typedef struct {
    sigma_u32 magic;             /* SPKG_MAGIC */
    sigma_u32 version;           /* Format version (e.g., 1) */
    char      name[64];          /* Package name */
    char      pkg_version[32];   /* Package version (e.g. 1.0.4-2) */
    char      architecture[16];  /* x86_64, aarch64, riscv64, any */
    
    sigma_u32 meta_offset;       /* Offset to metadata YAML */
    sigma_u32 meta_length;       /* Length of metadata */
    
    sigma_u32 data_offset;       /* Offset to zstd-compressed payload */
    sigma_u64 data_length;       /* Compressed length */
    sigma_u64 uncompressed_len;  /* Uncompressed length */
    
    sigma_u8  sha256_hash[32];   /* SHA256 of the uncompressed payload */
    sigma_u8  signature[64];     /* Ed25519 signature of the hash */
    
    sigma_u8  padding[324];      /* Pad to 512 bytes */
} sigma_pkg_header_t;

/* Dependency entry in metadata */
typedef struct {
    char name[64];
    char min_version[32];
} sigma_pkg_dep_t;

#endif /* SIGMA_PKG_FORMAT_H */
