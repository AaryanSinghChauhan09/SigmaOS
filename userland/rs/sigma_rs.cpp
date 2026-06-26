// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_rs.cpp — Reincarnation Server (MINIX 3 rs server-inspired)
 *
 * Monitors all registered SigmaOS services. On crash: restarts with backoff.
 * Essential services: kernel panic if unrecoverable. Non-essential: log and stop.
 * This is the layer above s6-style supervision — it owns the service registry.
 */
#include "sigma_rs.h"
#include "sigma_log.h"
#include <sys/wait.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include <errno.h>
#include <stdio.h>

extern "C" void sigma_panic(const char* msg);

#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

/* ── Service registry ─────────────────────────────────────────────────────── */

static sigma_rs_entry_t sigma_services[] = {
    { "sigma-ds",      "/sigma/sbin/sigma-ds",      true,  5, 0, -1, false },
    { "sigma-trustd",  "/sigma/sbin/sigma-trustd",  true,  3, 0, -1, false },
    { "sigma-healthd", "/sigma/sbin/sigma-healthd", false, 10, 0, -1, false },
    { "sigma-apid",    "/sigma/sbin/sigma-apid",    false, 5, 0, -1, false },
    { "sigma-pkg",     "/sigma/sbin/sigma-pkg",     false, 10, 0, -1, false },
    { "zenith-browser","/sigma/bin/zenith_browser", false, 3, 0, -1, false },
};
static const int SERVICE_COUNT = (int)ARRAY_SIZE(sigma_services);

/* ── Spawn a service ──────────────────────────────────────────────────────── */

static pid_t sigma_rs_spawn(const char* exec_path) {
    pid_t pid = fork();
    if (pid == 0) {
        execl(exec_path, exec_path, NULL);
        sigma_log_err("[sigma-rs] execl(%s) failed: %s\n", exec_path, strerror(errno));
        _exit(127);
    }
    return pid;
}

/* ── Main monitoring loop ─────────────────────────────────────────────────── */

void sigma_rs_monitor_loop(void) {
    /* Start all services */
    for (int i = 0; i < SERVICE_COUNT; i++) {
        sigma_rs_entry_t* svc = &sigma_services[i];
        svc->pid   = sigma_rs_spawn(svc->exec_path);
        svc->alive = (svc->pid > 0);
        sigma_log_info("[sigma-rs] started %s pid=%d\n", svc->name, (int)svc->pid);
    }

    /* Monitor loop — block on any child exit */
    for (;;) {
        int wstatus;
        pid_t dead = waitpid(-1, &wstatus, 0);
        if (dead <= 0) continue;

        for (int i = 0; i < SERVICE_COUNT; i++) {
            sigma_rs_entry_t* svc = &sigma_services[i];
            if (svc->pid != dead) continue;

            int code = WIFEXITED(wstatus) ? WEXITSTATUS(wstatus) : -1;
            svc->alive         = false;
            svc->restart_count++;

            sigma_log_warn("[sigma-rs] %s exited (code=%d restart=%u/%u)\n",
                           svc->name, code, svc->restart_count, svc->max_restarts);

            if (svc->restart_count > svc->max_restarts) {
                if (svc->essential) {
                    char msg[128];
                    snprintf(msg, sizeof(msg),
                             "[sigma-rs] Essential service %s unrecoverable — kernel halt",
                             svc->name);
                    sigma_panic(msg);
                }
                sigma_log_err("[sigma-rs] %s: max restarts exceeded — giving up\n",
                              svc->name);
                break;
            }

            /* Restart with a simple 1-second delay */
            sleep(1);
            svc->pid   = sigma_rs_spawn(svc->exec_path);
            svc->alive = (svc->pid > 0);
            sigma_log_info("[sigma-rs] restarted %s pid=%d\n",
                           svc->name, (int)svc->pid);
            break;
        }
    }
}
