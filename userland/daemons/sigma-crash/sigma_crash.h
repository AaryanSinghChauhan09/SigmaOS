// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_crash.h — Crash reporter
 *
 * Captures register state + stack trace on SIGSEGV/SIGABRT/SIGFPE.
 * Writes structured crash dump to /sigma/var/crashes/.
 * Notifies user via sigma-bus (sigma.Notifications).
 * Sovereign: never sends data off-device (no Sentry, no Crashlytics).
 */
#include <signal.h>
#include <sigma_kernel_types.h>

#define SIGMA_CRASH_DIR "/sigma/var/crashes"

typedef struct {
    sigma_u64 timestamp_ns;
    int       signal;
    sigma_u64 rip;         /* instruction pointer at crash              */
    sigma_u64 rsp;         /* stack pointer                             */
    sigma_u64 rbp;         /* frame pointer                             */
    sigma_u64 regs[16];    /* all general-purpose registers             */
    char      exe_path[256];
    sigma_u32 pid;
    char      backtrace[8192];  /* symbolicated stack trace             */
    char      crash_id[64];     /* UUID for cross-referencing            */
} sigma_crash_report_t;

/* Install crash handler for the calling process */
void sigma_crash_handler_install(void);

/* Called by the signal handler — do not call directly */
void sigma_crash_handle(int sig, siginfo_t* info, void* ctx);

/* Write a crash report to /sigma/var/crashes/<crash_id>.json */
int sigma_crash_write(const sigma_crash_report_t* report);

/* Symbolicate a backtrace using debug symbols from /sigma/var/debug/ */
int sigma_crash_symbolicate(sigma_u64* addrs, int count,
                              char* out, sigma_size_t out_len);

/* Notify the user via sigma-bus that a crash occurred */
int sigma_crash_notify(const sigma_crash_report_t* report);

/* List crash reports (for crash reporter UI) */
int sigma_crash_list(char** ids_out, int max);
