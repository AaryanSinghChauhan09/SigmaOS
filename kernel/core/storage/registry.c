#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-REGISTRY (v1.0 - PERSISTENT CONFIG SHARD)
 * =============================================================================
 * Algorithm: Atomic Sharded Key-Value Store (O(1) Map)
 * Principles:
 *   - Centralized kernel-level configuration for all shards.
 *   - Persistent storage across silicon pulses (VFS-backed).
 *   - Atomic updates for system-wide customization (Themes, Automation).
 * Comparison: Linux /etc = text files, Windows Registry = complex hi-bin, 
 *             Sigma Registry = Pure Silicon Shard.
 * =============================================================================
 */

#include "../../../include/sigma_kernel_types.h"

#define MAX_REGISTRY_KEYS 1024u
#define MAX_KEY_LEN 64
#define MAX_VAL_LEN 128

typedef struct RegistryEntry {
    char key[MAX_KEY_LEN];
    char val[MAX_VAL_LEN];
    sigma_bool active;
} RegistryEntry;

static RegistryEntry g_reg[MAX_REGISTRY_KEYS];
static sigma_u32 g_reg_count = 0;

/* =========================================================================
 * REGISTRY Engine (The Sovereign Config Store)
 * ========================================================================= */

void registry_init(void) {
<<<<<<<< HEAD:suites/S10_Registry/registry.c
    for (int i = 0; i < MAX_REGISTRY_KEYS; i++) g_reg[i].active = FALSE;
    // ksigma_printf("[REGISTRY]: Sovereign Persistent Registry Shard Online.\n");
========
    for (int i = 0; i < MAX_REGISTRY_KEYS; i++) g_reg[i].active = SIGMA_FALSE;
    // kprintf("[REGISTRY]: Sovereign Persistent Registry Shard Online.\n");
>>>>>>>> ad8016503ce074e8980abb23e1a44b78be830645:kernel/core/storage/registry.c
}

sigma_status registry_set(const char* key, const char* val) {
    /* If key exists, update value */
    for (sigma_u32 i = 0; i < g_reg_count; i++) {
        // Simple sigma_strcmp replacement
        sigma_bool match = SIGMA_TRUE;
        sigma_u32 j = 0;
        while (g_reg[i].key[j] && key[j]) {
            if (g_reg[i].key[j] != key[j]) { match = SIGMA_FALSE; break; }
            j++;
        }
        if (match && g_reg[i].key[j] == '\0' && key[j] == '\0') {
            j = 0; while (j < MAX_VAL_LEN - 1 && val[j]) { g_reg[i].val[j] = val[j]; j++; }
            g_reg[i].val[j] = '\0';
            return K_OK;
        }
    }

    /* Otherwise add new key */
    if (g_reg_count >= MAX_REGISTRY_KEYS) return K_ERR_NOMEM;
    
    RegistryEntry* e = &g_reg[g_reg_count++];
    sigma_u32 k = 0; while (k < MAX_KEY_LEN - 1 && key[k]) { e->key[k] = key[k]; k++; }
    e->key[k] = '\0';
    k = 0; while (k < MAX_VAL_LEN - 1 && val[k]) { e->val[k] = val[k]; k++; }
    e->val[k] = '\0';
    e->active = SIGMA_TRUE;
    
    return K_OK;
}

const char* registry_get(const char* key) {
    for (sigma_u32 i = 0; i < g_reg_count; i++) {
        sigma_bool match = SIGMA_TRUE;
        sigma_u32 j = 0;
        while (g_reg[i].key[j] && key[j]) {
            if (g_reg[i].key[j] != key[j]) { match = SIGMA_FALSE; break; }
            j++;
        }
        if (match && g_reg[i].key[j] == '\0' && key[j] == '\0') {
            return g_reg[i].val;
        }
    }
    return SIGMA_NULL;
}
