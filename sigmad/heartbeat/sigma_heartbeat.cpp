// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_heartbeat.cpp — Genode-style component liveness tracking
 * Detects STUCK services (alive but not making progress) — separate from
 * CRASHED services that sigma-rs handles via SIGCHLD.
 */
#include "sigma_heartbeat.h"
#include "sigma_log.h"
#include <signal.h>
#include <string.h>

#define MAX_ENTRIES 32
#define HEARTBEAT_INTERVAL_MS 500

extern "C" uint64_t sigma_clock_monotonic_ns(void);
extern "C" void     sigma_msleep(int ms);
extern "C" void     sigma_panic(const char* msg);

static sigma_heartbeat_entry_t g_entries[MAX_ENTRIES];
static int g_count = 0;

void sigma_heartbeat_register(const char* name, uint32_t pid,
                               uint64_t deadline_ns, bool vital) {
    if (g_count >= MAX_ENTRIES) return;
    sigma_heartbeat_entry_t* e = &g_entries[g_count++];
    strncpy(e->service_name, name, sizeof(e->service_name) - 1);
    e->pid           = pid;
    e->last_pong_ns  = sigma_clock_monotonic_ns();
    e->deadline_ns   = deadline_ns;
    e->stuck_count   = 0;
    e->vital         = vital;
    sigma_log_info("[heartbeat] registered %s pid=%u deadline=%llums vital=%d\n",
                   name, pid, (unsigned long long)(deadline_ns / 1000000), vital);
}

void sigma_heartbeat_pong(const char* name) {
    for (int i = 0; i < g_count; i++) {
        if (strncmp(g_entries[i].service_name, name,
                    sizeof(g_entries[i].service_name)) == 0) {
            g_entries[i].last_pong_ns = sigma_clock_monotonic_ns();
            g_entries[i].stuck_count  = 0;
            return;
        }
    }
}

void sigma_heartbeat_monitor_loop(void) {
    for (;;) {
        sigma_msleep(HEARTBEAT_INTERVAL_MS);
        uint64_t now = sigma_clock_monotonic_ns();

        for (int i = 0; i < g_count; i++) {
            sigma_heartbeat_entry_t* e = &g_entries[i];
            uint64_t age = now - e->last_pong_ns;

            if (age > e->deadline_ns) {
                e->stuck_count++;
                sigma_log_warn("[heartbeat] %s: stuck %llums (count=%u)\n",
                               e->service_name,
                               (unsigned long long)(age / 1000000),
                               e->stuck_count);

                if (e->stuck_count >= 3) {
                    sigma_log_err("[heartbeat] %s: STUCK — SIGKILL\n",
                                  e->service_name);
                    kill((pid_t)e->pid, SIGKILL);
                    /* sigma-rs will restart it */
                }

                if (e->vital && e->stuck_count >= 5) {
                    char msg[128];
                    snprintf(msg, sizeof(msg),
                             "[heartbeat] vital service %s unrecoverable — reset",
                             e->service_name);
                    sigma_panic(msg);
                }
            } else {
                e->stuck_count = 0;
            }
        }
    }
}
