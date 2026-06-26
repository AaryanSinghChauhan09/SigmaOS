// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
#include <sigma_kernel_types.h>

#define SIGMA_MAX_MIRRORS 8

typedef struct {
    char pkg_path[256];   /* relative path on mirror e.g. "zenith-browser-0.2.0.spkg" */
    char sha256[65];      /* expected SHA-256 hex                                       */
    char blake2b[129];    /* expected BLAKE2b-512 hex                                   */
    char dest[256];       /* local destination path                                     */
} sigma_pkg_download_t;

/* Fetch pkg_path from mirrors, verify both hashes, write to dest. */
int sigma_pkg_fetch(const char* pkg_path,
                    const char* expected_sha256,
                    const char* expected_blake2b,
                    const char* dest);

/* Fetch multiple packages — fails fast on any corruption. */
int sigma_pkg_fetch_batch(const sigma_pkg_download_t* items, int count);
