// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_pkg_journal.h — Structured transaction journal (rpm-ostree + systemd-journal-inspired)
 *
 * Append-only, HMAC-sealed log of every sigma-pkg operation.
 * Each entry chains to the previous via HMAC-SHA256 (like systemd-journal sealing).
 * CLI: sigma-pkg history <package>  shows full install/remove/error history.
 *
 * Journal file: /sigma/var/log/pkg-journal.bin (binary, not text)
 */
#include <sigma_kernel_types.h>
#include <stdint.h>

typedef enum {
    SIGMA_JENTRY_TXN_START    = 0,
    SIGMA_JENTRY_TXN_DOWNLOAD = 1,
    SIGMA_JENTRY_TXN_VERIFY   = 2,
    SIGMA_JENTRY_TXN_APPLY    = 3,
    SIGMA_JENTRY_TXN_COMMIT   = 4,
    SIGMA_JENTRY_TXN_ABORT    = 5,
    SIGMA_JENTRY_TXN_ERROR    = 6,
} sigma_jentry_type_t;

typedef struct {
    sigma_u64           timestamp_ns;
    sigma_jentry_type_t type;
    char                package_name[128];
    char                package_version[32];
    char                client_pid[16];        /* who initiated                  */
    char                initiating_unit[128];  /* e.g. "sigma-update.timer"      */
    int                 progress_pct;
    char                message[512];
    int                 error_code;
    sigma_u8            prev_mac[32];          /* HMAC-SHA256 of previous entry  */
    sigma_u8            entry_mac[32];         /* HMAC-SHA256 of this entry      */
} sigma_pkg_jentry_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

void sigma_pkg_journal_init(const char* journal_path);

/* Append a sealed journal entry */
void sigma_pkg_journal_write(const sigma_pkg_jentry_t* entry);

/* Convenience helper — fills timestamp, client info, prev_mac, entry_mac automatically */
void sigma_pkg_journal_log(sigma_jentry_type_t type,
                            const char* package_name,
                            const char* version,
                            int         progress_pct,
                            const char* message,
                            int         error_code);

/* Query entries for a package (newest first). Returns count written to out. */
int sigma_pkg_journal_query(const char*          package_name,
                             sigma_u64            since_ns,
                             sigma_u64            until_ns,
                             sigma_pkg_jentry_t*  out,
                             int                  max);

/* Verify the HMAC chain is intact (detect tampering) */
int sigma_pkg_journal_verify(const char* journal_path);
