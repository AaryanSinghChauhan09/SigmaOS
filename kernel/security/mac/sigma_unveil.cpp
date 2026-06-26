// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_unveil.cpp — per-process filesystem restriction implementation
 * (OpenBSD unveil-inspired).
 *
 * Called by VFS layer on every open/stat/access/exec operation:
 *   int rc = sigma_unveil_check(&current->unveil, path, SIGMA_UV_READ);
 *   if (rc != 0) return rc;   // returns -ENOENT to caller — path "doesn't exist"
 */

#include "sigma_unveil.h"
#include "sigma_log.h"

/* Freestanding string helpers from klib */
extern "C" int   sigma_strncmp(const char* a, const char* b, sigma_size_t n);
extern "C" void  sigma_strncpy(char* dst, const char* src, sigma_size_t n);
extern "C" sigma_size_t sigma_strlen(const char* s);

void sigma_unveil_ctx_init(sigma_unveil_ctx_t* ctx) {
    ctx->count  = 0;
    ctx->locked = false;
    for (int i = 0; i < SIGMA_UV_MAX_ENTRIES; i++) {
        ctx->entries[i].path[0] = '\0';
        ctx->entries[i].perms   = 0;
    }
}

int sigma_unveil(sigma_unveil_ctx_t* ctx, const char* path, sigma_u8 perms) {
    if (!ctx || !path) return -1;

    /* Cannot add entries after locking */
    if (ctx->locked) {
        sigma_log_warn("[sigma-unveil] sigma_unveil() called after lock — rejected\n");
        return -1; /* -EPERM */
    }

    if (ctx->count >= SIGMA_UV_MAX_ENTRIES) {
        sigma_log_err("[sigma-unveil] unveil table full (%d entries)\n",
                      SIGMA_UV_MAX_ENTRIES);
        return -1; /* -ENOMEM */
    }

    sigma_unveil_entry_t* e = &ctx->entries[ctx->count++];
    sigma_strncpy(e->path, path, sizeof(e->path) - 1);
    e->path[sizeof(e->path) - 1] = '\0';
    e->perms = perms;

    sigma_log_info("[sigma-unveil] unveiled '%s' perms=0x%02x (total %d)\n",
                   path, (int)perms, ctx->count);
    return 0;
}

void sigma_unveil_lock(sigma_unveil_ctx_t* ctx) {
    if (!ctx) return;
    ctx->locked = true;
    sigma_log_info("[sigma-unveil] table locked — %d path(s) visible\n", ctx->count);
}

int sigma_unveil_check(const sigma_unveil_ctx_t* ctx,
                       const char*              path,
                       sigma_u8                 required) {
    /* Not locked — no restriction */
    if (!ctx || !ctx->locked) return 0;

    sigma_size_t path_len = sigma_strlen(path);

    for (int i = 0; i < ctx->count; i++) {
        const sigma_unveil_entry_t* e = &ctx->entries[i];
        sigma_size_t prefix_len = sigma_strlen(e->path);

        /* Path matches if it equals the prefix, or is under it (prefix ends
         * with / or the next char in path is /). */
        if (path_len >= prefix_len &&
            sigma_strncmp(path, e->path, prefix_len) == 0 &&
            (path[prefix_len] == '\0' || path[prefix_len] == '/' ||
             e->path[prefix_len - 1] == '/'))
        {
            /* Found a matching prefix — check permissions */
            if ((e->perms & required) == required) {
                return 0; /* allowed */
            }
            /* Path is visible but access mode is denied */
            sigma_log_warn(
                "[sigma-unveil] DENIED '%s' required=0x%02x have=0x%02x\n",
                path, (int)required, (int)e->perms);
            return -1; /* -EACCES — could also return -ENOENT for full stealth */
        }
    }

    /* Path is not in the unveil table — return -ENOENT (path "doesn't exist") */
    sigma_log_info("[sigma-unveil] HIDDEN '%s' (not in unveil table)\n", path);
    return -2; /* -ENOENT */
}
