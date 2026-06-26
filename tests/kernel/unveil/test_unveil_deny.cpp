// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_unveil_deny — verify that paths outside the unveil table return -ENOENT
 * and that paths inside it with the correct permission are allowed.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include "sigma_unveil.h"

/* klib stubs */
extern "C" {
    int sigma_strncmp(const char* a, const char* b, sigma_size_t n) {
        return strncmp(a, b, n);
    }
    void sigma_strncpy(char* d, const char* s, sigma_size_t n) {
        strncpy(d, s, n);
    }
    sigma_size_t sigma_strlen(const char* s) { return strlen(s); }
    void sigma_log_info(const char*, ...) {}
    void sigma_log_warn(const char*, ...) {}
    void sigma_log_err(const char*, ...)  {}
}

int main(void) {
    sigma_unveil_ctx_t ctx;
    sigma_unveil_ctx_init(&ctx);

    /* ── Test 1: unlocked context — everything allowed ───────────────── */
    assert(sigma_unveil_check(&ctx, "/etc/shadow", SIGMA_UV_READ) == 0);

    /* ── Test 2: add an entry, lock, check allowed path ─────────────── */
    assert(sigma_unveil(&ctx, "/sigma/data/app", SIGMA_UV_READ | SIGMA_UV_WRITE) == 0);
    assert(sigma_unveil(&ctx, "/sigma/lib",      SIGMA_UV_READ | SIGMA_UV_EXEC)  == 0);
    sigma_unveil_lock(&ctx);

    /* Path under /sigma/data/app — READ allowed */
    assert(sigma_unveil_check(&ctx, "/sigma/data/app/config.toml",
                              SIGMA_UV_READ) == 0);

    /* Path under /sigma/lib — EXEC allowed */
    assert(sigma_unveil_check(&ctx, "/sigma/lib/libc.so",
                              SIGMA_UV_EXEC) == 0);

    /* ── Test 3: path NOT in the table — must return -ENOENT (-2) ────── */
    int rc = sigma_unveil_check(&ctx, "/etc/shadow", SIGMA_UV_READ);
    assert(rc == -2 && "/etc/shadow must be hidden (ENOENT)");

    rc = sigma_unveil_check(&ctx, "/home/user/.ssh/id_rsa", SIGMA_UV_READ);
    assert(rc == -2 && "/home must be hidden");

    /* ── Test 4: path in table but wrong permission ───────────────────── */
    rc = sigma_unveil_check(&ctx, "/sigma/data/app/config.toml",
                            SIGMA_UV_EXEC);
    assert(rc != 0 && "EXEC on read-write-only path must be denied");

    printf("test_unveil_deny: PASS\n");
    return 0;
}
