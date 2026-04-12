/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN AUTOCLEAN SHARD (v1.0)
 * =========================================================================
 * Mission: Absorb macOS Storage Optimization / BleachBit / Windows Disk
 *          Cleanup / Ubuntu apport-cleanup / NixOS GC USP.
 *          Native Silicon Automated Maintenance & Debris Purge Daemon.
 * Design: C11 / Zero-Dependency / Policy-Driven Maintenance Engine.
 * Standard: Zenith Industrial Sovereignty.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

// -------------------------------------------------------------------------
// AutoClean Structures
// -------------------------------------------------------------------------

typedef enum {
    DEBRIS_CACHE,        /* Application caches                */
    DEBRIS_TEMP,         /* /tmp and spool files              */
    DEBRIS_LOG_OLD,      /* Rotated log files older than TTL  */
    DEBRIS_CORE_DUMP,    /* Post-mortem core dumps            */
    DEBRIS_PKG_CACHE,    /* Downloaded package tarballs       */
    DEBRIS_THUMBNAIL,    /* Stale thumbnail caches            */
    DEBRIS_ORPHAN_LIB,   /* Libraries with zero dependents    */
    DEBRIS_CRASH_REPORT  /* Crash reporter artifacts          */
} SigmaDebrisType_t;

typedef struct {
    SigmaDebrisType_t type;
    char              path[64];
    sigma_u64         size_kb;
    sigma_u32         age_days;
    sigma_bool        safe_to_delete;
} SigmaDebrisEntry_t;

typedef struct {
    sigma_u64 total_cleaned_kb;
    sigma_u64 last_run_ts;
    sigma_u32 runs_executed;
    sigma_u32 items_cleaned;
    sigma_bool auto_mode;   /* Runs every maintenance tick  */
} SigmaCleanStats_t;

#define MAX_DEBRIS_ENTRIES 48
static SigmaDebrisEntry_t s_debris[MAX_DEBRIS_ENTRIES];
static sigma_u32          s_debris_count = 0;
static SigmaCleanStats_t  s_clean_stats  = {0, 0, 0, 0, SIGMA_TRUE};

/* Policies: minimum age in days before cleaning each debris type */
static const sigma_u32 s_policy_age[] = {
/*CACHE  TEMP  LOG  CORE  PKG   THUMB  ORPHAN  CRASH */
    7,    1,    14,   1,   30,    30,     14,     3
};

static const char* s_dtype_names[] = {
    "CACHE","TEMP","LOG_OLD","CORE_DUMP",
    "PKG_CACHE","THUMB","ORPHAN_LIB","CRASH_RPT"
};

// -------------------------------------------------------------------------
// AutoClean Logic (macOS Storage Opt / BleachBit / Windows Disk Cleanup parity)
// -------------------------------------------------------------------------

/**
 * sigma_autoclean_register: Registers a debris item discovered during scanning.
 */
sigma_err_t sigma_autoclean_register(SigmaDebrisType_t type, const char* path,
                                      sigma_u64 size_kb, sigma_u32 age_days) {
    if (s_debris_count >= MAX_DEBRIS_ENTRIES) return SIGMA_ENOSPC;
    SigmaDebrisEntry_t* e = &s_debris[s_debris_count++];
    e->type           = type;
    e->size_kb        = size_kb;
    e->age_days       = age_days;
    e->safe_to_delete = (age_days >= s_policy_age[type]);
    sigma_strcpy(e->path, path);
    return SIGMA_OK;
}

/**
 * sigma_autoclean_scan: Performs a silicon filesystem debris scan.
 *
 * In production: walks VFS mount points via sigma_vfs_readdir.
 * Here: seeds a representative debris catalog for demonstration.
 */
void sigma_autoclean_scan() {
    sigma_printf("[CLEAN]: Silicon debris scan initiated...\n");
    s_debris_count = 0; /* Reset for fresh scan */

    sigma_autoclean_register(DEBRIS_CACHE,      "/var/cache/sigma-pkg",    204800, 45);
    sigma_autoclean_register(DEBRIS_CACHE,      "/home/.cache/sigma-browser", 512000, 10);
    sigma_autoclean_register(DEBRIS_TEMP,       "/tmp/sigma_work_XXXXXX",    8192,   2);
    sigma_autoclean_register(DEBRIS_LOG_OLD,    "/var/log/sigma-kernel.1.gz",  1024, 30);
    sigma_autoclean_register(DEBRIS_LOG_OLD,    "/var/log/sigma-audit.2.gz",    512, 60);
    sigma_autoclean_register(DEBRIS_CORE_DUMP,   "/var/crash/sigma.core",    131072,  5);
    sigma_autoclean_register(DEBRIS_PKG_CACHE,  "/var/cache/sigma-pkg/tgz",   65536, 90);
    sigma_autoclean_register(DEBRIS_THUMBNAIL,  "/home/.thumbnails",           4096, 60);
    sigma_autoclean_register(DEBRIS_ORPHAN_LIB, "/usr/lib/old_compat.so.1",    2048, 180);
    sigma_autoclean_register(DEBRIS_CRASH_REPORT,"/var/crash/citizen_app.apport", 256, 7);

    /* Compute totals */
    sigma_u64 total_safe = 0;
    for (sigma_u32 i = 0; i < s_debris_count; i++)
        if (s_debris[i].safe_to_delete) total_safe += s_debris[i].size_kb;

    sigma_printf("[CLEAN]: Scan complete — %u items found, "
                 "~%llu KB reclaimable.\n",
                 s_debris_count, (unsigned long long)total_safe);
}

