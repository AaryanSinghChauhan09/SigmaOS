/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: ENVIRONMENT VARIABLE SUBSYSTEM
 * =============================================================================
 * Inspired by: glibc getenv/setenv/unsetenv (stdlib/setenv.c)
 *              musl libc env implementation (src/env/)
 *              busybox env/printenv applets
 * =============================================================================
 * Provides: Process-level environment variable storage with get/set/unset/list.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define ENV_MAX_VARS    128
#define ENV_KEY_LEN     64
#define ENV_VALUE_LEN   256

typedef struct {
    char key[ENV_KEY_LEN];
    char value[ENV_VALUE_LEN];
    sigma_bool active;
} sigma_env_entry_t;

typedef struct {
    sigma_env_entry_t vars[ENV_MAX_VARS];
    sigma_u32         count;
} sigma_environ_t;

static sigma_environ_t global_env;

static void env_strcpy(char* dst, const char* src, sigma_u32 max) {
    sigma_u32 i = 0;
    while (i < max - 1 && src[i]) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

static int env_strcmp(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return (int)(*(sigma_u8*)a) - (int)(*(sigma_u8*)b);
}

void sigma_env_init(void) {
    sigma_memset(&global_env, 0, sizeof(global_env));

    /* Populate default environment (inspired by systemd default env) */
    sigma_env_set("PATH",     "/usr/bin:/usr/sbin:/bin:/sbin");
    sigma_env_set("HOME",     "/root");
    sigma_env_set("SHELL",    "/usr/sh");
    sigma_env_set("TERM",     "sigma-256color");
    sigma_env_set("LANG",     "en_US.UTF-8");
    sigma_env_set("USER",     "sovereign");
    sigma_env_set("HOSTNAME", "sigmaos-zenith");
    sigma_env_set("EDITOR",   "sigma-vi");
    sigma_env_set("SIGMA_VERSION", "15.2");
    sigma_env_set("SIGMA_ARCH",    "x86_64");

    sigma_printf("[env] Environment initialized with %u default variables\n", global_env.count);
}

int sigma_env_set(const char* key, const char* value) {
    if (!key || !value) return -1;

    /* Update existing entry if key already exists */
    for (sigma_u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (global_env.vars[i].active && env_strcmp(global_env.vars[i].key, key) == 0) {
            env_strcpy(global_env.vars[i].value, value, ENV_VALUE_LEN);
            sigma_printf("[env] Updated: %s=%s\n", key, value);
            return 0;
        }
    }

    /* Add new entry */
    for (sigma_u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (!global_env.vars[i].active) {
            env_strcpy(global_env.vars[i].key, key, ENV_KEY_LEN);
            env_strcpy(global_env.vars[i].value, value, ENV_VALUE_LEN);
            global_env.vars[i].active = SIGMA_TRUE;
            global_env.count++;
            sigma_printf("[env] Set: %s=%s\n", key, value);
            return 0;
        }
    }

    sigma_printf("[env] ERR: Environment table full (%u/%u)\n", global_env.count, ENV_MAX_VARS);
    return -1;
}

const char* sigma_env_get(const char* key) {
    if (!key) return SIGMA_NULL;
    for (sigma_u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (global_env.vars[i].active && env_strcmp(global_env.vars[i].key, key) == 0) {
            return global_env.vars[i].value;
        }
    }
    return SIGMA_NULL;
}

int sigma_env_unset(const char* key) {
    if (!key) return -1;
    for (sigma_u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (global_env.vars[i].active && env_strcmp(global_env.vars[i].key, key) == 0) {
            global_env.vars[i].active = SIGMA_FALSE;
            global_env.count--;
            sigma_printf("[env] Unset: %s\n", key);
            return 0;
        }
    }
    return -1;
}

void sigma_env_list(void) {
    sigma_printf("\n--- Σ ENVIRONMENT VARIABLES (%u) ---\n", global_env.count);
    for (sigma_u32 i = 0; i < ENV_MAX_VARS; i++) {
        if (global_env.vars[i].active) {
            sigma_printf("  %s=%s\n", global_env.vars[i].key, global_env.vars[i].value);
        }
    }
    sigma_printf("-------------------------------------\n");
}
