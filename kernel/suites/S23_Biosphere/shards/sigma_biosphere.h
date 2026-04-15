/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN BIOSPHERE (Suite S23)
 * =========================================================================
 * Shard: Sovereign Sandbox Core
 * Parity: macOS Sandbox (AppSandbox), Linux Seccomp-BPF, Windows AppContainer
 * Design: Multilayered isolation using S08 Security and S10 Containers.
 * =========================================================================
 */

#ifndef SOVEREIGN_BIOSPHERE_H
#define SOVEREIGN_BIOSPHERE_H

#include "suites/S01_Genesis/shards/SovereignCommon.h"

typedef enum {
    BIO_POLICY_STRICT,     /* No net, no disk (except home) */
    BIO_POLICY_NETWORK,    /* Restricted net, no disk */
    BIO_POLICY_DATABASE,   /* Local storage only */
    BIO_POLICY_INTERATIVE  /* UI allowed, restricted IPC */
} biosphere_policy_t;

typedef struct {
    sigma_u32 allowed_syscalls[16]; /* Bitmask for syscalls */
    char      home_path[128];
    sigma_bool network_access;
} biosphere_config_t;

/* Public API */
void        sigma_biosphere_init(void);

/* Sandbox management */
sigma_err_t sigma_jail_process(sigma_u32 pid, biosphere_policy_t policy);
sigma_err_t sigma_apply_policy(sigma_u32 pid, biosphere_config_t* config);

/* Verification */
sigma_bool  sigma_is_jailed(sigma_u32 pid);

#endif /* SOVEREIGN_BIOSPHERE_H */
