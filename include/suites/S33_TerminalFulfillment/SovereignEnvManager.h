/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ENVIRONMENT MANAGER (v1.0 — PURE C11)
 * =========================================================================
 * Mission: POSIX-parity environment variable store, per-process inheritance.
 * Inspired By: UNIX environ[], bash/zsh export, Windows HKCU Registry env.
 * Principle: Zero-dependency. Linear-probed hash table. Sovereign.
 * =========================================================================
 */

#ifndef SOVEREIGN_ENV_MANAGER_H
#define SOVEREIGN_ENV_MANAGER_H

#include "suites/S01_Genesis/shards/sigma_types.h"

#define SIGMA_ENV_KEY_MAX   128
#define SIGMA_ENV_VAL_MAX   4096
#define SIGMA_ENV_CAPACITY  256   /* Max variables per environment block */

typedef struct {
    char       key[SIGMA_ENV_KEY_MAX];
    char       val[SIGMA_ENV_VAL_MAX];
    sigma_bool occupied;
} SigmaEnvEntry_t;

typedef struct {
    SigmaEnvEntry_t entries[SIGMA_ENV_CAPACITY];
    sigma_u32       count;
} SigmaEnvBlock_t;

/* -------------------------------------------------------------------------
 * Public API — mirrors setenv / getenv / unsetenv / environ
 * ---------------------------------------------------------------------- */
void        sigma_env_init    (SigmaEnvBlock_t *env);
sigma_err_t sigma_env_set     (SigmaEnvBlock_t *env,
                                const char *key, const char *val);
const char *sigma_env_get     (const SigmaEnvBlock_t *env, const char *key);
sigma_err_t sigma_env_unset   (SigmaEnvBlock_t *env, const char *key);
void        sigma_env_dump    (const SigmaEnvBlock_t *env);          /* printenv */
sigma_err_t sigma_env_inherit (SigmaEnvBlock_t *dst,
                                const SigmaEnvBlock_t *src);          /* fork() clone */

/* Global kernel environment (accessible to all processes) */
extern SigmaEnvBlock_t g_sigma_env;

void SovereignEnvManager_Init(void);

#endif /* SOVEREIGN_ENV_MANAGER_H */
