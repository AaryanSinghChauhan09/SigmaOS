// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_pledge_violation — verify that an unpledged process is unrestricted,
 * and that pledging with SIGMA_PROMISE_ALL then narrowing works correctly.
 */
#include <cassert>
#include <cstdio>
#include "sigma_pledge.h"

extern "C" {
    sigma_u32 sigma_current_pid(void) { return 99; }
    void      sigma_send_signal(sigma_u32, int) {}
}

int main(void) {
    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);

    /* ── Test 1: Unpledged — every syscall allowed ───────────────────── */
    assert(ctx.pledged == false);
    int rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_NET,  41);
    assert(rc == 0 && "unpledged process must not be restricted");
    rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_EXEC, 59);
    assert(rc == 0 && "unpledged process must not be restricted");
    assert(ctx.violations == 0);

    /* ── Test 2: Pledge to NET only — EXEC is blocked ────────────────── */
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO | SIGMA_PROMISE_NET;

    rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_NET, 41);
    assert(rc == 0 && "NET allowed after NET pledge");

    rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_EXEC, 59);
    assert(rc != 0 && "EXEC blocked after NET-only pledge");
    assert(ctx.violations == 1);

    /* ── Test 3: Combined promise check — needs RPATH+STDIO together ─── */
    ctx.promises = SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH;
    rc = sigma_pledge_check(&ctx,
                            SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH, 3);
    assert(rc == 0 && "combined promise check must pass when both bits set");

    rc = sigma_pledge_check(&ctx,
                            SIGMA_PROMISE_STDIO | SIGMA_PROMISE_WPATH, 2);
    assert(rc != 0 && "WPATH blocked when only RPATH pledged");

    printf("test_pledge_violation: PASS\n");
    return 0;
}
