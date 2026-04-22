/*
 * =========================================================================
 * S SIGMAOS userland/init/sigma_init.h
 * =========================================================================
 * Modular PID-1 service manager — gap-closes Linux systemd, macOS launchd,
 * BSD rc, and Windows SCM in a single zero-glibc C11 module.
 * =========================================================================
 * DESIGN PRINCIPLES (absorbed from competitors):
 *   • Dependency DAG   (systemd Wants/Requires)
 *   • Socket activation (systemd .socket units)
 *   • Run-level targets (SysV / systemd targets)
 *   • Auto-restart     (launchd KeepAlive)
 *   • Zombie reaping   (POSIX PID-1 contract)
 *   • Cgroup v2 hooks  (Linux resource isolation)
 * =========================================================================
 */

#ifndef SIGMA_INIT_H
#define SIGMA_INIT_H

/* ── Self-contained primitives ─────────────────────────────────────────── */
typedef unsigned int  si_u32;
typedef unsigned char si_bool;
#define SI_TRUE  ((si_bool)1)
#define SI_FALSE ((si_bool)0)
#define SI_NULL  ((void*)0)

/* ── Limits ────────────────────────────────────────────────────────────── */
#define SIGMA_INIT_MAX_SERVICES   128
#define SIGMA_INIT_NAME_LEN        48
#define SIGMA_INIT_PATH_LEN       256

/* ── Service states (mirrors systemd unit ActiveState) ─────────────────── */
typedef enum {
    SVC_INACTIVE  = 0,
    SVC_ACTIVATING,
    SVC_ACTIVE,
    SVC_DEACTIVATING,
    SVC_FAILED,
    SVC_ZOMBIE       /* child exited, not yet reaped */
} sigma_svc_state_t;

/* ── Restart policy (absorbed from launchd + systemd) ─────────────────── */
typedef enum {
    RESTART_NO = 0,
    RESTART_ON_FAILURE,
    RESTART_ALWAYS
} sigma_restart_policy_t;

/* ── Run-level targets ─────────────────────────────────────────────────── */
typedef enum {
    TARGET_RESCUE    = 0,  /* Single-user safe mode     */
    TARGET_MULTIUSER = 1,  /* Text multi-user           */
    TARGET_GRAPHICAL = 2   /* Full GUI + network        */
} sigma_run_target_t;

/* ── Service descriptor ─────────────────────────────────────────────────── */
typedef struct {
    char                  name[SIGMA_INIT_NAME_LEN];
    char                  exec_path[SIGMA_INIT_PATH_LEN];
    char                  requires[SIGMA_INIT_NAME_LEN]; /* dependency */
    si_u32                pid;
    sigma_svc_state_t     state;
    sigma_restart_policy_t restart;
    si_bool               socket_activated;  /* lazy-load on connection */
    si_bool               cgroup_isolated;   /* cgroup v2 namespace     */
    si_u32                restart_count;
    si_u32                max_restarts;      /* 0 = unlimited           */
} sigma_service_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void sigma_init_bootstrap(sigma_run_target_t target);
void sigma_init_register(const char* name, const char* path,
                         const char* requires,
                         sigma_restart_policy_t restart,
                         si_bool sock_act);
void sigma_init_start(const char* name);
void sigma_init_stop(const char* name);
void sigma_init_restart(const char* name);
void sigma_init_reap_zombies(void);
void sigma_init_status(void);
void sigma_init_event_loop(void);   /* PID-1 blocking loop */

#endif /* SIGMA_INIT_H */
