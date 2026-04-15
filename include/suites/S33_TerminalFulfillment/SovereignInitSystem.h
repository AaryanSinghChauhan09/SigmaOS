/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN INIT SYSTEM (v1.0 — PURE C11)
 * =========================================================================
 * Mission: PID-1 bootstrap, service supervision, runlevel management.
 * Inspired By: OpenRC (Gentoo), runit (Void), s6 (Skarnet), systemd (Fedora)
 * Principle: Zero-dependency. Minimal. Deterministic. Sovereign.
 * =========================================================================
 */

#ifndef SOVEREIGN_INIT_SYSTEM_H
#define SOVEREIGN_INIT_SYSTEM_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * Service states (mirrors OpenRC / systemd unit states)
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_SVC_STOPPED   = 0,
    SIGMA_SVC_STARTING  = 1,
    SIGMA_SVC_RUNNING   = 2,
    SIGMA_SVC_STOPPING  = 3,
    SIGMA_SVC_CRASHED   = 4,
    SIGMA_SVC_DISABLED  = 5,
} SigmaSvcState_t;

/* -------------------------------------------------------------------------
 * Service descriptor — one per managed service
 * ---------------------------------------------------------------------- */
#define SIGMA_SVC_NAME_MAX   64
#define SIGMA_SVC_CMD_MAX   256
#define SIGMA_MAX_SERVICES   64
#define SIGMA_MAX_DEPS        8

typedef struct {
    char           name[SIGMA_SVC_NAME_MAX];
    char           exec[SIGMA_SVC_CMD_MAX];        /* Binary / script path */
    SigmaSvcState_t state;
    pid_t          pid;
    sigma_bool     restart_on_crash;               /* runit-style supervision */
    sigma_u32      restart_count;
    char           deps[SIGMA_MAX_DEPS][SIGMA_SVC_NAME_MAX]; /* Dependency names */
    sigma_u32      dep_count;
} SigmaService_t;

/* -------------------------------------------------------------------------
 * Runlevels (OpenRC-inspired)
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_RL_SYSINIT  = 0,   /* Hardware init, mount root */
    SIGMA_RL_BOOT     = 1,   /* Core daemons (networking, logging) */
    SIGMA_RL_DEFAULT  = 2,   /* Normal multi-user */
    SIGMA_RL_SHUTDOWN = 3,   /* Teardown */
} SigmaRunlevel_t;

/* -------------------------------------------------------------------------
 * Init system context
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaService_t   services[SIGMA_MAX_SERVICES];
    sigma_u32        svc_count;
    SigmaRunlevel_t  current_runlevel;
} SigmaInitCtx_t;

/* -------------------------------------------------------------------------
 * Public API
 * ---------------------------------------------------------------------- */
void         sigma_init_setup   (SigmaInitCtx_t *ctx);
sigma_err_t  sigma_svc_register (SigmaInitCtx_t *ctx, const char *name,
                                  const char *exec, sigma_bool restart);
sigma_err_t  sigma_svc_start    (SigmaInitCtx_t *ctx, const char *name);
sigma_err_t  sigma_svc_stop     (SigmaInitCtx_t *ctx, const char *name);
sigma_err_t  sigma_svc_restart  (SigmaInitCtx_t *ctx, const char *name);
sigma_err_t  sigma_svc_status   (SigmaInitCtx_t *ctx, const char *name,
                                  SigmaSvcState_t *out_state);
void         sigma_init_reap    (SigmaInitCtx_t *ctx);        /* Wait for zombies */
void         sigma_init_switch_runlevel(SigmaInitCtx_t *ctx, SigmaRunlevel_t rl);
void         SovereignInitSystem_Init(void);

#endif /* SOVEREIGN_INIT_SYSTEM_H */
