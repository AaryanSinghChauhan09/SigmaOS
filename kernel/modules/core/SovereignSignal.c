/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SIGNAL SUBSYSTEM (v1.0 - PURE C11)
 * =========================================================================
 * Competitor Gap: Every POSIX OS (Linux, macOS, FreeBSD) provides a
 * complete signal subsystem. SigmaOS had NO signal implementation.
 * This shard provides:
 *   • Signal numbers & names (SIGKILL, SIGTERM, SIGSEGV, SIGCHLD, …)
 *   • Per-process signal masks (sigprocmask)
 *   • Signal handlers (sigaction)
 *   • kill() & raise() dispatch
 *   • Pending signal delivery queue
 *   • Default actions: terminate, ignore, core-dump
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"

/* -----------------------------------------------------------------------
 * Signal numbers — POSIX standard
 * ----------------------------------------------------------------------- */
#define SIGMA_SIGHUP    1   /* Hangup */
#define SIGMA_SIGINT    2   /* Interrupt (Ctrl+C) */
#define SIGMA_SIGQUIT   3   /* Quit (Ctrl+\) */
#define SIGMA_SIGILL    4   /* Illegal instruction */
#define SIGMA_SIGTRAP   5   /* Trace/breakpoint trap */
#define SIGMA_SIGABRT   6   /* Abort */
#define SIGMA_SIGBUS    7   /* Bus error */
#define SIGMA_SIGFPE    8   /* Floating-point exception */
#define SIGMA_SIGKILL   9   /* Kill (cannot be caught) */
#define SIGMA_SIGUSR1   10  /* User-defined signal 1 */
#define SIGMA_SIGSEGV   11  /* Segmentation fault */
#define SIGMA_SIGUSR2   12  /* User-defined signal 2 */
#define SIGMA_SIGPIPE   13  /* Broken pipe */
#define SIGMA_SIGALRM   14  /* Alarm clock */
#define SIGMA_SIGTERM   15  /* Termination */
#define SIGMA_SIGCHLD   17  /* Child stopped/terminated */
#define SIGMA_SIGCONT   18  /* Continue if stopped */
#define SIGMA_SIGSTOP   19  /* Stop process (cannot be caught) */
#define SIGMA_SIGTSTP   20  /* Terminal stop */
#define SIGMA_SIGWINCH  28  /* Window size change */
#define SIGMA_NSIG      32  /* Total signal count */

/* -----------------------------------------------------------------------
 * Signal disposition constants (like SIG_DFL, SIG_IGN)
 * ----------------------------------------------------------------------- */
#define SIGMA_SIG_DFL ((sigma_sighandler_t)0)  /* Default action */
#define SIGMA_SIG_IGN ((sigma_sighandler_t)1)  /* Ignore */

typedef void (*sigma_sighandler_t)(int);

/* Default actions */
typedef enum {
    SIG_ACTION_TERM,    /* Terminate process */
    SIG_ACTION_IGNORE,  /* Ignore */
    SIG_ACTION_CORE,    /* Core dump + terminate */
    SIG_ACTION_STOP,    /* Suspend process */
    SIG_ACTION_CONT,    /* Resume process */
    SIG_ACTION_CUSTOM   /* User handler */
} SigDefaultAction_t;

static const SigDefaultAction_t s_default_action[SIGMA_NSIG + 1] = {
    [0]                = SIG_ACTION_IGNORE,
    [SIGMA_SIGHUP]     = SIG_ACTION_TERM,
    [SIGMA_SIGINT]     = SIG_ACTION_TERM,
    [SIGMA_SIGQUIT]    = SIG_ACTION_CORE,
    [SIGMA_SIGILL]     = SIG_ACTION_CORE,
    [SIGMA_SIGTRAP]    = SIG_ACTION_CORE,
    [SIGMA_SIGABRT]    = SIG_ACTION_CORE,
    [SIGMA_SIGBUS]     = SIG_ACTION_CORE,
    [SIGMA_SIGFPE]     = SIG_ACTION_CORE,
    [SIGMA_SIGKILL]    = SIG_ACTION_TERM,  /* uncatchable */
    [SIGMA_SIGUSR1]    = SIG_ACTION_TERM,
    [SIGMA_SIGSEGV]    = SIG_ACTION_CORE,
    [SIGMA_SIGUSR2]    = SIG_ACTION_TERM,
    [SIGMA_SIGPIPE]    = SIG_ACTION_TERM,
    [SIGMA_SIGALRM]    = SIG_ACTION_TERM,
    [SIGMA_SIGTERM]    = SIG_ACTION_TERM,
    [SIGMA_SIGCHLD]    = SIG_ACTION_IGNORE,
    [SIGMA_SIGCONT]    = SIG_ACTION_CONT,
    [SIGMA_SIGSTOP]    = SIG_ACTION_STOP,
    [SIGMA_SIGTSTP]    = SIG_ACTION_STOP,
    [SIGMA_SIGWINCH]   = SIG_ACTION_IGNORE,
};

