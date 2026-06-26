// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_acquire.h — multi-queue parallel package downloader (Debian apt-inspired)
 *
 * Downloads up to max_parallel packages concurrently. Each item is verified
 * with two independent hash algorithms (SHA-256 + BLAKE2b-512) before being
 * committed to the package cache — this prevents length-extension attacks that
 * fool single-hash systems.
 *
 * Usage:
 *   sigma_acquire_t acq = {};
 *   acq.max_parallel = 3;
 *   sigma_strncpy(acq.cache_dir, "/sigma/cache/pkg", sizeof(acq.cache_dir));
 *
 *   sigma_acquire_add(&acq, "https://pkg.sigma.os/ffmpeg-6.0.tar.sig",
 *                     "<sha256hex>", "<blake2bhex>");
 *   sigma_acquire_run(&acq);   // blocks until all complete or fail
 */

#include <sigma_kernel_types.h>

typedef enum {
    SIGMA_ACQ_PENDING,    /* queued, not yet started                    */
    SIGMA_ACQ_FETCHING,   /* download in progress                       */
    SIGMA_ACQ_VERIFYING,  /* checking hashes                            */
    SIGMA_ACQ_DONE,       /* verified and committed to cache            */
    SIGMA_ACQ_FAILED,     /* download or hash mismatch                  */
} sigma_acq_state_t;

typedef struct {
    char              uri[512];
    char              dest_path[256];
    sigma_u64         expected_size;
    char              sha256[65];    /* 32 bytes → 64 hex chars + NUL     */
    char              blake2b[129];  /* 64 bytes → 128 hex chars + NUL    */
    sigma_acq_state_t state;
    int               retry_count;
    int               worker_fd;    /* pipe fd to download worker (-1 = idle) */
    sigma_u64         bytes_received;
} sigma_acq_item_t;

typedef struct {
    sigma_acq_item_t items[64];
    int              item_count;
    int              max_parallel;  /* default: 3, matching apt Queue-Mode  */
    char             cache_dir[128];
} sigma_acquire_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Queue a package for download. sha256 and blake2b are hex strings. */
int sigma_acquire_add(sigma_acquire_t* acq,
                      const char*     uri,
                      sigma_u64       expected_size,
                      const char*     sha256,
                      const char*     blake2b);

/*
 * Run the download loop until all items are DONE or FAILED.
 * Returns number of successfully downloaded+verified items.
 */
int sigma_acquire_run(sigma_acquire_t* acq);

/*
 * Verify a downloaded file against both expected hashes.
 * Returns 0 if both match, -1 if either fails.
 * On failure the cached file is deleted automatically.
 */
int sigma_acquire_verify(const sigma_acq_item_t* item);

/* Print a summary of all items and their final state to the serial log. */
void sigma_acquire_report(const sigma_acquire_t* acq);
