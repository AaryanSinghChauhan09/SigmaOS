// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_pkg_verity.h — dm-verity per-package integrity (snapd ContainerPlaceInfo-inspired)
 *
 * Every sigma package ships a companion .verity hash tree file.
 * dm-verity validates every kernel read against the tree — bit flips detected at wire.
 * Root hash is stored in the signed package assertion (cannot be tampered separately).
 */
#include <stdint.h>
#include <stddef.h>

typedef struct {
    char     package_name[128];
    char     package_version[32];
    char     verity_file_path[512];  /* /sigma/pkg/pool/<name>-<ver>.verity     */
    char     squashfs_path[512];     /* /sigma/pkg/pool/<name>-<ver>.spkg       */
    uint8_t  root_hash[32];          /* SHA-256 of hash tree root               */
    uint32_t data_block_size;        /* typically 4096                          */
    uint32_t hash_block_size;        /* typically 4096                          */
    uint64_t data_blocks;
} sigma_pkg_verity_t;

/* Generate companion .verity hash tree file after download */
int sigma_pkg_verity_create(const char* squashfs_path, sigma_pkg_verity_t* out);

/* Mount with dm-verity enforcement (every read verified) */
int sigma_pkg_verity_mount(const sigma_pkg_verity_t* v, const char* mount_point);

/* Verify root_hash against the signed assertion before mounting */
int sigma_pkg_verity_check_assertion(const sigma_pkg_verity_t* v,
                                      const uint8_t* asserted_root_hash);

/* Unmount and remove device-mapper entry */
int sigma_pkg_verity_umount(const char* package_name);
