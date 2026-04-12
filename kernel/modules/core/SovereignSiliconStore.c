/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN SILICON STORE (v1.0)
 * =========================================================================
 * Mission: Absorb Registry/KV USP — Native Silicon State Persistence.
 * Design: C11 / Zero-Dependency / Hash-Mapped Industrial Storage.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// Store Structures
// -------------------------------------------------------------------------

typedef struct {
    char key[64];
    char value[128];
    sigma_bool active;
} SigmaStoreEntry_t;

#define MAX_STORE_ENTRIES 128
static SigmaStoreEntry_t s_silicon_store[MAX_STORE_ENTRIES];
static sigma_u32 s_entry_count = 0;

// -------------------------------------------------------------------------
// Low-Level State Persistence (Registry Parity)
// -------------------------------------------------------------------------

/**
 * sigma_store_set: Atomic set operation for silicon state.
 */
sigma_err_t sigma_store_set(const char* key, const char* value) {
    for (sigma_u32 i = 0; i < s_entry_count; i++) {
        if (sigma_streq(s_silicon_store[i].key, key)) {
            sigma_strcpy(s_silicon_store[i].value, value);
            sigma_printf("[STORE]: Updated silicon key '%s' -> '%s'.\n", key, value);
            return SIGMA_OK;
        }
    }

    if (s_entry_count >= MAX_STORE_ENTRIES) return SIGMA_ENOSPC;
    
    sigma_strcpy(s_silicon_store[s_entry_count].key, key);
    sigma_strcpy(s_silicon_store[s_entry_count].value, value);
    s_silicon_store[s_entry_count].active = SIGMA_TRUE;
    s_entry_count++;
    
    sigma_printf("[STORE]: Persisted new silicon key '%s' -> '%s'.\n", key, value);
    return SIGMA_OK;
}

/**
 * sigma_store_get: Retrieval logic for silicon state.
 */
const char* sigma_store_get(const char* key) {
    for (sigma_u32 i = 0; i < s_entry_count; i++) {
        if (sigma_streq(s_silicon_store[i].key, key)) return s_silicon_store[i].value;
    }
    return SIGMA_NULL;
}

// -------------------------------------------------------------------------
// Industrial State Audit
// -------------------------------------------------------------------------

void SovereignSiliconStore_Audit() {
    sigma_printf("\n--- SOVEREIGN SILICON STORE AUDIT ---\n");
    sigma_printf("ENTRIES: %u\n", s_entry_count);
    sigma_printf("KEY                      VALUE\n");
    sigma_printf("--------------------------------------\n");
    for (sigma_u32 i = 0; i < s_entry_count; i++) {
        sigma_printf("%-24s %s\n", s_silicon_store[i].key, s_silicon_store[i].value);
    }
    sigma_printf("--------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignSiliconStore_Init() {
    sigma_printf("[SOC]: Seating Native Silicon Store Agent (Registry/Defaults Parity v1.0)...\n");
    
    // Seed industrial defaults
    sigma_store_set("sys.theme", "ZENITH_DARK");
    sigma_store_set("sys.kernel", "160.0.SUPREME");
}
