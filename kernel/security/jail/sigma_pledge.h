// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_pledge.h — Per-process syscall restriction (OpenBSD pledge-inspired)
 *
 * After a process calls sigma_pledge(promises), any syscall whose required
 * promise is NOT in the active promise set causes an immediate SIGABRT and
 * is logged to the audit ring buffer.  The restriction is irreversible —
 * once pledged a process cannot expand its own promise set.
 *
 * Usage (from a userland process):
 *   sigma_pledge(SIGMA_PROMISE_STDIO | SIGMA_PROMISE_RPATH);
 *   // From this point: only read/write/close/stat/open-for-read are allowed.
 *   // Any attempt to call socket(), exec(), fork(), etc. → SIGABRT.
 */

#include <sigma_kernel_types.h>

/* ── Promise bits ─────────────────────────────────────────────────────────── */
#define SIGMA_PROMISE_NONE    (0ULL)
#define SIGMA_PROMISE_STDIO   (1ULL << 0)  /* read/write/close/fstat on open fds  */
#define SIGMA_PROMISE_RPATH   (1ULL << 1)  /* open/stat/access on existing paths  */
#define SIGMA_PROMISE_WPATH   (1ULL << 2)  /* open/write/create/ftruncate         */
#define SIGMA_PROMISE_CPATH   (1ULL << 3)  /* create/rename/unlink/mkdir          */
#define SIGMA_PROMISE_NET     (1ULL << 4)  /* socket/bind/connect/send/recv       */
#define SIGMA_PROMISE_DNS     (1ULL << 5)  /* DNS resolution (subset of net)      */
#define SIGMA_PROMISE_EXEC    (1ULL << 6)  /* execve                              */
#define SIGMA_PROMISE_FORK    (1ULL << 7)  /* fork/clone                          */
#define SIGMA_PROMISE_PROC    (1ULL << 8)  /* getpid/getppid/kill-self/setpgid    */
#define SIGMA_PROMISE_ID      (1ULL << 9)  /* getuid/getgid/setresuid/setresgid   */
#define SIGMA_PROMISE_MMAP    (1ULL << 10) /* mmap/mprotect (anon only)           */
#define SIGMA_PROMISE_TIMER   (1ULL << 11) /* nanosleep/clock_gettime             */
#define SIGMA_PROMISE_IOCTL   (1ULL << 12) /* ioctl (heavily filtered)            */
#define SIGMA_PROMISE_SIGNAL  (1ULL << 13) /* sigaction/sigprocmask               */
#define SIGMA_PROMISE_ALL     (~0ULL)       /* unrestricted — for kernel threads   */

/* ── Per-process pledge context (embedded in sigma_process_t) ─────────────── */
typedef struct {
    sigma_u64 promises;      /* active promise bitmask                        */
    bool      pledged;       /* has process called sigma_pledge()?            */
    sigma_u32 violations;    /* total blocked syscall attempts since pledge   */
    sigma_u32 last_violating_syscall; /* syscall number of last violation     */
} sigma_pledge_ctx_t;

/* ── API ──────────────────────────────────────────────────────────────────── */

/*
 * sigma_pledge — called from userland to restrict the calling process.
 * Narrows the promise set (can only remove promises, never add).
 * Returns 0 on success, -EINVAL if trying to expand promises.
 */
int sigma_pledge(sigma_u64 promises);

/*
 * sigma_pledge_check — called by the syscall dispatcher on every kernel entry.
 *
 * @ctx              per-process pledge context
 * @required_promise the SIGMA_PROMISE_* bit(s) this syscall requires
 * @syscall_nr       syscall number (for audit log)
 *
 * Returns 0 if allowed, -EPERM if blocked.
 * On block: increments violation counter, logs to audit ring, sends SIGABRT.
 */
int sigma_pledge_check(sigma_pledge_ctx_t* ctx,
                       sigma_u64          required_promise,
                       sigma_u32          syscall_nr);

/* Initialise a pledge context to "not pledged" (all syscalls allowed). */
void sigma_pledge_ctx_init(sigma_pledge_ctx_t* ctx);
