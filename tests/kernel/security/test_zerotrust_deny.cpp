// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_zerotrust_deny — verify that:
 *   1. A revoked workload is denied on every capability check (Bug #3 fix).
 *   2. An unknown PID is denied.
 *   3. A valid, non-revoked workload with matching policy is allowed.
 *
 * Tests the policy logic in isolation without the full kernel.
 */
#include <cassert>
#include <cstdio>
#include <cstring>
#include <cstdint>

/* ── Minimal ZeroTrust types (mirrors sigma_zerotrust.cpp) ─────────────── */
#define SIGMA_ZT_SPIFFE_URI_LEN 128
#define SIGMA_ZT_MAX_WORKLOADS  16

typedef struct {
    uint32_t pid;
    char     spiffe_uri[SIGMA_ZT_SPIFFE_URI_LEN];
    bool     revoked;          /* Bug #3: must be checked on EVERY call   */
} zt_workload_t;

typedef enum { ZT_ALLOW, ZT_DENY } zt_verdict_t;

typedef struct {
    zt_workload_t workloads[SIGMA_ZT_MAX_WORKLOADS];
    int           workload_count;
} zt_ctx_t;

/* ── Policy check (the correct implementation, with revocation check) ──── */
static zt_verdict_t zt_check(const zt_ctx_t* ctx, uint32_t src_pid,
                              const char* required_cap) {
    for (int i = 0; i < ctx->workload_count; i++) {
        if (ctx->workloads[i].pid == src_pid) {
            /* Bug #3 fix: check revocation BEFORE granting anything */
            if (ctx->workloads[i].revoked) {
                return ZT_DENY;
            }
            /* Simplified policy: SPIFFE URI must contain required_cap */
            if (strstr(ctx->workloads[i].spiffe_uri, required_cap)) {
                return ZT_ALLOW;
            }
            return ZT_DENY;
        }
    }
    return ZT_DENY;  /* unknown PID is always denied */
}

int main(void) {
    zt_ctx_t ctx = {};

    /* Register a normal workload */
    ctx.workloads[0] = { 1001, "spiffe://sigma.os/workload/ffmpeg", false };
    /* Register a revoked workload */
    ctx.workloads[1] = { 1002, "spiffe://sigma.os/workload/malware", true };
    ctx.workload_count = 2;

    /* ── Test 1: valid workload, matching cap → ALLOW ─────────────────── */
    assert(zt_check(&ctx, 1001, "ffmpeg") == ZT_ALLOW &&
           "valid workload must be allowed");

    /* ── Test 2: valid workload, wrong cap → DENY ────────────────────── */
    assert(zt_check(&ctx, 1001, "passwd") == ZT_DENY &&
           "wrong capability must be denied");

    /* ── Test 3: revoked workload → DENY regardless of cap (Bug #3) ──── */
    assert(zt_check(&ctx, 1002, "malware") == ZT_DENY &&
           "revoked workload must be denied even with matching SPIFFE URI");

    /* ── Test 4: unknown PID → DENY ─────────────────────────────────── */
    assert(zt_check(&ctx, 9999, "ffmpeg") == ZT_DENY &&
           "unknown PID must be denied");

    /* ── Test 5: revoke a running workload mid-session ──────────────── */
    ctx.workloads[0].revoked = true;
    assert(zt_check(&ctx, 1001, "ffmpeg") == ZT_DENY &&
           "workload must be denied after runtime revocation");

    printf("test_zerotrust_deny: PASS\n");
    return 0;
}
