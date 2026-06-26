// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_supervisor.h — s6-style service supervision state machine
 *
 * Replaces the 5-loop sigma_init_watchdog() stub with a real state machine:
 *   DOWN → STARTING → UP → FINISH → LASTUP → LASTFINISH → (restart with backoff)
 *
 * Key s6 patterns adopted:
 *   - selfpipe for signal forwarding (avoids signal handler races)
 *   - notify_fd: service writes 1 byte when READY (not just when started)
 *   - Exponential backoff: 1s → 2s → 4s → 8s → cap at 30s
 *   - ctrl_fd: control socket accepts 'u'=up 'd'=down 't'=term 'k'=kill
 */
#include <sys/types.h>
#include <stdint.h>
#include <stdbool.h>

typedef enum {
    SVC_DOWN,         /* not running, not desired                           */
    SVC_STARTING,     /* exec'd, awaiting readiness notification            */
    SVC_UP,           /* running and ready (service wrote to notify_fd)     */
    SVC_FINISH,       /* finish script running after crash                  */
    SVC_LASTUP,       /* service exited, restart scheduled                  */
    SVC_LASTFINISH,   /* finish script ran, waiting for next start          */
} sigma_svc_state_t;

typedef struct {
    char              name[64];
    char              exec_path[256];
    char              finish_path[256];  /* optional: run on crash           */
    pid_t             pid;
    int               notify_fd;         /* write 1 byte here when ready     */
    int               ctrl_fd;           /* 'u','d','t','k' control commands */
    int               selfpipe[2];       /* signal→fd bridge                 */
    sigma_svc_state_t state;
    uint32_t          restart_count;
    uint64_t          last_start_ns;
    uint64_t          backoff_ms;        /* exponential: 1→2→4→8→30s cap    */
    bool              oneshot;           /* don't restart on exit if true     */
    bool              desired_up;        /* control plane desired state       */
} sigma_svc_t;

/* Main supervision loop — never returns (mirrors s6-supervise main()) */
void sigma_supervisor_run(sigma_svc_t* svc);

/* Send a control command: 'u'=up 'd'=down 't'=term 'k'=kill */
int  sigma_svc_control(const char* svc_name, char cmd);

/* Read and parse a .service.toml fault contract file */
int  sigma_svc_load(sigma_svc_t* out, const char* toml_path);
