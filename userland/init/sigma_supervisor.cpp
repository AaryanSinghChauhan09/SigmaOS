// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * sigma_supervisor.cpp — s6-style supervision state machine
 * (modeled on s6-supervise.c by Laurent Bercot)
 */
#include "sigma_supervisor.h"
#include "sigma_log.h"
#include <poll.h>
#include <signal.h>
#include <unistd.h>
#include <sys/wait.h>
#include <string.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

extern "C" uint64_t sigma_monotonic_ns(void);

#define BACKOFF_INITIAL_MS   1000
#define BACKOFF_MAX_MS      30000

/* ── Signal → selfpipe bridge (avoids handler races like s6) ─────────────── */
static int g_selfpipe_write = -1;

static void selfpipe_handler(int sig) {
    char c = (char)sig;
    if (g_selfpipe_write >= 0) write(g_selfpipe_write, &c, 1);
}

static void sigma_install_selfpipe(sigma_svc_t* svc) {
    pipe(svc->selfpipe);
    g_selfpipe_write = svc->selfpipe[1];
    signal(SIGCHLD, selfpipe_handler);
    signal(SIGTERM, selfpipe_handler);
    signal(SIGHUP,  selfpipe_handler);
}

/* ── Process lifecycle ───────────────────────────────────────────────────── */
static void sigma_supervisor_start(sigma_svc_t* svc) {
    svc->state         = SVC_STARTING;
    svc->last_start_ns = sigma_monotonic_ns();

    pid_t pid = fork();
    if (pid == 0) {
        /* Child — close supervisor fds, exec the service */
        close(svc->selfpipe[0]);
        close(svc->selfpipe[1]);
        execl(svc->exec_path, svc->exec_path, NULL);
        _exit(127);
    }
    if (pid < 0) {
        sigma_log_err("[supervisor] %s: fork failed: %s\n",
                      svc->name, strerror(errno));
        svc->state = SVC_LASTFINISH;
        return;
    }

    svc->pid = pid;
    sigma_log_info("[supervisor] %s: started pid=%d (attempt=%u)\n",
                   svc->name, (int)pid, svc->restart_count + 1);
}

static void sigma_supervisor_handle_chld(sigma_svc_t* svc) {
    int wstatus;
    pid_t dead = waitpid(-1, &wstatus, WNOHANG);
    if (dead <= 0 || dead != svc->pid) return;

    int exitcode = WIFEXITED(wstatus) ? WEXITSTATUS(wstatus) : -1;
    sigma_log_warn("[supervisor] %s: pid=%d exited (code=%d)\n",
                   svc->name, (int)dead, exitcode);

    svc->pid   = -1;
    svc->state = SVC_LASTFINISH;
    svc->restart_count++;
}

/* ── Timeout until next restart (for poll()) ─────────────────────────────── */
static int sigma_supervisor_next_timeout(const sigma_svc_t* svc) {
    if (svc->state != SVC_LASTFINISH) return -1; /* indefinite */
    uint64_t elapsed = sigma_monotonic_ns() - svc->last_start_ns;
    uint64_t wait_ns = svc->backoff_ms * 1000000ULL;
    if (elapsed >= wait_ns) return 0;
    return (int)((wait_ns - elapsed) / 1000000ULL);
}

/* ── Main supervision loop (s6-supervise pattern) ────────────────────────── */
void sigma_supervisor_run(sigma_svc_t* svc) {
    svc->backoff_ms = BACKOFF_INITIAL_MS;

    sigma_install_selfpipe(svc);

    /* Start immediately on first entry */
    if (svc->desired_up && !svc->oneshot) {
        sigma_supervisor_start(svc);
    }

    struct pollfd fds[3] = {
        { .fd = svc->selfpipe[0], .events = POLLIN },
        { .fd = svc->notify_fd,   .events = POLLIN },
        { .fd = svc->ctrl_fd,     .events = POLLIN },
    };

    for (;;) {
        int timeout = sigma_supervisor_next_timeout(svc);
        poll(fds, 3, timeout);

        /* ── Signal channel ───────────────────────────────────────────── */
        if (fds[0].revents & POLLIN) {
            char sig;
            read(svc->selfpipe[0], &sig, 1);
            if ((int)sig == SIGCHLD) {
                sigma_supervisor_handle_chld(svc);
            } else if ((int)sig == SIGTERM) {
                sigma_log_info("[supervisor] %s: SIGTERM — shutting down\n",
                               svc->name);
                if (svc->pid > 0) kill(svc->pid, SIGTERM);
                svc->state = SVC_DOWN;
                return;
            }
        }

        /* ── Readiness notification (s6 sd_notify pattern) ───────────── */
        if (fds[1].revents & POLLIN && svc->state == SVC_STARTING) {
            char c;
            read(svc->notify_fd, &c, 1);
            svc->state = SVC_UP;
            /* Reset backoff on successful start */
            svc->backoff_ms = BACKOFF_INITIAL_MS;
            svc->restart_count = 0;
            sigma_log_info("[supervisor] %s: READY (notified)\n", svc->name);
        }

        /* ── Control commands ─────────────────────────────────────────── */
        if (fds[2].revents & POLLIN) {
            char cmd;
            read(svc->ctrl_fd, &cmd, 1);
            if (cmd == 'd') { svc->desired_up = false; if (svc->pid > 0) kill(svc->pid, SIGTERM); }
            if (cmd == 'u') { svc->desired_up = true;  if (svc->state == SVC_DOWN) sigma_supervisor_start(svc); }
            if (cmd == 't') { if (svc->pid > 0) kill(svc->pid, SIGTERM); }
            if (cmd == 'k') { if (svc->pid > 0) kill(svc->pid, SIGKILL); }
        }

        /* ── Backoff restart ──────────────────────────────────────────── */
        if (svc->state == SVC_LASTFINISH && svc->desired_up && !svc->oneshot) {
            uint64_t elapsed = sigma_monotonic_ns() - svc->last_start_ns;
            if (elapsed >= svc->backoff_ms * 1000000ULL) {
                sigma_supervisor_start(svc);
                /* Exponential backoff: cap at BACKOFF_MAX_MS */
                svc->backoff_ms = (svc->backoff_ms * 2 > BACKOFF_MAX_MS)
                                    ? BACKOFF_MAX_MS
                                    : svc->backoff_ms * 2;
            }
        }
    }
}

int sigma_svc_control(const char* svc_name, char cmd) {
    char sock_path[128];
    snprintf(sock_path, sizeof(sock_path), "/run/sigma/sv/%s/ctrl", svc_name);
    int fd = open(sock_path, O_WRONLY | O_NONBLOCK);
    if (fd < 0) return -errno;
    write(fd, &cmd, 1);
    close(fd);
    return 0;
}
