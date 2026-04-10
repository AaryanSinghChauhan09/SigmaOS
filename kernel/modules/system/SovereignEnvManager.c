/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ENVIRONMENT MANAGER — IMPLEMENTATION (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignEnvManager.h"

/* Global kernel environment block */
SigmaEnvBlock_t g_sigma_env;

/* -------------------------------------------------------------------------
 * Internal: djb2 hash for linear-probing table lookup
 * ---------------------------------------------------------------------- */
static sigma_u32 env_hash(const char *key) {
    sigma_u32 h = 5381;
    while (*key) {
        h = ((h << 5) + h) ^ (sigma_u8)*key++;
    }
    return h % SIGMA_ENV_CAPACITY;
}

/* -------------------------------------------------------------------------
 * sigma_env_init — Zero-fill the environment block
 * ---------------------------------------------------------------------- */
void sigma_env_init(SigmaEnvBlock_t *env) {
    sigma_memset(env, 0, sizeof(*env));
}

/* -------------------------------------------------------------------------
 * sigma_env_set — setenv() equivalent (overwrites if exists)
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_env_set(SigmaEnvBlock_t *env,
                           const char *key, const char *val) {
    if (!key || !val) return SIGMA_EINVAL;

    sigma_u32 idx = env_hash(key);
    sigma_u32 probe;

    /* Linear probe — find existing key or first empty slot */
    for (sigma_u32 i = 0; i < SIGMA_ENV_CAPACITY; i++) {
        probe = (idx + i) % SIGMA_ENV_CAPACITY;
        SigmaEnvEntry_t *e = &env->entries[probe];

        if (!e->occupied) {
            /* Empty slot — insert */
            sigma_strcpy(e->key, key, SIGMA_ENV_KEY_MAX);
            sigma_strcpy(e->val, val, SIGMA_ENV_VAL_MAX);
            e->occupied = SIGMA_TRUE;
            env->count++;
            return SIGMA_OK;
        }
        if (sigma_streq(e->key, key)) {
            /* Overwrite */
            sigma_strcpy(e->val, val, SIGMA_ENV_VAL_MAX);
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOSPC;
}

/* -------------------------------------------------------------------------
 * sigma_env_get — getenv() equivalent
 * ---------------------------------------------------------------------- */
const char *sigma_env_get(const SigmaEnvBlock_t *env, const char *key) {
    if (!key) return SIGMA_NULL;

    sigma_u32 idx = env_hash(key);
    for (sigma_u32 i = 0; i < SIGMA_ENV_CAPACITY; i++) {
        sigma_u32 probe = (idx + i) % SIGMA_ENV_CAPACITY;
        const SigmaEnvEntry_t *e = &env->entries[probe];
        if (!e->occupied) return SIGMA_NULL;   /* Past linear probe chain */
        if (sigma_streq(e->key, key)) return e->val;
    }
    return SIGMA_NULL;
}

/* -------------------------------------------------------------------------
 * sigma_env_unset — unsetenv() equivalent (tombstone slot)
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_env_unset(SigmaEnvBlock_t *env, const char *key) {
    if (!key) return SIGMA_EINVAL;

    sigma_u32 idx = env_hash(key);
    for (sigma_u32 i = 0; i < SIGMA_ENV_CAPACITY; i++) {
        sigma_u32 probe = (idx + i) % SIGMA_ENV_CAPACITY;
        SigmaEnvEntry_t *e = &env->entries[probe];
        if (!e->occupied) return SIGMA_ENOENT;
        if (sigma_streq(e->key, key)) {
            sigma_memset(e, 0, sizeof(*e));     /* tombstone */
            env->count--;
            return SIGMA_OK;
        }
    }
    return SIGMA_ENOENT;
}

/* -------------------------------------------------------------------------
 * sigma_env_dump — printenv equivalent
 * ---------------------------------------------------------------------- */
void sigma_env_dump(const SigmaEnvBlock_t *env) {
    sigma_printf("Σ [ENV]: Environment dump (%u variables):\n", env->count);
    for (sigma_u32 i = 0; i < SIGMA_ENV_CAPACITY; i++) {
        const SigmaEnvEntry_t *e = &env->entries[i];
        if (e->occupied) {
            sigma_printf("  %s=%s\n", e->key, e->val);
        }
    }
}

/* -------------------------------------------------------------------------
 * sigma_env_inherit — Copy parent env into child (fork semantics)
 * ---------------------------------------------------------------------- */
sigma_err_t sigma_env_inherit(SigmaEnvBlock_t *dst, const SigmaEnvBlock_t *src) {
    sigma_memcpy(dst, src, sizeof(*src));
    return SIGMA_OK;
}

/* -------------------------------------------------------------------------
 * SovereignEnvManager_Init — Bootstrap with standard kernel defaults
 * ---------------------------------------------------------------------- */
void SovereignEnvManager_Init(void) {
    sigma_printf("Σ [ENV]: Initialising Sovereign Environment Manager...\n");
    sigma_env_init(&g_sigma_env);

    /* Seed standard POSIX environment variables */
    sigma_env_set(&g_sigma_env, "PATH",    "/bin:/usr/bin:/usr/local/bin:/sbin");
    sigma_env_set(&g_sigma_env, "HOME",    "/root");
    sigma_env_set(&g_sigma_env, "USER",    "root");
    sigma_env_set(&g_sigma_env, "SHELL",   "/bin/sigma-sh");
    sigma_env_set(&g_sigma_env, "TERM",    "sigma-256color");
    sigma_env_set(&g_sigma_env, "LANG",    "en_US.UTF-8");
    sigma_env_set(&g_sigma_env, "LOGNAME", "root");
    sigma_env_set(&g_sigma_env, "TMPDIR",  "/tmp");
    sigma_env_set(&g_sigma_env, "EDITOR",  "sigma-ed");
    sigma_env_set(&g_sigma_env, "PAGER",   "sigma-less");

    /* Demonstrate get / unset */
    const char *path = sigma_env_get(&g_sigma_env, "PATH");
    sigma_printf("Σ [ENV]: PATH=%s\n", path ? path : "(null)");

    sigma_env_dump(&g_sigma_env);
    sigma_printf("Σ [ENV]: Environment Manager online.\n");
}