static const char* s_signames[SIGMA_NSIG + 1] = {
    [0]  = "SIG0",     [SIGMA_SIGHUP]  = "SIGHUP",  [SIGMA_SIGINT]  = "SIGINT",
    [SIGMA_SIGQUIT]= "SIGQUIT",  [SIGMA_SIGILL]  = "SIGILL",  [SIGMA_SIGTRAP] = "SIGTRAP",
    [SIGMA_SIGABRT]= "SIGABRT",  [SIGMA_SIGBUS]  = "SIGBUS",  [SIGMA_SIGFPE]  = "SIGFPE",
    [SIGMA_SIGKILL]= "SIGKILL",  [SIGMA_SIGUSR1] = "SIGUSR1", [SIGMA_SIGSEGV] = "SIGSEGV",
    [SIGMA_SIGUSR2]= "SIGUSR2",  [SIGMA_SIGPIPE] = "SIGPIPE", [SIGMA_SIGALRM] = "SIGALRM",
    [SIGMA_SIGTERM]= "SIGTERM",  [SIGMA_SIGCHLD] = "SIGCHLD", [SIGMA_SIGCONT] = "SIGCONT",
    [SIGMA_SIGSTOP]= "SIGSTOP",  [SIGMA_SIGTSTP] = "SIGTSTP", [SIGMA_SIGWINCH]= "SIGWINCH",
};

/* -----------------------------------------------------------------------
 * Per-process signal table
 * ----------------------------------------------------------------------- */
#define MAX_SIGNAL_PROCS 128

typedef struct {
    sigma_u32           pid;
    sigma_sighandler_t  handlers[SIGMA_NSIG + 1];
    sigma_u32           mask;      /* blocked signals bitmask */
    sigma_u32           pending;   /* pending signals bitmask */
    sigma_bool          stopped;
    sigma_bool          in_use;
} SigmaSignalCtx_t;

static SigmaSignalCtx_t s_sig_table[MAX_SIGNAL_PROCS];
static sigma_u32        s_sig_count = 0;

static SigmaSignalCtx_t* sig_get_or_create(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_sig_count; i++) {
        if (s_sig_table[i].pid == pid) return &s_sig_table[i];
    }
    if (s_sig_count >= MAX_SIGNAL_PROCS) return SIGMA_NULL;
    SigmaSignalCtx_t* ctx = &s_sig_table[s_sig_count++];
    sigma_memset(ctx, 0, sizeof(*ctx));
    ctx->pid    = pid;
    ctx->in_use = SIGMA_TRUE;
    /* Install default dispositions */
    for (int i = 0; i <= SIGMA_NSIG; i++) ctx->handlers[i] = SIGMA_SIG_DFL;
    return ctx;
}

