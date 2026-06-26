// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_pledge.cpp — implementation of the per-process syscall restriction
 * system (OpenBSD pledge-inspired).
 *
 * Integrated into the syscall dispatcher in kernel/core/syscall/dispatcher.cpp:
 *   Before every handler call, add:
 *     sigma_pledge_check(&current->pledge, SIGMA_PROMISE_NET, SYSCALL_SOCKET);
 */

#include "sigma_pledge.h"
#include "sigma_log.h"
#include "sigma_kernel_types.h"

/* Forward declarations for kernel primitives */
extern "C" sigma_u32 sigma_current_pid(void);
extern "C" void      sigma_send_signal(sigma_u32 pid, int signo);

#define SIGABRT 6

void sigma_pledge_ctx_init(sigma_pledge_ctx_t* ctx) {
    ctx->promises                = SIGMA_PROMISE_ALL;
    ctx->pledged                 = false;
    ctx->violations              = 0;
    ctx->last_violating_syscall  = 0;
}

int sigma_pledge(sigma_u64 promises) {
    /* In a real implementation this syscall looks up current->pledge.
     * Here we express the contract: pledging can only restrict, not expand. */
    (void)promises;
    /* Kernel-side: retrieve the calling process's pledge context, narrow it.
     * Actual process table lookup is arch-specific; stubbed for portability. */
    sigma_log_info("[sigma-pledge] pid %u pledged with promises=0x%llx\n",
                   sigma_current_pid(), (unsigned long long)promises);
    return 0;
}

int sigma_pledge_check(sigma_pledge_ctx_t* ctx,
                       sigma_u64          required_promise,
                       sigma_u32          syscall_nr) {
    /* Not pledged — no restriction applies. */
    if (!ctx->pledged) {
        return 0;
    }

    /* All required promises are present — allow. */
    if ((ctx->promises & required_promise) == required_promise) {
        return 0;
    }

    /* ── Violation path ────────────────────────────────────────────────── */
    ctx->violations++;
    ctx->last_violating_syscall = syscall_nr;

    sigma_log_err(
        "[sigma-pledge] VIOLATION pid=%u syscall=%u "
        "required=0x%llx active=0x%llx (violation #%u) — SIGABRT\n",
        sigma_current_pid(),
        syscall_nr,
        (unsigned long long)required_promise,
        (unsigned long long)ctx->promises,
        ctx->violations);

    /* Kill the offending process immediately — same as OpenBSD kern_pledge.c */
    sigma_send_signal(sigma_current_pid(), SIGABRT);

    return -1; /* -EPERM */
}
