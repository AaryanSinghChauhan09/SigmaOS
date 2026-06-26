// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_pkg_fetch.cpp — mirror fallback + dual-hash verification (Gentoo-inspired)
 *
 * Mirrors Gentoo portage's fetch.py behavior:
 *   - Try each mirror in order; skip on network failure.
 *   - Verify BOTH SHA-256 and BLAKE2b-512 on every download.
 *   - Any hash mismatch: delete the file and try the next mirror.
 *   - If all mirrors fail: return -1 (fatal — do not install).
 *
 * Unlike single-hash systems, dual verification prevents length-extension
 * attacks that could fool SHA-256 alone.
 */

#include "sigma_pkg_fetch.h"
#include "sigma_log.h"
#include <string.h>
#include <stdio.h>
#include <unistd.h>

/* ── Mirror list ─────────────────────────────────────────────────────────── */

static const char* sigma_mirrors[SIGMA_MAX_MIRRORS] = {
    "https://pkg.sigma.os/main",
    "https://mirror1.sigma.os/pkg",
    "https://mirror2.sigma.os/pkg",
    "https://cdn.sigma.os/packages",
    NULL   /* sentinel */
};

/* ── Forward declarations for platform primitives ───────────────────────── */

/* Implemented in sigmad-process Go daemon — proxies to system HTTP client */
extern "C" int sigma_http_download(const char* uri, const char* dest_path);
extern "C" int sigma_sha256_verify(const char* file_path, const char* expected_hex);
extern "C" int sigma_blake2b_verify(const char* file_path, const char* expected_hex);

/* ── Fetch with mirror fallback ──────────────────────────────────────────── */

int sigma_pkg_fetch(const char* pkg_path,
                    const char* expected_sha256,
                    const char* expected_blake2b,
                    const char* dest) {
    char uri[512];

    for (int i = 0; i < SIGMA_MAX_MIRRORS && sigma_mirrors[i] != NULL; i++) {
        snprintf(uri, sizeof(uri), "%s/%s", sigma_mirrors[i], pkg_path);

        sigma_log_info("[sigma-pkg] Trying mirror %d/%d: %s\n",
                       i + 1, SIGMA_MAX_MIRRORS, uri);

        /* Attempt download */
        int rc = sigma_http_download(uri, dest);
        if (rc != 0) {
            sigma_log_warn("[sigma-pkg] Mirror %d failed (rc=%d), trying next\n",
                           i + 1, rc);
            continue;
        }

        /* ── Verify SHA-256 first ─────────────────────────────────────────── */
        if (!sigma_sha256_verify(dest, expected_sha256)) {
            sigma_log_err("[sigma-pkg] SHA-256 MISMATCH from mirror %d — discarding\n",
                          i + 1);
            sigma_log_err("[sigma-pkg]   expected: %s\n", expected_sha256);
            unlink(dest);
            continue;  /* try next mirror */
        }

        /* ── Verify BLAKE2b-512 second ────────────────────────────────────── */
        if (!sigma_blake2b_verify(dest, expected_blake2b)) {
            sigma_log_err("[sigma-pkg] BLAKE2b MISMATCH from mirror %d — discarding\n",
                          i + 1);
            sigma_log_err("[sigma-pkg]   expected: %s\n", expected_blake2b);
            unlink(dest);
            /*
             * Both hashes matched against different values — this mirror may
             * be serving a tampered package. Try the next mirror.
             */
            continue;
        }

        /* ── Both hashes verified ─────────────────────────────────────────── */
        sigma_log_info("[sigma-pkg] ✓ Package verified: SHA-256 + BLAKE2b OK\n");
        sigma_log_info("[sigma-pkg]   saved to: %s\n", dest);
        return 0;
    }

    /* All mirrors exhausted or all downloads corrupted */
    sigma_log_err("[sigma-pkg] All %d mirrors exhausted. Package unavailable: %s\n",
                  SIGMA_MAX_MIRRORS, pkg_path);
    return -1;
}

/* ── Batch fetch helper ──────────────────────────────────────────────────── */

int sigma_pkg_fetch_batch(const sigma_pkg_download_t* items, int count) {
    int succeeded = 0;
    for (int i = 0; i < count; i++) {
        sigma_log_info("[sigma-pkg] Fetching [%d/%d]: %s\n",
                       i + 1, count, items[i].pkg_path);
        int rc = sigma_pkg_fetch(items[i].pkg_path,
                                  items[i].sha256,
                                  items[i].blake2b,
                                  items[i].dest);
        if (rc == 0) {
            succeeded++;
        } else {
            sigma_log_err("[sigma-pkg] FATAL: failed to fetch %s\n", items[i].pkg_path);
            /* Unlike apt which continues, we fail fast on any corruption */
            return -1;
        }
    }
    sigma_log_info("[sigma-pkg] Batch complete: %d/%d packages fetched\n",
                   succeeded, count);
    return succeeded;
}