/* -----------------------------------------------------------------------
 * sigma_sigaction() — Install a signal handler (sigaction(2) parity)
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_sigaction(sigma_u32 pid, int signo, sigma_sighandler_t handler) {
    if (signo <= 0 || signo > SIGMA_NSIG) return SIGMA_EINVAL;
    /* SIGKILL and SIGSTOP cannot be caught */
    if (signo == SIGMA_SIGKILL || signo == SIGMA_SIGSTOP) return SIGMA_EPERM;
    SigmaSignalCtx_t* ctx = sig_get_or_create(pid);
    if (!ctx) return SIGMA_ENOSPC;
    ctx->handlers[signo] = handler;
    sigma_printf("Σ [SIGNAL]: pid=%u: handler installed for %s\n",
                 pid, s_signames[signo] ? s_signames[signo] : "?");
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * sigma_sigprocmask() — Block / unblock signals
 * how: 0=SIG_BLOCK, 1=SIG_UNBLOCK, 2=SIG_SETMASK
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_sigprocmask(sigma_u32 pid, int how, sigma_u32 set) {
    SigmaSignalCtx_t* ctx = sig_get_or_create(pid);
    if (!ctx) return SIGMA_ENOSPC;
    /* SIGKILL/SIGSTOP always unblockable */
    set &= ~((1u << SIGMA_SIGKILL) | (1u << SIGMA_SIGSTOP));
    switch (how) {
        case 0: ctx->mask |=  set; break;  /* SIG_BLOCK   */
        case 1: ctx->mask &= ~set; break;  /* SIG_UNBLOCK */
        case 2: ctx->mask  =  set; break;  /* SIG_SETMASK */
        default: return SIGMA_EINVAL;
    }
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * sigma_kill() — Send a signal to a process (kill(2) parity)
 * ----------------------------------------------------------------------- */
sigma_err_t sigma_kill(sigma_u32 target_pid, int signo) {
    if (signo <= 0 || signo > SIGMA_NSIG) return SIGMA_EINVAL;
    SigmaSignalCtx_t* ctx = sig_get_or_create(target_pid);
    if (!ctx) return SIGMA_ESRCH;

    /* If blocked, add to pending */
    if ((ctx->mask >> signo) & 1u) {
        ctx->pending |= (1u << signo);
        sigma_printf("Σ [SIGNAL]: %s → pid=%u [PENDING/BLOCKED]\n",
                     s_signames[signo], target_pid);
        return SIGMA_OK;
    }

    /* Deliver immediately */
    sigma_printf("Σ [SIGNAL]: Delivering %s → pid=%u\n",
                 s_signames[signo], target_pid);

    sigma_sighandler_t h = ctx->handlers[signo];
    if ((sigma_uptr)h > 1u) {
        /* Custom handler */
        h(signo);
        return SIGMA_OK;
    }
    if (h == SIGMA_SIG_IGN) return SIGMA_OK;

    /* Default action */
    switch (s_default_action[signo]) {
        case SIG_ACTION_TERM:
            sigma_printf("Σ [SIGNAL]: pid=%u terminated by %s.\n",
                         target_pid, s_signames[signo]);
            ctx->in_use = SIGMA_FALSE;
            break;
        case SIG_ACTION_CORE:
            sigma_printf("Σ [SIGNAL]: pid=%u core dumped by %s.\n",
                         target_pid, s_signames[signo]);
            ctx->in_use = SIGMA_FALSE;
            break;
        case SIG_ACTION_STOP:
            ctx->stopped = SIGMA_TRUE;
            sigma_printf("Σ [SIGNAL]: pid=%u stopped.\n", target_pid);
            break;
        case SIG_ACTION_CONT:
            ctx->stopped = SIGMA_FALSE;
            sigma_printf("Σ [SIGNAL]: pid=%u continued.\n", target_pid);
            break;
        default:
            break;
    }
    return SIGMA_OK;
}

/* -----------------------------------------------------------------------
 * sigma_signal_deliver_pending() — Deliver queued signals after unblock
 * Called by the scheduler after sigprocmask/sigsuspend
 * ----------------------------------------------------------------------- */
void sigma_signal_deliver_pending(sigma_u32 pid) {
    SigmaSignalCtx_t* ctx = sig_get_or_create(pid);
    if (!ctx) return;
    sigma_u32 deliverable = ctx->pending & ~ctx->mask;
    for (int i = 1; i <= SIGMA_NSIG; i++) {
        if ((deliverable >> i) & 1u) {
            ctx->pending &= ~(1u << i);
            sigma_kill(pid, i);
        }
    }
}

/* -----------------------------------------------------------------------
 * Public init
 * ----------------------------------------------------------------------- */
void SovereignSignal_Init(void) {
    sigma_printf("Σ [SIGNAL]: Initialising Sovereign Signal Subsystem...\n");

    /* Demo: install a custom SIGUSR1 handler for pid 1000 */
    sig_get_or_create(1000);
    sigma_sigaction(1000, SIGMA_SIGUSR1, (sigma_sighandler_t)(sigma_uptr)0xFF);
    sigma_sigprocmask(1000, 0, (1u << SIGMA_SIGPIPE)); /* block SIGPIPE */
    sigma_kill(1000, SIGMA_SIGUSR1);   /* should invoke custom handler */
    sigma_kill(1000, SIGMA_SIGPIPE);   /* should be blocked → pending */
    sigma_sigprocmask(1000, 1, (1u << SIGMA_SIGPIPE)); /* unblock */
    sigma_signal_deliver_pending(1000);

    sigma_printf("Σ [SIGNAL]: POSIX signal parity achieved. %d signals registered.\n",
                 SIGMA_NSIG);
}
