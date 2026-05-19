/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SIGNAL HANDLING SUBSYSTEM
 * =============================================================================
 * Inspired by: Linux kernel kernel/signal.c
 *              POSIX.1-2017 signal semantics (sigaction, sigprocmask)
 *              OpenBSD signal hardening (W^X enforcement on signal stacks)
 * =============================================================================
 * Provides: Signal registration, delivery, masking, and default handlers.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

/* Signal numbers (POSIX-compatible subset) */
#define SIGMA_SIGHUP     1
#define SIGMA_SIGINT     2
#define SIGMA_SIGQUIT    3
#define SIGMA_SIGILL     4
#define SIGMA_SIGTRAP    5
#define SIGMA_SIGABRT    6
#define SIGMA_SIGBUS     7
#define SIGMA_SIGFPE     8
#define SIGMA_SIGKILL    9
#define SIGMA_SIGUSR1   10
#define SIGMA_SIGSEGV   11
#define SIGMA_SIGUSR2   12
#define SIGMA_SIGPIPE   13
#define SIGMA_SIGALRM   14
#define SIGMA_SIGTERM   15
#define SIGMA_SIGCHLD   17
#define SIGMA_SIGCONT   18
#define SIGMA_SIGSTOP   19
#define SIGMA_SIGTSTP   20
#define SIGMA_NSIG      32

/* Signal actions */
#define SIGMA_SIG_DFL   ((sigma_sighandler_t)0)
#define SIGMA_SIG_IGN   ((sigma_sighandler_t)1)

typedef void (*sigma_sighandler_t)(int signo);

typedef struct {
    sigma_sighandler_t handler;
    sigma_u32          flags;
    sigma_u32          blocked;   /* 1 = masked */
} sigma_sigaction_t;

typedef struct {
    sigma_u32          pending;   /* bitmask of pending signals */
    sigma_u32          blocked;   /* bitmask of blocked signals */
    sigma_sigaction_t  actions[SIGMA_NSIG];
} sigma_signal_state_t;

/* Per-process signal table (simplified: single-process for now) */
static sigma_signal_state_t signal_state;

static const char* signal_name(int signo) {
    switch (signo) {
        case SIGMA_SIGHUP:  return "SIGHUP";
        case SIGMA_SIGINT:  return "SIGINT";
        case SIGMA_SIGQUIT: return "SIGQUIT";
        case SIGMA_SIGILL:  return "SIGILL";
        case SIGMA_SIGTRAP: return "SIGTRAP";
        case SIGMA_SIGABRT: return "SIGABRT";
        case SIGMA_SIGBUS:  return "SIGBUS";
        case SIGMA_SIGFPE:  return "SIGFPE";
        case SIGMA_SIGKILL: return "SIGKILL";
        case SIGMA_SIGUSR1: return "SIGUSR1";
        case SIGMA_SIGSEGV: return "SIGSEGV";
        case SIGMA_SIGUSR2: return "SIGUSR2";
        case SIGMA_SIGPIPE: return "SIGPIPE";
        case SIGMA_SIGALRM: return "SIGALRM";
        case SIGMA_SIGTERM: return "SIGTERM";
        case SIGMA_SIGCHLD: return "SIGCHLD";
        case SIGMA_SIGCONT: return "SIGCONT";
        case SIGMA_SIGSTOP: return "SIGSTOP";
        case SIGMA_SIGTSTP: return "SIGTSTP";
        default:            return "UNKNOWN";
    }
}

void sigma_signal_init(void) {
    sigma_memset(&signal_state, 0, sizeof(signal_state));
    /* SIGKILL and SIGSTOP cannot be caught or blocked */
    for (int i = 0; i < SIGMA_NSIG; i++) {
        signal_state.actions[i].handler = SIGMA_SIG_DFL;
    }
    sigma_printf("[signal] Signal subsystem initialized (%d signals registered)\n", SIGMA_NSIG);
}

sigma_sighandler_t sigma_signal_register(int signo, sigma_sighandler_t handler) {
    if (signo < 1 || signo >= SIGMA_NSIG) return SIGMA_SIG_DFL;
    /* SIGKILL and SIGSTOP cannot be overridden (POSIX mandate) */
    if (signo == SIGMA_SIGKILL || signo == SIGMA_SIGSTOP) {
        sigma_printf("[signal] ERR: Cannot override %s (POSIX constraint)\n", signal_name(signo));
        return SIGMA_SIG_DFL;
    }
    sigma_sighandler_t old = signal_state.actions[signo].handler;
    signal_state.actions[signo].handler = handler;
    sigma_printf("[signal] Handler registered for %s\n", signal_name(signo));
    return old;
}

void sigma_signal_mask(int signo) {
    if (signo >= 1 && signo < SIGMA_NSIG && signo != SIGMA_SIGKILL && signo != SIGMA_SIGSTOP) {
        signal_state.blocked |= (1u << signo);
        sigma_printf("[signal] %s masked\n", signal_name(signo));
    }
}

void sigma_signal_unmask(int signo) {
    if (signo >= 1 && signo < SIGMA_NSIG) {
        signal_state.blocked &= ~(1u << signo);
        sigma_printf("[signal] %s unmasked\n", signal_name(signo));
    }
}

int sigma_signal_send(sigma_u32 target_pid, int signo) {
    if (signo < 1 || signo >= SIGMA_NSIG) return -1;
    (void)target_pid; /* Simplified: single-process kernel */

    sigma_printf("[signal] Sending %s to PID %u\n", signal_name(signo), target_pid);

    /* Check if signal is blocked */
    if (signal_state.blocked & (1u << signo)) {
        signal_state.pending |= (1u << signo);
        sigma_printf("[signal] %s is blocked — queued as pending\n", signal_name(signo));
        return 0;
    }

    /* Deliver immediately */
    sigma_sighandler_t h = signal_state.actions[signo].handler;
    if (h == SIGMA_SIG_IGN) {
        sigma_printf("[signal] %s ignored (SIG_IGN)\n", signal_name(signo));
    } else if (h == SIGMA_SIG_DFL) {
        /* Default action */
        switch (signo) {
            case SIGMA_SIGCHLD:
            case SIGMA_SIGCONT:
                sigma_printf("[signal] %s default: ignored\n", signal_name(signo));
                break;
            case SIGMA_SIGKILL:
            case SIGMA_SIGTERM:
            case SIGMA_SIGSEGV:
            case SIGMA_SIGABRT:
                sigma_printf("[signal] %s default: TERMINATE process\n", signal_name(signo));
                break;
            case SIGMA_SIGSTOP:
            case SIGMA_SIGTSTP:
                sigma_printf("[signal] %s default: STOP process\n", signal_name(signo));
                break;
            default:
                sigma_printf("[signal] %s default: terminate\n", signal_name(signo));
                break;
        }
    } else {
        sigma_printf("[signal] Invoking user handler for %s\n", signal_name(signo));
        h(signo);
    }
    return 0;
}

void sigma_signal_deliver_pending(void) {
    sigma_u32 deliverable = signal_state.pending & ~signal_state.blocked;
    if (deliverable == 0) return;

    for (int i = 1; i < SIGMA_NSIG; i++) {
        if (deliverable & (1u << i)) {
            signal_state.pending &= ~(1u << i);
            sigma_printf("[signal] Delivering pending %s\n", signal_name(i));
            sigma_signal_send(0, i);
        }
    }
}
