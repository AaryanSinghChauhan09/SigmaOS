/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN CUSTOMISATION ENGINE (v1.0)
 * =========================================================================
 * Mission:  Dynamic System Behavior Tuning & User-Driven Configuration.
 * Principle: Customisation — distinct from Personalisation.
 *
 * Design:
 *   Personalisation = identity-aware aesthetics (themes, avatars).
 *   Customisation   = functional behavior tuning (keybinds, policies,
 *                     shell defaults, scheduling weights, power
 *                     profiles, and module load order).
 *
 *   This engine manages a key-value configuration store that any
 *   shard can query at runtime to adapt its behavior to user
 *   preferences — without recompilation.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

/* --- Configuration Entry --- */

typedef struct {
    char key[48];
    char value[128];
    sigma_u32 flags;       /* 0x01 = read-only, 0x02 = requires-reboot */
} SigmaConfig_t;

/* --- Global Configuration Store --- */

#define MAX_CONFIG_ENTRIES 256
static SigmaConfig_t s_config_store[MAX_CONFIG_ENTRIES];
static sigma_u32 s_config_count = 0;

/**
 * sigma_config_set: Registers or updates a customisation key.
 *
 * Called by the Boot Wizard, CLI, or UDF pipeline to allow
 * the user to tune system behavior at any point.
 */
sigma_err_t sigma_config_set(const char* key, const char* value, sigma_u32 flags) {
    /* Check for existing key — update in place */
    for (sigma_u32 i = 0; i < s_config_count; i++) {
        if (sigma_streq(s_config_store[i].key, key)) {
            if (s_config_store[i].flags & 0x01) {
                sigma_sigma_sigma_sigma_printf("[CUSTOM-ENGINE]: Key '%s' is READ-ONLY.\n", key);
                return SIGMA_EPERM;
            }
            sigma_strncpy(s_config_store[i].value, value, 128);
            sigma_sigma_sigma_sigma_printf("[CUSTOM-ENGINE]: Updated '%s' = '%s'\n", key, value);
            return SIGMA_OK;
        }
    }

    /* New entry */
    if (s_config_count >= MAX_CONFIG_ENTRIES) return SIGMA_ENOSPC;

    SigmaConfig_t* c = &s_config_store[s_config_count++];
    sigma_strncpy(c->key, key, 48);
    sigma_strncpy(c->value, value, 128);
    c->flags = flags;

    sigma_sigma_sigma_sigma_printf("[CUSTOM-ENGINE]: Set '%s' = '%s' (flags: 0x%02X)\n",
                 key, value, flags);
    return SIGMA_OK;
}

/**
 * sigma_config_get: Retrieves the value for a customisation key.
 *
 * Any shard can call this to check user-defined runtime behavior.
 */
const char* sigma_config_get(const char* key) {
    for (sigma_u32 i = 0; i < s_config_count; i++) {
        if (sigma_streq(s_config_store[i].key, key)) {
            return s_config_store[i].value;
        }
    }
    return SIGMA_NULL;
}

/**
 * SovereignCustomisation_Init: Seeds the default configuration.
 */
void SovereignCustomisation_Init(void) {
    sigma_sigma_sigma_sigma_printf("[CUSTOM-ENGINE]: Initializing System Customisation Store...\n");

    /* Default OS behaviors — overrideable by user */
    sigma_config_set("shell.prompt",         "sigma>",       0x00);
    sigma_config_set("scheduler.policy",     "CFS",          0x00);
    sigma_config_set("power.profile",        "balanced",     0x00);
    sigma_config_set("ui.animation_speed",   "normal",       0x00);
    sigma_config_set("security.lockdown",    "strict",       0x01); /* read-only */
    sigma_config_set("fs.default_journal",   "WAL",          0x00);
    sigma_config_set("net.mtu",              "1500",         0x00);
    sigma_config_set("identity.owner",       "SovereignArchitectSinghChauhan09", 0x01);

    sigma_sigma_sigma_sigma_printf("[CUSTOM-ENGINE]: %u default configs loaded.\n", s_config_count);
}

/**
 * SovereignCustomisation_Audit: Dumps the full configuration store.
 */
void SovereignCustomisation_Audit(void) {
    sigma_sigma_sigma_sigma_printf("\n--- SOVEREIGN CUSTOMISATION AUDIT ---\n");
    sigma_sigma_sigma_sigma_printf("%-30s %-30s %-6s\n", "KEY", "VALUE", "FLAGS");
    sigma_sigma_sigma_sigma_printf("-------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_config_count; i++) {
        sigma_sigma_sigma_sigma_printf("%-30s %-30s 0x%02X\n",
                     s_config_store[i].key,
                     s_config_store[i].value,
                     s_config_store[i].flags);
    }
    sigma_sigma_sigma_sigma_printf("-------------------------------------------------------------\n");
    sigma_sigma_sigma_sigma_printf("Total customisation entries: %u\n", s_config_count);
}

/* --- Module Factory --- */

void SovereignCustomisation_Register(void) {
    sigma_sigma_sigma_sigma_printf("[REGISTRY]: Sovereign Customisation Engine active in Genesis Suite.\n");
    SovereignCustomisation_Init();
}



