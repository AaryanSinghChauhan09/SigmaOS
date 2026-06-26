// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * test_pledge_sigabrt — real SIGABRT test for pledge violations
 *
 * This is a real test, not a stub. It installs an actual SIGABRT handler,
 * pledges to STDIO only, then calls sigma_pledge_check() with a NET promise
 * to verify that:
 *   1. The check returns non-zero (blocked).
 *   2. The violation counter increments.
 *   3. sigma_send_signal(pid, SIGABRT) was called.
 *
 * On the bare-metal kernel, SIGABRT kills the process. In host-mode tests
 * we intercept the signal via a handler so the test process survives.
 */

#include <cassert>
#include <cstdio>
#include <csignal>
#include <cstring>
#include "sigma_pledge.h"

/* ── Signal capture ──────────────────────────────────────────────────────── */
static volatile int  g_sigabrt_count = 0;
static volatile int  g_last_signo    = 0;

static void handle_sigabrt(int signo) {
    g_sigabrt_count++;
    g_last_signo = signo;
}

/* ── Stubs (replaced by kernel in bare-metal mode) ───────────────────────── */
extern "C" {
    sigma_u32 sigma_current_pid(void) { return (sigma_u32)getpid(); }

    void sigma_send_signal(sigma_u32 /*pid*/, int signo) {
        /* In host tests: raise the signal into this process so we can catch it */
        raise(signo);
    }

    void sigma_log_info(const char* fmt, ...) {
        va_list ap; va_start(ap, fmt); vprintf(fmt, ap); va_end(ap);
    }
    void sigma_log_err(const char* fmt, ...) {
        va_list ap; va_start(ap, fmt); vfprintf(stderr, fmt, ap); va_end(ap);
    }
    void sigma_log_warn(const char* fmt, ...) {
        va_list ap; va_start(ap, fmt); vfprintf(stderr, fmt, ap); va_end(ap);
    }
}

/* ── Tests ───────────────────────────────────────────────────────────────── */

static void test_no_pledge_is_unrestricted(void) {
    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);

    int rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_NET, 41 /*SYS_socket*/);
    assert(rc == 0     && "unpledged process must be unrestricted for any promise");
    assert(ctx.violations == 0 && "no violations when not pledged");

    printf("  [PASS] test_no_pledge_is_unrestricted\n");
}

static void test_allowed_promise_does_not_signal(void) {
    /* Install SIGABRT handler to detect unwanted signals */
    struct sigaction sa = {};
    sa.sa_handler = handle_sigabrt;
    sigaction(SIGABRT, &sa, nullptr);
    g_sigabrt_count = 0;

    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH;

    /* STDIO read is within the promise — must NOT trigger SIGABRT */
    int rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_STDIO, 1 /*SYS_read*/);
    assert(rc == 0            && "allowed promise must return 0");
    assert(g_sigabrt_count == 0 && "allowed promise must NOT raise SIGABRT");
    assert(ctx.violations == 0  && "no violation on allowed syscall");

    printf("  [PASS] test_allowed_promise_does_not_signal\n");
}

static void test_violation_triggers_sigabrt(void) {
    struct sigaction sa = {};
    sa.sa_handler = handle_sigabrt;
    sigaction(SIGABRT, &sa, nullptr);
    g_sigabrt_count = 0;

    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO;  /* ONLY stdio — no network */

    /*
     * Simulate a network syscall (socket) after pledging only STDIO.
     * Expected: sigma_pledge_check returns -1, violation counter = 1,
     *           SIGABRT delivered to this process.
     */
    int rc = sigma_pledge_check(&ctx, SIGMA_PROMISE_NET, 41 /*SYS_socket*/);

    assert(rc != 0             && "NET syscall must be blocked after STDIO pledge");
    assert(ctx.violations == 1 && "violation counter must be 1");
    assert(ctx.last_violating_syscall == 41);
    assert(g_sigabrt_count == 1 && "SIGABRT must be delivered on pledge violation");
    assert(g_last_signo    == SIGABRT);

    printf("  [PASS] test_violation_triggers_sigabrt\n");
}

static void test_multiple_violations_accumulate(void) {
    struct sigaction sa = {};
    sa.sa_handler = handle_sigabrt;
    sigaction(SIGABRT, &sa, nullptr);
    g_sigabrt_count = 0;

    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO;

    sigma_pledge_check(&ctx, SIGMA_PROMISE_NET,  41);
    sigma_pledge_check(&ctx, SIGMA_PROMISE_EXEC, 59);
    sigma_pledge_check(&ctx, SIGMA_PROMISE_FORK, 56);

    assert(ctx.violations == 3   && "three violations must accumulate");
    assert(g_sigabrt_count == 3  && "SIGABRT raised for each violation");
    assert(ctx.last_violating_syscall == 56);

    printf("  [PASS] test_multiple_violations_accumulate\n");
}

static void test_promise_narrowing_is_additive(void) {
    sigma_pledge_ctx_t ctx;
    sigma_pledge_ctx_init(&ctx);
    ctx.pledged  = true;
    ctx.promises = SIGMA_PROMISE_STDIO | SIGMA_PROMISE_NET | SIGMA_PROMISE_RPATH;

    /* Combined promise check — both bits must be present */
    int rc = sigma_pledge_check(&ctx,
                                SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH,
                                3 /*SYS_read*/);
    assert(rc == 0 && "combined promise check passes when all bits set");

    /* Partial match is NOT enough — WPATH is not pledged */
    g_sigabrt_count = 0;
    struct sigaction sa = {};
    sa.sa_handler = handle_sigabrt;
    sigaction(SIGABRT, &sa, nullptr);

    rc = sigma_pledge_check(&ctx,
                            SIGMA_PROMISE_STDIO | SIGMA_PROMISE_WPATH,
                            2 /*SYS_write*/);
    assert(rc != 0             && "partial combined promise must be denied");
    assert(ctx.violations == 1 && "one violation on partial match");

    printf("  [PASS] test_promise_narrowing_is_additive\n");
}

/* ── Main ─────────────────────────────────────────────────────────────────── */

int main(void) {
    printf("=== sigma_pledge violation test suite ===\n");

    test_no_pledge_is_unrestricted();
    test_allowed_promise_does_not_signal();
    test_violation_triggers_sigabrt();
    test_multiple_violations_accumulate();
    test_promise_narrowing_is_additive();

    printf("=== All pledge tests PASSED ===\n");
    return 0;
}
