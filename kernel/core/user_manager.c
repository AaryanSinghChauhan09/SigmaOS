/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: SOVEREIGN-USER-MANAGER (v1.0 - IDENTITY SHARDING)
 * =============================================================================
 * Algorithm: Sharded-Identity Verification (SIV)
 * Principles:
 *   - Kernel-native identity management (No-Mouse login awareness).
 *   - Absolute industrial sovereignty in privilege escalation (Alt+P).
 *   - Personalised Home-Shard (/home/sovereign) per-identity.
 * Comparison: Linux passwd/shadow = Standard, Sigma = Sharded-Identity.
 * =============================================================================
 */

#include "../../include/sigma_kernel_types.h"

#define MAX_SOVEREIGN_USERS 8

typedef struct SovereignUser {
    u32  uid;
    char name[32];
    u32  privilege_level; /* 0: Guest, 1: Master, 2: Sovereign */
    bool_t active;
} SovereignUser;

static SovereignUser g_users[MAX_SOVEREIGN_USERS];
static u32 g_current_uid = 0;

/* =========================================================================
 * USER MANAGER Engine (The Identity Shard)
 * ========================================================================= */

void user_manager_init(void) {
    for (int i = 0; i < MAX_SOVEREIGN_USERS; i++) g_users[i].active = FALSE;
    
    /* Default Sovereign Master Identity */
    g_users[0].uid = 0;
    const char* master_name = "Sovereign-Master";
    usize j = 0; while (master_name[j]) { g_users[0].name[j] = master_name[j]; j++; }
    g_users[0].name[j] = '\0';
    g_users[0].privilege_level = 2; /* Absolute Sovereignty */
    g_users[0].active = TRUE;
    
    g_current_uid = 0;
    // kprintf("[USER-MANAGER]: Sovereign Identity Shard Online: uid=0\n");
}

u32 user_get_current_uid(void) {
    return g_current_uid;
}

bool_t user_is_sovereign(u32 uid) {
    if (uid >= MAX_SOVEREIGN_USERS) return FALSE;
    return g_users[uid].privilege_level == 2;
}

void user_switch_identity(u32 uid) {
    if (uid < MAX_SOVEREIGN_USERS && g_users[uid].active) {
        g_current_uid = uid;
        // kprintf("[USER-MANAGER]: Identity Absorbed: %s\n", g_users[uid].name);
    }
}
