/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD REGISTRY (v3.0 — SELF-CONTAINED)
 * =========================================================================
 * Design: Zero external includes. All primitives declared inline.
 *         This prevents any recursive preamble loops from the host
 *         toolchain's stdint.h when the file is opened standalone.
 * =========================================================================
 */

#ifndef SOVEREIGN_REGISTRY_H
#define SOVEREIGN_REGISTRY_H

/* ── Inline primitives (no external includes) ─────────────────────────── */
typedef unsigned char      sr_u8;
typedef unsigned int       sr_u32;
typedef unsigned long long sr_u64;
typedef signed   int       sr_err_t;
typedef unsigned long long sr_size_t;
typedef unsigned char      sr_bool;

#define SR_OK    ((sr_err_t) 0)
#define SR_ERROR ((sr_err_t)-1)
#define SR_NULL  ((void*)0)

/* ── Registry limits ──────────────────────────────────────────────────── */
#define MAX_SHARDS     1024
#define SHARD_NAME_MAX   64

/* ── Shard category ───────────────────────────────────────────────────── */
typedef enum {
    SHARD_CAT_CORE,     /* VFS, MM, SCHED — Suites S01–S05  */
    SHARD_CAT_DISTRO,   /* Nix, Arch, Gentoo parity         */
    SHARD_CAT_SECURITY, /* PQC, LSM, Enclaves               */
    SHARD_CAT_PLATFORM, /* Android / Windows / macOS parity */
    SHARD_CAT_TOOL      /* Excel, Python, PowerBI shards    */
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
    sr_u32           version;
    sr_u64           load_timestamp;
} sovereign_shard_t;

/* ── Master registry ──────────────────────────────────────────────────── */
typedef struct {
    sovereign_shard_t shards[MAX_SHARDS];
    sr_u32            shard_count;
    sr_u32            active_count;
    sr_u64            registry_lock;
} sovereign_registry_t;

/* ── Public API ───────────────────────────────────────────────────────── */
void        SovereignRegistry_Init(void);
sr_err_t    SovereignRegistry_Register(const char* name,
                                       shard_category_t cat,
                                       shard_init_fn init);
void        SovereignRegistry_Finalize(void);
void        SovereignRegistry_Audit(void);

#endif /* SOVEREIGN_REGISTRY_H */