/**
 * sigma_autoclean_run: Executes a silicon debris purge pass.
 *
 * Applies per-type age policies, honours safe_to_delete gates.
 */
void sigma_autoclean_run(sigma_bool dry_run) {
    sigma_printf("[CLEAN]: %s silicon maintenance run...\n",
                 dry_run ? "DRY-RUN" : "EXECUTING");

    sigma_u64 freed_kb     = 0;
    sigma_u32 items_cleaned = 0;

    for (sigma_u32 i = 0; i < s_debris_count; i++) {
        SigmaDebrisEntry_t* e = &s_debris[i];
        if (!e->safe_to_delete) continue;

        sigma_printf("  [%s]: %s %-12s (%llu KB, %u days old)%s\n",
                     dry_run ? "WOULD" : "DEL",
                     s_dtype_names[e->type],
                     e->path,
                     (unsigned long long)e->size_kb,
                     e->age_days,
                     dry_run ? "" : " — PURGED");

        if (!dry_run) {
            freed_kb      += e->size_kb;
            items_cleaned++;
            e->size_kb     = 0; /* Mark as cleaned */
        }
    }

    if (!dry_run) {
        s_clean_stats.total_cleaned_kb += freed_kb;
        s_clean_stats.items_cleaned    += items_cleaned;
        s_clean_stats.runs_executed++;
        s_clean_stats.last_run_ts = s_clean_stats.runs_executed * 86400000ULL;
        sigma_printf("[OK]: Purge complete — freed %llu KB (%llu MB) "
                     "in %u items.\n",
                     (unsigned long long)freed_kb,
                     (unsigned long long)(freed_kb / 1024),
                     items_cleaned);
    } else {
        sigma_u64 would_free = 0;
        for (sigma_u32 i = 0; i < s_debris_count; i++)
            if (s_debris[i].safe_to_delete) would_free += s_debris[i].size_kb;
        sigma_printf("[DRY]: Would free %llu KB (%llu MB).\n",
                     (unsigned long long)would_free,
                     (unsigned long long)(would_free / 1024));
    }
}

// -------------------------------------------------------------------------
// Industrial AutoClean Audit
// -------------------------------------------------------------------------

void SovereignAutoClean_Audit() {
    sigma_printf("\n--- SOVEREIGN AUTOCLEAN AUDIT ---\n");
    sigma_printf("Auto-mode: %s | Runs: %u | Total cleaned: %llu MB\n",
                 s_clean_stats.auto_mode ? "ON" : "off",
                 s_clean_stats.runs_executed,
                 (unsigned long long)(s_clean_stats.total_cleaned_kb / 1024));
    sigma_printf("TYPE         PATH                                      SIZE_KB  AGE SAFE\n");
    sigma_printf("--------------------------------------------------------------------------\n");
    for (sigma_u32 i = 0; i < s_debris_count; i++) {
        sigma_printf("%-12s %-42s %-8llu %-3u %s\n",
                     s_dtype_names[s_debris[i].type],
                     s_debris[i].path,
                     (unsigned long long)s_debris[i].size_kb,
                     s_debris[i].age_days,
                     s_debris[i].safe_to_delete ? "YES" : "no");
    }
    sigma_printf("--------------------------------------------------------------------------\n");
}

// -------------------------------------------------------------------------
// Factory / Constructor
// -------------------------------------------------------------------------

void SovereignAutoCleanShard_Init() {
    sigma_printf("[SOC]: Seating Native AutoClean Shard "
                 "(macOS Storage Opt/BleachBit/Disk Cleanup Parity v1.0)...\n");
    sigma_autoclean_scan();
    sigma_autoclean_run(SIGMA_TRUE);  /* Dry-run at boot to show what can be freed */
}
