// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * =============================================================================
 * Σ SIGMAOS: PID 1 EVENT LOOP (signalfd-style, systemd / OpenRC inspired)
 * =============================================================================
 * PID 1 must never return. This module implements the infinite event loop that:
 *   - Reaps zombie children (SIGCHLD)  via waitpid(-1, WNOHANG)
 *   - Restarts failed registered services (max 3 retries, 1 s delay)
 *   - Handles SIGTERM / SIGINT for ordered shutdown
 *
 * On bare-metal this uses the kernel's internal signal queue primitives rather
 * than the POSIX signalfd(2) syscall (which requires a running VFS). The API
 * is kept POSIX-like so the logic can be unit-tested on Linux hosts.
 * =============================================================================
 */

#include "../sigma_libc.h"
#include "sigma_service.h"

/* Maximum restart attempts before a service is permanently marked FAILED. */
#define SIGMA_SVC_MAX_RESTARTS 3

/* Milliseconds between restart attempts (kernel tick-based). */
#define SIGMA_SVC_RESTART_DELAY_MS 1000

/* ---- Kernel primitives provided by arch/x86_64 --------------------------- */
extern void sigma_cpu_halt(void);          /* execute HLT, yield until IRQ  */
extern int  sigma_signal_pending(void);    /* non-zero if a signal is queued */
extern int  sigma_signal_read(int* signo); /* dequeue one signal, return 0=ok */
extern int  sigma_waitpid_nohang(void);    /* waitpid(-1,NULL,WNOHANG); >0=pid */
extern void sigma_msleep(int ms);          /* busy-sleep using kernel tick    */

/* ---- Forward declarations from init.c ------------------------------------ */
extern void sigma_service_start_all(void);
extern int  sigma_service_get_failed_count(void);
extern int  sigma_service_restart_failed(void);  /* returns number restarted  */

/* ---- Shutdown ------------------------------------------------------------ */
void sigma_init_shutdown(const char* reason) {
    sigma_printf("[init] Shutdown: %s\n", reason);
    sigma_printf("[init] Stopping all services...\n");
    /* TODO: call stop hooks in reverse dependency order */
    sigma_printf("[init] System halted.\n");
    for (;;) {
        sigma_cpu_halt();
    }
}

/* ---- Main PID 1 event loop ----------------------------------------------- */
void sigma_init_event_loop(void) {
    sigma_printf("[init] PID 1 event loop started.\n");

    for (;;) {
        /* ------------------------------------------------------------------ */
        /* 1. Reap all zombie children that have exited since last iteration.  */
        /* ------------------------------------------------------------------ */
        while (sigma_waitpid_nohang() > 0) {
            /* Each call reaps one zombie; loop until none left. */
        }

        /* ------------------------------------------------------------------ */
        /* 2. Restart any registered services that have failed.               */
        /* ------------------------------------------------------------------ */
        if (sigma_service_get_failed_count() > 0) {
            sigma_printf("[init] Detected failed service(s) — attempting restart.\n");
            int restarted = sigma_service_restart_failed();
            if (restarted > 0) {
                sigma_printf("[init] %d service(s) restarted.\n", restarted);
            }
        }

        /* ------------------------------------------------------------------ */
        /* 3. Process pending signals.                                         */
        /* ------------------------------------------------------------------ */
        while (sigma_signal_pending()) {
            int signo = 0;
            if (sigma_signal_read(&signo) != 0) break;

            if (signo == SIGMA_SIGCHLD) {
                /* Already reaped above; nothing extra to do. */
            } else if (signo == SIGMA_SIGTERM || signo == SIGMA_SIGINT) {
                sigma_init_shutdown("SIGTERM/SIGINT received");
                /* sigma_init_shutdown() never returns */
            } else {
                sigma_printf("[init] Ignoring signal %d\n", signo);
            }
        }

        /* ------------------------------------------------------------------ */
        /* 4. Yield CPU until the next interrupt / tick.                       */
        /*    This prevents PID 1 from burning 100 % CPU while idle.          */
        /* ------------------------------------------------------------------ */
        sigma_cpu_halt();
    }
    /* Unreachable — loop above never exits. */
}
