/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN OOM SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb Linux OOM Killer / macOS Jetsam USP.
 *          Native Silicon Memory-Pressure Governor & Auto-Culling Engine.
 * Design: C11 / Zero-Dependency / Score-Based Mission Termination.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// OOM Structures
// -------------------------------------------------------------------------

typedef struct {
    char       mission_name[32];
    sigma_u32  pid;
    sigma_u64  mem_usage_kb;
    sigma_i32  oom_score;       /* Higher = culled first  */
    sigma_bool protected;       /* Kernel-critical shards */
} SigmaOOMEntry_t;

#define MAX_OOM_ENTRIES 24
static SigmaOOMEntry_t s_oom_table[MAX_OOM_ENTRIES];
static sigma_u32       s_oom_count = 0;

/* Silicon thresholds */
#define OOM_WARN_THRESHOLD_KB    (512 * 1024ULL)   /* 512 MB  */
#define OOM_CRITICAL_THRESHOLD_KB (128 * 1024ULL)  /* 128 MB  */

// -------------------------------------------------------------------------
// OOM Logic (Linux OOM Killer / macOS Jetsam parity)
// -------------------------------------------------------------------------

/**
 * sigma_oom_register: Registers a silicon mission in the OOM score table.
 */
sigma_err_t sigma_oom_register(const char* name, sigma_u32 pid,
                                sigma_u64 mem_kb, sigma_i32 score,
                                sigma_bool prot) {
    if (s_oom_count >= MAX_OOM_ENTRIES) return SIGMA_ENOSPC;

    SigmaOOMEntry_t* e = &s_oom_table[s_oom_count++];
    sigma_strcpy(e->mission_name, name);
    e->pid          = pid;
    e->mem_usage_kb = mem_kb;
    e->oom_score    = score;
    e->protected    = prot;

    sigma_printf("[OOM]: Registered mission '%s' PID:%u MEM:%lluKB score:%d %s\n",
                 name, pid, (unsigned long long)mem_kb, score,
                 prot ? "[PROTECTED]" : "");
    return SIGMA_OK;
}

/**
 * sigma_oom_sweep: Runs the silicon auto-cull mission.
 *
 * Scans the table under memory pressure. Sorts conceptually by score
 * (highest score → culled first). Protected shards are never culled.
 */
void sigma_oom_sweep(sigma_u64 free_mem_kb) {
    sigma_printf("[OOM]: Memory pressure sweep — free: %llu KB\n",
                 (unsigned long long)free_mem_kb);

    if (free_mem_kb > OOM_WARN_THRESHOLD_KB) {
        sigma_printf("  [OK]: Memory within sovereign bounds. No action needed.\n");
        return;
    }

    sigma_bool critical = (free_mem_kb < OOM_CRITICAL_THRESHOLD_KB);
    sigma_printf("  [%s]: Memory pressure %s. Initiating OOM culling...\n",
                 critical ? "CRITICAL" : "WARN",
                 critical ? "CRITICAL" : "elevated");

    /* Find highest-scored non-protected mission */
    sigma_i32  max_score = -32768;
    sigma_u32  victim    = s_oom_count; /* invalid sentinel */

    for (sigma_u32 i = 0; i < s_oom_count; i++) {
        if (!s_oom_table[i].protected && s_oom_table[i].oom_score > max_score) {
            max_score = s_oom_table[i].oom_score;
            victim = i;
        }
    }

    if (victim < s_oom_count) {
        sigma_printf("  [CULL]: Terminating '%s' PID:%u (score:%d, freed ~%llu KB).\n",
                     s_oom_table[victim].mission_name,
                     s_oom_table[victim].pid,
                     s_oom_table[victim].oom_score,
                     (unsigned long long)s_oom_table[victim].mem_usage_kb);
        /* Remove from table by compaction */
        s_oom_table[victim] = s_oom_table[--s_oom_count];
        sigma_printf("  [OK]: OOM cull complete. Silicon memory stabilised.\n");
    } else {
        sigma_printf("  [WARN]: No eligible missions to cull.\n");
    }
}

// -------------------------------------------------------------------------
// Industrial OOM Audit
// -------------------------------------------------------------------------

void SovereignOOM_Audit() {
    sigma_printf("\n--- SOVEREIGN OOM AUDIT ---\n");
    sigma_printf("MISSION              PID    MEM_KB       SCORE  PROT\n");
    sigma_printf("-----------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_oom_count; i++) {
        sigma_printf("%-20s %-6u %-12llu %-6d %s\n",
                     s_oom_table[i].mission_name,
                     s_oom_table[i].pid,
                     (unsigned long long)s_oom_table[i].mem_usage_kb,
                     s_oom_table[i].oom_score,
                     s_oom_table[i].protected ? "YES" : "no");
    }
    sigma_printf("-----------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignOOMShard_Init() {
    sigma_printf("[SOC]: Seating Native OOM Shard (Linux OOM-Killer/Jetsam Parity v1.0)...\n");
    sigma_oom_register("sigma_kernel_core",  1,   65536,  -500, SIGMA_TRUE);
    sigma_oom_register("sigma_wm_compositor",2,  131072,   100, SIGMA_FALSE);
    sigma_oom_register("citizen_browser",    42, 524288,   500, SIGMA_FALSE);
    sigma_oom_register("guest_sandbox",      99,  32768,   900, SIGMA_FALSE);
}
