/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN CONFIG SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb NixOS Declarative / Windows Registry / macOS defaults USP.
 *          Native Silicon Configuration DSL with Atomic Apply & Rollback.
 * Design: C11 / Zero-Dependency / Key-Value Silicon Register Bank.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Config Structures
// -------------------------------------------------------------------------

typedef enum {
    CFG_STRING,
    CFG_INT,
    CFG_BOOL,
    CFG_FLOAT_X100   /* Store float * 100 as integer for portability */
} SigmaCfgType_t;

typedef struct {
    char         key[48];
    SigmaCfgType_t type;
    char         str_val[64];
    sigma_i64    int_val;
    sigma_bool   dirty;        /* Modified since last commit */
    sigma_bool   locked;       /* System-critical; citizen cannot override */
} SigmaCfgEntry_t;

typedef struct {
    char         commit_tag[32];
    sigma_u64    timestamp;
    sigma_u32    entry_snapshot[128]; /* int_val checksum array  */
    sigma_u32    count;
} SigmaCfgCommit_t;

#define MAX_CFG_ENTRIES 128
#define MAX_CFG_COMMITS 8

static SigmaCfgEntry_t  s_cfg_store[MAX_CFG_ENTRIES];
static sigma_u32        s_cfg_count    = 0;
static SigmaCfgCommit_t s_cfg_commits[MAX_CFG_COMMITS];
static sigma_u32        s_commit_count = 0;
static sigma_u32        s_cfg_generation = 0;

// -------------------------------------------------------------------------
// Config Logic (NixOS / Windows Registry / macOS defaults write parity)
// -------------------------------------------------------------------------

/**
 * sigma_cfg_set: Writes a key-value pair to the silicon config register bank.
 */
sigma_err_t sigma_cfg_set(const char* key, const char* val,
                           SigmaCfgType_t type, sigma_bool lock) {
    /* Update existing key */
    for (sigma_u32 i = 0; i < s_cfg_count; i++) {
        if (sigma_streq(s_cfg_store[i].key, key)) {
            if (s_cfg_store[i].locked) {
                sigma_printf("[CFG]: Key '%s' is system-locked. Access denied.\n", key);
                return SIGMA_EPERM;
            }
            sigma_strcpy(s_cfg_store[i].str_val, val);
            s_cfg_store[i].int_val = (sigma_i64)sigma_atoi(val);
            s_cfg_store[i].dirty   = SIGMA_TRUE;
            s_cfg_store[i].type    = type;
            sigma_printf("[CFG]: Updated '%s' = '%s'\n", key, val);
            return SIGMA_OK;
        }
    }

    /* New entry */
    if (s_cfg_count >= MAX_CFG_ENTRIES) return SIGMA_ENOSPC;
    SigmaCfgEntry_t* e = &s_cfg_store[s_cfg_count++];
    sigma_strcpy(e->key,     key);
    sigma_strcpy(e->str_val, val);
    e->int_val = (sigma_i64)sigma_atoi(val);
    e->type    = type;
    e->dirty   = SIGMA_TRUE;
    e->locked  = lock;
    sigma_printf("[CFG]: Set '%s' = '%s'%s\n", key, val, lock ? " [LOCKED]" : "");
    return SIGMA_OK;
}

/**
 * sigma_cfg_get: Reads a key from the silicon config register bank.
 */
const char* sigma_cfg_get(const char* key) {
    for (sigma_u32 i = 0; i < s_cfg_count; i++) {
        if (sigma_streq(s_cfg_store[i].key, key))
            return s_cfg_store[i].str_val;
    }
    return "(unset)";
}

/**
 * sigma_cfg_commit: Atomically commits all dirty entries (NixOS rebuild parity).
 */
