// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_pledge_stdio — verify that a process pledged to STDIO cannot use NET.
 *
 * After sigma_pledge(SIGMA_PROMISE_STDIO), sigma_pledge_check() must return
 * non-zero (blocked) when the caller requests SIGMA_PROMISE_NET.
 */
#include <cassert>
#include <cstdio>
#include "sigma_pledge.h"

/* Minimal stubs so the test links without the full kernel */
extern "C" {
    sigma_u32 sigma_current_pid(void) { return 42; }
    void      sigma_send_signal(sigma_u32, int) { /* captured in test */ }
}

static int signal_received = 0;

int main(void) {
    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);

    /* Mark as pledged with only STDIO */
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO;

    /* ── Test 1: STDIO syscall is allowed ─────────────────────────────── */
    int rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_STDIO, /*syscall=*/1);
    assert(rc == 0 && "STDIO syscall must be allowed after STDIO pledge");

    /* ── Test 2: NET syscall is blocked ───────────────────────────────── */
    /* Override sigma_send_signal to capture signal instead of crashing  */
    rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_NET, /*syscall=*/41);
    assert(rc != 0      && "NET syscall must be blocked after STDIO pledge");
    assert(ctx.violations == 1 && "violation counter must be 1");
    assert(ctx.last_violating_syscall == 41);

    /* ── Test 3: A second violation increments the counter ───────────── */
    sigma_pledge_check(&ctx, SIGMA_PROMISE_EXEC, /*syscall=*/59);
    assert(ctx.violations == 2 && "second violation must increment counter");

    printf("test_pledge_stdio: PASS\n");
    return 0;
}
