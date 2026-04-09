/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INIT SYSTEM — IMPLEMENTATION (v1.0 — PURE C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignInitSystem.h"

/* -------------------------------------------------------------------------
 * Helpers
 * ---------------------------------------------------------------------- */
static SigmaService_t *find_svc(SigmaInitCtx_t *ctx, const char *name) {
    for (sigma_u32 i = 0; i < ctx->svc_count; i++) {
        if (sigma_streq(ctx->services[i].name, name))
            return &ctx->services[i];
    }
    return SIGMA_NULL;
}

static const char *svc_state_str(SigmaSvcState_t s) {
    switch (s) {
        case SIGMA_SVC_STOPPED:  return "stopped";
        case SIGMA_SVC_STARTING: return "starting";
        case SIGMA_SVC_RUNNING:  return "running";
        case SIGMA_SVC_STOPPING: return "stopping";
        case SIGMA_SVC_CRASHED:  return "crashed";
        case SIGMA_SVC_DISABLED: return "disabled";
        default:                 return "unknown";
    }
}

/* -------------------------------------------------------------------------
 * sigma_init_setup — Zero-init the context and seed default runlevel
 * ---------------------------------------------------------------------- */
void sigma_init_setup(SigmaInitCtx_t *ctx) {
    sigma_memset(ctx, 0, sizeof(*ctx));
    ctx->current_runlevel = SIGMA_RL_SYSINIT;
    sigma_printf("Σ [INIT]: SigmaOS PID-1 context initialised.\n");
}

/* -------------------------------------------------------------------------
 * sigma_svc_register — Add a service descriptor
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_svc_register(SigmaInitCtx_t *ctx, const char *name,
                                const char *exec, sigma_bool restart) {
    if (ctx->svc_count >= SIGMA_MAX_SERVICES) return SIGMA_ENOSPC;
    if (find_svc(ctx, name))                   return SIGMA_EBUSY;

    SigmaService_t *svc = &ctx->services[ctx->svc_count++];
    sigma_memset(svc, 0, sizeof(*svc));
    sigma_strcpy(svc->name, name, SIGMA_SVC_NAME_MAX);
    sigma_strcpy(svc->exec, exec, SIGMA_SVC_CMD_MAX);
    svc->state            = SIGMA_SVC_STOPPED;
    svc->pid              = -1;
    svc->restart_on_crash = restart;

    sigma_printf("Σ [INIT]: Service registered: %s -> %s (restart=%s)\n",
                 name, exec, restart ? "yes" : "no");
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_svc_start
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_svc_start(SigmaInitCtx_t *ctx, const char *name) {
    SigmaService_t *svc = find_svc(ctx, name);
    if (!svc)                                return SIGMA_ENOENT;
    if (svc->state == SIGMA_SVC_RUNNING)     return SIGMA_OK;
    if (svc->state == SIGMA_SVC_DISABLED)    return SIGMA_EPERM;

    sigma_printf("Σ [INIT]: Starting service: %s (%s)\n", svc->name, svc->exec);
    svc->state = SIGMA_SVC_STARTING;

    /* In a live kernel: sigma_fork() + sigma_execve() here.
     * Simulated: assign a synthetic PID and mark RUNNING.             */
    svc->pid   = (pid_t)(1000 + (sigma_i32)(svc - ctx->services));
    svc->state = SIGMA_SVC_RUNNING;

    sigma_printf("Σ [INIT]: Service %s is now running (PID %d).\n",
                 svc->name, (int)svc->pid);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_svc_stop
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_svc_stop(SigmaInitCtx_t *ctx, const char *name) {
    SigmaService_t *svc = find_svc(ctx, name);
    if (!svc)                              return SIGMA_ENOENT;
    if (svc->state != SIGMA_SVC_RUNNING)   return SIGMA_EINVAL;

    sigma_printf("Σ [INIT]: Stopping service: %s (PID %d)\n",
                 svc->name, (int)svc->pid);
    svc->state = SIGMA_SVC_STOPPING;
    /* In a live kernel: send SIGTERM, wait(). */
    svc->state = SIGMA_SVC_STOPPED;
    svc->pid   = -1;
    sigma_printf("Σ [INIT]: Service %s stopped.\n", svc->name);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_svc_restart
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_svc_restart(SigmaInitCtx_t *ctx, const char *name) {
    sigma_err_t rc = sigma_svc_stop(ctx, name);
    if (rc != SIGMA_OK && rc != SIGMA_EINVAL) return rc;   /* ignore "not running" */
    return sigma_svc_start(ctx, name);
}

/* -------------------------------------------------------------------------
 * sigma_svc_status
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_svc_status(SigmaInitCtx_t *ctx, const char *name,
                              SigmaSvcState_t *out_state) {
    SigmaService_t *svc = find_svc(ctx, name);
    if (!svc) return SIGMA_ENOENT;
    *out_state = svc->state;
    sigma_printf("Σ [INIT]: %s — %s (PID %d, restarts: %u)\n",
                 svc->name, svc_state_str(svc->state),
                 (int)svc->pid, svc->restart_count);
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * sigma_init_reap — Collect zombie processes (wait4 loop equivalent)
 * ---------------------------------------------------------------------- */
void sigma_init_reap(SigmaInitCtx_t *ctx) {
    for (sigma_u32 i = 0; i < ctx->svc_count; i++) {
        SigmaService_t *svc = &ctx->services[i];
        if (svc->state == SIGMA_SVC_CRASHED) {
            sigma_printf("Σ [INIT]: Reaping crashed service: %s\n", svc->name);
            if (svc->restart_on_crash) {
                svc->restart_count++;
                sigma_printf("Σ [INIT]: Auto-restarting %s (attempt #%u).\n",
                             svc->name, svc->restart_count);
                sigma_svc_start(ctx, svc->name);
            } else {
                svc->state = SIGMA_SVC_STOPPED;
                svc->pid   = -1;
            }
        }
    }
}

/* -------------------------------------------------------------------------
 * sigma_init_switch_runlevel — Transition between runlevels
 * ---------------------------------------------------------------------- */
void sigma_init_switch_runlevel(SigmaInitCtx_t *ctx, SigmaRunlevel_t rl) {
    static const char *const rl_names[] = {
        "sysinit", "boot", "default", "shutdown"
    };
    sigma_printf("Σ [INIT]: Switching runlevel: %s -> %s\n",
                 rl_names[ctx->current_runlevel], rl_names[rl]);
    ctx->current_runlevel = rl;
    if (rl == SIGMA_RL_SHUTDOWN) {
        /* Stop all running services in reverse order */
        sigma_i32 i = (sigma_i32)ctx->svc_count - 1;
        for (; i >= 0; i--) {
            if (ctx->services[i].state == SIGMA_SVC_RUNNING)
                sigma_svc_stop(ctx, ctx->services[i].name);
        }
        sigma_printf("Σ [INIT]: System halted. Sovereignty maintained.\n");
    }
}

/* -------------------------------------------------------------------------
 * SovereignInitSystem_Init — Bootstrap demo
 * ---------------------------------------------------------------------- */
void SovereignInitSystem_Init(void) {
    sigma_printf("Σ [INIT]: Bootstrapping Sovereign Init System (PID 1)...\n");
    static SigmaInitCtx_t ctx;
    sigma_init_setup(&ctx);

    /* Register essential system services */
    sigma_svc_register(&ctx, "sigma-logger",   "/sbin/sigma-logger",   SIGMA_TRUE);
    sigma_svc_register(&ctx, "sigma-netd",     "/sbin/sigma-netd",     SIGMA_TRUE);
    sigma_svc_register(&ctx, "sigma-sshd",     "/usr/sbin/sigma-sshd", SIGMA_TRUE);
    sigma_svc_register(&ctx, "sigma-cron",     "/usr/sbin/sigma-cron", SIGMA_FALSE);
    sigma_svc_register(&ctx, "sigma-desktop",  "/usr/bin/sigma-zenith-wm", SIGMA_TRUE);

    /* Transition to boot runlevel and start core services */
    sigma_init_switch_runlevel(&ctx, SIGMA_RL_BOOT);
    sigma_svc_start(&ctx, "sigma-logger");
    sigma_svc_start(&ctx, "sigma-netd");

    /* Transition to default (multi-user) */
    sigma_init_switch_runlevel(&ctx, SIGMA_RL_DEFAULT);
    sigma_svc_start(&ctx, "sigma-sshd");
    sigma_svc_start(&ctx, "sigma-cron");
    sigma_svc_start(&ctx, "sigma-desktop");

    /* Inspect status */
    SigmaSvcState_t st;
    sigma_svc_status(&ctx, "sigma-netd", &st);
    sigma_svc_status(&ctx, "sigma-desktop", &st);

    /* Simulate a crash + auto-restart */
    ctx.services[1].state = SIGMA_SVC_CRASHED; /* sigma-netd crashed */
    sigma_init_reap(&ctx);

    sigma_printf("Σ [INIT]: Init system online. All services supervised.\n");
}
