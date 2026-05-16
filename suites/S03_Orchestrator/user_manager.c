#include "../../include/libc/SovereignLibC.h"
#include "../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN-USER-MANAGER (v1.0 - IDENTITY SHARDING)
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
    sigma_u32  uid;
    char name[32];
    sigma_u32  privilege_level; /* 0: Guest, 1: Master, 2: Sovereign */
    sigma_bool active;
} SovereignUser;

static SovereignUser g_users[MAX_SOVEREIGN_USERS];
static sigma_u32 g_current_uid = 0;

/* =========================================================================
 * USER MANAGER Engine (The Identity Shard)
 * ========================================================================= */

void user_manager_init(void) {
    for (int i = 0; i < MAX_SOVEREIGN_USERS; i++) g_users[i].active = SIGMA_FALSE;
    
    /* Default Sovereign Master Identity */
    g_users[0].uid = 0;
    const char* master_name = "Sovereign-Master";
    sigma_usize j = 0; while (master_name[j]) { g_users[0].name[j] = master_name[j]; j++; }
    g_users[0].name[j] = '\0';
    g_users[0].privilege_level = 2; /* Absolute Sovereignty */
    g_users[0].active = SIGMA_TRUE;
    
    g_current_uid = 0;
    // ksigma_printf("[USER-MANAGER]: Sovereign Identity Shard Online: uid=0\n");
}

sigma_u32 user_get_current_uid(void) {
    return g_current_uid;
}

sigma_bool user_is_sovereign(sigma_u32 uid) {
    if (uid >= MAX_SOVEREIGN_USERS) return SIGMA_FALSE;
    return g_users[uid].privilege_level == 2;
}

void user_switch_identity(sigma_u32 uid) {
    if (uid < MAX_SOVEREIGN_USERS && g_users[uid].active) {
        g_current_uid = uid;
        // ksigma_printf("[USER-MANAGER]: Identity Absorbed: %s\n", g_users[uid].name);
    }
}
