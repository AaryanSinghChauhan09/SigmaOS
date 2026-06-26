// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_unveil.h — Per-process filesystem restriction (OpenBSD unveil-inspired)
 *
 * After a process builds its unveil table and calls sigma_unveil_lock(), any
 * VFS operation on a path NOT covered by the table returns -ENOENT — the path
 * appears to not exist at all.  This is stronger than chroot because it is
 * per-path, not per-root, and the denial is indistinguishable from absence.
 *
 * Usage (Zenith browser sandbox):
 *   sigma_unveil(&ctx, "/sigma/data/zenith", SIGMA_UV_READ | SIGMA_UV_WRITE);
 *   sigma_unveil(&ctx, "/sigma/lib",         SIGMA_UV_READ | SIGMA_UV_EXEC);
 *   sigma_unveil_lock(&ctx);
 *   // /etc/shadow, /home, /proc — all return ENOENT from this point.
 */

#include <sigma_kernel_types.h>

/* ── Permission bits ──────────────────────────────────────────────────────── */
#define SIGMA_UV_READ   0x01   /* open for read, stat, access              */
#define SIGMA_UV_WRITE  0x02   /* open for write, truncate                 */
#define SIGMA_UV_EXEC   0x04   /* execve, open with O_EXEC                 */
#define SIGMA_UV_CREATE 0x08   /* create, rename, unlink inside this path  */

/* Maximum number of unveiled paths per process (OpenBSD uses ~128 vnodes). */
#define SIGMA_UV_MAX_ENTRIES 128

/* ── Data structures ──────────────────────────────────────────────────────── */
typedef struct {
    char    path[256];   /* unveiled path prefix (directory or file)   */
    sigma_u8 perms;      /* OR of SIGMA_UV_* bits                      */
} sigma_unveil_entry_t;

typedef struct {
    sigma_unveil_entry_t entries[SIGMA_UV_MAX_ENTRIES];
    int                  count;
    bool                 locked;  /* true after sigma_unveil_lock()     */
} sigma_unveil_ctx_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/* Initialise to "not locked, no entries" — equivalent to unrestricted VFS. */
void sigma_unveil_ctx_init(sigma_unveil_ctx_t* ctx);

/*
 * sigma_unveil — add an entry to the unveil table.
 * Returns 0 on success, -EPERM if already locked, -ENOMEM if table full.
 */
int sigma_unveil(sigma_unveil_ctx_t* ctx, const char* path, sigma_u8 perms);

/*
 * sigma_unveil_lock — seal the table.
 * After this call, sigma_unveil() returns -EPERM and the VFS check is active.
 */
void sigma_unveil_lock(sigma_unveil_ctx_t* ctx);

/*
 * sigma_unveil_check — called by VFS on every path operation.
 *
 * @ctx           per-process unveil context
 * @path          absolute path being accessed
 * @required      SIGMA_UV_* bit(s) needed for this operation
 *
 * Returns 0 (allow) or -ENOENT (deny — path appears absent).
 * If ctx is not locked, always returns 0 (unrestricted).
 */
int sigma_unveil_check(const sigma_unveil_ctx_t* ctx,
                       const char*              path,
                       sigma_u8                 required);
