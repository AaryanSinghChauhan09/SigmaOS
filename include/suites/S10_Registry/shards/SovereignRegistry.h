/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SHARD REGISTRY (v4.0 — LATTICE CORE)
 * =========================================================================
 * Purpose: Central authority for shard lifecycle management.
 * Design: Powered by SovereignCommon.h. Zero legacy dependencies.
 * =========================================================================
 */

#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

#include "suites/S01_Genesis/shards/SovereignCommon.h"

/* ── Registry limits ──────────────────────────────────────────────────── */
#define MAX_SHARDS       2048
#define SHARD_NAME_MAX   128

/* ── Shard category ───────────────────────────────────────────────────── */
typedef enum {
    SHARD_CAT_CORE,     /* S01–S05: Genesis, Boot, Sched, HAL, MM */
    SHARD_CAT_KERNEL,   /* S06–S10: VFS, Net, LSM, AI, Containers */
    SHARD_CAT_HARDWARE, /* S11–S15: PQC, Distro, GPU, USB, Audio */
    SHARD_CAT_UNIVERSAL,/* S16–S33: Terminal, GUI, IDE, WASM, etc.*/
} shard_category_t;

/* ── Shard lifecycle ──────────────────────────────────────────────────── */
typedef enum {
    SHARD_STATUS_REGISTERED,
    SHARD_STATUS_INITIALIZING,
    SHARD_STATUS_ACTIVE,
    SHARD_STATUS_FAILED,
    SHARD_STATUS_ZOMBIE
} shard_status_t;

/* ── Shard init callback ──────────────────────────────────────────────── */
typedef void (*shard_init_fn)(void);

/* ── Shard descriptor ─────────────────────────────────────────────────── */
typedef struct {
    char             name[SHARD_NAME_MAX];
    shard_category_t category;
    shard_status_t   status;
    shard_init_fn    init;
    sigma_u32        version;
    sigma_u64        load_timestamp;
    sigma_u32        dependencies[8];  /* shard IDs */
} sovereign_shard_t;

/* ── Master registry ──────────────────────────────────────────────────── */
typedef struct {
    sovereign_shard_t shards[MAX_SHARDS];
    sigma_u32         shard_count;
    sigma_u32         active_count;
    sigma_u64         registry_lock;
} sovereign_registry_t;

/* ── Public API ───────────────────────────────────────────────────────── */
void        SovereignRegistry_Init(void);
sigma_err_t SovereignRegistry_Register(const char* name,
                                       shard_category_t cat,
                                       shard_init_fn init);
void        SovereignRegistry_Finalize(void);
void        SovereignRegistry_Audit(void);

#endif /* SOVEREIGN_REGISTRY_H */