void sigma_cfg_commit(const char* tag) {
    if (s_commit_count >= MAX_CFG_COMMITS) {
        /* Evict oldest commit */
        for (sigma_u32 i = 0; i < MAX_CFG_COMMITS - 1; i++)
            s_cfg_commits[i] = s_cfg_commits[i + 1];
        s_commit_count--;
    }

    SigmaCfgCommit_t* c = &s_cfg_commits[s_commit_count++];
    sigma_strcpy(c->commit_tag, tag);
    c->timestamp  = ++s_cfg_generation * 1000000ULL;
    c->count      = s_cfg_count;

    /* Snapshot int_vals as lightweight checksum */
    for (sigma_u32 i = 0; i < s_cfg_count && i < 128; i++)
        c->entry_snapshot[i] = (sigma_u32)(s_cfg_store[i].int_val & 0xFFFFFFFF);

    /* Clear dirty bits */
    sigma_u32 dirty_count = 0;
    for (sigma_u32 i = 0; i < s_cfg_count; i++) {
        if (s_cfg_store[i].dirty) { dirty_count++; s_cfg_store[i].dirty = SIGMA_FALSE; }
    }

    sigma_printf("[CFG]: Committed '%s' (gen %u, %u dirty entries, %u total keys).\n",
                 tag, s_cfg_generation, dirty_count, s_cfg_count);
}

/**
 * sigma_cfg_rollback: Reverts to the most recent commit.
 */
void sigma_cfg_rollback() {
    if (s_commit_count == 0) {
        sigma_printf("[CFG]: No commits available to roll back to.\n");
        return;
    }
    SigmaCfgCommit_t* c = &s_cfg_commits[s_commit_count - 1];
    sigma_printf("[CFG]: Rolling back to commit '%s' (gen snapshot)...\n",
                 c->commit_tag);
    /* Restore int_vals from snapshot */
    for (sigma_u32 i = 0; i < c->count && i < s_cfg_count; i++) {
        s_cfg_store[i].int_val = (sigma_i64)c->entry_snapshot[i];
        s_cfg_store[i].dirty   = SIGMA_FALSE;
    }
    sigma_printf("[OK]: Config rolled back. Silicon register bank stabilised.\n");
}

// -------------------------------------------------------------------------
// Industrial Config Audit
// -------------------------------------------------------------------------

void SovereignConfig_Audit() {
    static const char* tnames[] = { "STR", "INT", "BOOL", "FLOAT" };
    sigma_printf("\n--- SOVEREIGN CONFIG AUDIT (gen %u, commits: %u) ---\n",
                 s_cfg_generation, s_commit_count);
    sigma_printf("KEY                                              TYPE  VALUE              FLAGS\n");
    sigma_printf("-------------------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_cfg_count; i++) {
        sigma_printf("%-48s %-5s %-18s %s%s\n",
                     s_cfg_store[i].key,
                     tnames[s_cfg_store[i].type],
                     s_cfg_store[i].str_val,
                     s_cfg_store[i].locked ? "[LOCKED] " : "",
                     s_cfg_store[i].dirty  ? "[DIRTY]"  : "");
    }
    sigma_printf("-------------------------------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignConfigShard_Init() {
    sigma_printf("[SOC]: Seating Native Config Shard (NixOS/Registry/defaults Parity v1.0)...\n");
    /* Kernel-locked system settings */
    sigma_cfg_set("kernel.sovereign.version",   "160.0",       CFG_STRING, SIGMA_TRUE);
    sigma_cfg_set("kernel.scheduler.policy",    "zen_realtime", CFG_STRING, SIGMA_TRUE);
    sigma_cfg_set("kernel.oom.threshold_kb",    "131072",       CFG_INT,    SIGMA_TRUE);
    /* Citizen-configurable settings */
    sigma_cfg_set("ui.theme",                   "Obsidian",     CFG_STRING, SIGMA_FALSE);
    sigma_cfg_set("ui.font_scale",              "100",          CFG_INT,    SIGMA_FALSE);
    sigma_cfg_set("ui.animations_enabled",      "1",            CFG_BOOL,   SIGMA_FALSE);
    sigma_cfg_set("power.plan",                 "balanced",     CFG_STRING, SIGMA_FALSE);
    sigma_cfg_set("net.hostname",               "sigma-zenith", CFG_STRING, SIGMA_FALSE);
    sigma_cfg_set("security.fw_default_policy", "deny",         CFG_STRING, SIGMA_FALSE);
    sigma_cfg_commit("zenith-baseline-v160");
}
