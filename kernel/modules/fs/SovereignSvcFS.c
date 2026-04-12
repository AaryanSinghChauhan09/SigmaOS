/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN SERVICE FILESYSTEM (SvcFS)
 * =========================================================================
 * Mission: Absorb Plan 9 USP — Everything is a File / Distributed Resource.
 * Design: C11 / Zero-Dependency / Managed Virtual Inodes.
 * Shard: SVC_FS_SHARD
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// SvcFS Structures
// -------------------------------------------------------------------------

typedef struct {
    char name[32];
    char status[16];
    sigma_err_t (*trigger)(void);
} SigmaSvcNode_t;

#define MAX_SVC_NODES 16

static SigmaSvcNode_t s_svc_table[MAX_SVC_NODES];
static sigma_u32 s_svc_count = 0;

// -------------------------------------------------------------------------
// Native Registration (Ring 0)
// -------------------------------------------------------------------------

void sigma_svcfs_register(const char* name, sigma_err_t (*trigger)(void)) {
    if (s_svc_count >= MAX_SVC_NODES) return;
    
    sigma_strcpy(s_svc_table[s_svc_count].name, name);
    sigma_strcpy(s_svc_table[s_svc_count].status, "ONLINE");
    s_svc_table[s_svc_count].trigger = trigger;
    s_svc_count++;
}

// -------------------------------------------------------------------------
// VFS Methods (Plan 9 Parity)
// -------------------------------------------------------------------------

void sigma_svcfs_ls(void) {
    sigma_printf("\nΣ [SVCFS]: Mapping industrial services at /svc/...\n");
    sigma_printf("DRIVE  NAME              STATUS     CAPABILITY\n");
    sigma_printf("----------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_svc_count; i++) {
        sigma_printf("S:     %-17s [%-8s] SHARD_EXEC\n", s_svc_table[i].name, s_svc_table[i].status);
    }
    sigma_printf("\n");
}

sigma_err_t sigma_svcfs_execute(const char* name) {
    for (sigma_u32 i = 0; i < s_svc_count; i++) {
        if (sigma_streq(s_svc_table[i].name, name)) {
            sigma_printf("[SVCFS]: Writing '1' to /svc/%s trigger...\n", name);
            if (s_svc_table[i].trigger) return s_svc_table[i].trigger();
            return SIGMA_OK;
        }
    }
    sigma_printf("[ERROR]: Service '%s' not found in Silicon SvcFS.\n", name);
    return SIGMA_ENOENT;
}

// -------------------------------------------------------------------------
// Initialization
// -------------------------------------------------------------------------

void SovereignSvcFS_Init() {
    sigma_printf("[SOC]: Seating Plan 9 Service-as-File Shard (SvcFS v1.0)...\n");
    
    // Auto-register key kernel automations
    sigma_svcfs_register("ai_train", SIGMA_NULL);
    sigma_svcfs_register("scrub_all", SIGMA_NULL);
    sigma_svcfs_register("personalize", SIGMA_NULL);
}
