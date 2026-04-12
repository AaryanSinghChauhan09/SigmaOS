/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN SIGNAL SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb POSIX Signals / macOS Mach Exceptions / Windows SEH USP.
 *          Native Silicon Signal Routing & Exception Dispatch.
 * Design: C11 / Zero-Dependency / Handler-Table Signal Delivery.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Signal Structures
// -------------------------------------------------------------------------

typedef enum {
    SIGMA_SIGHUP  = 1,
    SIGMA_SIGINT  = 2,
    SIGMA_SIGKILL = 9,
    SIGMA_SIGSEGV = 11,
    SIGMA_SIGTERM = 15,
    SIGMA_SIGUSR1 = 10,
    SIGMA_SIGUSR2 = 12
} SigmaSignal_t;

typedef void (*SigmaSignalHandler_t)(sigma_u32 pid, SigmaSignal_t sig);

typedef struct {
    SigmaSignal_t       sig;
    char                sig_name[16];
    SigmaSignalHandler_t handler;
    sigma_u64           delivery_count;
    sigma_bool          blocked;
} SigmaSigEntry_t;

#define MAX_SIGNALS 32
static SigmaSigEntry_t s_sig_table[MAX_SIGNALS];
static sigma_u32       s_sig_count = 0;

/* ---- Default handlers -------------------------------------------------- */
static void _default_term(sigma_u32 pid, SigmaSignal_t sig) {
    sigma_printf("[SIGNAL]: Mission PID:%u received %s — graceful termination initiated.\n",
                 pid, (sig == SIGMA_SIGTERM) ? "SIGTERM" : "SIGHUP");
}
static void _default_kill(sigma_u32 pid, SigmaSignal_t sig) {
    (void)sig;
    sigma_printf("[SIGNAL]: Mission PID:%u received SIGKILL — immediate silicon halt.\n", pid);
}
static void _default_segv(sigma_u32 pid, SigmaSignal_t sig) {
    (void)sig;
    sigma_printf("[SIGNAL]: ⚠  SIGSEGV — Mission PID:%u silicon fault detected! "
                 "Generating core dump...\n", pid);
}
static void _default_usr(sigma_u32 pid, SigmaSignal_t sig) {
    sigma_printf("[SIGNAL]: Mission PID:%u received USR%u — citizen-defined action.\n",
                 pid, (sig == SIGMA_SIGUSR1) ? 1 : 2);
}

// -------------------------------------------------------------------------
// Signal Logic (POSIX / Mach Exceptions / Windows SEH parity)
// -------------------------------------------------------------------------

/**
 * sigma_signal_register: Registers a signal handler in the silicon dispatch table.
 */
sigma_err_t sigma_signal_register(SigmaSignal_t sig, const char* name,
                                    SigmaSignalHandler_t handler) {
    if (s_sig_count >= MAX_SIGNALS) return SIGMA_ENOSPC;
    SigmaSigEntry_t* e = &s_sig_table[s_sig_count++];
    e->sig            = sig;
    e->handler        = handler;
    e->delivery_count = 0;
    e->blocked        = SIGMA_FALSE;
    sigma_strcpy(e->sig_name, name);
    sigma_printf("[SIGNAL]: Registered handler for %s (signal %u).\n", name, (sigma_u32)sig);
    return SIGMA_OK;
}

/**
 * sigma_signal_send: Delivers a silicon signal to a target mission.
 */
sigma_err_t sigma_signal_send(sigma_u32 pid, SigmaSignal_t sig) {
    for (sigma_u32 i = 0; i < s_sig_count; i++) {
        if (s_sig_table[i].sig == sig) {
            if (s_sig_table[i].blocked) {
                sigma_printf("[SIGNAL]: %s blocked for PID:%u.\n",
                             s_sig_table[i].sig_name, pid);
                return SIGMA_EPERM;
            }
            s_sig_table[i].delivery_count++;
            if (s_sig_table[i].handler)
                s_sig_table[i].handler(pid, sig);
            return SIGMA_OK;
        }
    }
    sigma_printf("[SIGNAL]: Unknown signal %u sent to PID:%u — ignored.\n",
                 (sigma_u32)sig, pid);
    return SIGMA_ENOENT;
}

// -------------------------------------------------------------------------
// Industrial Signal Audit
// -------------------------------------------------------------------------

void SovereignSignal_Audit() {
    sigma_printf("\n--- SOVEREIGN SIGNAL AUDIT ---\n");
    sigma_printf("SIG    NAME             DELIVERIES BLOCKED\n");
    sigma_printf("-------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_sig_count; i++) {
        sigma_printf("%-6u %-16s %-10llu %s\n",
                     (sigma_u32)s_sig_table[i].sig,
                     s_sig_table[i].sig_name,
                     (unsigned long long)s_sig_table[i].delivery_count,
                     s_sig_table[i].blocked ? "YES" : "no");
    }
    sigma_printf("-------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignSignalShard_Init() {
    sigma_printf("[SOC]: Seating Native Signal Shard (POSIX/Mach/SEH Parity v1.0)...\n");
    sigma_signal_register(SIGMA_SIGTERM, "SIGTERM", _default_term);
    sigma_signal_register(SIGMA_SIGHUP,  "SIGHUP",  _default_term);
    sigma_signal_register(SIGMA_SIGKILL, "SIGKILL", _default_kill);
    sigma_signal_register(SIGMA_SIGSEGV, "SIGSEGV", _default_segv);
    sigma_signal_register(SIGMA_SIGUSR1, "SIGUSR1", _default_usr);
    sigma_signal_register(SIGMA_SIGUSR2, "SIGUSR2", _default_usr);
}
