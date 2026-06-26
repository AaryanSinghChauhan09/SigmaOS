// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_unveil_lock — verify that sigma_unveil() after sigma_unveil_lock()
 * is rejected, and that table overflow is handled gracefully.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include "sigma_unveil.h"

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

    /* ── Test 1: unveil after lock must return -EPERM ─────────────────── */
    assert(sigma_unveil(&ctx, "/sigma/data", SIGMA_UV_READ) == 0);
    sigma_unveil_lock(&ctx);

    int rc = sigma_unveil(&ctx, "/sigma/extra", SIGMA_UV_READ);
    assert(rc != 0 && "unveil after lock must be rejected");

    /* ── Test 2: table overflow ───────────────────────────────────────── */
    sigma_unveil_ctx_t big;
    sigma_unveil_ctx_init(&big);

    char path[32];
    for (int i = 0; i < SIGMA_UV_MAX_ENTRIES; i++) {
        snprintf(path, sizeof(path), "/sigma/p%d", i);
        assert(sigma_unveil(&big, path, SIGMA_UV_READ) == 0);
    }
    /* One more — must fail */
    rc = sigma_unveil(&big, "/sigma/overflow", SIGMA_UV_READ);
    assert(rc != 0 && "table overflow must return error");
    assert(big.count == SIGMA_UV_MAX_ENTRIES && "count must not exceed max");

    printf("test_unveil_lock: PASS\n");
    return 0;
}
